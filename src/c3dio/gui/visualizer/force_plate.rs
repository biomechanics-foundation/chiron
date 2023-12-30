use super::State;
use bevy::prelude::*;
use bevy_c3d::prelude::*;

pub struct ForcePlatePlugin;

impl Plugin for ForcePlatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, add_force_plates)
            .add_systems(Update, update_force_vectors);
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

pub fn add_force_plates(
    mut events: EventReader<C3dLoadedEvent>,
    c3d_state: Res<C3dState>,
    c3d_assets: Res<Assets<C3dAsset>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    // if let Some(_) = events.iter().last() {
    //     let asset = c3d_assets.get(&c3d_state.handle);
    //     match asset {
    //         Some(asset) => {
    //             for force_platform in asset.c3d.forces.iter() {
    //                 let corners = &force_platform.corners;
    //                     let mut position: Vec<[Vec3; 4]> = Vec::new();
    //                     for j in 0..corners.len() {
    //                         let mut corner = [Vec3::default(); 4];
    //                         for k in 0..4 {
    //                             corner[k] =
    //                                 Vec3::new(corners[j][k][0], corners[j][k][1], corners[j][k][2]);
    //                         }
    //                         position.push(corner);
    //                     }
    //                     commands
    //                         .spawn(PbrBundle {
    //                             mesh: meshes.add(Mesh::from(shape::Plane {
    //                                 size: 1.0,
    //                                 subdivisions: 0,
    //                             })),
    //                             material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
    //                             transform: Transform::from_translation(Vec3::new(
    //                                 position[i][0][0],
    //                                 position[i][0][1],
    //                                 position[i][0][2],
    //                             )),
    //                             ..Default::default()
    //                         })
    //                         .insert(ForcePlate { index: i });
    //                     // Add a cylinder with a cone on top to represent the force vector
    //                     commands
    //                         .spawn(PbrBundle {
    //                             mesh: meshes.add(Mesh::from(shape::Cylinder {
    //                                 radius: 0.01,
    //                                 height: 0.1,
    //                                 ..Default::default()
    //                             })),
    //                             material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
    //                             transform: Transform::from_translation(Vec3::new(
    //                                 position[i][0][0],
    //                                 position[i][0][1],
    //                                 position[i][0][2],
    //                             )),
    //                             ..Default::default()
    //                         })
    //                         .insert(ForceVector {
    //                             force_plate_index: i,
    //                         });
    //             }
    //         }
    //         None => {}
    //     }
    // }
}

pub fn update_force_vectors(
    state: ResMut<State>,
    mut query: Query<(&mut Transform, &ForceVector)>,
    c3d_state: Res<C3dState>,
    c3d_assets: Res<Assets<C3dAsset>>,
) {
    if state.updated_frame {
        let asset = c3d_assets.get(&c3d_state.handle);
        match asset {
            Some(asset) => {
                for (mut transform, force_vector) in query.iter_mut() {
                    let force = asset.c3d.force(force_vector.force_plate_index, state.frame);
                    let center_of_pressure = asset
                        .c3d
                        .center_of_pressure(force_vector.force_plate_index, state.frame);
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
