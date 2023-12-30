use super::settings;
use super::windows::Window;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::time::{Duration, Instant};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const TOAST_DURATION: Duration = Duration::from_secs(5);
const TOAST_WIDTH: f32 = 300.;
const TOAST_HEIGHT: f32 = 70.;
const TOAST_INNER_PADDING: f32 = 8.;
const TOAST_OUTER_PADDING: f32 = 6.;
const TOAST_ICON_SIZE: f32 = 32.;

const OVERLAY_ICON_SIZE: f32 = 64.;

#[derive(Clone, Debug)]
pub struct Toast {
    id: uuid::Uuid,
    pub title: String,
    pub icon: Icon,
    pub color: egui::Color32,
    created_at: Option<Instant>,
}

impl Toast {
    pub fn success(id: &str, title: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            title: title.to_owned(),
            icon: Icon::Success,
            color: egui::Color32::from_rgb(82, 196, 26),
            created_at: Some(Instant::now()),
        }
    }

    pub fn warning(id: &str, title: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            title: title.to_owned(),
            icon: Icon::Warning,
            color: egui::Color32::from_rgb(250, 173, 20),
            created_at: Some(Instant::now()),
        }
    }

    pub fn error(id: &str, title: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            title: title.to_owned(),
            icon: Icon::Error,
            color: egui::Color32::from_rgb(255, 77, 79),
            created_at: Some(Instant::now()),
        }
    }

    pub fn info(id: &str, title: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            title: title.to_owned(),
            icon: Icon::Info,
            color: egui::Color32::from_rgb(22, 119, 255),
            created_at: Some(Instant::now()),
        }
    }

    pub fn show(&self, ui: &mut egui::Ui, retained_images: &RetainedImages) {
        egui::Frame::none()
            .fill(self.color)
            .rounding(5.0)
            .inner_margin(TOAST_INNER_PADDING)
            .outer_margin(TOAST_OUTER_PADDING)
            .show(ui, |ui| {
                ui.set_width(TOAST_WIDTH - TOAST_OUTER_PADDING * 3.);
                let icon_image = retained_images.get(self.icon);
                ui.horizontal(|ui| {
                    if let Some(icon_image) = icon_image {
                        icon_image.toast.show(ui);
                    }
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

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Icon {
    Info,
    Success,
    Warning,
    Error,
    Upload,
}

impl Icon {
    fn initialize_image_set(bytes: &[u8]) -> Result<RetainedImageSet, String> {
        let toast = egui_extras::image::RetainedImage::from_svg_bytes_with_size(
            "toast",
            bytes,
            egui_extras::image::FitTo::Size(TOAST_ICON_SIZE as u32, TOAST_ICON_SIZE as u32),
        )?;
        let overlay = egui_extras::image::RetainedImage::from_svg_bytes_with_size(
            "overlay",
            bytes,
            egui_extras::image::FitTo::Size(OVERLAY_ICON_SIZE as u32, OVERLAY_ICON_SIZE as u32),
        )?;
        Ok(RetainedImageSet { toast, overlay })
    }

    pub fn initialize_images(&self, images: &mut RetainedImages) {
        match self {
            Icon::Info => {
                let info =
                    Icon::initialize_image_set(include_bytes!("icons/exclamation-circle.svg"));
                if let Ok(info) = info {
                    images.insert(Icon::Info, info);
                }
            }
            Icon::Success => {
                let success = Icon::initialize_image_set(include_bytes!("icons/check-circle.svg"));
                if let Ok(success) = success {
                    images.insert(Icon::Success, success);
                }
            }
            Icon::Warning => {
                let warning = Icon::initialize_image_set(include_bytes!("icons/warning.svg"));
                if let Ok(warning) = warning {
                    images.insert(Icon::Warning, warning);
                }
            }
            Icon::Error => {
                let error = Icon::initialize_image_set(include_bytes!("icons/close-circle.svg"));
                if let Ok(error) = error {
                    images.insert(Icon::Error, error);
                }
            }
            Icon::Upload => {
                let upload = Icon::initialize_image_set(include_bytes!("icons/upload.svg"));
                if let Ok(upload) = upload {
                    images.insert(Icon::Upload, upload);
                }
            }
        }
    }
}

#[derive(Default)]
pub struct NotificationQueue {
    toasts: Vec<Toast>,
    overlay: Option<Overlay>,
    retained_images: RetainedImages,
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
            toast.show(&mut child_ui, &self.retained_images);
        }
        for id in ids_to_remove {
            self.pop_by_id(id);
        }
        if let Some(overlay) = &self.overlay {
            if overlay.window_outline {
                let icon_image = self.retained_images.get(overlay.icon);
                if let Some(icon_image) = icon_image {
                    ui.painter().rect_filled(
                        egui::Rect::from_center_size(
                            center,
                            egui::Vec2::new(OVERLAY_ICON_SIZE, OVERLAY_ICON_SIZE),
                        ),
                        5.0,
                        egui::Color32::from_rgba_unmultiplied(255, 255, 255, 50),
                    );
                    icon_image.overlay.show(&mut ui.child_ui(
                        egui::Rect::from_center_size(
                            center,
                            egui::Vec2::new(OVERLAY_ICON_SIZE, OVERLAY_ICON_SIZE),
                        ),
                        egui::Layout::default(),
                    ));
                }
            }
        }
    }
}

pub struct RetainedImageSet {
    toast: egui_extras::image::RetainedImage,
    overlay: egui_extras::image::RetainedImage,
}

pub struct RetainedImages {
    images: HashMap<Icon, RetainedImageSet>,
}

impl Default for RetainedImages {
    fn default() -> Self {
        let mut retained_images = Self {
            images: HashMap::new(),
        };
        for icon in Icon::iter() {
            icon.initialize_images(&mut retained_images);
        }
        retained_images
    }
}

impl RetainedImages {
    pub fn get(&self, icon: Icon) -> Option<&RetainedImageSet> {
        self.images.get(&icon)
    }

    pub fn insert(&mut self, icon: Icon, image: RetainedImageSet) {
        self.images.insert(icon, image);
    }
}

#[derive(Default)]
pub struct NotificationUi;

impl Window for NotificationUi {
    fn ui(
        &mut self,
        ctx: &mut egui::Context,
        _settings: &mut settings::GlobalUiSettings,
        notifications: &mut NotificationQueue,
    ) {
        let mut stroke = egui::Stroke::new(5.0, egui::Color32::TRANSPARENT);
        let center = ctx.screen_rect().center();
        if let Some(overlay) = &notifications.overlay {
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
                notifications.show(ui, center);
                ui.allocate_space(ui.available_size());
            });
    }

    fn title(&mut self) -> egui::WidgetText {
        "Notifications".into()
    }
}
