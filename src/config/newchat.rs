use super::options::{ActivityKind, TimerMode};
use serde::{Deserialize, Serialize};

const sparkleIcon: &str = "https://cdn.jsdelivr.net/gh/jdecked/twemoji@15.1.0/assets/72x72/2728.png";

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(default)]
pub struct NewChatSettings {
    pub enabled: bool,
    pub maxPrompts: u32,
    pub activityKind: ActivityKind,
    pub timerMode: TimerMode,
    pub activityName: String,
    pub detailsTemplate: String,
    pub stateTemplate: String,
    pub largeImage: String,
    pub largeTextTemplate: String,
    pub smallImage: String,
    pub smallTextTemplate: String,
}

impl Default for NewChatSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            maxPrompts: 1,
            activityKind: ActivityKind::Watching,
            timerMode: TimerMode::Session,
            activityName: String::new(),
            detailsTemplate: "Начинает новый чат".to_string(),
            stateTemplate: "{project} · {model}".to_string(),
            largeImage: String::new(),
            largeTextTemplate: String::new(),
            smallImage: sparkleIcon.to_string(),
            smallTextTemplate: "Новый чат".to_string(),
        }
    }
}
