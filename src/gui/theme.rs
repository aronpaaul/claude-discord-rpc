use eframe::egui::{Color32, Context, CornerRadius, FontFamily, FontId, Stroke, TextStyle, Visuals};

pub struct Palette;

impl Palette {
    pub const background: Color32 = Color32::from_rgb(20, 20, 21);
    pub const sidebar: Color32 = Color32::from_rgb(26, 26, 28);
    pub const card: Color32 = Color32::from_rgb(33, 33, 36);
    pub const field: Color32 = Color32::from_rgb(43, 43, 47);
    pub const border: Color32 = Color32::from_rgb(52, 52, 57);
    pub const accent: Color32 = Color32::from_rgb(217, 119, 87);
    pub const accentSoft: Color32 = Color32::from_rgb(70, 42, 33);
    pub const text: Color32 = Color32::from_rgb(236, 236, 238);
    pub const muted: Color32 = Color32::from_rgb(150, 150, 158);
    pub const online: Color32 = Color32::from_rgb(59, 165, 93);
    pub const danger: Color32 = Color32::from_rgb(237, 66, 69);
    pub const discord: Color32 = Color32::from_rgb(88, 101, 242);
}

pub struct ThemeInstaller;

impl ThemeInstaller {
    pub fn install(context: &Context) {
        let mut visuals = Visuals::dark();
        visuals.panel_fill = Palette::background;
        visuals.window_fill = Palette::card;
        visuals.extreme_bg_color = Palette::field;
        visuals.override_text_color = Some(Palette::text);
        visuals.selection.bg_fill = Palette::accent;
        visuals.selection.stroke = Stroke::new(1.0, Palette::text);
        visuals.hyperlink_color = Palette::accent;
        let radius = CornerRadius::same(8);
        for widget in [&mut visuals.widgets.inactive, &mut visuals.widgets.hovered, &mut visuals.widgets.active] {
            widget.corner_radius = radius;
        }
        visuals.widgets.inactive.weak_bg_fill = Palette::field;
        visuals.widgets.inactive.bg_fill = Palette::field;
        visuals.widgets.hovered.weak_bg_fill = Palette::border;
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, Palette::accent);
        visuals.widgets.active.weak_bg_fill = Palette::accentSoft;
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, Palette::border);
        context.set_visuals(visuals);
        context.global_style_mut(|style| {
            style.spacing.item_spacing = [10.0, 10.0].into();
            style.spacing.button_padding = [14.0, 7.0].into();
            style.spacing.interact_size.y = 30.0;
            style.text_styles.insert(TextStyle::Body, FontId::new(15.0, FontFamily::Proportional));
            style.text_styles.insert(TextStyle::Button, FontId::new(15.0, FontFamily::Proportional));
            style.text_styles.insert(TextStyle::Small, FontId::new(12.5, FontFamily::Proportional));
            style.text_styles.insert(TextStyle::Heading, FontId::new(22.0, FontFamily::Name("semibold".into())));
        });
    }
}
