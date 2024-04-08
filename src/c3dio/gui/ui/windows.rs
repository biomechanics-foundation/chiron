use super::EguiTab;
use super::UiState;
use bevy::prelude::*;

pub struct WindowsPlugin;

impl Plugin for WindowsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AddWindowEvent>()
            .add_systems(Update, add_windows_system);
    }
}

pub trait Window {
    fn ui(&mut self, world: &mut World, ctx: &mut egui::Context);
    fn title(&mut self) -> egui::WidgetText;
}

#[derive(Event)]
pub struct AddWindowEvent {
    pub window: EguiTab,
}

pub fn add_windows_system(
    mut ui_state: ResMut<UiState>,
    mut add_window_events: EventReader<AddWindowEvent>,
) {
    for event in add_window_events.read() {
        ui_state.tree.add_window(vec![event.window.clone()]);
    }
}
