use crate::gui::card::Card;
use crate::gui::fields::Fields;
use crate::gui::preview::DiscordPreview;
use crate::gui::state::AppState;
use crate::gui::theme::Palette;
use eframe::egui::{Color32, RichText, Ui};

pub struct OverviewPage;

impl OverviewPage {
    pub fn show(ui: &mut Ui, state: &mut AppState) {
        Card::pageTitle(ui, "Обзор", "Так ваш статус выглядит в Discord прямо сейчас");
        let isBusy = state.snapshot.as_ref().is_some_and(|snapshot| snapshot.isBusy);
        Card::show(ui, "Предпросмотр", "обновляется раз в секунду", |ui| {
            DiscordPreview::show(ui, state.preview.as_ref(), isBusy);
        });
        Card::show(ui, "Состояние", "", |ui| Self::statusLines(ui, state));
        Card::show(ui, "Подключение к Discord", "Application ID приложения из Discord Developer Portal", |ui| {
            Fields::text(ui, "Application ID", "имя приложения станет заголовком статуса (например «Claude Code»)", &mut state.settings.clientId);
            ui.hyperlink_to("Открыть Discord Developer Portal →", "https://discord.com/developers/applications");
        });
    }

    fn statusLines(ui: &mut Ui, state: &AppState) {
        let service = state.status.as_ref();
        let discordOk = service.is_some_and(|status| status.discordConnected);
        Self::line(ui, "Сервис claude-discord-rpc", service.is_some(), if service.is_some() { "работает" } else { "остановлен" });
        Self::line(ui, "Discord", discordOk, if discordOk { "подключён" } else { "не подключён" });
        match &state.snapshot {
            Some(snapshot) => {
                let status = if snapshot.isBusy { "работает над задачей" } else { "ожидает" };
                Self::line(ui, "Сессия Claude", true, &format!("{} — {}", snapshot.sessionName, status));
                Self::line(ui, "Открыто сессий", true, &format!("{} (в работе: {})", snapshot.liveSessions, snapshot.busySessions));
            }
            None => Self::line(ui, "Сессия Claude", false, "нет открытых сессий Claude Code"),
        }
        if let Some(error) = service.map(|status| status.lastError.as_str()).filter(|error| !error.is_empty()) {
            ui.label(RichText::new(error).color(Palette::danger));
        }
    }

    fn line(ui: &mut Ui, title: &str, isGood: bool, value: &str) {
        ui.horizontal(|ui| {
            let color: Color32 = if isGood { Palette::online } else { Palette::danger };
            ui.label(RichText::new("●").color(color));
            ui.label(RichText::new(title).color(Palette::muted));
            ui.label(value);
        });
    }
}
