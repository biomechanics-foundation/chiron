use super::CameraMode;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::render::camera::Projection;
use bevy::window::PrimaryWindow;

/// Tags an entity as capable of panning and orbiting.
#[derive(Component)]
pub struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub radius: f32,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
        }
    }
}

/// Pan the camera with middle mouse click, zoom with scroll wheel, orbit with right mouse click.
pub fn pan_orbit_camera(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform, &Projection)>,
    camera_mode: Res<CameraMode>,
    camera_query: Query<&Camera>,
) {
    if *camera_mode != CameraMode::PanOrbit {
        return;
    }
    // change input mapping for orbit and panning here
    let orbit_button = MouseButton::Right;
    let pan_button = MouseButton::Middle;

    let mut pan = Vec2::ZERO;
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;

    // only allow panning and orbiting if the mouse is inside the window
    let camera = camera_query.single();
    let window = windows.single();
    if camera.viewport.is_none() {
        return;
    }
    let viewport = camera.clone().viewport.unwrap();
    let mouse_in_viewport = {
        let viewport_size = viewport.physical_size;
        let viewport_pos = viewport.physical_position;
        let window_rect = Rect::from_corners(
            Vec2 {
                x: viewport_pos.x as f32,
                y: viewport_pos.y as f32,
            },
            Vec2 {
                x: viewport_pos.x as f32 + viewport_size.x as f32,
                y: viewport_pos.y as f32 + viewport_size.y as f32,
            },
        );
        let cursor = window.cursor_position();
        match cursor {
            None => false,
            Some(cursor) => window_rect.contains(cursor),
        }
    };

    if input_mouse.pressed(orbit_button) && mouse_in_viewport {
        for ev in ev_motion.read() {
            rotation_move += ev.delta;
        }
    } else if input_mouse.pressed(pan_button) && mouse_in_viewport {
        // Pan only if we're not rotating at the moment
        for ev in ev_motion.read() {
            pan += ev.delta;
        }
    }
    for ev in ev_scroll.read() {
        if mouse_in_viewport {
            scroll += ev.y;
        }
    }

    for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
        let mut any = false;
        if rotation_move.length_squared() > 0.0 {
            any = true;
            let window = get_primary_window_size(&windows);
            let delta_x = {
                //                let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
                rotation_move.x / window.x * std::f32::consts::PI * 2.0
                //                if pan_orbit.upside_down {
                //                    -delta
                //                } else {
                //                    delta
                //                }
            };
            let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
            let yaw = Quat::from_rotation_z(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation = transform.rotation * pitch; // rotate around local x axis
        } else if pan.length_squared() > 0.0 {
            any = true;
            // make panning distance independent of resolution and FOV,
            let window = get_primary_window_size(&windows);
            if let Projection::Perspective(projection) = projection {
                pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
            }
            // translate by local axes
            let right = transform.rotation * Vec3::X * -pan.x;
            let up = transform.rotation * Vec3::Y * pan.y;
            // make panning proportional to distance away from focus point
            let translation = (right + up) * pan_orbit.radius;
            pan_orbit.focus += translation;
        } else if scroll.abs() > 0.0 {
            any = true;
            pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
            // dont allow zoom to reach zero or you get stuck
            pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
        }

        if any {
            // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
            // parent = x and y rotation
            // child = z-offset
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation =
                pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
        }
    }

    // consume any remaining events, so they don't pile up if we don't need them
    // (and also to avoid Bevy warning us about not checking events every frame update)
    ev_motion.clear();
}

fn get_primary_window_size(windows: &Query<&Window, With<PrimaryWindow>>) -> Vec2 {
    let window = windows.single();
    let window = Vec2::new(window.resolution.width(), window.resolution.height());
    window
}
