use crate::config::Settings;

pub struct ButtonCollector;

impl ButtonCollector {
    pub fn collect(settings: &Settings) -> Vec<(String, String)> {
        let candidates = [
            (settings.firstButtonEnabled, &settings.firstButtonLabel, &settings.firstButtonUrl),
            (settings.secondButtonEnabled, &settings.secondButtonLabel, &settings.secondButtonUrl),
        ];
        candidates
            .iter()
            .filter(|(enabled, label, url)| *enabled && !label.trim().is_empty() && Self::isWebLink(url))
            .map(|(_, label, url)| (label.trim().chars().take(32).collect(), url.trim().to_string()))
            .collect()
    }

    fn isWebLink(url: &str) -> bool {
        let trimmed = url.trim();
        trimmed.starts_with("https://") || trimmed.starts_with("http://")
    }
}
