use super::State;
use crate::ui::notifications::{Icon, Toast};
use crate::ui::UiState;
use bevy::prelude::*;
use bevy_c3d::*;

pub fn update_c3d(mut state: ResMut<State>) {
    if state.updated_frame {
        state.updated_frame = false;
    }
}

pub fn c3d_drag_and_drop(
    mut events: EventReader<FileDragAndDrop>,
    mut c3d_state: ResMut<C3dState>,
    asset_server: Res<AssetServer>,
    mut ui_state: ResMut<UiState>,
) {
    for event in events.read() {
        match event {
            FileDragAndDrop::DroppedFile { window, path_buf } => {
                ui_state.notifications.remove_overlay();
                c3d_state.path = path_buf.to_str().unwrap().to_string();
                c3d_state.handle = asset_server.load(&c3d_state.path);
            }
            FileDragAndDrop::HoveredFile { window, path_buf } => {
                ui_state.notifications.overlay(Icon::Upload, true);
            }
            FileDragAndDrop::HoveredFileCanceled { window } => {
                ui_state.notifications.remove_overlay();
            }
        }
    }
}

pub fn load_c3d(
    mut events: EventReader<C3dLoadedEvent>,
    c3d_state: ResMut<C3dState>,
    c3d_assets: Res<Assets<C3dAsset>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    mut ui_state: ResMut<UiState>,
) {
    if let Some(_) = events.read().last() {
        let asset = c3d_assets.get(&c3d_state.handle);
        match asset {
            Some(asset) => {
                ui_state
                    .notifications
                    .push(Toast::success("C3DLoad", "C3D file loaded successfully"));
                for i in 0..asset.c3d.points.labels.len() {
                    let matrix = Mat4::from_scale_rotation_translation(
                        Vec3::new(1.0, 1.0, 1.0),
                        Quat::from_rotation_y(0.0),
                        Vec3::new(
                            asset.c3d.points.points[0][i][0] as f32 / 1000.0,
                            asset.c3d.points.points[0][i][1] as f32 / 1000.0,
                            asset.c3d.points.points[0][i][2] as f32 / 1000.0,
                        ),
                    );
                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(
                                shape::UVSphere {
                                    radius: 0.01,
                                    ..default()
                                }
                                .into(),
                            ),
                            material: materials.add(StandardMaterial {
                                base_color: Color::rgb_u8(255, 0, 255),
                                ..default()
                            }),
                            transform: Transform::from_matrix(matrix),
                            ..default()
                        },
                        Marker,
                    ));
                }
            }
            None => {}
        }
    }
}

#[derive(Component)]
pub struct Marker;

pub fn markers(
    state: ResMut<State>,
    mut query: Query<(&mut Transform, &Marker)>,
    c3d_state: ResMut<C3dState>,
    c3d_assets: Res<Assets<C3dAsset>>,
) {
    if !state.updated_frame {
        return;
    }
    let asset = c3d_assets.get(&c3d_state.handle);
    match asset {
        Some(asset) => {
            let point_data = &asset.c3d.points.points;
            if state.frame >= point_data.rows() {
                return;
            }
            let mut i = 0;
            for (mut transform, _) in query.iter_mut() {
                transform.translation = Vec3::new(
                    point_data[state.frame][i][0] as f32 / 1000.0,
                    point_data[state.frame][i][1] as f32 / 1000.0,
                    point_data[state.frame][i][2] as f32 / 1000.0,
                );
                i += 1;
            }
        }
        None => {}
    }
}
