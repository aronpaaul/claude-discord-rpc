use super::theme::Palette;
use crate::claude::IsoTime;
use crate::config::ActivityKind;
use crate::presence::PresencePayload;
use eframe::egui::{vec2, Align2, Color32, FontId, Frame, Margin, RichText, Sense, Ui};

pub struct DiscordPreview;

impl DiscordPreview {
    pub fn show(ui: &mut Ui, payload: Option<&PresencePayload>, isBusy: bool) {
        Frame::new()
            .fill(Color32::from_rgb(17, 18, 20))
            .corner_radius(12)
            .inner_margin(Margin::same(16))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                let Some(content) = payload else {
                    ui.label(RichText::new("Статус скрыт — Discord ничего не покажет").color(Palette::muted));
                    return;
                };
                ui.label(RichText::new(Self::heading(content.activityKind)).small().strong().color(Palette::muted));
                ui.horizontal(|ui| {
                    Self::artwork(ui, content, isBusy);
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Claude Code").strong().size(15.0));
                        Self::line(ui, &content.details, Palette::text);
                        Self::line(ui, &content.state, Palette::muted);
                        if let Some(start) = content.startSeconds {
                            let elapsed = IsoTime::nowMillis() / 1000 - start;
                            Self::line(ui, &format!("⏱ {}", Self::clock(elapsed)), Palette::online);
                        }
                    });
                });
                for (label, _) in &content.buttons {
                    let button = eframe::egui::Button::new(label.as_str()).fill(Color32::from_rgb(54, 57, 63));
                    ui.add_sized([ui.available_width(), 32.0], button);
                }
            });
    }

    fn artwork(ui: &mut Ui, content: &PresencePayload, isBusy: bool) {
        let (rect, response) = ui.allocate_exact_size(vec2(76.0, 76.0), Sense::hover());
        let painter = ui.painter();
        painter.rect_filled(rect, 10.0, Palette::accent);
        painter.text(rect.center(), Align2::CENTER_CENTER, "✳", FontId::proportional(42.0), Palette::text);
        if !content.smallImage.is_empty() {
            let center = rect.right_bottom() - vec2(6.0, 6.0);
            let color = if isBusy { Color32::from_rgb(250, 168, 26) } else { Palette::discord };
            painter.circle_filled(center, 13.0, Color32::from_rgb(17, 18, 20));
            painter.circle_filled(center, 10.0, color);
        }
        if !content.largeText.is_empty() {
            response.on_hover_text(content.largeText.as_str());
        }
    }

    fn line(ui: &mut Ui, text: &str, color: Color32) {
        if !text.is_empty() {
            ui.add_space(-6.0);
            ui.label(RichText::new(text).color(color));
        }
    }

    fn heading(kind: ActivityKind) -> &'static str {
        match kind {
            ActivityKind::Playing => "ИГРАЕТ",
            ActivityKind::Listening => "СЛУШАЕТ",
            ActivityKind::Watching => "СМОТРИТ",
            ActivityKind::Competing => "СОРЕВНУЕТСЯ",
        }
    }

    fn clock(seconds: i64) -> String {
        let total = seconds.max(0);
        format!("{:02}:{:02}:{:02}", total / 3600, total % 3600 / 60, total % 60)
    }
}
