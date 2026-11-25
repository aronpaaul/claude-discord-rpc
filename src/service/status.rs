use crate::claude::{IsoTime, ProcessProbe};
use crate::config::AppPaths;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct ServiceStatus {
    pub pid: u32,
    pub startedAt: i64,
    pub heartbeat: i64,
    pub rpcEnabled: bool,
    pub discordConnected: bool,
    pub sessionName: String,
    pub details: String,
    pub state: String,
    pub presenceVisible: bool,
    pub lastError: String,
}

impl ServiceStatus {
    pub fn read() -> Option<Self> {
        let text = std::fs::read_to_string(AppPaths::statusFile()).ok()?;
        serde_json::from_str(&text).ok()
    }

    pub fn write(&self) {
        let Ok(text) = serde_json::to_string_pretty(self) else {
            return;
        };
        let target = AppPaths::statusFile();
        let temporary = target.with_extension("json.tmp");
        if std::fs::write(&temporary, text).is_ok() {
            let _ = std::fs::rename(&temporary, &target);
        }
    }

    pub fn isAlive(&self) -> bool {
        let freshMillis = 20_000;
        ProcessProbe::isAlive(self.pid) && IsoTime::nowMillis() - self.heartbeat < freshMillis
    }

    pub fn current() -> Option<Self> {
        Self::read().filter(Self::isAlive)
    }
}
