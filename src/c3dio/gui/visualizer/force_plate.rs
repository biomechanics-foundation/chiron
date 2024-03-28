use super::C3dFrame;
use bevy::prelude::*;
use bevy_c3d::prelude::*;
use std::cmp::min;

pub struct ForcePlatePlugin;

impl Plugin for ForcePlatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, force_plates)
            .add_systems(PostUpdate, add_force_plates);
    }
}

#[derive(Component, Debug)]
pub struct ForcePlate {
    pub index: usize,
}

#[derive(Component, Debug)]
pub struct ForceVector {
    pub force_plate_index: usize,
}

pub fn force_plates(
    c3d_frame: Res<C3dFrame>,
    force_plate: Query<&ForcePlate>,
    c3d_state: Res<C3dState>,
    c3d_assets: Res<Assets<C3dAsset>>,
    mut gizmos: Gizmos,
) {
    if !c3d_state.loaded {
        return;
    }
    let asset = c3d_assets.get(&c3d_state.handle);
    match asset {
        Some(asset) => {
            for force_plate in force_plate.iter() {
                let force = asset
                    .c3d
                    .force(force_plate.index, c3d_frame.frame() as usize);
                if force.is_none() {
                    continue;
                }
                let force = force.unwrap();
                let center_of_pressure = asset
                    .c3d
                    .center_of_pressure(force_plate.index, c3d_frame.frame() as usize);
                if center_of_pressure.is_none() {
                    continue;
                }
                let origin = &asset.c3d.forces[force_plate.index].origin;
                //                println!(
                //                    "Force Plate {} Force: {:?} CoP: {:?} Origin: {:?}",
                //                    force_plate.index, force, center_of_pressure, origin
                //                );
                let force_plate_data = &asset.c3d.forces[force_plate.index];
                let start = Vec3::new(
                    force_plate_data.corners[0][0] / 1000.0,
                    force_plate_data.corners[0][1] / 1000.0,
                    force_plate_data.corners[0][2] / 1000.0,
                );
                let end = Vec3::new(
                    -force[0]/500. + force_plate_data.corners[0][0] / 1000.0,
                    -force[1]/500. + force_plate_data.corners[0][1] / 1000.0,
                    -force[2]/500. + force_plate_data.corners[0][2] / 1000.0,
                );
                gizmos.arrow(start, end, Color::rgb(0.0, 1.0, 0.0));
            }
        }
        None => {}
    }
}

pub fn add_force_plates(
    mut events: EventReader<C3dLoadedEvent>,
    c3d_state: ResMut<C3dState>,
    c3d_assets: Res<Assets<C3dAsset>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    force_plates: Query<Entity, With<ForcePlate>>,
    force_vectors: Query<Entity, With<ForceVector>>,
) {
    if let Some(_) = events.read().last() {
        let asset = c3d_assets.get(&c3d_state.handle);
        match asset {
            Some(asset) => {
                for force_plate in force_plates.iter() {
                    commands.entity(force_plate).despawn();
                }
                for force_vector in force_vectors.iter() {
                    commands.entity(force_vector).despawn();
                }
                for (i, force_platform) in asset.c3d.forces.iter().enumerate() {
                    let cuboid = Cuboid::new(
                        (force_platform.corners[2][0] - force_platform.corners[0][0]).abs()
                            / 1000.0,
                        (force_platform.corners[2][1] - force_platform.corners[0][1]).abs()
                            / 1000.0,
                        (force_platform.corners[2][2] - force_platform.corners[0][2]).abs()
                            / 1000.0,
                    );
                    let cuboid_size = cuboid.size();
                    //find corner with smallest x and y
                    let min_x = match force_platform.corners[0][0] > force_platform.corners[2][0] {
                        true => force_platform.corners[2][0],
                        false => force_platform.corners[0][0],
                    };
                    let min_y = match force_platform.corners[0][1] > force_platform.corners[2][1] {
                        true => force_platform.corners[2][1],
                        false => force_platform.corners[0][1],
                    };
                    let min_z = match force_platform.corners[0][2] > force_platform.corners[2][2] {
                        true => force_platform.corners[2][2],
                        false => force_platform.corners[0][2],
                    };
                    commands
                        .spawn(PbrBundle {
                            mesh: meshes.add(cuboid),
                            material: materials.add(Color::rgb_u8(0, 0, 255)),
                            transform: Transform::from_translation(Vec3::new(
                                min_x / 1000.0 + cuboid_size.x / 2.0,
                                min_y / 1000.0 + cuboid_size.y / 2.0,
                                min_z / 1000.0 + cuboid_size.z / 2.0,
                            )),
                            ..Default::default()
                        })
                        .insert(ForcePlate { index: i });
                }
            }
            None => {}
        }
    }
}

pub fn update_force_vectors(
    c3d_frame: ResMut<C3dFrame>,
    mut query: Query<(&mut Transform, &ForceVector)>,
    c3d_state: Res<C3dState>,
    c3d_assets: Res<Assets<C3dAsset>>,
) {
    if c3d_frame.updated() {
        let asset = c3d_assets.get(&c3d_state.handle);
        match asset {
            Some(asset) => {
                for (mut transform, force_vector) in query.iter_mut() {
                    let force = asset
                        .c3d
                        .force(force_vector.force_plate_index, c3d_frame.frame() as usize);
                    let center_of_pressure = asset.c3d.center_of_pressure(
                        force_vector.force_plate_index,
                        c3d_frame.frame() as usize,
                    );
                    let origin = asset.c3d.forces.origin(force_vector.force_plate_index);
                    if force.is_none() || center_of_pressure.is_none() || origin.is_none() {
                        continue;
                    }
                    let force = force.unwrap();
                    let center_of_pressure = center_of_pressure.unwrap();
                    let origin = origin.unwrap();
                    transform.translation = Vec3::new(
                        origin[0] + center_of_pressure[0],
                        origin[1] + center_of_pressure[1],
                        origin[2],
                    );
                    transform.scale = Vec3::new(force[0], force[1], force[2]);
                }
            }
            None => {}
        }
    }
}
