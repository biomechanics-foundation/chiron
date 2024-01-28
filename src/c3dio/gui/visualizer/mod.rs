use crate::ui::bottom_menu::C3dFrame;
use bevy::prelude::*;
use bevy::{
    //    gltf::GltfPlugin,
    //    audio::AudioPlugin,
    //    gilrs::GilrsPlugin,
    a11y::AccessibilityPlugin,
    animation::AnimationPlugin,
    asset::AssetPlugin,
    core::FrameCountPlugin,
    core::TaskPoolPlugin,
    core::TypeRegistrationPlugin,
    core_pipeline::CorePipelinePlugin,
    diagnostic::DiagnosticsPlugin,
    hierarchy::HierarchyPlugin,
    input::InputPlugin,
    log::LogPlugin,
    pbr::PbrPlugin,
    render::pipelined_rendering::PipelinedRenderingPlugin,
    render::texture::ImagePlugin,
    render::RenderPlugin,
    scene::ScenePlugin,
    sprite::SpritePlugin,
    text::TextPlugin,
    time::TimePlugin,
    transform::TransformPlugin,
    ui::UiPlugin,
    window::WindowPlugin,
    winit::WinitPlugin,
};

mod c3d;
mod camera;
mod force_plate;
mod lighting;

pub struct VisualizerPlugin;

impl Plugin for VisualizerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VisualizerPlugins)
            .add_plugins(camera::CameraPlugin)
            .add_plugins(bevy_c3d::C3dPlugin)
            .add_systems(Startup, lighting::setup)
            .add_systems(Update, c3d::c3d_drag_and_drop)
            .add_systems(Update, c3d::load_c3d)
            .add_systems(Update, c3d::markers)
            .add_plugins(force_plate::ForcePlatePlugin)
            .init_resource::<C3dFrame>();
    }
}

struct VisualizerPlugins;

impl Plugin for VisualizerPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(LogPlugin::default())
            .add_plugins(TaskPoolPlugin::default())
            .add_plugins(TypeRegistrationPlugin::default())
            .add_plugins(FrameCountPlugin::default())
            .add_plugins(TimePlugin::default())
            .add_plugins(TransformPlugin::default())
            .add_plugins(HierarchyPlugin::default())
            .add_plugins(DiagnosticsPlugin::default())
            .add_plugins(InputPlugin::default())
            .add_plugins(WindowPlugin::default())
            .add_plugins(AccessibilityPlugin)
            .add_plugins(AssetPlugin::default())
            .add_plugins(ScenePlugin::default())
            .add_plugins(WinitPlugin::default())
            .add_plugins(RenderPlugin::default())
            .add_plugins(ImagePlugin::default_nearest())
            .add_plugins(PipelinedRenderingPlugin::default())
            .add_plugins(CorePipelinePlugin::default())
            .add_plugins(SpritePlugin::default())
            .add_plugins(TextPlugin::default())
            .add_plugins(UiPlugin::default())
            .add_plugins(PbrPlugin::default())
            .add_plugins(AnimationPlugin::default());
    }
}
