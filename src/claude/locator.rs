use crate::config::AppPaths;
use std::path::PathBuf;

pub struct TranscriptLocator;

impl TranscriptLocator {
    pub fn find(sessionId: &str) -> Option<PathBuf> {
        let fileName = format!("{sessionId}.jsonl");
        std::fs::read_dir(AppPaths::projectsDir())
            .ok()?
            .flatten()
            .map(|project| project.path().join(&fileName))
            .find(|candidate| candidate.is_file())
    }
}
