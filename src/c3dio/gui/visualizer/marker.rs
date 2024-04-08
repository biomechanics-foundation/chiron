use crate::visualizer::C3dFrame;
use bevy::prelude::*;
use bevy_c3d::*;

pub struct MarkerPlugin;

impl Plugin for MarkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, markers)
            .add_systems(PostUpdate, add_markers);
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
            if frame >= point_data.rows() {
                return;
            }
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
                    asset.c3d.points.points[0][i][0] as f32 / 1000.0,
                    asset.c3d.points.points[0][i][1] as f32 / 1000.0,
                    asset.c3d.points.points[0][i][2] as f32 / 1000.0,
                ),
            );
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Sphere::new(0.01).mesh()),
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
}
