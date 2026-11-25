use super::launcher::Launcher;
use super::singleton::SingleInstance;
use super::ticker::ServiceTicker;
use super::watcher::SettingsWatcher;
use std::time::Duration;

pub struct Worker;

impl Worker {
    pub fn run() {
        let mutexName = format!("Local\\{}", Launcher::serviceName);
        let Some(instance) = SingleInstance::acquire(&mutexName) else {
            return;
        };
        std::mem::forget(instance);
        let mut watcher = SettingsWatcher::new();
        let mut ticker = ServiceTicker::new();
        loop {
            watcher.refresh();
            let status = ticker.tick(&watcher.settings);
            status.write();
            let interval = watcher.settings.updateIntervalSecs.clamp(2, 120);
            Self::sleepWatching(&mut watcher, interval);
        }
    }

    fn sleepWatching(watcher: &mut SettingsWatcher, seconds: u32) {
        let wasEnabled = watcher.settings.rpcEnabled;
        let steps = seconds * 4;
        for _ in 0..steps {
            std::thread::sleep(Duration::from_millis(250));
            let before = watcher.settings.clone();
            watcher.refresh();
            if watcher.settings != before || watcher.settings.rpcEnabled != wasEnabled {
                return;
            }
        }
    }
}
