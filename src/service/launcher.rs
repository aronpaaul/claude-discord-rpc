use super::status::ServiceStatus;
use crate::claude::ProcessProbe;
use std::os::windows::process::CommandExt;
use std::process::{Command, Stdio};

pub struct Launcher;

const detachedProcess: u32 = 0x0000_0008;
const newProcessGroup: u32 = 0x0000_0200;
const createNoWindow: u32 = 0x0800_0000;

impl Launcher {
    pub const serviceFlag: &'static str = "--service";
    pub const serviceName: &'static str = "claude-discord-rpc";

    pub fn exePath() -> String {
        std::env::current_exe().map(|path| path.display().to_string()).unwrap_or_default()
    }

    pub fn isRunning() -> bool {
        ServiceStatus::current().is_some()
    }

    pub fn ensureRunning() -> bool {
        if Self::isRunning() {
            return true;
        }
        Command::new(Self::exePath())
            .arg(Self::serviceFlag)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .creation_flags(detachedProcess | newProcessGroup | createNoWindow)
            .spawn()
            .is_ok()
    }

    pub fn stop() {
        if let Some(status) = ServiceStatus::read() {
            if ProcessProbe::isAlive(status.pid) {
                ProcessProbe::terminate(status.pid);
                Self::waitForExit(status.pid);
            }
            ServiceStatus { heartbeat: 0, ..status }.write();
        }
    }

    fn waitForExit(pid: u32) {
        for _ in 0..40 {
            if !ProcessProbe::isAlive(pid) {
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }
}
