use bevy::prelude::*;
use egui::TopBottomPanel;
use bevy_egui::EguiContext;
use bevy::window::PrimaryWindow;
use super::tabs::AddTabEvent;
use super::plot::PlotData;
use super::EguiTab;

pub struct TopMenuPlugin;

impl Plugin for TopMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, top_menu_system);
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
                if ui.button("Open").clicked() {}
                if ui.button("Save").clicked() {}
            });
            ui.menu_button("View", |ui| {
                if ui.button("3D View").clicked() {
                    world.send_event(AddTabEvent {
                        tab: EguiTab::ThreeDView,
                    });
                }
                if ui.button("Plot").clicked() {
                    world.send_event(AddTabEvent {
                        tab: EguiTab::PlotView(PlotData::default()),
                    });
                }
                if ui.button("Parameters").clicked() {
                    world.send_event(AddTabEvent {
                        tab: EguiTab::ParameterListView("".into(), "".into()),
                    });
                }
                if ui.button("Data").clicked() {
                    world.send_event(AddTabEvent {
                        tab: EguiTab::DataView,
                    });
                }
            });
            ui.menu_button("Tools", |ui| {
                ui.menu_button("Notifications", |ui| if ui.button("Clear").clicked() {});
            });
            ui.menu_button("Help", |ui| if ui.button("About").clicked() {});
        });
    });
}
