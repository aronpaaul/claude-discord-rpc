use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct LiveSession {
    pub pid: u32,
    pub sessionId: String,
    pub cwd: String,
    pub startedAt: i64,
    pub version: String,
    pub entrypoint: String,
    pub name: Option<String>,
    pub status: Option<String>,
    pub updatedAt: i64,
    pub statusUpdatedAt: i64,
}

impl LiveSession {
    pub fn isBusy(&self) -> bool {
        self.status.as_deref() == Some("busy")
    }

    pub fn isDesktop(&self) -> bool {
        self.entrypoint == "claude-desktop"
    }

    pub fn displayName(&self) -> String {
        match self.name.as_deref().map(str::trim) {
            Some(text) if !text.is_empty() => text.to_string(),
            _ => "Новая сессия".to_string(),
        }
    }

    pub fn projectName(&self) -> String {
        let separators = ['\\', '/'];
        let trimmed = self.cwd.trim_end_matches(separators);
        let last = trimmed.rsplit(separators).next().unwrap_or("");
        if last.is_empty() || last.ends_with(':') {
            trimmed.to_string()
        } else {
            last.to_string()
        }
    }
}
