use bevy::prelude::*;
use clap::Command;

mod args;
mod forces;
mod gui;
mod info;
mod marker_labels;
mod markers;
use gui::ui;
use gui::visualizer;

fn main() {
    let matches = Command::new("c3dio")
        .version("0.6.0")
        .author("Claire V. Hammond")
        .about("A command line tool for working with C3D files")
        .subcommand(info::info_command())
        .subcommand(markers::markers_command())
        .subcommand(forces::force_command())
        .subcommand(marker_labels::marker_labels_command())
        .get_matches();

    match matches.subcommand() {
        Some(("info", sub_matches)) => {
            info::process_info_command(sub_matches.clone());
        }
        Some(("markers", sub_matches)) => {
            markers::process_markers_command(sub_matches.clone());
        }
        Some(("forces", sub_matches)) => {
            forces::process_forces_command(sub_matches.clone());
        }
        Some(("marker-labels", sub_matches)) => {
            marker_labels::process_marker_labels_command(sub_matches.clone());
        }
        Some(("marker-labels", sub_matches)) => {
            marker_labels::process_marker_labels_command(sub_matches.clone());
        }
        _ => {
            App::new()
                .add_plugins(visualizer::VisualizerPlugin)
                .add_plugins(bevy_egui::EguiPlugin)
                .add_plugins(ui::UiPlugin)
                .run();
        }
    }
}
