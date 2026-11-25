use std::path::PathBuf;

pub struct AppPaths;

impl AppPaths {
    pub fn dataDir() -> PathBuf {
        let appData = std::env::var("APPDATA").unwrap_or_else(|_| ".".to_string());
        let directory = PathBuf::from(appData).join("claude-discord-rpc");
        let _ = std::fs::create_dir_all(&directory);
        directory
    }

    pub fn settingsFile() -> PathBuf {
        Self::dataDir().join("settings.json")
    }

    pub fn statusFile() -> PathBuf {
        Self::dataDir().join("status.json")
    }

    pub fn claudeHome() -> PathBuf {
        let profile = std::env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(profile).join(".claude")
    }

    pub fn liveSessionsDir() -> PathBuf {
        Self::claudeHome().join("sessions")
    }

    pub fn projectsDir() -> PathBuf {
        Self::claudeHome().join("projects")
    }
}
