use std::path::PathBuf;

pub struct AppPaths;

impl AppPaths {
    pub fn dataDir() -> PathBuf {
        let directory = Self::userProfile().join(".claude-discord-rpc");
        let _ = std::fs::create_dir_all(&directory);
        directory
    }

    fn userProfile() -> PathBuf {
        PathBuf::from(std::env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string()))
    }

    pub fn settingsFile() -> PathBuf {
        Self::dataDir().join("settings.json")
    }

    pub fn statusFile() -> PathBuf {
        Self::dataDir().join("status.json")
    }

    pub fn claudeHome() -> PathBuf {
        Self::userProfile().join(".claude")
    }

    pub fn liveSessionsDir() -> PathBuf {
        Self::claudeHome().join("sessions")
    }

    pub fn projectsDir() -> PathBuf {
        Self::claudeHome().join("projects")
    }
}
