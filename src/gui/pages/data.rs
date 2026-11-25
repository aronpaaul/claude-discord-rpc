use crate::config::{Currency, ModelStyle, TokenStyle};
use crate::gui::card::Card;
use crate::gui::fields::Fields;
use crate::gui::state::AppState;
use eframe::egui::Ui;

pub struct DataPage;

impl DataPage {
    pub fn show(ui: &mut Ui, state: &mut AppState) {
        Card::pageTitle(ui, "Данные", "Как считать и форматировать токены, стоимость и модель");
        let settings = &mut state.settings;
        Card::show(ui, "Токены", "", |ui| {
            let styles = [
                (TokenStyle::Compact, "Кратко — 12.4k"),
                (TokenStyle::Spaced, "С пробелами — 12 400"),
                (TokenStyle::Full, "Полностью — 12400"),
            ];
            Fields::choice(ui, "Формат чисел", "", &mut settings.tokenStyle, &styles);
            Fields::toggle(ui, "Учитывать кэш", "добавлять в {tokens} токены, прочитанные и записанные в кэш", &mut settings.includeCacheTokens);
        });
        Card::show(ui, "Стоимость", "оценка по публичным ценам API, подписку не учитывает", |ui| {
            let currencies = [(Currency::Usd, "Доллары $"), (Currency::Rub, "Рубли ₽"), (Currency::Eur, "Евро €")];
            Fields::choice(ui, "Валюта", "", &mut settings.currency, &currencies);
            Fields::decimal(ui, "Курс доллара к рублю", "", &mut settings.usdToRubRate, 1.0..=300.0);
            Fields::decimal(ui, "Курс доллара к евро", "", &mut settings.usdToEurRate, 0.1..=3.0);
        });
        Card::show(ui, "Модель", "", |ui| {
            let styles = [
                (ModelStyle::Pretty, "Красиво — Opus 5.5"),
                (ModelStyle::Family, "Семейство — Opus"),
                (ModelStyle::RawId, "ID — claude-opus-5-5"),
            ];
            Fields::choice(ui, "Название модели", "", &mut settings.modelStyle, &styles);
        });
        if let Some(snapshot) = &state.snapshot {
            Card::show(ui, "Текущая сессия", "сырые значения", |ui| {
                let usage = &snapshot.usage;
                ui.label(format!("Вход: {}   Выход: {}", usage.input, usage.output));
                ui.label(format!("Кэш — запись: {}   чтение: {}", usage.cacheWrite, usage.cacheRead));
                ui.label(format!("Запросов: {}   Инструментов: {}   ≈ ${:.2}", snapshot.prompts, snapshot.toolCalls, snapshot.costUsd));
            });
        }
    }
}
