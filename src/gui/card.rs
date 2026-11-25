use super::theme::Palette;
use eframe::egui::{FontFamily, FontId, Frame, Margin, RichText, Stroke, Ui};

pub struct Card;

impl Card {
    pub fn show(ui: &mut Ui, title: &str, subtitle: &str, contents: impl FnOnce(&mut Ui)) {
        Frame::new()
            .fill(Palette::card)
            .stroke(Stroke::new(1.0, Palette::border))
            .corner_radius(14)
            .inner_margin(Margin::same(18))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.label(RichText::new(title).font(FontId::new(17.0, FontFamily::Name("semibold".into()))));
                if !subtitle.is_empty() {
                    ui.add_space(-6.0);
                    ui.label(RichText::new(subtitle).small().color(Palette::muted));
                }
                ui.add_space(4.0);
                contents(ui);
            });
        ui.add_space(12.0);
    }

    pub fn pageTitle(ui: &mut Ui, title: &str, subtitle: &str) {
        ui.heading(title);
        ui.add_space(-6.0);
        ui.label(RichText::new(subtitle).color(Palette::muted));
        ui.add_space(8.0);
    }
}
