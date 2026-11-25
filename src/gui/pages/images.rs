use crate::config::{ActivityKind, StatusField, TimerMode};
use crate::gui::card::Card;
use crate::gui::fields::Fields;
use crate::gui::state::AppState;
use eframe::egui::Ui;

pub struct ImagesPage;

impl ImagesPage {
    pub fn show(ui: &mut Ui, state: &mut AppState) {
        Card::pageTitle(ui, "Картинки и вид", "Иконки, тип активности и таймер");
        let settings = &mut state.settings;
        Card::show(ui, "Большая картинка", "ссылка https:// на PNG/JPG или ключ ассета приложения", |ui| {
            Fields::text(ui, "Картинка", "", &mut settings.largeImage);
            Fields::text(ui, "Подсказка при наведении", "поддерживает переменные", &mut settings.largeTextTemplate);
        });
        Card::show(ui, "Маленькая картинка", "значок статуса в углу большой картинки", |ui| {
            Fields::toggle(ui, "Показывать значок", "", &mut settings.showSmallImage);
            Fields::text(ui, "Когда работает", "", &mut settings.smallImageBusy);
            Fields::text(ui, "Когда ждёт", "", &mut settings.smallImageIdle);
            Fields::text(ui, "Подсказка при наведении", "поддерживает переменные", &mut settings.smallTextTemplate);
        });
        Card::show(ui, "Вид активности", "", |ui| {
            let kinds = [
                (ActivityKind::Playing, "Играет в"),
                (ActivityKind::Listening, "Слушает"),
                (ActivityKind::Watching, "Смотрит"),
                (ActivityKind::Competing, "Соревнуется в"),
            ];
            Fields::choice(ui, "Тип активности", "глагол перед названием", &mut settings.activityKind, &kinds);
            let fields = [
                (StatusField::Name, "Название приложения"),
                (StatusField::Details, "Первая строка"),
                (StatusField::State, "Вторая строка"),
            ];
            Fields::choice(ui, "В списке участников", "что видно рядом с ником", &mut settings.statusField, &fields);
            let timers = [
                (TimerMode::Task, "Текущая задача"),
                (TimerMode::Session, "Вся сессия"),
                (TimerMode::ServiceStart, "С запуска сервиса"),
                (TimerMode::Hidden, "Не показывать"),
            ];
            Fields::choice(ui, "Таймер", "от какого момента считать время", &mut settings.timerMode, &timers);
        });
    }
}
