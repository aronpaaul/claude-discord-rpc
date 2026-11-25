use super::options::{ActivityKind, Currency, ModelStyle, SessionPick, StatusField, TimerMode, TokenStyle};
use super::settings::Settings;

const claudeIcon: &str = "https://claude.ai/images/claude_app_icon.png";
const busyIcon: &str = "https://cdn.jsdelivr.net/gh/jdecked/twemoji@15.1.0/assets/72x72/26a1.png";
const idleIcon: &str = "https://cdn.jsdelivr.net/gh/jdecked/twemoji@15.1.0/assets/72x72/1f4a4.png";

impl Default for Settings {
    fn default() -> Self {
        Self {
            rpcEnabled: true,
            clientId: String::new(),
            updateIntervalSecs: 5,
            sessionPick: SessionPick::BusyFirst,
            sessionNameFilter: String::new(),
            desktopOnly: false,
            activityKind: ActivityKind::Playing,
            statusField: StatusField::Name,
            activityName: "Claude Code".to_string(),
            busyLabel: "Работает".to_string(),
            idleLabel: "Ожидает".to_string(),
            detailsTemplate: "{session}".to_string(),
            stateTemplate: "{status} · {tokens} токенов".to_string(),
            idleDetailsTemplate: "{session}".to_string(),
            idleStateTemplate: "{status} · {cost}".to_string(),
            timerMode: TimerMode::Task,
            largeImage: claudeIcon.to_string(),
            largeTextTemplate: "{model} · {project}".to_string(),
            showSmallImage: true,
            smallImageBusy: busyIcon.to_string(),
            smallImageIdle: idleIcon.to_string(),
            smallTextTemplate: "{prompts} запросов · {tools} инструментов".to_string(),
            firstButtonEnabled: false,
            firstButtonLabel: "Claude Code".to_string(),
            firstButtonUrl: "https://claude.com/claude-code".to_string(),
            secondButtonEnabled: false,
            secondButtonLabel: "GitHub".to_string(),
            secondButtonUrl: "https://github.com/".to_string(),
            tokenStyle: TokenStyle::Compact,
            includeCacheTokens: false,
            currency: Currency::Usd,
            usdToRubRate: 80.0,
            usdToEurRate: 0.86,
            modelStyle: ModelStyle::Pretty,
            hideProjectName: false,
            projectAlias: "секретный проект".to_string(),
            hideSessionName: false,
            sessionAlias: "приватная сессия".to_string(),
            clearWhenNoSession: true,
            noSessionText: "Нет открытых сессий".to_string(),
            idleClearMinutes: 0,
            maxLineLength: 128,
            autostart: true,
        }
    }
}
