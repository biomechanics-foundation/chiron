use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy::window::PrimaryWindow;

mod pan_orbit;

use crate::ui;
use pan_orbit::pan_orbit_camera;
use pan_orbit::PanOrbitCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, pan_orbit_camera)
            .add_systems(Startup, setup_camera)
            .add_systems(PostUpdate, set_camera_viewport.after(ui::show_ui_system))
            .init_resource::<CameraMode>();
    }
}

#[derive(Resource, Default, PartialEq)]
pub enum CameraMode {
    Fly,
    #[default]
    PanOrbit,
}

#[derive(Component)]
pub struct MainCamera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 4., 1.).looking_at(Vec3::new(0., 0., 1.), Vec3::Z),
            ..Default::default()
        },
        {
            PanOrbitCamera {
                ..Default::default()
            }
        },
        MainCamera,
    ));
}

pub fn set_camera_viewport(
    ui_state: Res<ui::UiState>,
    primary_window: Query<&mut Window, With<PrimaryWindow>>,
    egui_settings: Res<bevy_egui::EguiSettings>,
    mut cameras: Query<&mut Camera, With<MainCamera>>,
) {
    let mut cam = cameras.single_mut();

    let Ok(window) = primary_window.get_single() else {
        return;
    };

    let scale_factor = window.scale_factor() * egui_settings.scale_factor;

    let viewport_pos = ui_state.viewport_rect.left_top().to_vec2() * scale_factor as f32;
    let viewport_size = ui_state.viewport_rect.size() * scale_factor as f32;

    cam.viewport = Some(Viewport {
        physical_position: UVec2::new(viewport_pos.x as u32, viewport_pos.y as u32),
        physical_size: UVec2::new(viewport_size.x as u32, viewport_size.y as u32),
        depth: 0.0..1.0,
    });
}
