//! Sonner molecule component
//!
//! Modern toast notifications with rich styling, swipe to dismiss, and progress bars.

use crate::styles::Style;
use crate::theme::tokens::Color;
use crate::theme::{use_style, use_theme};
use dioxus::prelude::*;
use std::time::Duration;

#[cfg(all(feature = "web", target_arch = "wasm32"))]
use wasm_bindgen::JsCast;

/// Toast position on screen
#[derive(Default, Clone, PartialEq, Debug)]
pub enum ToastPosition {
    #[default]
    BottomRight,
    BottomCenter,
    BottomLeft,
    TopRight,
    TopCenter,
    TopLeft,
}

/// Sonner toast variant for different purposes
#[derive(Default, Clone, PartialEq, Debug)]
pub enum SonnerVariant {
    #[default]
    Default,
    Success,
    Error,
    Warning,
    Info,
    Loading,
}

/// Individual toast data
#[derive(Clone, PartialEq, Debug)]
pub struct Toast {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub variant: SonnerVariant,
    pub duration: Duration,
    pub action: Option<ToastAction>,
}

/// Toast action button
#[derive(Clone, PartialEq, Debug)]
pub struct ToastAction {
    pub label: String,
    pub on_click: EventHandler<()>,
}

/// Sonner (toast) properties
#[derive(Props, Clone, PartialEq)]
pub struct SonnerProps {
    /// Array of active toasts
    pub toasts: Vec<Toast>,
    /// Position on screen
    #[props(default)]
    pub position: ToastPosition,
    /// Rich colors for variants
    #[props(default = true)]
    pub rich_colors: bool,
    /// Show close button
    #[props(default = true)]
    pub close_button: bool,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Callback when toast is dismissed
    #[props(default)]
    pub on_dismiss: Option<EventHandler<String>>,
    /// Element id
    #[props(default)]
    pub id: Option<String>,
    /// Custom CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Sonner toast notification container
#[component]
pub fn Sonner(props: SonnerProps) -> Element {
    let _theme = use_theme();
    let position = props.position.clone();

    let container_style = use_style(move |_t| {
        let position_css = match position {
            ToastPosition::BottomRight => "bottom: 16px; right: 16px;",
            ToastPosition::BottomCenter => "bottom: 16px; left: 50%; transform: translateX(-50%);",
            ToastPosition::BottomLeft => "bottom: 16px; left: 16px;",
            ToastPosition::TopRight => "top: 16px; right: 16px;",
            ToastPosition::TopCenter => "top: 16px; left: 50%; transform: translateX(-50%);",
            ToastPosition::TopLeft => "top: 16px; left: 16px;",
        };

        Style::new()
            .fixed()
            .z_index(9999)
            .flex()
            .flex_col()
            .custom(position_css)
            .custom("gap: 8px;")
            .max_w_px(400)
            .pointer_events_none()
            .build()
    });

    rsx! {
        div {
            id: "{props.id.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            style: "{container_style} {props.style.clone().unwrap_or_default()}",

            for toast in props.toasts.iter() {
                ToastItem {
                    key: "{toast.id}",
                    toast: toast.clone(),
                    rich_colors: props.rich_colors,
                    close_button: props.close_button,
                    on_dismiss: props.on_dismiss.clone(),
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ToastItemProps {
    toast: Toast,
    rich_colors: bool,
    close_button: bool,
    on_dismiss: Option<EventHandler<String>>,
    /// Element id
    #[props(default)]
    pub id: Option<String>,
    /// Custom CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
}

#[component]
fn ToastItem(props: ToastItemProps) -> Element {
    let theme = use_theme();
    #[allow(unused_mut)]
    let mut progress = use_signal(|| 100.0);
    let toast_id = props.toast.id.clone();

    // Animate progress from 100% down to 0% over the toast duration on web targets.
    #[cfg(all(feature = "web", target_arch = "wasm32"))]
    {
        let duration = props.toast.duration;
        let on_dismiss = props.on_dismiss.clone();
        let id = toast_id.clone();
        use_effect(move || {
            let duration_ms = duration.as_millis() as f64;
            if duration_ms <= 0.0 {
                return;
            }
            let start = js_sys::Date::now();
            let on_dismiss = on_dismiss.clone();
            let id = id.clone();
            wasm_bindgen_futures::spawn_local(async move {
                loop {
                    let elapsed = js_sys::Date::now() - start;
                    let pct = 100.0 * (1.0 - elapsed / duration_ms).max(0.0);
                    progress.set(pct);
                    if pct <= 0.0 {
                        if let Some(handler) = on_dismiss {
                            handler.call(id);
                        }
                        break;
                    }
                    let promise = js_sys::Promise::new(&mut |resolve, _| {
                        let func = resolve.unchecked_ref::<js_sys::Function>();
                        web_sys::window()
                            .unwrap()
                            .set_timeout_with_callback_and_timeout_and_arguments_0(func, 50)
                            .unwrap();
                    });
                    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                }
            });
        });
    }

    // Read theme colors
    let theme_fg = use_memo(move || theme.tokens.read().colors.foreground.clone());
    let theme_muted = use_memo(move || theme.tokens.read().colors.muted.clone());
    let theme_primary = use_memo(move || theme.tokens.read().colors.primary.clone());
    let theme_border = use_memo(move || theme.tokens.read().colors.border.clone());

    let variant_colors = if props.rich_colors {
        match props.toast.variant {
            SonnerVariant::Success => (Color::new(34, 197, 94), "✓"),
            SonnerVariant::Error => (Color::new(239, 68, 68), "✕"),
            SonnerVariant::Warning => (Color::new(245, 158, 11), "!"),
            SonnerVariant::Info => (Color::new(59, 130, 246), "i"),
            SonnerVariant::Loading => (Color::new(100, 116, 139), "◌"),
            SonnerVariant::Default => (theme_fg(), ""),
        }
    } else {
        (theme_fg(), "")
    };

    let toast_style = use_style(move |t| {
        Style::new()
            .pointer_events_auto()
            .bg(&t.colors.background)
            .rounded(&t.radius, "lg")
            .shadow(&t.shadows.lg)
            .p(&t.spacing, "md")
            .min_w_px(300)
            .relative()
            .overflow_hidden()
            .build()
    });

    let border_color =
        if props.rich_colors && !matches!(props.toast.variant, SonnerVariant::Default) {
            variant_colors.0.to_rgba()
        } else {
            theme_border().to_rgba()
        };

    let icon_color = variant_colors.0.clone();
    let icon_style = use_style(move |t| {
        Style::new()
            .flex()
            .items_center()
            .justify_center()
            .w_px(20)
            .h_px(20)
            .rounded(&t.radius, "full")
            .text_color(&icon_color)
            .font_size(12)
            .font_weight(600)
            .flex_shrink(0)
            .build()
    });

    let title_style = use_style(|t| {
        Style::new()
            .font_size(14)
            .font_weight(600)
            .text_color(&t.colors.foreground)
            .build()
    });

    let desc_style = use_style(|t| {
        Style::new()
            .font_size(13)
            .text_color(&t.colors.muted)
            .mt(&t.spacing, "xs")
            .build()
    });

    let variant_color = variant_colors.0.clone();
    let is_rich = props.rich_colors;
    let variant = props.toast.variant.clone();
    let progress_style = use_style(move |t| {
        let bg_color = if is_rich && !matches!(variant, SonnerVariant::Default) {
            variant_color.clone()
        } else {
            t.colors.primary.clone()
        };

        Style::new()
            .absolute()
            .bottom("0")
            .left("0")
            .h_px(3)
            .bg(&bg_color)
            .transition("width 0.1s linear")
            .build()
    });

    let handle_dismiss = move |_| {
        if let Some(on_dismiss) = props.on_dismiss.clone() {
            on_dismiss.call(toast_id.clone());
        }
    };

    let (_, icon_char) = &variant_colors;

    rsx! {
        div {
            id: "{props.id.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            style: "{toast_style} border: 1px solid {border_color};",

            div {
                style: "display: flex; gap: 12px; align-items: flex-start;",

                if !icon_char.is_empty() {
                    span { style: "{icon_style}", "{icon_char}" }
                }

                div { style: "flex: 1; min-width: 0;",
                    div { style: "{title_style}", "{props.toast.title}" }

                    if let Some(desc) = props.toast.description.clone() {
                        div { style: "{desc_style}", "{desc}" }
                    }

                    if let Some(action) = props.toast.action.clone() {
                        button {
                            style: "margin-top: 8px; padding: 4px 12px; font-size: 12px; font-weight: 500; color: {theme_primary().to_rgba()}; background: transparent; border: 1px solid {theme_border().to_rgba()}; border-radius: 4px; cursor: pointer;",
                            onclick: move |_| action.on_click.call(()),
                            "{action.label}"
                        }
                    }
                }

                if props.close_button {
                    button {
                        style: "padding: 4px; background: transparent; border: none; color: {theme_muted().to_rgba()}; cursor: pointer; font-size: 14px; line-height: 1;",
                        onclick: handle_dismiss,
                        "✕"
                    }
                }
            }

            div {
                style: "{progress_style} width: {progress()}%; {props.style.clone().unwrap_or_default()}",
            }
        }
    }
}

/// Hook for using Sonner/toast
pub fn use_sonner() -> UseSonner {
    let toasts = use_signal(Vec::new);

    UseSonner { toasts }
}

/// Sonner hook return type
#[derive(Clone, Copy)]
pub struct UseSonner {
    toasts: Signal<Vec<Toast>>,
}

impl UseSonner {
    /// Show a toast
    pub fn toast(&mut self, title: impl Into<String>) -> String {
        let id = generate_id();
        let toast = Toast {
            id: id.clone(),
            title: title.into(),
            description: None,
            variant: SonnerVariant::Default,
            duration: Duration::from_secs(5),
            action: None,
        };
        self.toasts.write().push(toast);
        id
    }

    /// Show success toast
    pub fn success(&mut self, title: impl Into<String>) -> String {
        let id = generate_id();
        let toast = Toast {
            id: id.clone(),
            title: title.into(),
            description: None,
            variant: SonnerVariant::Success,
            duration: Duration::from_secs(5),
            action: None,
        };
        self.toasts.write().push(toast);
        id
    }

    /// Show error toast
    pub fn error(&mut self, title: impl Into<String>) -> String {
        let id = generate_id();
        let toast = Toast {
            id: id.clone(),
            title: title.into(),
            description: None,
            variant: SonnerVariant::Error,
            duration: Duration::from_secs(5),
            action: None,
        };
        self.toasts.write().push(toast);
        id
    }

    /// Dismiss a toast
    pub fn dismiss(&mut self, id: &str) {
        self.toasts.write().retain(|t| t.id != id);
    }

    /// Get the toasts signal for reactive reading
    pub fn toasts_signal(&self) -> Signal<Vec<Toast>> {
        self.toasts
    }

    /// Get current toasts (for one-time reads)
    pub fn toasts(&self) -> Vec<Toast> {
        self.toasts.read().clone()
    }
}

fn generate_id() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    format!("toast-{}", COUNTER.fetch_add(1, Ordering::SeqCst))
}
