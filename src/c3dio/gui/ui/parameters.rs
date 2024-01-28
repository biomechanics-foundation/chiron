use super::EguiTab;
use super::tabs::AddTabEvent;
use bevy::prelude::*;
use bevy_c3d::prelude::*;
use egui_extras::{Column, TableBuilder};

pub fn draw_parameters_list(
    ui: &mut egui::Ui,
    world: &mut World,
    group: &mut String,
    parameter: &mut String,
) {
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
            let c3d_asset = c3d_asset.get(&c3d_state.handle).unwrap();
            for group in c3d_asset.c3d.parameters.groups() {
                ui.collapsing(group, |ui| {
                    for parameter in c3d_asset.c3d.parameters.get_group(group).unwrap().keys() {
                        if ui.button(parameter).clicked() {
                            world.send_event(AddTabEvent {
                                tab: EguiTab::ParameterListView(group.clone(), parameter.clone()),
                            });
                        }
                    }
                });
            }
        });
    });
    ui.separator();
    world.resource_scope::<C3dState, _>(|world, c3d_state| {
        world.resource_scope::<Assets<C3dAsset>, _>(|_, c3d_asset| {
            let c3d_asset = c3d_asset.get(&c3d_state.handle).unwrap();
            draw_parameter_table(ui, &c3d_asset.c3d, group, parameter);
        })
    });
}

pub fn draw_parameter_table(ui: &mut egui::Ui, c3d: &C3d, group: &str, parameter: &str) {
    let parameter_data = c3d.parameters.get(group, parameter);
    if let Some(parameter) = parameter_data {
        match parameter.data.clone() {
            ParameterData::Char(mut data) => {
                match parameter.dimensions.len() {
                    1 => {
                        let mut output = String::new();
                        for character in &mut data {
                            output.push(*character);
                        }
                        ui.strong(output);
                    }
                    2 => {
                        let mut table = TableBuilder::new(ui)
                            .striped(true)
                            .auto_shrink([false, false])
                            .vscroll(false)
                            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                            .column(Column::auto())
                            .header(24., |mut header| {
                                header.col(|ui| {
                                    ui.strong("Character");
                                });
                            })
                            .body(|mut body| {
                                for word in 0..parameter.dimensions[1] {
                                    let mut output = String::new();
                                    for character in 0..parameter.dimensions[0] {
                                        output.push(
                                            data[(word * parameter.dimensions[0] + character)
                                                as usize],
                                        );
                                    }
                                    body.row(16., |mut row| {
                                        row.col(|ui| {
                                            ui.strong(output);
                                        });
                                    });
                                }
                            });
                    }
                    _ => {}
                };
            }
            ParameterData::Byte(mut data) => match parameter.dimensions.len() {
                _ => {
                    let mut output = String::new();
                    for byte in &mut data {
                        output.push_str(&byte.to_string());
                        output.push_str(" ");
                    }
                    ui.strong(output);
                }
            },
            ParameterData::Integer(mut data) => {
                match parameter.dimensions.len() {
                    1 => {
                        let mut output = String::new();
                        for integer in &mut data {
                            output.push_str(&integer.to_string());
                            output.push_str(" ");
                        }
                        ui.strong(output);
                    }
                    2 => {
                        let mut table = TableBuilder::new(ui)
                            .striped(true)
                            .auto_shrink([false, false])
                            .vscroll(false)
                            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                            .column(Column::auto())
                            .header(24., |mut header| {
                                header.col(|ui| {
                                    ui.strong("Integer");
                                });
                            })
                            .body(|mut body| {
                                for word in 0..parameter.dimensions[1] {
                                    let mut output = String::new();
                                    for integer in 0..parameter.dimensions[0] {
                                        output.push_str(
                                            &data[(word * parameter.dimensions[0] + integer)
                                                as usize]
                                                .to_string(),
                                        );
                                        output.push_str(" ");
                                    }
                                    body.row(16., |mut row| {
                                        row.col(|ui| {
                                            ui.strong(output);
                                        });
                                    });
                                }
                            });
                    }
                    _ => {}
                };
            }
            ParameterData::Float(mut data) => {
                match parameter.dimensions.len() {
                    1 => {
                        let mut output = String::new();
                        for float in &mut data {
                            output.push_str(&float.to_string());
                            output.push_str(" ");
                        }
                        ui.strong(output);
                    }
                    2 => {
                        let mut table = TableBuilder::new(ui)
                            .striped(true)
                            .auto_shrink([false, false])
                            .vscroll(false)
                            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                            .column(Column::auto())
                            .header(24., |mut header| {
                                header.col(|ui| {
                                    ui.strong("Float");
                                });
                            })
                            .body(|mut body| {
                                for word in 0..parameter.dimensions[1] {
                                    let mut output = String::new();
                                    for float in 0..parameter.dimensions[0] {
                                        output.push_str(
                                            &data
                                                [(word * parameter.dimensions[0] + float) as usize]
                                                .to_string(),
                                        );
                                        output.push_str(" ");
                                    }
                                    body.row(16., |mut row| {
                                        row.col(|ui| {
                                            ui.strong(output);
                                        });
                                    });
                                }
                            });
                    }
                    _ => {}
                };
            }
        }
    }
}
