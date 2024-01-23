use super::settings;
use super::windows::Window;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContext;
use egui::Color32;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Settings>()
            .init_resource::<SettingsMenuIsOpen>()
            .init_resource::<PlotLineColor>()
            .add_systems(Update, settings::show_settings_system);
    }
}

#[derive(Resource, Default)]
pub struct Settings {
    pub plot_line_color: Color32,
}

#[derive(Resource, Default)]
pub struct SettingsMenuIsOpen(pub bool);

#[derive(Resource, Default)]
pub struct PlotLineColor(pub Color32);

impl Window for Settings {
    fn ui(&mut self, world: &mut World, ctx: &mut egui::Context) {
        world.resource_scope::<SettingsMenuIsOpen, _>(|world, mut settings_menu_is_open| {
            world.resource_scope::<PlotLineColor, _>(|_, mut plot_line_color| {
                egui::Window::new(self.title())
                    .open(&mut settings_menu_is_open.0)
                    .collapsible(false)
                    .show(ctx, |ui| {
                        ui.heading("Settings");
                        ui.separator();
                        ui.label("Plot line color");
                        ui.color_edit_button_srgba(&mut plot_line_color.0);
                    });
            });
        });
    }

    fn title(&mut self) -> egui::WidgetText {
        "Settings".into()
    }
}

pub fn show_settings_system(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    world.resource_scope::<Settings, _>(|world, mut settings| {
        settings.ui(world, egui_context.get_mut())
    });
}
