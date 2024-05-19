use crate::visualizer::C3dFrame;
use bevy::prelude::*;
use bevy_c3d::*;
use bevy_mod_outline::{OutlineBundle, OutlinePlugin, OutlineStencil, OutlineVolume};

pub struct MarkerPlugin;

impl Plugin for MarkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(OutlinePlugin)
            .add_event::<SelectMarkerEvent>()
            .init_resource::<SelectedMarkers>()
            .add_systems(Update, (markers, marker_selected))
            .add_systems(PostUpdate, add_markers);
    }
}

#[derive(Component)]
pub struct Marker;

pub fn markers(
    c3d_frame: Res<C3dFrame>,
    mut query: Query<(&mut Transform, &mut OutlineVolume, &Marker)>,
    selected_markers: Res<SelectedMarkers>,
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
            let marker_labels = &asset.c3d.points.labels;
            let frame = c3d_frame.frame() as usize;
            if frame >= point_data.rows() {
                return;
            }
            for (i, (mut transform, mut outline_volume, _)) in query.iter_mut().enumerate() {
                transform.translation = Vec3::new(
                    point_data[frame][i][0] as f32 * get_marker_scale(asset.c3d.points.units),
                    point_data[frame][i][1] as f32 * get_marker_scale(asset.c3d.points.units),
                    point_data[frame][i][2] as f32 * get_marker_scale(asset.c3d.points.units),
                );
                if selected_markers.0.contains(&marker_labels[i]) {
                    outline_volume.visible = true;
                } else {
                    outline_volume.visible = false;
                }
            }
        }
        None => {}
    }
}

pub fn add_markers(
    mut events: EventReader<C3dLoadedEvent>,
    c3d_state: ResMut<C3dState>,
    c3d_assets: Res<Assets<C3dAsset>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    markers: Query<Entity, With<Marker>>,
) {
    if let Some(_) = events.read().last() {
        let asset = c3d_assets.get(&c3d_state.handle);
        if asset.is_none() {
            return;
        }
        let asset = asset.unwrap();
        for marker in markers.iter() {
            commands.entity(marker).despawn();
        }
        for i in 0..asset.c3d.points.labels.len() {
            let matrix = Mat4::from_scale_rotation_translation(
                Vec3::new(1.0, 1.0, 1.0),
                Quat::from_rotation_y(0.0),
                Vec3::new(
                    asset.c3d.points.points[0][i][0] as f32
                        * get_marker_scale(asset.c3d.points.units),
                    asset.c3d.points.points[0][i][1] as f32
                        * get_marker_scale(asset.c3d.points.units),
                    asset.c3d.points.points[0][i][2] as f32
                        * get_marker_scale(asset.c3d.points.units),
                ),
            );
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Sphere::new(0.01).mesh().uv(30, 30)),
                    material: materials.add(StandardMaterial {
                        base_color: Color::DARK_GREEN,
                        ..default()
                    }),
                    transform: Transform::from_matrix(matrix),
                    ..default()
                },
                OutlineBundle {
                    outline: OutlineVolume {
                        visible: false,
                        colour: Color::RED,
                        width: 2.,
                    },
                    stencil: OutlineStencil {
                        offset: 0.,
                        ..default()
                    },
                    ..default()
                },
                Marker,
            ));
        }
    }
}

#[derive(Event)]
pub struct SelectMarkerEvent(pub String);

#[derive(Resource, Default)]
pub struct SelectedMarkers(pub Vec<String>);

pub fn marker_selected(
    mut events: EventReader<SelectMarkerEvent>,
    mut selected_markers: ResMut<SelectedMarkers>,
) {
    for event in events.read() {
        if selected_markers.0.contains(&event.0) {
            selected_markers.0.retain(|x| x != &event.0);
        } else {
            selected_markers.0.push(event.0.clone());
        }
    }
}

fn get_marker_scale(units: [char; 4]) -> f32 {
    match units {
        ['m', ' ', ' ', ' '] => 1.0,
        ['c', 'm', ' ', ' '] => 0.01,
        ['m', 'm', ' ', ' '] => 0.001,
        _ => 0.001,
    }
}
