use crate::config::ModelStyle;

pub struct ModelNamer;

impl ModelNamer {
    pub fn format(modelId: &str, style: ModelStyle) -> String {
        if modelId.is_empty() {
            return "Claude".to_string();
        }
        let core = modelId.trim_start_matches("claude-");
        let mut parts = core.split('-');
        let family = parts.next().map(Self::capitalize).unwrap_or_default();
        let version: Vec<&str> = parts.filter(|part| part.len() <= 2 && part.chars().all(|c| c.is_ascii_digit())).collect();
        match style {
            ModelStyle::RawId => modelId.to_string(),
            ModelStyle::Family => family,
            ModelStyle::Pretty if version.is_empty() => family,
            ModelStyle::Pretty => format!("{} {}", family, version.join(".")),
        }
    }

    fn capitalize(word: &str) -> String {
        let mut characters = word.chars();
        match characters.next() {
            Some(first) => first.to_uppercase().chain(characters).collect(),
            None => String::new(),
        }
    }
}
