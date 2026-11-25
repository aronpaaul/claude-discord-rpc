use crate::claude::ClaudeSnapshot;
use crate::config::TimerMode;

pub struct TimerResolver;

impl TimerResolver {
    pub fn startSeconds(mode: TimerMode, snapshot: &ClaudeSnapshot, serviceStartMillis: i64) -> Option<i64> {
        let millis = match mode {
            TimerMode::Hidden => None,
            TimerMode::Session => Some(snapshot.sessionStartMillis),
            TimerMode::Task => Some(snapshot.statusSinceMillis),
            TimerMode::ServiceStart => Some(serviceStartMillis),
        };
        millis.map(|value| value / 1000)
    }
}
