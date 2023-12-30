use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
) {

    // Spawn a light and the camera
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 2.0, 3.0)),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        brightness: 0.8,
        ..Default::default()
    });
}
