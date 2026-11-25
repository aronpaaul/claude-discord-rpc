use serde_json::Value;

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct UsageTotals {
    pub input: u64,
    pub output: u64,
    pub cacheWrite: u64,
    pub cacheRead: u64,
}

impl UsageTotals {
    pub fn fromJson(usage: &Value) -> Self {
        let field = |name: &str| usage.get(name).and_then(Value::as_u64).unwrap_or(0);
        Self {
            input: field("input_tokens"),
            output: field("output_tokens"),
            cacheWrite: field("cache_creation_input_tokens"),
            cacheRead: field("cache_read_input_tokens"),
        }
    }

    pub fn add(&mut self, other: &UsageTotals) {
        self.input += other.input;
        self.output += other.output;
        self.cacheWrite += other.cacheWrite;
        self.cacheRead += other.cacheRead;
    }

    pub fn total(&self, includeCache: bool) -> u64 {
        let base = self.input + self.output;
        if includeCache {
            base + self.cacheWrite + self.cacheRead
        } else {
            base
        }
    }
}
