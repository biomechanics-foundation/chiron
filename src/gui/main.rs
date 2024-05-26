use bevy::prelude::*;

mod visualizer;
mod ui;

fn main() {
    App::new()
        .add_plugins(visualizer::VisualizerPlugin)
        .add_plugins(bevy_egui::EguiPlugin)
        .add_plugins(ui::UiPlugin)
        .run();
}
