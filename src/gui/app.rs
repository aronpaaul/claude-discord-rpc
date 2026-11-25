use super::header::Header;
use super::nav::{Page, Sidebar};
use super::pages::PageRouter;
use super::state::AppState;
use super::theme::Palette;
use crate::config::Settings;
use eframe::egui::{CentralPanel, Frame, Margin, Panel, ScrollArea, Stroke, Ui};
use std::time::Duration;

pub struct RpcApp {
    state: AppState,
    page: Page,
}

impl RpcApp {
    pub fn new(settings: Settings) -> Self {
        Self {
            state: AppState::new(settings),
            page: Page::Overview,
        }
    }
}

impl eframe::App for RpcApp {
    fn ui(&mut self, ui: &mut Ui, _: &mut eframe::Frame) {
        self.state.refreshIfDue();
        let headerFrame = Frame::new()
            .fill(Palette::sidebar)
            .inner_margin(Margin::symmetric(20, 14))
            .stroke(Stroke::new(1.0, Palette::border));
        Panel::top("header").frame(headerFrame).show(ui, |ui| Header::show(ui, &mut self.state));
        let sideFrame = Frame::new().fill(Palette::sidebar).inner_margin(Margin::same(14));
        Panel::left("navigation")
            .exact_size(200.0)
            .resizable(false)
            .frame(sideFrame)
            .show(ui, |ui| Sidebar::show(ui, &mut self.page));
        let centralFrame = Frame::new().fill(Palette::background).inner_margin(Margin::symmetric(24, 18));
        CentralPanel::default().frame(centralFrame).show(ui, |ui| {
            ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                PageRouter::show(ui, self.page, &mut self.state);
            });
        });
        self.state.persistIfChanged();
        ui.ctx().request_repaint_after(Duration::from_millis(500));
    }
}
