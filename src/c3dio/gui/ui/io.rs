use bevy::prelude::*;
use bevy_c3d::*;
use rfd::FileDialog;
use std::path::PathBuf;

pub struct IoPlugin;

impl Plugin for IoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (load_file_dialog, load_file_system))
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
