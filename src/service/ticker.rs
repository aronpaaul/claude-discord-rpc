use super::status::ServiceStatus;
use crate::claude::{ClaudeMonitor, IsoTime};
use crate::config::Settings;
use crate::presence::{DiscordLink, PresenceBuilder};

pub struct ServiceTicker {
    monitor: ClaudeMonitor,
    link: DiscordLink,
    startedAt: i64,
}

impl ServiceTicker {
    pub fn new() -> Self {
        Self {
            monitor: ClaudeMonitor::default(),
            link: DiscordLink::default(),
            startedAt: IsoTime::nowMillis(),
        }
    }

    pub fn tick(&mut self, settings: &Settings) -> ServiceStatus {
        let now = IsoTime::nowMillis();
        let snapshot = self.monitor.poll(settings);
        let payload = PresenceBuilder::build(settings, snapshot.as_ref(), self.startedAt, now);
        if settings.rpcEnabled {
            self.link.publish(&settings.clientId, payload.as_ref());
        } else {
            self.link.disconnect();
            self.link.lastError = None;
        }
        let visible = settings.rpcEnabled && self.link.isConnected() && payload.is_some();
        ServiceStatus {
            pid: std::process::id(),
            startedAt: self.startedAt,
            heartbeat: now,
            rpcEnabled: settings.rpcEnabled,
            discordConnected: self.link.isConnected(),
            sessionName: snapshot.map(|current| current.sessionName).unwrap_or_default(),
            details: payload.as_ref().map(|content| content.details.clone()).unwrap_or_default(),
            state: payload.as_ref().map(|content| content.state.clone()).unwrap_or_default(),
            presenceVisible: visible,
            lastError: self.link.lastError.clone().unwrap_or_default(),
        }
    }
}
