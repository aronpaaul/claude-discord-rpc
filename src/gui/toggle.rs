use super::theme::Palette;
use eframe::egui::{lerp, pos2, vec2, Response, Sense, StrokeKind, Ui, Widget, WidgetInfo, Role};

pub struct ToggleSwitch<'a> {
    value: &'a mut bool,
    scale: f32,
}

impl<'a> ToggleSwitch<'a> {
    pub fn new(value: &'a mut bool) -> Self {
        Self { value, scale: 1.0 }
    }

    pub fn large(value: &'a mut bool) -> Self {
        Self { value, scale: 1.3 }
    }
}

impl Widget for ToggleSwitch<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let size = vec2(40.0, 22.0) * self.scale;
        let (rect, mut response) = ui.allocate_exact_size(size, Sense::click());
        if response.clicked() {
            *self.value = !*self.value;
            response.mark_changed();
        }
        let current = *self.value;
        response.widget_info(|| WidgetInfo::selected(Role::CheckBox, ui.is_enabled(), current, ""));
        if ui.is_rect_visible(rect) {
            let progress = ui.ctx().animate_bool_responsive(response.id, current);
            let radius = rect.height() / 2.0;
            let track = if current { Palette::accent } else { Palette::border };
            let painter = ui.painter();
            painter.rect(rect, radius, track, (0.0, Palette::border), StrokeKind::Inside);
            let knobX = lerp((rect.left() + radius)..=(rect.right() - radius), progress);
            let knobRadius = radius - 3.0 * self.scale;
            painter.circle_filled(pos2(knobX, rect.center().y), knobRadius, Palette::text);
        }
        response
    }
}
