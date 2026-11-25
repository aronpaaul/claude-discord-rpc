use super::format::ValueFormatter;
use super::modelname::ModelNamer;
use crate::claude::ClaudeSnapshot;
use crate::config::Settings;

pub struct VariableSet;

impl VariableSet {
    pub fn catalog() -> &'static [(&'static str, &'static str)] {
        &[
            ("{session}", "название текущей сессии"),
            ("{project}", "папка проекта"),
            ("{branch}", "git-ветка"),
            ("{model}", "модель Claude"),
            ("{status}", "статус: работает / ожидает"),
            ("{tokens}", "всего токенов в сессии"),
            ("{input}", "входные токены"),
            ("{output}", "выходные токены"),
            ("{cacheRead}", "прочитано из кэша"),
            ("{cacheWrite}", "записано в кэш"),
            ("{cost}", "примерная стоимость сессии"),
            ("{prompts}", "сколько запросов отправлено"),
            ("{tools}", "вызовов инструментов"),
            ("{sessionTime}", "длительность сессии"),
            ("{taskTime}", "сколько длится текущий статус"),
            ("{sessions}", "открыто сессий"),
            ("{busy}", "сессий в работе"),
            ("{version}", "версия Claude Code"),
        ]
    }

    pub fn build(snapshot: &ClaudeSnapshot, settings: &Settings, nowMillis: i64) -> Vec<(&'static str, String)> {
        let tokens = |count: u64| ValueFormatter::tokens(count, settings.tokenStyle);
        let usage = &snapshot.usage;
        vec![
            ("session", Self::sessionName(snapshot, settings)),
            ("project", Self::projectName(snapshot, settings)),
            ("branch", snapshot.branch.clone()),
            ("model", ModelNamer::format(&snapshot.model, settings.modelStyle)),
            ("status", Self::status(snapshot, settings)),
            ("tokens", tokens(usage.total(settings.includeCacheTokens))),
            ("input", tokens(usage.input)),
            ("output", tokens(usage.output)),
            ("cacheRead", tokens(usage.cacheRead)),
            ("cacheWrite", tokens(usage.cacheWrite)),
            ("cost", ValueFormatter::cost(snapshot.costUsd, settings)),
            ("prompts", snapshot.prompts.to_string()),
            ("tools", snapshot.toolCalls.to_string()),
            ("sessionTime", ValueFormatter::duration(nowMillis - snapshot.sessionStartMillis)),
            ("taskTime", ValueFormatter::duration(nowMillis - snapshot.statusSinceMillis)),
            ("sessions", snapshot.liveSessions.to_string()),
            ("busy", snapshot.busySessions.to_string()),
            ("version", snapshot.version.clone()),
        ]
    }

    fn sessionName(snapshot: &ClaudeSnapshot, settings: &Settings) -> String {
        if settings.hideSessionName {
            settings.sessionAlias.clone()
        } else {
            snapshot.sessionName.clone()
        }
    }

    fn projectName(snapshot: &ClaudeSnapshot, settings: &Settings) -> String {
        if settings.hideProjectName {
            settings.projectAlias.clone()
        } else {
            snapshot.projectName.clone()
        }
    }

    fn status(snapshot: &ClaudeSnapshot, settings: &Settings) -> String {
        if snapshot.isBusy {
            settings.busyLabel.clone()
        } else {
            settings.idleLabel.clone()
        }
    }
}
