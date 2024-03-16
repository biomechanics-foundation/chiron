use super::plot::PlotData;
use super::tabs::AddTabEvent;
use super::EguiTab;
use bevy::prelude::*;
use bevy_c3d::prelude::*;

pub fn draw_analog_data_view(ui: &mut egui::Ui, world: &mut World) {
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
    ui.collapsing("Analog Data", |ui| {
        world.resource_scope::<C3dState, _>(|world, c3d_state| {
            world.resource_scope::<Assets<C3dAsset>, _>(|world, c3d_asset| {
                let c3d_asset = c3d_asset.get(&c3d_state.handle);
                if let Some(c3d_asset) = c3d_asset {
                    draw_analog_data(ui, world, &c3d_asset.c3d);
                }
            });
        });
    });
}

fn draw_analog_data(ui: &mut egui::Ui, world: &mut World, c3d: &C3d) {
    for i in 0..c3d.analog.analog.rows() {
        let analog = if let Some(label) = c3d.analog.labels.get(i) {
            label
        } else {
            "Unknown"
        };
        if ui.button(analog).clicked() {
            let data = c3d
                .analog
                .iter_col(i)
                .enumerate()
                .map(|(i, v)| [i as f64, *v as f64])
                .collect();
            world.send_event(AddTabEvent {
                tab: EguiTab::PlotView(PlotData {
                    title: analog.to_string(),
                    data,
                }),
            });
        }
    }
}