use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_c3d::prelude::*;
use bevy_egui::EguiContext;
use bevy_egui::EguiSet;
use egui_dock::{DockArea, DockState, NodeIndex, Style, Tree};

mod data;
pub mod notifications;
mod parameters;
mod plot;
mod settings;
mod three_d;
mod windows;
use notifications::NotificationQueue;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiState::new())
            .add_systems(PostUpdate, show_ui_system.before(EguiSet::ProcessOutput));
    }
}

#[derive(Debug, Clone)]
pub enum EguiTab {
    ThreeDView,
    PlotView(plot::PlotData),
    ParameterListView(String, String),
    DataView,
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
                parameters::draw_parameters_list(
                    ui,
                    tab_viewer.world,
                    tab_viewer.added_tabs,
                    group,
                    parameter,
                );
            }
            EguiTab::DataView => {
                data::draw_data_view(ui, tab_viewer);
            }
        }
    }

    fn title(&mut self) -> egui::WidgetText {
        match self {
            EguiTab::ThreeDView => "3D View".into(),
            EguiTab::PlotView(plot_ui) => "Plot".into(),
            EguiTab::ParameterListView(group, parameter) => "Parameters".into(),
            EguiTab::DataView => "Data".into(),
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
    added_tabs: &'a mut Vec<EguiTab>,
    settings: &'a mut settings::GlobalUiSettings,
    windows: &'a mut windows::Windows,
    notifications: &'a mut NotificationQueue,
}

#[derive(Resource)]
pub struct UiState {
    tree: DockState<EguiTab>,
    pub viewport_rect: egui::Rect,
    settings: settings::GlobalUiSettings,
    windows: windows::Windows,
    pub notifications: NotificationQueue,
}

impl UiState {
    fn ui(&mut self, world: &mut World, ctx: &mut egui::Context) {
        let mut added_tabs = Vec::new();

        self.windows
            .show(ctx, &mut self.settings, &mut self.notifications);

        let mut tab_viewer = TabViewer {
            world,
            viewport_rect: &mut self.viewport_rect,
            added_tabs: &mut added_tabs,
            settings: &mut self.settings,
            windows: &mut self.windows,
            notifications: &mut self.notifications,
        };

        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut tab_viewer);

        if !added_tabs.is_empty() {
            let new_tab = added_tabs.pop().unwrap();
            for node in self.tree.main_surface_mut().iter_mut() {
                match node {
                    egui_dock::Node::Leaf {
                        rect,
                        viewport,
                        tabs,
                        active,
                        scroll,
                    } => {
                        for i in 0..tabs.len() {
                            if std::mem::discriminant(&tabs[i]) == std::mem::discriminant(&new_tab)
                            {
                                tabs[i] = new_tab.clone();
                                break;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn new() -> Self {
        let mut tree = DockState::new(vec![EguiTab::ThreeDView]);
        let [right, _] = tree.main_surface_mut().split_left(
            NodeIndex::root(),
            0.2,
            vec![
                EguiTab::DataView,
                //              EguiTab::ParameterListView("".into(), "".into()),
            ],
        );
        let [_main, _bottom] = tree.main_surface_mut().split_below(
            right,
            0.8,
            vec![EguiTab::PlotView(plot::PlotData::default())],
        );

        Self {
            tree,
            viewport_rect: egui::Rect::NOTHING,
            settings: settings::GlobalUiSettings::default(),
            windows: windows::Windows::default(),
            notifications: NotificationQueue::default(),
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

pub fn get_c3d(world: &mut World) -> Option<&C3d> {
    let c3d_state = world.get_resource::<C3dState>();
    if let Some(c3d_state) = c3d_state {
        let c3d_asset = world.get_resource::<Assets<C3dAsset>>();
        if let Some(c3d_asset) = c3d_asset {
            let c3d_asset = c3d_asset.get(&c3d_state.handle);
            if let Some(c3d_asset) = c3d_asset {
                return Some(&c3d_asset.c3d);
            }
        }
    }
    None
}
