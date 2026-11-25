use super::buttons::ButtonCollector;
use super::fallback::NoSessionPresence;
use super::payload::PresencePayload;
use super::template::TemplateRenderer;
use super::variables::VariableSet;
use crate::claude::ClaudeSnapshot;
use crate::config::{Settings, TimerMode};

pub struct PresenceBuilder;

impl PresenceBuilder {
    pub fn build(settings: &Settings, snapshot: Option<&ClaudeSnapshot>, serviceStartMillis: i64, nowMillis: i64) -> Option<PresencePayload> {
        let Some(snapshot) = snapshot else {
            return NoSessionPresence::build(settings, serviceStartMillis);
        };
        if Self::idleExpired(settings, snapshot, nowMillis) {
            return None;
        }
        let variables = VariableSet::build(snapshot, settings, nowMillis);
        let limit = settings.maxLineLength as usize;
        let render = |template: &str| TemplateRenderer::render(template, &variables, limit);
        let (detailsTemplate, stateTemplate) = if snapshot.isBusy {
            (&settings.detailsTemplate, &settings.stateTemplate)
        } else {
            (&settings.idleDetailsTemplate, &settings.idleStateTemplate)
        };
        Some(PresencePayload {
            name: render(&settings.activityName),
            details: render(detailsTemplate),
            state: render(stateTemplate),
            largeImage: settings.largeImage.trim().to_string(),
            largeText: render(&settings.largeTextTemplate),
            smallImage: Self::smallImage(settings, snapshot),
            smallText: if settings.showSmallImage { render(&settings.smallTextTemplate) } else { String::new() },
            startSeconds: Self::startMillis(settings, snapshot, serviceStartMillis).map(|millis| millis / 1000),
            buttons: ButtonCollector::collect(settings),
            activityKind: settings.activityKind,
            statusField: settings.statusField,
        })
    }

    fn idleExpired(settings: &Settings, snapshot: &ClaudeSnapshot, nowMillis: i64) -> bool {
        let limitMillis = settings.idleClearMinutes as i64 * 60_000;
        !snapshot.isBusy && limitMillis > 0 && nowMillis - snapshot.statusSinceMillis > limitMillis
    }

    fn smallImage(settings: &Settings, snapshot: &ClaudeSnapshot) -> String {
        if !settings.showSmallImage {
            return String::new();
        }
        let image = if snapshot.isBusy { &settings.smallImageBusy } else { &settings.smallImageIdle };
        image.trim().to_string()
    }

    fn startMillis(settings: &Settings, snapshot: &ClaudeSnapshot, serviceStartMillis: i64) -> Option<i64> {
        match settings.timerMode {
            TimerMode::Hidden => None,
            TimerMode::Session => Some(snapshot.sessionStartMillis),
            TimerMode::Task => Some(snapshot.statusSinceMillis),
            TimerMode::ServiceStart => Some(serviceStartMillis),
        }
    }
}
