use eframe::egui::{Context, FontData, FontDefinitions, FontFamily};
use std::sync::Arc;

pub struct FontInstaller;

impl FontInstaller {
    pub fn install(context: &Context) {
        let mut definitions = FontDefinitions::default();
        let candidates = [
            ("segoeRegular", r"C:\Windows\Fonts\segoeui.ttf", FontFamily::Proportional),
            ("segoeSemibold", r"C:\Windows\Fonts\seguisb.ttf", FontFamily::Name("semibold".into())),
            ("segoeSymbols", r"C:\Windows\Fonts\seguisym.ttf", FontFamily::Proportional),
        ];
        for (name, path, family) in candidates {
            let Ok(bytes) = std::fs::read(path) else {
                continue;
            };
            definitions.font_data.insert(name.to_string(), Arc::new(FontData::from_owned(bytes)));
            let list = definitions.families.entry(family.clone()).or_default();
            if name == "segoeSymbols" {
                list.push(name.to_string());
            } else {
                list.insert(0, name.to_string());
            }
        }
        let fallback = definitions.families.get(&FontFamily::Proportional).cloned().unwrap_or_default();
        let semibold = definitions.families.entry(FontFamily::Name("semibold".into())).or_default();
        semibold.extend(fallback);
        context.set_fonts(definitions);
    }
}
