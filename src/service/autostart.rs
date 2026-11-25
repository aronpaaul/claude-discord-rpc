use super::launcher::Launcher;
use std::os::windows::process::CommandExt;
use std::process::Command;

pub struct AutoStart;

const runKey: &str = r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run";
const valueName: &str = "claude-discord-rpc";
const createNoWindow: u32 = 0x0800_0000;

impl AutoStart {
    pub fn apply(enabled: bool) {
        if enabled == Self::isEnabled() && (!enabled || Self::pointsToCurrentExe()) {
            return;
        }
        if enabled {
            let command = format!("\"{}\" {}", Launcher::exePath(), Launcher::serviceFlag);
            Self::reg(&["add", runKey, "/v", valueName, "/t", "REG_SZ", "/d", &command, "/f"]);
        } else {
            Self::reg(&["delete", runKey, "/v", valueName, "/f"]);
        }
    }

    pub fn isEnabled() -> bool {
        Self::query().is_some()
    }

    fn pointsToCurrentExe() -> bool {
        Self::query().is_some_and(|value| value.to_lowercase().contains(&Launcher::exePath().to_lowercase()))
    }

    fn query() -> Option<String> {
        let output = Command::new("reg")
            .args(["query", runKey, "/v", valueName])
            .creation_flags(createNoWindow)
            .output()
            .ok()?;
        output.status.success().then(|| String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn reg(arguments: &[&str]) {
        let _ = Command::new("reg").args(arguments).creation_flags(createNoWindow).output();
    }
}
