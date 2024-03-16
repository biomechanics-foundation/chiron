use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_c3d::prelude::*;
use bevy_egui::EguiContext;
use egui::widgets::Button;
use egui::DragValue;
use egui::RichText;
use egui::Slider;
use egui::TopBottomPanel;

pub struct BottomMenuPlugin;

impl Plugin for BottomMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, update_frame)
            .add_systems(Update, bottom_menu_system)
            .add_systems(Last, update_c3d_frame)
            .init_resource::<PlayerControl>();
    }
}

#[derive(Resource)]
pub struct PlayerControl {
    pub is_playing: bool,
    pub loop_playback: bool,
    pub playback_speed: f32,
}

impl Default for PlayerControl {
    fn default() -> Self {
        Self {
            is_playing: false,
            loop_playback: false,
            playback_speed: 1.,
        }
    }
}

#[derive(Resource, Default, Debug)]
pub struct C3dFrame {
    frame: f32,
    updated_frame: bool,
    new_frame: f32,
}

impl C3dFrame {
    pub fn update_frame(&mut self, frame: f32) {
        self.new_frame = frame;
    }

    pub fn frame(&self) -> f32 {
        self.frame
    }

    pub fn updated(&self) -> bool {
        self.updated_frame
    }
}

pub fn update_c3d_frame(mut c3d_frame: ResMut<C3dFrame>) {
    c3d_frame.updated_frame = c3d_frame.new_frame != c3d_frame.frame;
    if c3d_frame.updated_frame {
        c3d_frame.frame = c3d_frame.new_frame;
    }
}

pub fn update_frame(
    mut player_control: ResMut<PlayerControl>,
    mut c3d_frame: ResMut<C3dFrame>,
    c3d_state: Res<C3dState>,
    c3d_assets: Res<Assets<C3dAsset>>,
    time: Res<Time>,
) {
    if !c3d_state.loaded {
        return;
    }
    let asset = c3d_assets.get(&c3d_state.handle);
    match asset {
        Some(asset) => {
            if player_control.is_playing {
                c3d_frame.frame = c3d_frame.frame
                    + (time.delta().as_secs_f32()
                        * asset.c3d.points.frame_rate
                        * player_control.playback_speed) as f32;
                c3d_frame.new_frame = c3d_frame.frame;
                c3d_frame.updated_frame = true;
            }
            if c3d_frame.frame() >= asset.c3d.points.rows() as f32 {
                if player_control.loop_playback {
                    c3d_frame.frame = 0.;
                    c3d_frame.new_frame = c3d_frame.frame;
                    c3d_frame.updated_frame = true;
                } else {
                    c3d_frame.frame = (asset.c3d.points.rows() - 1) as f32;
                    c3d_frame.new_frame = c3d_frame.frame;
                    c3d_frame.updated_frame = true;
                    player_control.is_playing = false;
                }
            }
        }
        None => {}
    }
}

pub fn bottom_menu_system(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single_mut(world)
    else {
        return;
    };
    let mut ctx = egui_context.clone();

    world.resource_scope::<PlayerControl, _>(|world, mut player_control| {
        world.resource_scope::<C3dFrame, _>(|_, mut c3d_frame| {
            TopBottomPanel::bottom("bottom_panel")
                .exact_height(48.)
                .show(ctx.get_mut(), |ui| {
                    ui.horizontal_centered(|ui| {
                        //  ui.button(RichText::new("⏮").size(24.))
                        //      .on_hover_text("First frame");
                        if ui
                            .button(RichText::new("⏪").size(24.))
                            .on_hover_text("Previous frame")
                            .clicked()
                        {
                            if c3d_frame.frame() > 0. {
                                let frame = c3d_frame.frame();
                                c3d_frame.update_frame(frame - 1.);
                                if player_control.is_playing {
                                    player_control.is_playing = false;
                                }
                            }
                        }
                        if ui
                            .add(
                                Button::new(RichText::new("▶").size(24.))
                                    .selected(player_control.is_playing),
                            )
                            .on_hover_text("Play/Pause")
                            .clicked()
                        {
                            player_control.is_playing = !player_control.is_playing;
                        }
                        if ui
                            .button(RichText::new("⏩").size(24.))
                            .on_hover_text("Next frame")
                            .clicked()
                        {
                            let frame = c3d_frame.frame();
                            c3d_frame.update_frame(frame + 1.);
                            if player_control.is_playing {
                                player_control.is_playing = false;
                            }
                        }
                        if ui
                            .add(
                                Button::new(RichText::new("🔁").size(24.))
                                    .selected(player_control.loop_playback),
                            )
                            .on_hover_text("Loop playback")
                            .clicked()
                        {
                            player_control.loop_playback = !player_control.loop_playback;
                        }
                        //  ui.button(RichText::new("⏭").size(24.))
                        //      .on_hover_text("Last frame");
                        ui.separator();
                        ui.label("Speed:");
                        ui.add(
                            DragValue::new(&mut player_control.playback_speed)
                                .speed(0.025)
                                .clamp_range(0.05..=1.),
                        );
                        ui.separator();
                        ui.label(format!("Frame: {:.0}", c3d_frame.frame()));
                        //https://github.com/emilk/egui/discussions/3908
                        ui.add(
                            Slider::new(&mut c3d_frame.frame, 0.0..=1000.0)
                                .clamp_to_range(true)
                                .fixed_decimals(0)
                                .handle_shape(egui::style::HandleShape::Rect { aspect_ratio: 0.25 }),
                        );
                    });
                });
        });
    });
}
