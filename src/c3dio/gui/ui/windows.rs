use super::notifications;
use super::settings;
use bevy::prelude::*;

#[derive(Default)]
pub struct Windows;

impl Windows {
    pub fn show(&mut self, world: &mut World, ctx: &mut egui::Context) {
        world.resource_scope::<settings::Settings, _>(|world, mut settings| {
            settings.ui(world, ctx);
        });
        world.resource_scope::<notifications::Notifications, _>(|world, mut notifications| {
            notifications.ui(world, ctx);
        });
    }
}

pub trait Window {
    fn ui(&mut self, world: &mut World, ctx: &mut egui::Context);
    fn title(&mut self) -> egui::WidgetText;
}
