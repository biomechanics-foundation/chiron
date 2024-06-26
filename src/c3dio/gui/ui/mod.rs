use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContext;
use bevy_egui::EguiSet;
use egui_dock::{DockArea, DockState, NodeIndex, Style};

use self::bottom_menu::BottomMenuPlugin;
use self::notifications::NotificationsPlugin;
use self::settings::SettingsPlugin;
use self::tabs::TabsPlugin;
use self::top_menu::TopMenuPlugin;

mod analog_data;
mod force_data;
pub mod bottom_menu;
mod io;
mod marker_data;
pub mod notifications;
mod parameters;
mod plot;
mod settings;
mod tabs;
mod three_d;
mod top_menu;
mod windows;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SettingsPlugin)
            .add_plugins(NotificationsPlugin)
            .add_plugins(windows::WindowsPlugin)
            .add_plugins(TopMenuPlugin)
            .add_plugins(TabsPlugin)
            .add_plugins(BottomMenuPlugin)
            .init_resource::<UiState>()
            .add_systems(PostUpdate, show_ui_system.before(EguiSet::ProcessOutput));
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EguiTab {
    ThreeDView,
    PlotView(plot::PlotData),
    ParameterListView(String, String),
    MarkerDataView,
    AnalogDataView,
    ForceDataView,
}

impl Tab for EguiTab {
    fn ui(&mut self, ui: &mut egui::Ui, tab_viewer: &mut TabViewer) {
        match self {
            EguiTab::ThreeDView => {
                *tab_viewer.viewport_rect = ui.clip_rect();
                three_d::draw_three_d_overlay(ui, tab_viewer);
            }
            EguiTab::PlotView(plot_ui) => {
                plot::draw_plot(ui, tab_viewer.world, plot_ui);
            }
            EguiTab::ParameterListView(group, parameter) => {
                parameters::draw_parameters_list(ui, tab_viewer.world, group, parameter);
            }
            EguiTab::MarkerDataView => {
                marker_data::draw_marker_data_view(ui, tab_viewer.world);
            }
            EguiTab::AnalogDataView => {
                analog_data::draw_analog_data_view(ui, tab_viewer.world);
            }
            EguiTab::ForceDataView => {
                force_data::draw_force_data_view(ui, tab_viewer.world);
            }
        }
    }

    fn title(&mut self) -> egui::WidgetText {
        match self {
            EguiTab::ThreeDView => "3D Viewer".into(),
            EguiTab::PlotView(plot_ui) => plot_ui.title.clone().into(),
            EguiTab::ParameterListView(group, parameter) => "Parameters".into(),
            EguiTab::MarkerDataView => "Markers".into(),
            EguiTab::AnalogDataView => "Analog".into(),
            EguiTab::ForceDataView => "Forces".into(),
        }
    }
}

pub trait Tab {
    fn ui(&mut self, ui: &mut egui::Ui, tab_viewer: &mut TabViewer);
    fn title(&mut self) -> egui::WidgetText;
}

pub struct TabViewer<'a> {
    world: &'a mut World,
    viewport_rect: &'a mut egui::Rect,
}

#[derive(Resource)]
pub struct UiState {
    tree: DockState<EguiTab>,
    pub viewport_rect: egui::Rect,
}

impl Default for UiState {
    fn default() -> Self {
        Self::new()
    }
}

impl UiState {
    fn ui(&mut self, world: &mut World, ctx: &mut egui::Context) {
        let mut tab_viewer = TabViewer {
            world,
            viewport_rect: &mut self.viewport_rect,
        };

        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut tab_viewer);
    }

    pub fn new() -> Self {
        let mut tree = DockState::new(vec![EguiTab::ThreeDView]);
        let [right, _] = tree.main_surface_mut().split_left(
            NodeIndex::root(),
            0.2,
            vec![
                EguiTab::MarkerDataView,
                EguiTab::AnalogDataView,
                EguiTab::ForceDataView,
                //              EguiTab::ParameterListView("".into(), "".into()),
            ],
        );
 //       let [_main, _bottom] = tree.main_surface_mut().split_below(
 //           right,
 //           0.8,
 //           vec![EguiTab::PlotView(plot::PlotData::default())],
 //       );

        Self {
            tree,
            viewport_rect: egui::Rect::NOTHING,
        }
    }
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = EguiTab;

    fn ui(&mut self, ui: &mut egui_dock::egui::Ui, window: &mut Self::Tab) {
        window.ui(ui, self);
    }

    fn title(&mut self, window: &mut Self::Tab) -> egui_dock::egui::WidgetText {
        window.title()
    }

    fn clear_background(&self, window: &Self::Tab) -> bool {
        !matches!(window, EguiTab::ThreeDView)
    }
}

pub fn show_ui_system(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    world.resource_scope::<UiState, _>(|world, mut ui_state| {
        ui_state.ui(world, egui_context.get_mut())
    });
}
