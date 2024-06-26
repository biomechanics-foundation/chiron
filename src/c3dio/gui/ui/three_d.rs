use super::TabViewer;
use crate::ui::settings::SettingsMenuIsOpen;

pub fn draw_three_d_overlay(ui: &mut egui::Ui, tab_viewer: &mut TabViewer) {
//    ui.strong("Hello world!");
//    if tab_viewer.images.len() > 0 {
//        let image = &tab_viewer.images[0];
//        ui.centered_and_justified(|ui| {
//            image.show(ui);
//        });
//    }
    ui.with_layout(egui::Layout::bottom_up(egui::Align::Max), |ui| {
        let button = ui.button("\u{2699}");
        if button.clicked() {
            let mut open = tab_viewer.world.get_resource_mut::<SettingsMenuIsOpen>().unwrap();
            open.0 = !open.0;
        }
    });
}
