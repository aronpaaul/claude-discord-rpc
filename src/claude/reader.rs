use super::entry::EntryParser;
use super::locator::TranscriptLocator;
use super::stats::TranscriptStats;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

pub struct TranscriptReader {
    pub sessionId: String,
    path: Option<PathBuf>,
    offset: u64,
    stats: TranscriptStats,
}

impl TranscriptReader {
    pub fn new(sessionId: &str) -> Self {
        Self {
            sessionId: sessionId.to_string(),
            path: None,
            offset: 0,
            stats: TranscriptStats::default(),
        }
    }

    pub fn stats(&self) -> &TranscriptStats {
        &self.stats
    }

    pub fn refresh(&mut self) {
        if self.path.is_none() {
            self.path = TranscriptLocator::find(&self.sessionId);
        }
        let Some(path) = self.path.clone() else {
            return;
        };
        let Ok(mut file) = std::fs::File::open(&path) else {
            self.path = None;
            return;
        };
        let length = file.metadata().map(|meta| meta.len()).unwrap_or(0);
        if length < self.offset {
            self.offset = 0;
            self.stats = TranscriptStats::default();
        }
        if length == self.offset || file.seek(SeekFrom::Start(self.offset)).is_err() {
            return;
        }
        let mut buffer = Vec::new();
        if file.read_to_end(&mut buffer).is_err() {
            return;
        }
        self.consume(&buffer);
    }

    fn consume(&mut self, buffer: &[u8]) {
        let Some(lastNewline) = buffer.iter().rposition(|byte| *byte == b'\n') else {
            return;
        };
        let complete = &buffer[..=lastNewline];
        for line in String::from_utf8_lossy(complete).lines() {
            if !line.trim().is_empty() {
                EntryParser::apply(line, &mut self.stats);
            }
        }
        self.offset += complete.len() as u64;
    }
}
