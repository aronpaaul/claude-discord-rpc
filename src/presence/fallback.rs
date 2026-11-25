use super::buttons::ButtonCollector;
use super::payload::PresencePayload;
use super::template::TemplateRenderer;
use crate::config::{Settings, TimerMode};

pub struct NoSessionPresence;

impl NoSessionPresence {
    pub fn build(settings: &Settings, serviceStartMillis: i64) -> Option<PresencePayload> {
        if settings.clearWhenNoSession {
            return None;
        }
        let limit = settings.maxLineLength as usize;
        let startSeconds = match settings.timerMode {
            TimerMode::Hidden => None,
            _ => Some(serviceStartMillis / 1000),
        };
        Some(PresencePayload {
            details: TemplateRenderer::render(&settings.noSessionText, &[], limit),
            state: String::new(),
            largeImage: settings.largeImage.trim().to_string(),
            largeText: "Claude Code".to_string(),
            smallImage: if settings.showSmallImage { settings.smallImageIdle.trim().to_string() } else { String::new() },
            smallText: String::new(),
            startSeconds,
            buttons: ButtonCollector::collect(settings),
            activityKind: settings.activityKind,
            statusField: settings.statusField,
        })
    }
}
