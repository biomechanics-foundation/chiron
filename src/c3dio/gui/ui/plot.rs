use bevy::prelude::*;
use egui_plot::{Line, Plot};

#[derive(Debug, Clone, PartialEq)]
pub struct PlotData {
    pub title: String,
    pub data: Vec<[f64; 2]>,
}

impl Default for PlotData {
    fn default() -> Self {
        PlotData {
            title: "Plot".to_string(),
            data: vec![],
        }
    }
}

pub fn draw_plot(ui: &mut egui::Ui, world: &mut World, plot_ui: &mut PlotData) {
    if plot_ui.data.len() == 0 {
        ui.label("No data");
        return;
    }
    let step = (plot_ui.data.len() / 1000).max(1);
    dbg!(step);
    let data: Vec<[f64; 2]> = plot_ui
        .data
        .iter()
        .enumerate()
        .filter(|(i, _)| i % step == 0)
        .map(|(_, v)| *v)
        .collect();
    let line = Line::new(data);
    let plot = Plot::new(&plot_ui.title).show(ui, |plot_ui| plot_ui.line(line));
    if plot.response.hovered() {
        if let Some(pos) = plot.response.hover_pos() {
            let c3d_frame = world.get_resource_mut::<crate::ui::bottom_menu::C3dFrame>();
            if let Some(mut c3d_frame) = c3d_frame {
                c3d_frame.update_frame(plot.transform.value_from_position(pos).x as f32);
            }
        }
    }
}

