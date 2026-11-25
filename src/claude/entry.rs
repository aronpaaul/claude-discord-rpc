use super::isotime::IsoTime;
use super::stats::TranscriptStats;
use super::usage::UsageTotals;
use serde_json::Value;

pub struct EntryParser;

impl EntryParser {
    pub fn apply(line: &str, stats: &mut TranscriptStats) {
        let Ok(entry) = serde_json::from_str::<Value>(line) else {
            return;
        };
        if entry.get("isSidechain").and_then(Value::as_bool) == Some(true) {
            return;
        }
        let timestamp = entry.get("timestamp").and_then(Value::as_str).and_then(IsoTime::parseMillis);
        if let Some(moment) = timestamp {
            stats.firstTimestamp = Some(stats.firstTimestamp.map_or(moment, |first| first.min(moment)));
        }
        if let Some(branch) = entry.get("gitBranch").and_then(Value::as_str) {
            stats.branch = branch.to_string();
        }
        match entry.get("type").and_then(Value::as_str) {
            Some("assistant") => Self::applyAssistant(&entry, stats),
            Some("user") => Self::applyUser(&entry, stats, timestamp),
            _ => {}
        }
    }

    fn applyAssistant(entry: &Value, stats: &mut TranscriptStats) {
        let Some(message) = entry.get("message") else {
            return;
        };
        let model = message.get("model").and_then(Value::as_str).unwrap_or("");
        if model.is_empty() || model.starts_with('<') {
            return;
        }
        stats.model = model.to_string();
        if let (Some(id), Some(usage)) = (message.get("id").and_then(Value::as_str), message.get("usage")) {
            let totals = UsageTotals::fromJson(usage);
            stats.usageByMessage.insert(id.to_string(), (model.to_string(), totals));
        }
        for block in message.get("content").and_then(Value::as_array).into_iter().flatten() {
            if Self::blockType(block) == Some("tool_use") {
                if let Some(id) = block.get("id").and_then(Value::as_str) {
                    stats.toolIds.insert(id.to_string());
                }
            }
        }
    }

    fn applyUser(entry: &Value, stats: &mut TranscriptStats, timestamp: Option<i64>) {
        if entry.get("isMeta").and_then(Value::as_bool) == Some(true) {
            return;
        }
        let content = entry.get("message").and_then(|message| message.get("content"));
        let isPrompt = match content {
            Some(Value::String(text)) => !text.starts_with('<'),
            Some(Value::Array(blocks)) => {
                let hasText = blocks.iter().any(|block| Self::blockType(block) == Some("text"));
                let hasResult = blocks.iter().any(|block| Self::blockType(block) == Some("tool_result"));
                hasText && !hasResult
            }
            _ => false,
        };
        if isPrompt {
            stats.prompts += 1;
            stats.lastPromptAt = timestamp.or(stats.lastPromptAt);
        }
    }

    fn blockType(block: &Value) -> Option<&str> {
        block.get("type").and_then(Value::as_str)
    }
}
