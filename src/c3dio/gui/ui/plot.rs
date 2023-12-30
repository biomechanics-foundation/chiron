use bevy::prelude::*;
use egui_plot::{Line, Plot, PlotPoints};

#[derive(Debug, Clone)]
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
    ui.label(&plot_ui.title);
    if plot_ui.data.len() == 0 {
        ui.label("No data");
        return;
    }
    let step = (plot_ui.data.len() / 1000).max(1);
    let data: Vec<[f64; 2]> = plot_ui
        .data
        .iter()
        .enumerate()
        .filter(|(i, _)| i % step == 0)
        .map(|(_, v)| *v)
        .collect();
    let line = Line::new(data);
    let plot = Plot::new("my_plot").show(ui, |plot_ui| plot_ui.line(line));
    if plot.response.hovered() {
        if let Some(pos) = plot.response.hover_pos() {
            let state = world.get_resource_mut::<crate::visualizer::State>();
            if let Some(mut state) = state {
                state.frame = plot.transform.value_from_position(pos).x as usize;
                state.updated_frame = true;
            }
        }
    }
}

pub fn select_plot(ui: &mut egui::Ui) {
    let sin: PlotPoints = (0..1000)
        .map(|i| {
            let x = i as f64 * 0.01;
            [x, x.sin()]
        })
        .collect();
    let line = Line::new(sin);
    Plot::new("my_plot").show(ui, |plot_ui| plot_ui.line(line));
}
