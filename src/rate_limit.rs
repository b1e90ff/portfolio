use std::net::{IpAddr, Ipv6Addr};
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;

use governor::clock::{Clock, DefaultClock};
use governor::middleware::NoOpMiddleware;
use governor::state::keyed::DashMapStateStore;
use governor::{Quota, RateLimiter as KeyedLimiter};
use tokio::time::{MissedTickBehavior, interval};

pub const HOUSEKEEPING_INTERVAL: Duration = Duration::from_secs(60);

#[derive(Clone, Copy, Debug)]
pub struct RateLimitConfig {
    pub max: NonZeroU32,
    pub window: Duration,
}

impl RateLimitConfig {
    pub const CONTACT_DEFAULT: Self = Self {
        max: NonZeroU32::new(5).unwrap(),
        window: Duration::from_secs(15 * 60),
    };

    fn quota(self) -> Quota {
        let replenish = self.window / self.max.get();
        Quota::with_period(replenish)
            .expect("rate limit window must be non-zero")
            .allow_burst(self.max)
    }
}

pub struct RateLimiter<C: Clock = DefaultClock> {
    inner: KeyedLimiter<IpAddr, DashMapStateStore<IpAddr>, C, NoOpMiddleware<C::Instant>>,
}

impl RateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        Self::with_clock(config, DefaultClock::default())
    }
}

impl<C: Clock> RateLimiter<C> {
    fn with_clock(config: RateLimitConfig, clock: C) -> Self {
        Self {
            inner: KeyedLimiter::dashmap_with_clock(config.quota(), clock),
        }
    }

    pub fn check(&self, ip: IpAddr) -> bool {
        self.inner.check_key(&client_key(ip)).is_ok()
    }

    /// Drops clients whose quota has fully replenished and releases the freed capacity.
    pub fn housekeeping(&self) {
        self.inner.retain_recent();
        self.inner.shrink_to_fit();
    }

    #[cfg(test)]
    fn tracked_clients(&self) -> usize {
        self.inner.len()
    }
}

pub fn spawn_housekeeping(limiter: Arc<RateLimiter>, every: Duration) {
    tokio::spawn(async move {
        let mut ticker = interval(every);
        ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);
        loop {
            ticker.tick().await;
            limiter.housekeeping();
        }
    });
}

/// IPv6 clients are keyed by their /64, which a single subscriber usually controls.
fn client_key(ip: IpAddr) -> IpAddr {
    match ip {
        IpAddr::V4(_) => ip,
        IpAddr::V6(v6) => match v6.to_ipv4_mapped() {
            Some(v4) => IpAddr::V4(v4),
            None => {
                let prefix = u128::from(v6) & !((1u128 << 64) - 1);
                IpAddr::V6(Ipv6Addr::from(prefix))
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use governor::clock::FakeRelativeClock;
    use std::net::Ipv4Addr;

    fn limiter(max: u32, window_secs: u64) -> (RateLimiter<FakeRelativeClock>, FakeRelativeClock) {
        let clock = FakeRelativeClock::default();
        let config = RateLimitConfig {
            max: NonZeroU32::new(max).unwrap(),
            window: Duration::from_secs(window_secs),
        };
        (RateLimiter::with_clock(config, clock.clone()), clock)
    }

    #[test]
    fn allows_burst_up_to_max_then_blocks() {
        let (rl, _) = limiter(3, 60);
        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        assert!(rl.check(ip));
        assert!(rl.check(ip));
        assert!(rl.check(ip));
        assert!(!rl.check(ip));
    }

    #[test]
    fn replenishes_one_request_per_window_fraction() {
        let (rl, clock) = limiter(2, 60);
        let ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1));
        assert!(rl.check(ip));
        assert!(rl.check(ip));
        assert!(!rl.check(ip));

        clock.advance(Duration::from_secs(30));
        assert!(rl.check(ip));
        assert!(!rl.check(ip));

        clock.advance(Duration::from_secs(60));
        assert!(rl.check(ip));
        assert!(rl.check(ip));
        assert!(!rl.check(ip));
    }

    #[test]
    fn separate_ips_have_separate_buckets() {
        let (rl, _) = limiter(1, 60);
        let a = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));
        let b = IpAddr::V4(Ipv4Addr::new(2, 2, 2, 2));
        assert!(rl.check(a));
        assert!(!rl.check(a));
        assert!(rl.check(b));
    }

    #[test]
    fn ipv6_clients_share_their_slash_64() {
        let (rl, _) = limiter(1, 60);
        let a: IpAddr = "2001:db8:1:2::1".parse().unwrap();
        let same_prefix: IpAddr = "2001:db8:1:2:ffff::9".parse().unwrap();
        let other_prefix: IpAddr = "2001:db8:1:3::1".parse().unwrap();
        assert!(rl.check(a));
        assert!(!rl.check(same_prefix));
        assert!(rl.check(other_prefix));
    }

    #[test]
    fn ipv4_mapped_ipv6_shares_bucket_with_ipv4() {
        let (rl, _) = limiter(1, 60);
        let v4 = IpAddr::V4(Ipv4Addr::new(192, 0, 2, 7));
        let mapped: IpAddr = "::ffff:192.0.2.7".parse().unwrap();
        assert!(rl.check(v4));
        assert!(!rl.check(mapped));
    }

    #[test]
    fn housekeeping_forgets_fully_replenished_clients() {
        let (rl, clock) = limiter(2, 60);
        let idle = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1));
        let busy = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 2));
        assert!(rl.check(idle));
        clock.advance(Duration::from_secs(90));
        assert!(rl.check(busy));
        assert_eq!(rl.tracked_clients(), 2);

        rl.housekeeping();
        assert_eq!(rl.tracked_clients(), 1);
        assert!(rl.check(busy));
        assert!(!rl.check(busy));
    }

    #[test]
    fn contact_default_allows_five_per_quarter_hour() {
        let quota = RateLimitConfig::CONTACT_DEFAULT.quota();
        assert_eq!(quota.burst_size().get(), 5);
        assert_eq!(quota.replenish_interval(), Duration::from_secs(180));
    }
}
