use super::plot::PlotData;
use super::tabs::AddTabEvent;
use super::EguiTab;
use bevy::prelude::*;
use bevy_c3d::prelude::*;

pub fn draw_force_data_view(ui: &mut egui::Ui, world: &mut World) {
    let c3d_loaded = world.resource_scope::<C3dState, bool>(|world, c3d_state| {
        let c3d_asset = world.get_resource::<Assets<C3dAsset>>();
        if let Some(c3d_asset) = c3d_asset {
            let c3d_asset = c3d_asset.get(&c3d_state.handle);
            if let Some(_) = c3d_asset {
                return true;
            }
        }
        false
    });
    if !c3d_loaded {
        return;
    }
    world.resource_scope::<C3dState, _>(|world, c3d_state| {
        world.resource_scope::<Assets<C3dAsset>, _>(|world, c3d_asset| {
            let c3d_asset = c3d_asset.get(&c3d_state.handle);
            if let Some(c3d_asset) = c3d_asset {
                draw_force_data(ui, world, &c3d_asset.c3d);
            }
        });
    });
}

fn draw_force_data(ui: &mut egui::Ui, world: &mut World, c3d: &C3d) {
    for (i, force_plate) in c3d.forces.iter().enumerate() {
        ui.collapsing(format!("Force Plate {}", i), |ui| {
            ui.label(format!("Origin: {:?}", force_plate.origin));
            ui.label(format!("Corners: {:?}", force_plate.corners));
            ui.label(format!("Type: {:?}", force_plate.plate_type));
//            ui.label(format!("Force: {:?}", force_plate.force));
//            ui.label(format!("Center of Pressure: {:?}", force_plate.center_of_pressure));
        });
    }
}
