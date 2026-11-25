use super::paths::AppPaths;
use super::settings::Settings;
use std::time::SystemTime;

pub struct SettingsStore;

impl SettingsStore {
    pub fn load() -> Settings {
        std::fs::read_to_string(AppPaths::settingsFile())
            .ok()
            .and_then(|text| serde_json::from_str(&text).ok())
            .unwrap_or_default()
    }

    pub fn save(settings: &Settings) {
        let Ok(text) = serde_json::to_string_pretty(settings) else {
            return;
        };
        let target = AppPaths::settingsFile();
        let temporary = target.with_extension("json.tmp");
        if std::fs::write(&temporary, text).is_ok() {
            let _ = std::fs::rename(&temporary, &target);
        }
    }

    pub fn modifiedAt() -> Option<SystemTime> {
        std::fs::metadata(AppPaths::settingsFile()).and_then(|meta| meta.modified()).ok()
    }

    pub fn ensureExists() {
        if Self::modifiedAt().is_none() {
            Self::save(&Settings::default());
        }
    }
}
