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
                c3d_state.loaded = false;
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
    mut c3d_frame: ResMut<C3dFrame>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    commands: Commands,
    mut notifications: ResMut<Notifications>,
 //   markers: Query<Entity, With<Marker>>,
) {
    if let Some(_) = events.read().last() {
        let asset = c3d_assets.get(&c3d_state.handle);
        match asset {
            Some(asset) => {
                notifications.add(Toast::success("C3D file loaded successfully"));
//                add_markers(&asset, commands, meshes, materials, markers);
                c3d_frame.update_frame(0.);
            }
            None => {}
        }
    }
}

