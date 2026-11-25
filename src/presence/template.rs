pub struct TemplateRenderer;

impl TemplateRenderer {
    pub fn render(template: &str, variables: &[(&'static str, String)], maxLength: usize) -> String {
        let mut text = template.to_string();
        for (key, value) in variables {
            let placeholder = format!("{{{key}}}");
            if text.contains(&placeholder) {
                text = text.replace(&placeholder, value);
            }
        }
        Self::fit(text.trim(), maxLength)
    }

    fn fit(text: &str, maxLength: usize) -> String {
        let limit = maxLength.clamp(2, 128);
        let length = text.chars().count();
        if length == 0 {
            return String::new();
        }
        if length < 2 {
            return format!("{text} ");
        }
        if length <= limit {
            return text.to_string();
        }
        let shortened: String = text.chars().take(limit - 1).collect();
        format!("{}…", shortened.trim_end())
    }
}
