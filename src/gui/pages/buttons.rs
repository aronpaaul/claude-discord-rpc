use crate::gui::card::Card;
use crate::gui::fields::Fields;
use crate::gui::state::AppState;
use crate::gui::theme::Palette;
use eframe::egui::{RichText, Ui};

pub struct ButtonsPage;

impl ButtonsPage {
    pub fn show(ui: &mut Ui, state: &mut AppState) {
        Card::pageTitle(ui, "Кнопки", "До двух кнопок-ссылок под статусом");
        let settings = &mut state.settings;
        Card::show(ui, "Первая кнопка", "", |ui| {
            Fields::toggle(ui, "Показывать", "", &mut settings.firstButtonEnabled);
            Fields::text(ui, "Надпись", "до 32 символов", &mut settings.firstButtonLabel);
            Fields::text(ui, "Ссылка", "должна начинаться с https://", &mut settings.firstButtonUrl);
        });
        Card::show(ui, "Вторая кнопка", "", |ui| {
            Fields::toggle(ui, "Показывать", "", &mut settings.secondButtonEnabled);
            Fields::text(ui, "Надпись", "до 32 символов", &mut settings.secondButtonLabel);
            Fields::text(ui, "Ссылка", "должна начинаться с https://", &mut settings.secondButtonUrl);
        });
        ui.label(
            RichText::new("Discord не показывает кнопки в вашем собственном профиле — их видят другие пользователи.")
                .small()
                .color(Palette::muted),
        );
    }
}
