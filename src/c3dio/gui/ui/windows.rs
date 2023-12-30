use super::notifications;
use super::settings;

pub struct Windows {
    pub settings: settings::SettingsUi,
    pub notifications: notifications::NotificationUi,
}

impl Default for Windows {
    fn default() -> Self {
        Self {
            settings: settings::SettingsUi::default(),
            notifications: notifications::NotificationUi::default(),
        }
    }
}

impl Windows {
    pub fn show(
        &mut self,
        ctx: &mut egui::Context,
        settings: &mut settings::GlobalUiSettings,
        notifications: &mut notifications::NotificationQueue,
    ) {
        self.settings.ui(ctx, settings, notifications);
        self.notifications.ui(ctx, settings, notifications);
    }
}

pub trait Window {
    fn ui(
        &mut self,
        ctx: &mut egui::Context,
        settings: &mut settings::GlobalUiSettings,
        notifications: &mut notifications::NotificationQueue,
    );
    fn title(&mut self) -> egui::WidgetText;
}
