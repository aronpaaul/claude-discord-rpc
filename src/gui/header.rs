use super::state::AppState;
use super::theme::Palette;
use super::toggle::ToggleSwitch;
use crate::service::Launcher;
use eframe::egui::{vec2, Align, Align2, FontId, Layout, RichText, Sense, Ui};

pub struct Header;

impl Header {
    pub fn show(ui: &mut Ui, state: &mut AppState) {
        ui.horizontal(|ui| {
            Self::logo(ui);
            ui.vertical(|ui| {
                ui.add_space(2.0);
                ui.label(RichText::new("Claude Discord RPC").size(19.0).family(eframe::egui::FontFamily::Name("semibold".into())));
                ui.add_space(-8.0);
                ui.label(RichText::new("статус Claude Code в вашем профиле Discord").small().color(Palette::muted));
            });
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.add(ToggleSwitch::large(&mut state.settings.rpcEnabled));
                let caption = if state.settings.rpcEnabled { "RPC включён" } else { "RPC выключен" };
                ui.label(RichText::new(caption).strong());
                ui.add_space(14.0);
                Self::serviceControls(ui, state);
            });
        });
    }

    fn serviceControls(ui: &mut Ui, state: &mut AppState) {
        let isRunning = state.status.is_some();
        let label = if isRunning { "Остановить сервис" } else { "Запустить сервис" };
        if ui.button(label).clicked() {
            if isRunning {
                Launcher::stop();
            } else {
                Launcher::ensureRunning();
            }
            state.forceRefresh();
        }
        let (color, text) = if isRunning {
            (Palette::online, "claude-discord-rpc работает")
        } else {
            (Palette::danger, "claude-discord-rpc остановлен")
        };
        ui.label(RichText::new(text).color(Palette::muted).small());
        let (rect, _) = ui.allocate_exact_size(vec2(10.0, 10.0), Sense::hover());
        ui.painter().circle_filled(rect.center(), 5.0, color);
    }

    fn logo(ui: &mut Ui) {
        let (rect, _) = ui.allocate_exact_size(vec2(42.0, 42.0), Sense::hover());
        let painter = ui.painter();
        painter.rect_filled(rect, 11.0, Palette::accent);
        painter.text(rect.center(), Align2::CENTER_CENTER, "✳", FontId::proportional(26.0), Palette::text);
    }
}
