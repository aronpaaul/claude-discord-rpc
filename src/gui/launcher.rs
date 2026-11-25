use super::app::RpcApp;
use super::fonts::FontInstaller;
use super::theme::ThemeInstaller;
use crate::config::{SettingsStore, Settings};
use crate::service::{AutoStart, Launcher as ServiceLauncher};

pub struct Launcher;

impl Launcher {
    pub fn run() {
        SettingsStore::ensureExists();
        let settings = SettingsStore::load();
        Self::prepareService(&settings);
        let viewport = eframe::egui::ViewportBuilder::default()
            .with_title("Claude Discord RPC")
            .with_inner_size([980.0, 680.0])
            .with_min_inner_size([820.0, 560.0]);
        let options = eframe::NativeOptions {
            viewport,
            centered: true,
            ..Default::default()
        };
        let _ = eframe::run_native(
            "claude-discord-rpc",
            options,
            Box::new(move |creation| {
                FontInstaller::install(&creation.egui_ctx);
                ThemeInstaller::install(&creation.egui_ctx);
                Ok(Box::new(RpcApp::new(settings)))
            }),
        );
    }

    fn prepareService(settings: &Settings) {
        AutoStart::apply(settings.autostart);
        ServiceLauncher::ensureRunning();
    }
}
