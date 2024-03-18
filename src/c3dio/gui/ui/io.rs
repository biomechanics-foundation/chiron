use crate::gui::ui::notifications::Notifications;
use crate::ui::notifications::Toast;
use crate::visualizer::c3d::Marker;
use bevy::prelude::*;
use bevy_c3d::*;
use rfd::FileDialog;
use std::path::PathBuf;

pub struct IoPlugin;

impl Plugin for IoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                load_file_dialog,
                load_file_system,
                save_file_system,
                save_as_file_system,
                close_c3d_system,
            ),
        )
        .add_event::<CloseC3DEvent>()
        .add_event::<SaveAsFileEvent>()
        .add_event::<SaveFileEvent>()
        .add_event::<LoadFileEvent>();
    }
}

#[derive(Event)]
pub struct LoadFileEvent;

#[derive(Component)]
struct LoadFile(Option<PathBuf>);

fn load_file_dialog(mut commands: Commands, mut events: EventReader<LoadFileEvent>) {
    for _ in events.read() {
      let file = FileDialog::new().add_filter("C3D Files", &["c3d"]).pick_file();
        commands.spawn(LoadFile(file));
    }
}

fn load_file_system(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut LoadFile)>,
    mut c3d_state: ResMut<C3dState>,
    asset_server: Res<AssetServer>,
) {
    for (entity, mut task) in tasks.iter_mut() {
        if let Some(path) = task.0.take() {
            c3d_state.path = path.to_str().unwrap().to_string();
            c3d_state.handle = asset_server.load(&c3d_state.path);
            c3d_state.loaded = false;
            commands.entity(entity).remove::<LoadFile>();
        }
    }
}

fn save_file(path: &str, c3d_state: &Res<C3dState>, c3d_assets: &Res<Assets<C3dAsset>>, notifications: &mut ResMut<Notifications>) {
        if c3d_state.loaded {
            let c3d_asset = c3d_assets.get(&c3d_state.handle);
            if let Some(c3d_asset) = c3d_asset {
                match c3d_asset.c3d.write(path) {
                    Ok(_) => notifications.add(Toast::success("File saved")),
                    Err(_) => notifications.add(Toast::error("Failed to save file")),
                }
            } else {
                notifications.add(Toast::error("Failed to save file"));
            }
        }
        else {
            notifications.add(Toast::error("No file loaded"));
        }
}

#[derive(Event)]
pub struct SaveFileEvent;

fn save_file_system(
    mut events: EventReader<SaveFileEvent>,
    c3d_state: Res<C3dState>,
    c3d_assets: Res<Assets<C3dAsset>>,
    mut notifications: ResMut<Notifications>,
    ) {
    for _ in events.read() {
        save_file(&c3d_state.path, &c3d_state, &c3d_assets, &mut notifications);
    }
}

#[derive(Event)]
pub struct SaveAsFileEvent;

fn save_as_file_system(
    mut events: EventReader<SaveAsFileEvent>,
    c3d_state: Res<C3dState>,
    c3d_assets: Res<Assets<C3dAsset>>,
    mut notifications: ResMut<Notifications>,
    ) {
    for _ in events.read() {
        let file = FileDialog::new().add_filter("C3D Files", &["c3d"]).save_file();
        if let Some(file) = file {
            save_file(file.to_str().unwrap(), &c3d_state, &c3d_assets, &mut notifications);
        }
    }
}

#[derive(Event)]
pub struct CloseC3DEvent;

fn close_c3d_system(
    mut commands: Commands,
    mut events: EventReader<CloseC3DEvent>,
    mut c3d_state: ResMut<C3dState>,
    mut c3d_assets: ResMut<Assets<C3dAsset>>,
    mut notifications: ResMut<Notifications>,
    entities: Query<Entity, With<Marker>>,
    ) {
    for _ in events.read() {
        c3d_state.loaded = false;
        c3d_state.path = "".to_string();
        c3d_assets.remove(&c3d_state.handle);
        c3d_state.handle = Handle::default();
        for entity in entities.iter() {
            //TODO: Add entity struct that indicates if it should be removed on file close
            // and use Query<Entity, With<CloseOnFileClose>> instead of Query<Entity, With<Marker>>
            commands.entity(entity).despawn_recursive();
        }
        notifications.add(Toast::info("File closed"));
    }
}
