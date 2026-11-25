use crate::claude::{ClaudeMonitor, ClaudeSnapshot, IsoTime};
use crate::config::{Settings, SettingsStore};
use crate::presence::{PresenceBuilder, PresencePayload};
use crate::service::{AutoStart, ServiceStatus};
use std::time::{Duration, Instant};

pub struct AppState {
    pub settings: Settings,
    saved: Settings,
    pub status: Option<ServiceStatus>,
    pub snapshot: Option<ClaudeSnapshot>,
    pub preview: Option<PresencePayload>,
    monitor: ClaudeMonitor,
    lastRefresh: Option<Instant>,
    openedAt: i64,
}

impl AppState {
    pub fn new(settings: Settings) -> Self {
        Self {
            saved: settings.clone(),
            settings,
            status: None,
            snapshot: None,
            preview: None,
            monitor: ClaudeMonitor::default(),
            lastRefresh: None,
            openedAt: IsoTime::nowMillis(),
        }
    }

    pub fn refreshIfDue(&mut self) {
        let isDue = self.lastRefresh.is_none_or(|moment| moment.elapsed() >= Duration::from_secs(1));
        if isDue {
            self.lastRefresh = Some(Instant::now());
            self.status = ServiceStatus::current();
            self.snapshot = self.monitor.poll(&self.settings);
            self.rebuildPreview();
        }
    }

    pub fn persistIfChanged(&mut self) {
        if self.settings == self.saved {
            return;
        }
        if self.settings.autostart != self.saved.autostart {
            AutoStart::apply(self.settings.autostart);
        }
        SettingsStore::save(&self.settings);
        self.saved = self.settings.clone();
        self.rebuildPreview();
    }

    pub fn resetToDefaults(&mut self) {
        let clientId = self.settings.clientId.clone();
        self.settings = Settings { clientId, ..Settings::default() };
    }

    pub fn forceRefresh(&mut self) {
        self.lastRefresh = None;
    }

    fn rebuildPreview(&mut self) {
        let start = self.status.as_ref().map(|status| status.startedAt).unwrap_or(self.openedAt);
        self.preview = PresenceBuilder::build(&self.settings, self.snapshot.as_ref(), start, IsoTime::nowMillis());
    }
}
