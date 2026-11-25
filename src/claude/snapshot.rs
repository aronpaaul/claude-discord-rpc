use super::usage::UsageTotals;

#[derive(Clone, Debug, Default)]
pub struct ClaudeSnapshot {
    pub sessionName: String,
    pub projectName: String,
    pub branch: String,
    pub model: String,
    pub version: String,
    pub isBusy: bool,
    pub statusSinceMillis: i64,
    pub sessionStartMillis: i64,
    pub usage: UsageTotals,
    pub costUsd: f64,
    pub prompts: u32,
    pub toolCalls: u32,
    pub liveSessions: usize,
    pub busySessions: usize,
}
