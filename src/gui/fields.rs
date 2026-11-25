use super::theme::Palette;
use super::toggle::ToggleSwitch;
use eframe::egui::{Align, ComboBox, Layout, RichText, Slider, TextEdit, Ui};

pub struct Fields;

impl Fields {
    pub fn toggle(ui: &mut Ui, title: &str, hint: &str, value: &mut bool) {
        Self::row(ui, title, hint, |ui| {
            ui.add(ToggleSwitch::new(value));
        });
    }

    pub fn text(ui: &mut Ui, title: &str, hint: &str, value: &mut String) {
        Self::caption(ui, title, hint);
        ui.add(TextEdit::singleline(value).desired_width(f32::INFINITY).margin(eframe::egui::Margin::symmetric(10, 7)));
        ui.add_space(4.0);
    }

    pub fn number(ui: &mut Ui, title: &str, hint: &str, value: &mut u32, range: std::ops::RangeInclusive<u32>, suffix: &str) {
        Self::row(ui, title, hint, |ui| {
            ui.add(Slider::new(value, range).suffix(suffix));
        });
    }

    pub fn decimal(ui: &mut Ui, title: &str, hint: &str, value: &mut f32, range: std::ops::RangeInclusive<f32>) {
        Self::row(ui, title, hint, |ui| {
            ui.add(Slider::new(value, range).max_decimals(2));
        });
    }

    pub fn choice<T: PartialEq + Copy>(ui: &mut Ui, title: &str, hint: &str, value: &mut T, options: &[(T, &str)]) {
        let selected = options.iter().find(|(option, _)| option == value).map(|(_, name)| *name).unwrap_or("—");
        Self::row(ui, title, hint, |ui| {
            ComboBox::from_id_salt(title).width(210.0).selected_text(selected).show_ui(ui, |ui| {
                for (option, name) in options {
                    ui.selectable_value(value, *option, *name);
                }
            });
        });
    }

    fn row(ui: &mut Ui, title: &str, hint: &str, control: impl FnOnce(&mut Ui)) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_max_width(ui.available_width() * 0.6);
                Self::caption(ui, title, hint);
            });
            ui.with_layout(Layout::right_to_left(Align::Center), control);
        });
        ui.add_space(2.0);
    }

    fn caption(ui: &mut Ui, title: &str, hint: &str) {
        ui.label(RichText::new(title).color(Palette::text));
        if !hint.is_empty() {
            ui.add_space(-8.0);
            ui.label(RichText::new(hint).small().color(Palette::muted));
        }
    }
}
