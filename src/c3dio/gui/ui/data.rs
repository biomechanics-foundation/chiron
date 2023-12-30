use super::notifications::Toast;
use super::plot::PlotData;
use super::TabViewer;
use super::{get_c3d, EguiTab};

pub fn draw_data_view(ui: &mut egui::Ui, tab_viewer: &mut TabViewer) {
    if let Some(c3d) = get_c3d(tab_viewer.world) {
        ui.collapsing("Marker Data", |ui| {
            for i in 0..c3d.points.points.cols() {
                ui.push_id(i, |ui| {
                    let marker = if let Some(label) = c3d.points.labels.get(i) {
                        label
                    } else {
                        "Unknown"
                    };
                    ui.collapsing(marker, |ui| {
                        for (j, dimension) in ["X", "Y", "Z"].iter().enumerate() {
                            let data = c3d
                                .points
                                .iter_col(i)
                                .enumerate()
                                .map(|(i, v)| match j {
                                    0 => [i as f64, v[0] as f64],
                                    1 => [i as f64, v[1] as f64],
                                    2 => [i as f64, v[2] as f64],
                                    _ => unreachable!(),
                                })
                                .collect();
                            if ui.button(dimension.to_owned()).clicked() {
                                tab_viewer.added_tabs.push(EguiTab::PlotView(PlotData {
                                    title: format!("{} {}", marker, dimension),
                                    data,
                                }));
                                tab_viewer.notifications.push(Toast::info(
                                    "Plot added",
                                    format!("{} {}", marker, dimension).as_str(),
                                ));
                            }
                        }
                    });
                });
            }
        });
        ui.collapsing("Analog Data", |ui| {
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
                    tab_viewer.added_tabs.push(EguiTab::PlotView(PlotData {
                        title: analog.to_string(),
                        data,
                    }));
                }
            }
        });
    }
}
