use super::notifications;
use super::windows::Window;
use egui::Color32;

#[derive(Default)]
pub struct GlobalUiSettings {
    pub plot_line_color: Color32,
}

pub struct SettingsUi {
    pub open: bool,
}

impl Default for SettingsUi {
    fn default() -> Self {
        Self { open: false }
    }
}

impl Window for SettingsUi {
    fn ui(
        &mut self,
        ctx: &mut egui::Context,
        settings: &mut GlobalUiSettings,
        notifications: &mut notifications::NotificationQueue,
    ) {
        egui::Window::new(self.title())
            .open(&mut self.open)
            .show(ctx, |ui| {
                ui.heading("Settings");
                ui.separator();
                ui.label("Plot line color");
                ui.color_edit_button_srgba(&mut settings.plot_line_color);
            });
    }

    fn title(&mut self) -> egui::WidgetText {
        "Settings".into()
    }
}
