use super::io::{CloseC3DEvent, IoPlugin, LoadFileEvent, SaveAsFileEvent, SaveFileEvent};
use super::plot::PlotData;
use super::tabs::AddTabEvent;
use super::EguiTab;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContext;
use egui::TopBottomPanel;

pub struct TopMenuPlugin;

impl Plugin for TopMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(IoPlugin)
            .add_systems(Update, top_menu_system);
    }
}

pub fn top_menu_system(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single_mut(world)
    else {
        return;
    };
    let mut ctx = egui_context.clone();

    TopBottomPanel::top("top_panel").show(ctx.get_mut(), |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open...").clicked() {
                    world.send_event(LoadFileEvent);
                    ui.close_menu();
                }
                if ui.button("Save").clicked() {
                    world.send_event(SaveFileEvent);
                    ui.close_menu();
                }
                if ui.button("Save As...").clicked() {
                    world.send_event(SaveAsFileEvent);
                    ui.close_menu();
                }
                if ui.button("Close Current File").clicked() {
                    world.send_event(CloseC3DEvent);
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Exit").clicked() {
                    std::process::exit(0);
                }
            });
            ui.menu_button("View", |ui| {
                if ui.button("3D View").clicked() {
                    world.send_event(AddTabEvent {
                        tab: EguiTab::ThreeDView,
                    });
                    ui.close_menu();
                }
                if ui.button("Marker Data").clicked() {
                    world.send_event(AddTabEvent {
                        tab: EguiTab::MarkerDataView,
                    });
                    ui.close_menu();
                }
                if ui.button("Analog Data").clicked() {
                    world.send_event(AddTabEvent {
                        tab: EguiTab::AnalogDataView,
                    });
                    ui.close_menu();
                }
                if ui.button("Plot").clicked() {
                    world.send_event(AddTabEvent {
                        tab: EguiTab::PlotView(PlotData::default()),
                    });
                    ui.close_menu();
                }
                if ui.button("Parameters").clicked() {
                    world.send_event(AddTabEvent {
                        tab: EguiTab::ParameterListView("".into(), "".into()),
                    });
                    ui.close_menu();
                }
            });
            ui.menu_button("Tools", |ui| {
                ui.menu_button("Notifications", |ui| {
                    if ui.button("Clear").clicked() {
                        ui.close_menu();
                    }
                });
            });
            ui.menu_button("Help", |ui| {
                if ui.button("About").clicked() {
                    ui.close_menu();
                }
            });
        });
    });
}
