use crate::config::{AppPaths, SessionPick};
use crate::gui::card::Card;
use crate::gui::fields::Fields;
use crate::gui::state::AppState;
use crate::gui::theme::Palette;
use crate::service::Launcher;
use eframe::egui::{RichText, Ui};

pub struct ServicePage;

impl ServicePage {
    pub fn show(ui: &mut Ui, state: &mut AppState) {
        Card::pageTitle(ui, "Сервис", "Фоновый процесс claude-discord-rpc и выбор сессии");
        Card::show(ui, "Какую сессию показывать", "", |ui| {
            let picks = [
                (SessionPick::BusyFirst, "Работающую, иначе последнюю"),
                (SessionPick::MostRecent, "Последнюю активную"),
                (SessionPick::NameFilter, "По названию"),
            ];
            Fields::choice(ui, "Выбор сессии", "", &mut state.settings.sessionPick, &picks);
            Fields::text(ui, "Фильтр названия", "для режима «По названию»", &mut state.settings.sessionNameFilter);
            Fields::toggle(ui, "Только Desktop", "игнорировать сессии Claude Code из терминала", &mut state.settings.desktopOnly);
        });
        Card::show(ui, "Работа сервиса", "", |ui| {
            Fields::number(ui, "Интервал обновления", "как часто читать данные Claude", &mut state.settings.updateIntervalSecs, 2..=60, " с");
            Fields::toggle(ui, "Автозапуск с Windows", "сервис стартует при входе в систему", &mut state.settings.autostart);
            ui.horizontal(|ui| {
                if ui.button("Перезапустить сервис").clicked() {
                    Launcher::stop();
                    Launcher::ensureRunning();
                    state.forceRefresh();
                }
                if ui.button("Открыть папку настроек").clicked() {
                    let _ = std::process::Command::new("explorer").arg(AppPaths::dataDir()).spawn();
                }
                if ui.button("Сбросить настройки").clicked() {
                    state.resetToDefaults();
                }
            });
        });
        if let Some(status) = &state.status {
            let text = format!("PID {} · Discord: {}", status.pid, if status.discordConnected { "подключён" } else { "нет" });
            ui.label(RichText::new(text).small().color(Palette::muted));
        }
    }
}
