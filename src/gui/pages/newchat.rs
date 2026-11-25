use crate::config::{ActivityKind, TimerMode};
use crate::gui::card::Card;
use crate::gui::fields::Fields;
use crate::gui::state::AppState;
use eframe::egui::Ui;

pub struct NewChatPage;

impl NewChatPage {
    pub fn show(ui: &mut Ui, state: &mut AppState) {
        Card::pageTitle(ui, "Новый чат", "Отдельный статус, пока чат только начат");
        let chat = &mut state.settings.newChat;
        Card::show(ui, "Когда включается", "", |ui| {
            Fields::toggle(ui, "Отдельный статус для нового чата", "", &mut chat.enabled);
            Fields::number(ui, "Новый, пока запросов не больше", "0 — только до первого сообщения", &mut chat.maxPrompts, 0..=20, " шт.");
        });
        Card::show(ui, "Тексты", "поддерживают переменные; пустое название — как в общих настройках", |ui| {
            Fields::text(ui, "Название", "", &mut chat.activityName);
            Fields::text(ui, "Первая строка", "details", &mut chat.detailsTemplate);
            Fields::text(ui, "Вторая строка", "state", &mut chat.stateTemplate);
        });
        Card::show(ui, "Иконки", "ссылка https://; пустая большая картинка — как в общих настройках", |ui| {
            Fields::text(ui, "Большая картинка", "", &mut chat.largeImage);
            Fields::text(ui, "Подсказка большой картинки", "", &mut chat.largeTextTemplate);
            Fields::text(ui, "Маленькая картинка", "пусто — без значка", &mut chat.smallImage);
            Fields::text(ui, "Подсказка маленькой картинки", "", &mut chat.smallTextTemplate);
        });
        Card::show(ui, "Вид", "", |ui| {
            let kinds = [
                (ActivityKind::Playing, "Играет в"),
                (ActivityKind::Listening, "Слушает"),
                (ActivityKind::Watching, "Смотрит"),
                (ActivityKind::Competing, "Соревнуется в"),
            ];
            Fields::choice(ui, "Тип активности", "", &mut chat.activityKind, &kinds);
            let timers = [
                (TimerMode::Session, "Вся сессия"),
                (TimerMode::Task, "Текущая задача"),
                (TimerMode::ServiceStart, "С запуска сервиса"),
                (TimerMode::Hidden, "Не показывать"),
            ];
            Fields::choice(ui, "Таймер", "", &mut chat.timerMode, &timers);
        });
    }
}
