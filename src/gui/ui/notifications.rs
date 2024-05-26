use super::windows::Window;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContext;
use std::ops::{Deref, DerefMut};
use std::time::{Duration, Instant};

const TOAST_DURATION: Duration = Duration::from_secs(5);
const TOAST_WIDTH: f32 = 300.;
const TOAST_HEIGHT: f32 = 70.;
const TOAST_INNER_PADDING: f32 = 8.;
const TOAST_OUTER_PADDING: f32 = 6.;
const TOAST_ICON_SIZE: f32 = 32.;

const OVERLAY_ICON_SIZE: f32 = 64.;

pub struct NotificationsPlugin;

impl Plugin for NotificationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Toast>()
            .init_resource::<Notifications>()
            .add_systems(Update, (collect_toast_events, show_notifications_system));
    }
}

pub fn collect_toast_events(
    mut events: EventReader<Toast>,
    mut notifications: ResMut<Notifications>,
) {
    for event in events.read() {
        notifications.add(event.clone());
    }
}

#[derive(Event, Clone, Debug)]
pub struct Toast {
    id: uuid::Uuid,
    pub title: String,
    pub icon: Icon,
    pub color: egui::Color32,
    created_at: Option<Instant>,
}

impl Toast {
    pub fn success(title: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            title: title.to_owned(),
            icon: Icon::Success,
            color: egui::Color32::from_rgb(82, 196, 26),
            created_at: Some(Instant::now()),
        }
    }

    pub fn warning(title: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            title: title.to_owned(),
            icon: Icon::Warning,
            color: egui::Color32::from_rgb(250, 173, 20),
            created_at: Some(Instant::now()),
        }
    }

    pub fn error(title: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            title: title.to_owned(),
            icon: Icon::Error,
            color: egui::Color32::from_rgb(255, 77, 79),
            created_at: Some(Instant::now()),
        }
    }

    pub fn info(title: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            title: title.to_owned(),
            icon: Icon::Info,
            color: egui::Color32::from_rgb(22, 119, 255),
            created_at: Some(Instant::now()),
        }
    }

    pub fn show(&self, ui: &mut egui::Ui) {
        egui::Frame::none()
            .fill(self.color)
            .rounding(5.0)
            .inner_margin(TOAST_INNER_PADDING)
            .outer_margin(TOAST_OUTER_PADDING)
            .show(ui, |ui| {
                ui.set_width(TOAST_WIDTH - TOAST_OUTER_PADDING * 3.);
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(self.icon.to_emoji())
                            .color(egui::Color32::BLACK)
                            .size(TOAST_ICON_SIZE),
                    );
                    ui.label(
                        egui::RichText::new(&self.title)
                            .color(egui::Color32::BLACK)
                            .strong()
                            .size(14.0),
                    );
                });
            });
    }
}

#[derive(Clone, Debug)]
pub struct Overlay {
    pub icon: Icon,
    pub window_outline: bool,
}

impl Overlay {
    pub fn new(icon: Icon, window_outline: bool) -> Self {
        Self {
            icon,
            window_outline,
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub enum Icon {
    Info,
    Success,
    Warning,
    Error,
    Upload,
}

impl Icon {
    pub fn to_emoji(&self) -> &str {
        match self {
            Icon::Info => "🚩",
            Icon::Success => "✅",
            Icon::Warning => "⚠️",
            Icon::Error => "❌",
            Icon::Upload => "📤",
        }
    }
}

#[derive(Resource, Default)]
pub struct Notifications {
    pub queue: NotificationQueue,
}

impl Notifications {
    pub fn add(&mut self, toast: Toast) {
        self.queue.push(toast);
    }

    pub fn pop_by_id(&mut self, id: uuid::Uuid) {
        self.queue.pop_by_id(id);
    }

    pub fn overlay(&mut self, icon: Icon, window_outline: bool) {
        self.queue.overlay(icon, window_outline);
    }

    pub fn remove_overlay(&mut self) {
        self.queue.remove_overlay();
    }
}

#[derive(Default)]
pub struct NotificationQueue {
    toasts: Vec<Toast>,
    overlay: Option<Overlay>,
}

impl Deref for NotificationQueue {
    type Target = Vec<Toast>;

    fn deref(&self) -> &Self::Target {
        &self.toasts
    }
}

impl DerefMut for NotificationQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.toasts
    }
}

impl NotificationQueue {
    pub fn pop_by_id(&mut self, id: uuid::Uuid) {
        self.toasts.retain(|notification| notification.id != id);
    }

    pub fn overlay(&mut self, icon: Icon, window_outline: bool) {
        self.overlay = Some(Overlay::new(icon, window_outline));
    }

    pub fn remove_overlay(&mut self) {
        self.overlay = None;
    }

    fn show(&mut self, ui: &mut egui::Ui, center: egui::Pos2) {
        let mut ids_to_remove = Vec::new();
        let mut child_ui = ui.child_ui(
            egui::Rect::from_min_size(
                egui::Pos2::new(
                    ui.available_width() - TOAST_WIDTH - TOAST_OUTER_PADDING * 2.,
                    TOAST_OUTER_PADDING,
                ),
                egui::Vec2::new(TOAST_WIDTH, TOAST_HEIGHT),
            ),
            egui::Layout::default(),
        );
        for toast in self.toasts.iter() {
            if let Some(created_at) = toast.created_at {
                if created_at.elapsed() > TOAST_DURATION {
                    ids_to_remove.push(toast.id.clone());
                }
            }
            toast.show(&mut child_ui);
        }
        for id in ids_to_remove {
            self.pop_by_id(id);
        }
        if let Some(overlay) = &self.overlay {
            if overlay.window_outline {
                ui.painter().rect_filled(
                    egui::Rect::from_center_size(
                        center,
                        egui::Vec2::new(OVERLAY_ICON_SIZE, OVERLAY_ICON_SIZE),
                    ),
                    5.0,
                    egui::Color32::from_rgba_unmultiplied(255, 255, 255, 50),
                );
                ui.with_layout(
                    egui::Layout::centered_and_justified(egui::Direction::TopDown),
                    |ui| {
                        ui.label(
                            egui::RichText::new(overlay.icon.to_emoji())
                                .color(egui::Color32::WHITE)
                                .size(OVERLAY_ICON_SIZE * 0.8),
                        );
                    },
                );
            }
        }
    }
}

impl Window for Notifications {
    fn ui(&mut self, _world: &mut World, ctx: &mut egui::Context) {
        let mut stroke = egui::Stroke::new(5.0, egui::Color32::TRANSPARENT);
        let center = ctx.screen_rect().center();
        if let Some(overlay) = &self.queue.overlay {
            if overlay.window_outline {
                stroke.color = egui::Color32::WHITE;
            }
        }
        egui::Window::new(self.title())
            .frame(egui::Frame {
                fill: egui::Color32::TRANSPARENT,
                stroke,
                ..Default::default()
            })
            .title_bar(false)
            .resizable(false)
            .collapsible(false)
            .interactable(false)
            .fixed_rect(ctx.screen_rect())
            .show(ctx, |ui| {
                self.queue.show(ui, center);
                ui.allocate_space(ui.available_size() - egui::Vec2::new(0., OVERLAY_ICON_SIZE))
            });
    }

    fn title(&mut self) -> egui::WidgetText {
        "Notifications".into()
    }
}

pub fn show_notifications_system(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    world.resource_scope::<Notifications, _>(|world, mut notifications| {
        notifications.ui(world, egui_context.get_mut())
    });
}
