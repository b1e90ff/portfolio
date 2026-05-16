use std::sync::Arc;

use crate::config::Settings;
use crate::i18n::I18n;

#[derive(Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,
    pub i18n: Arc<I18n>,
}

impl AppState {
    pub fn new(settings: Settings, i18n: I18n) -> Self {
        Self {
            settings: Arc::new(settings),
            i18n: Arc::new(i18n),
        }
    }
}
