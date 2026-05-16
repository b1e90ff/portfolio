use std::sync::Arc;

use crate::config::Settings;
use crate::email::Mailer;
use crate::i18n::I18n;
use crate::rate_limit::{RateLimitConfig, RateLimiter};

#[derive(Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,
    pub i18n: Arc<I18n>,
    pub mailer: Option<Arc<Mailer>>,
    pub contact_rate_limit: Arc<RateLimiter>,
}

impl AppState {
    pub fn new(settings: Settings, i18n: I18n) -> Self {
        let mailer = settings.smtp.as_ref().and_then(|s| match Mailer::new(s) {
            Ok(m) => Some(Arc::new(m)),
            Err(err) => {
                tracing::error!(?err, "smtp transport init failed; contact form disabled");
                None
            }
        });

        Self {
            settings: Arc::new(settings),
            i18n: Arc::new(i18n),
            mailer,
            contact_rate_limit: Arc::new(RateLimiter::new(RateLimitConfig::CONTACT_DEFAULT)),
        }
    }
}
