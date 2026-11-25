use super::process::ProcessProbe;
use super::session::LiveSession;
use crate::config::{AppPaths, SessionPick, Settings};

pub struct SessionScanner;

impl SessionScanner {
    pub fn liveSessions(settings: &Settings) -> Vec<LiveSession> {
        let Ok(entries) = std::fs::read_dir(AppPaths::liveSessionsDir()) else {
            return Vec::new();
        };
        entries
            .flatten()
            .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "json"))
            .filter_map(|entry| std::fs::read_to_string(entry.path()).ok())
            .filter_map(|text| serde_json::from_str::<LiveSession>(&text).ok())
            .filter(|session| !session.sessionId.is_empty())
            .filter(|session| !settings.desktopOnly || session.isDesktop())
            .filter(|session| ProcessProbe::isAlive(session.pid))
            .collect()
    }

    pub fn pick(sessions: &[LiveSession], settings: &Settings) -> Option<LiveSession> {
        let recency = |session: &&LiveSession| session.updatedAt.max(session.statusUpdatedAt);
        let chosen = match settings.sessionPick {
            SessionPick::MostRecent => sessions.iter().max_by_key(recency),
            SessionPick::BusyFirst => sessions
                .iter()
                .filter(|session| session.isBusy())
                .max_by_key(recency)
                .or_else(|| sessions.iter().max_by_key(recency)),
            SessionPick::NameFilter => {
                let needle = settings.sessionNameFilter.to_lowercase();
                sessions
                    .iter()
                    .filter(|session| session.displayName().to_lowercase().contains(&needle))
                    .max_by_key(recency)
            }
        };
        chosen.cloned()
    }
}
