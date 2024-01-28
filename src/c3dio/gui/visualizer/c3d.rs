use crate::gui::ui::notifications::Notifications;
use crate::ui::notifications::{Icon, Toast};
use crate::visualizer::C3dFrame;
use bevy::prelude::*;
use bevy_c3d::*;

pub fn c3d_drag_and_drop(
    mut events: EventReader<FileDragAndDrop>,
    mut c3d_state: ResMut<C3dState>,
    asset_server: Res<AssetServer>,
    mut notifications: ResMut<Notifications>,
) {
    for event in events.read() {
        match event {
            FileDragAndDrop::DroppedFile { window, path_buf } => {
                notifications.remove_overlay();
                c3d_state.path = path_buf.to_str().unwrap().to_string();
                c3d_state.handle = asset_server.load(&c3d_state.path);
            }
            FileDragAndDrop::HoveredFile { window, path_buf } => {
                notifications.overlay(Icon::Upload, true);
            }
            FileDragAndDrop::HoveredFileCanceled { window } => {
                notifications.remove_overlay();
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
    mut notifications: ResMut<Notifications>,
) {
    if let Some(_) = events.read().last() {
        let asset = c3d_assets.get(&c3d_state.handle);
        match asset {
            Some(asset) => {
                notifications.add(Toast::success("C3D file loaded successfully"));
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
    c3d_frame: Res<C3dFrame>,
    mut query: Query<(&mut Transform, &Marker)>,
    c3d_state: ResMut<C3dState>,
    c3d_assets: Res<Assets<C3dAsset>>,
) {
    if !c3d_frame.updated() {
        return;
    }
    if !c3d_state.loaded {
        return;
    }
    let asset = c3d_assets.get(&c3d_state.handle);
    match asset {
        Some(asset) => {
            let point_data = &asset.c3d.points.points;
            let frame = c3d_frame.frame() as usize;
            for (i, (mut transform, _)) in query.iter_mut().enumerate() {
                transform.translation = Vec3::new(
                    point_data[frame][i][0] as f32 / 1000.0,
                    point_data[frame][i][1] as f32 / 1000.0,
                    point_data[frame][i][2] as f32 / 1000.0,
                );
            }
        }
        None => {}
    }
}
