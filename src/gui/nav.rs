use super::theme::Palette;
use eframe::egui::{vec2, Button, RichText, Ui};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Overview,
    Texts,
    NewChat,
    Images,
    Buttons,
    Data,
    Privacy,
    Service,
}

pub struct Sidebar;

impl Sidebar {
    pub fn show(ui: &mut Ui, current: &mut Page) {
        let entries = [
            (Page::Overview, "◉  Обзор"),
            (Page::Texts, "✎  Тексты"),
            (Page::NewChat, "✦  Новый чат"),
            (Page::Images, "▣  Картинки"),
            (Page::Buttons, "↗  Кнопки"),
            (Page::Data, "∑  Данные"),
            (Page::Privacy, "◐  Приватность"),
            (Page::Service, "⚙  Сервис"),
        ];
        ui.add_space(6.0);
        for (page, title) in entries {
            let isActive = *current == page;
            let color = if isActive { Palette::text } else { Palette::muted };
            let fill = if isActive { Palette::accentSoft } else { Palette::sidebar };
            let button = Button::new(RichText::new(title).color(color).size(15.0))
                .fill(fill)
                .corner_radius(10)
                .min_size(vec2(ui.available_width(), 38.0));
            if ui.add(button).clicked() {
                *current = page;
            }
        }
    }
}
