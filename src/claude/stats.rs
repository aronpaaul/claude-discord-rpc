use super::usage::UsageTotals;
use std::collections::{HashMap, HashSet};

#[derive(Default, Debug, Clone)]
pub struct TranscriptStats {
    pub usageByMessage: HashMap<String, (String, UsageTotals)>,
    pub toolIds: HashSet<String>,
    pub prompts: u32,
    pub model: String,
    pub branch: String,
    pub firstTimestamp: Option<i64>,
    pub lastPromptAt: Option<i64>,
}

impl TranscriptStats {
    pub fn totals(&self) -> UsageTotals {
        let mut sum = UsageTotals::default();
        for (_, usage) in self.usageByMessage.values() {
            sum.add(usage);
        }
        sum
    }

    pub fn usageByModel(&self) -> HashMap<String, UsageTotals> {
        let mut grouped: HashMap<String, UsageTotals> = HashMap::new();
        for (model, usage) in self.usageByMessage.values() {
            grouped.entry(model.clone()).or_default().add(usage);
        }
        grouped
    }

    pub fn toolCalls(&self) -> u32 {
        self.toolIds.len() as u32
    }
}
