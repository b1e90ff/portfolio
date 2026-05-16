use std::collections::HashMap;
use std::net::IpAddr;
use std::time::{Duration, Instant};

use parking_lot::Mutex;

#[derive(Clone, Copy, Debug)]
pub struct RateLimitConfig {
    pub max: u32,
    pub window: Duration,
}

impl RateLimitConfig {
    pub const CONTACT_DEFAULT: Self = Self {
        max: 5,
        window: Duration::from_secs(15 * 60),
    };
}

#[derive(Debug)]
struct Bucket {
    count: u32,
    reset_at: Instant,
}

pub struct RateLimiter {
    config: RateLimitConfig,
    buckets: Mutex<HashMap<IpAddr, Bucket>>,
}

impl RateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            buckets: Mutex::new(HashMap::new()),
        }
    }

    pub fn check(&self, ip: IpAddr) -> bool {
        self.check_at(ip, Instant::now())
    }

    fn check_at(&self, ip: IpAddr, now: Instant) -> bool {
        let mut buckets = self.buckets.lock();
        if buckets.len() > 4096 {
            buckets.retain(|_, b| b.reset_at > now);
        }

        let bucket = buckets.entry(ip).or_insert(Bucket {
            count: 0,
            reset_at: now + self.config.window,
        });

        if now >= bucket.reset_at {
            bucket.count = 0;
            bucket.reset_at = now + self.config.window;
        }

        bucket.count = bucket.count.saturating_add(1);
        bucket.count <= self.config.max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn allows_up_to_max_then_blocks() {
        let rl = RateLimiter::new(RateLimitConfig {
            max: 3,
            window: Duration::from_secs(60),
        });
        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        assert!(rl.check(ip));
        assert!(rl.check(ip));
        assert!(rl.check(ip));
        assert!(!rl.check(ip));
    }

    #[test]
    fn resets_after_window() {
        let rl = RateLimiter::new(RateLimitConfig {
            max: 2,
            window: Duration::from_secs(60),
        });
        let ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1));
        let t0 = Instant::now();
        assert!(rl.check_at(ip, t0));
        assert!(rl.check_at(ip, t0));
        assert!(!rl.check_at(ip, t0));
        let later = t0 + Duration::from_secs(61);
        assert!(rl.check_at(ip, later));
    }

    #[test]
    fn separate_ips_have_separate_buckets() {
        let rl = RateLimiter::new(RateLimitConfig {
            max: 1,
            window: Duration::from_secs(60),
        });
        let a = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));
        let b = IpAddr::V4(Ipv4Addr::new(2, 2, 2, 2));
        assert!(rl.check(a));
        assert!(!rl.check(a));
        assert!(rl.check(b));
    }
}
