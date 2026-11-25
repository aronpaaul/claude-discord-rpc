use super::pricing::PriceTable;
use super::reader::TranscriptReader;
use super::scanner::SessionScanner;
use super::session::LiveSession;
use super::snapshot::ClaudeSnapshot;
use crate::config::Settings;

#[derive(Default)]
pub struct ClaudeMonitor {
    reader: Option<TranscriptReader>,
}

impl ClaudeMonitor {
    pub fn poll(&mut self, settings: &Settings) -> Option<ClaudeSnapshot> {
        let sessions = SessionScanner::liveSessions(settings);
        let session = SessionScanner::pick(&sessions, settings)?;
        let reader = self.readerFor(&session.sessionId);
        reader.refresh();
        let stats = reader.stats();
        let busySessions = sessions.iter().filter(|candidate| candidate.isBusy()).count();
        Some(ClaudeSnapshot {
            sessionName: session.displayName(),
            projectName: session.projectName(),
            branch: stats.branch.clone(),
            model: stats.model.clone(),
            version: session.version.clone(),
            isBusy: session.isBusy(),
            statusSinceMillis: Self::statusSince(&session),
            sessionStartMillis: stats.firstTimestamp.unwrap_or(session.startedAt),
            usage: stats.totals(),
            costUsd: PriceTable::costUsd(&stats.usageByModel()),
            prompts: stats.prompts,
            toolCalls: stats.toolCalls(),
            liveSessions: sessions.len(),
            busySessions,
        })
    }

    fn readerFor(&mut self, sessionId: &str) -> &mut TranscriptReader {
        let isSame = self.reader.as_ref().is_some_and(|reader| reader.sessionId == sessionId);
        if !isSame {
            self.reader = Some(TranscriptReader::new(sessionId));
        }
        self.reader.get_or_insert_with(|| TranscriptReader::new(sessionId))
    }

    fn statusSince(session: &LiveSession) -> i64 {
        if session.statusUpdatedAt > 0 {
            session.statusUpdatedAt
        } else {
            session.updatedAt.max(session.startedAt)
        }
    }
}
