mod buttons;
mod data;
mod images;
mod overview;
mod privacy;
mod service;
mod texts;

use super::nav::Page;
use super::state::AppState;
use eframe::egui::Ui;

pub struct PageRouter;

impl PageRouter {
    pub fn show(ui: &mut Ui, page: Page, state: &mut AppState) {
        match page {
            Page::Overview => overview::OverviewPage::show(ui, state),
            Page::Texts => texts::TextsPage::show(ui, state),
            Page::Images => images::ImagesPage::show(ui, state),
            Page::Buttons => buttons::ButtonsPage::show(ui, state),
            Page::Data => data::DataPage::show(ui, state),
            Page::Privacy => privacy::PrivacyPage::show(ui, state),
            Page::Service => service::ServicePage::show(ui, state),
        }
    }
}
