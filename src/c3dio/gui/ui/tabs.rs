use bevy::prelude::*;
use super::UiState;
use super::EguiTab;

pub struct TabsPlugin;

impl Plugin for TabsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AddTabEvent>()
            .add_systems(Update, add_tabs_system);
    }
}

#[derive(Event)]
pub struct AddTabEvent {
    tab: EguiTab,
}

pub fn add_tabs_system(
    mut ui_state: ResMut<UiState>,
    mut add_tab_events: EventReader<AddTabEvent>,
) {
    for event in add_tab_events.read() {
        let new_tab = event.tab.clone();
        let mut found_tab = None;
        let mut tree = ui_state.tree.clone();
        for node in tree.main_surface_mut().iter_mut() {
            match node {
                egui_dock::Node::Leaf {
                    rect,
                    viewport,
                    tabs,
                    active,
                    scroll,
                } => {
                    for i in 0..tabs.len() {
                        if std::mem::discriminant(&tabs[i]) == std::mem::discriminant(&new_tab) {
                            let tree = ui_state.tree.clone();
                            found_tab = Some(tree.find_tab(&tabs[i]).unwrap());
                            if let Some(found_tab) = found_tab {
                                ui_state
                                    .tree
                                    .set_focused_node_and_surface((found_tab.0, found_tab.1));
                                ui_state.tree.set_active_tab(found_tab);
                            }
                            break;
                        }
                    }
                    if found_tab.is_some() {
                        break;
                    }
                }
                _ => {}
            }
        }
        if found_tab.is_none() {
            ui_state
                .tree
                .main_surface_mut()
                .push_to_focused_leaf(new_tab);
        }
    }
}
