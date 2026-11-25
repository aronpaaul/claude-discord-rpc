use crate::config::{Settings, SettingsStore};
use std::time::SystemTime;

pub struct SettingsWatcher {
    pub settings: Settings,
    modifiedAt: Option<SystemTime>,
}

impl SettingsWatcher {
    pub fn new() -> Self {
        SettingsStore::ensureExists();
        Self {
            settings: SettingsStore::load(),
            modifiedAt: SettingsStore::modifiedAt(),
        }
    }

    pub fn refresh(&mut self) {
        let current = SettingsStore::modifiedAt();
        if current != self.modifiedAt {
            self.modifiedAt = current;
            self.settings = SettingsStore::load();
        }
    }
}
