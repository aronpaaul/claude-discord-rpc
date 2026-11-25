use crate::gui::card::Card;
use crate::gui::fields::Fields;
use crate::gui::state::AppState;
use eframe::egui::Ui;

pub struct PrivacyPage;

impl PrivacyPage {
    pub fn show(ui: &mut Ui, state: &mut AppState) {
        Card::pageTitle(ui, "Приватность", "Что скрывать от людей, которые смотрят ваш профиль");
        let settings = &mut state.settings;
        Card::show(ui, "Сессия", "", |ui| {
            Fields::toggle(ui, "Скрыть название сессии", "вместо {session} показывать псевдоним", &mut settings.hideSessionName);
            Fields::text(ui, "Псевдоним сессии", "", &mut settings.sessionAlias);
        });
        Card::show(ui, "Проект", "", |ui| {
            Fields::toggle(ui, "Скрыть название проекта", "вместо {project} показывать псевдоним", &mut settings.hideProjectName);
            Fields::text(ui, "Псевдоним проекта", "", &mut settings.projectAlias);
        });
        Card::show(ui, "Когда скрывать статус целиком", "", |ui| {
            Fields::toggle(ui, "Скрывать без сессий", "если Claude Code закрыт — статуса нет", &mut settings.clearWhenNoSession);
            Fields::number(
                ui,
                "Скрывать при простое",
                "через сколько минут ожидания убрать статус (0 — никогда)",
                &mut settings.idleClearMinutes,
                0..=240,
                " мин",
            );
        });
    }
}
