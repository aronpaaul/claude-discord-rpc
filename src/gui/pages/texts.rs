use crate::gui::card::Card;
use crate::gui::fields::Fields;
use crate::gui::state::AppState;
use crate::gui::theme::Palette;
use crate::presence::VariableSet;
use eframe::egui::{Grid, RichText, Ui};

pub struct TextsPage;

impl TextsPage {
    pub fn show(ui: &mut Ui, state: &mut AppState) {
        Card::pageTitle(ui, "Тексты", "Шаблоны строк статуса с переменными в фигурных скобках");
        let settings = &mut state.settings;
        Card::show(ui, "Заголовок", "название активности вместо имени приложения", |ui| {
            Fields::text(ui, "Название", "например «Claude Code» или «{model}»", &mut settings.activityName);
        });
        Card::show(ui, "Когда Claude работает", "", |ui| {
            Fields::text(ui, "Первая строка", "details", &mut settings.detailsTemplate);
            Fields::text(ui, "Вторая строка", "state", &mut settings.stateTemplate);
        });
        Card::show(ui, "Когда Claude ждёт", "", |ui| {
            Fields::text(ui, "Первая строка", "details", &mut settings.idleDetailsTemplate);
            Fields::text(ui, "Вторая строка", "state", &mut settings.idleStateTemplate);
        });
        Card::show(ui, "Подписи статуса", "что подставляется в {status}", |ui| {
            Fields::text(ui, "Работает", "", &mut settings.busyLabel);
            Fields::text(ui, "Ожидает", "", &mut settings.idleLabel);
            Fields::text(ui, "Нет сессий", "текст, если сессий нет и статус не скрывается", &mut settings.noSessionText);
            Fields::number(ui, "Максимальная длина строки", "длинные строки обрежутся с «…»", &mut settings.maxLineLength, 16..=128, " симв.");
        });
        Card::show(ui, "Переменные", "нажмите, чтобы скопировать", |ui| Self::catalog(ui));
    }

    fn catalog(ui: &mut Ui) {
        Grid::new("variables").num_columns(2).spacing([18.0, 6.0]).show(ui, |ui| {
            for (key, description) in VariableSet::catalog() {
                let response = ui.add(eframe::egui::Button::new(RichText::new(*key).monospace().color(Palette::accent)).frame(false));
                if response.clicked() {
                    ui.ctx().copy_text(key.to_string());
                }
                ui.label(RichText::new(*description).color(Palette::muted));
                ui.end_row();
            }
        });
    }
}
