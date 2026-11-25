use super::buttons::ButtonCollector;
use super::payload::PresencePayload;
use super::template::TemplateRenderer;
use super::timer::TimerResolver;
use crate::claude::ClaudeSnapshot;
use crate::config::Settings;

pub struct NewChatPresence;

impl NewChatPresence {
    pub fn applies(settings: &Settings, snapshot: &ClaudeSnapshot) -> bool {
        settings.newChat.enabled && snapshot.prompts <= settings.newChat.maxPrompts
    }

    pub fn build(settings: &Settings, snapshot: &ClaudeSnapshot, variables: &[(&'static str, String)], serviceStartMillis: i64) -> PresencePayload {
        let chat = &settings.newChat;
        let render = |template: &str| TemplateRenderer::render(template, variables, settings.maxLineLength as usize);
        let pick = |own: &str, common: &str| if own.trim().is_empty() { common.trim().to_string() } else { own.trim().to_string() };
        PresencePayload {
            name: render(&pick(&chat.activityName, &settings.activityName)),
            details: render(&chat.detailsTemplate),
            state: render(&chat.stateTemplate),
            largeImage: pick(&chat.largeImage, &settings.largeImage),
            largeText: render(&pick(&chat.largeTextTemplate, &settings.largeTextTemplate)),
            smallImage: chat.smallImage.trim().to_string(),
            smallText: render(&chat.smallTextTemplate),
            startSeconds: TimerResolver::startSeconds(chat.timerMode, snapshot, serviceStartMillis),
            buttons: ButtonCollector::collect(settings),
            activityKind: chat.activityKind,
            statusField: settings.statusField,
        }
    }
}
