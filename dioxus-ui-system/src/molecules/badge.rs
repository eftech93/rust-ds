//! Badge molecule component
//!
//! Small status indicators for highlighting items.

use dioxus::prelude::*;
use crate::theme::{use_theme, use_style};
use crate::styles::Style;
use crate::atoms::{Icon, IconSize, IconColor};

/// Badge variants
#[derive(Default, Clone, PartialEq)]
pub enum BadgeVariant {
    /// Default badge
    #[default]
    Default,
    /// Secondary badge
    Secondary,
    /// Success badge
    Success,
    /// Warning badge
    Warning,
    /// Destructive/error badge
    Destructive,
    /// Outline badge
    Outline,
    /// Ghost/subtle badge
    Ghost,
}

/// Badge sizes
#[derive(Default, Copy, Clone, PartialEq)]
pub enum BadgeSize {
    /// Small badge
    Sm,
    /// Medium (default) badge
    #[default]
    Md,
    /// Large badge
    Lg,
}

/// Badge properties
#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    /// Badge content
    pub children: Element,
    /// Visual variant
    #[props(default)]
    pub variant: BadgeVariant,
    /// Badge size
    #[props(default)]
    pub size: BadgeSize,
    /// Optional leading icon
    #[props(default)]
    pub icon: Option<String>,
    /// Click handler (makes badge interactive)
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Badge molecule component
///
/// # Example
/// ```rust,ignore
/// use dioxus_ui_system::molecules::{Badge, BadgeVariant};
///
/// rsx! {
///     Badge {
///         variant: BadgeVariant::Success,
///         icon: "check".to_string(),
///         "Active"
///     }
/// }
/// ```
#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let theme = use_theme();
    let variant = props.variant.clone();
    let size = props.size.clone();
    let has_icon = props.icon.is_some();
    let has_onclick = props.onclick.is_some();
    
    // Interactive state
    let mut is_hovered = use_signal(|| false);
    
    let style = use_style(move |t| {
        let base = Style::new()
            .inline_flex()
            .items_center()
            .justify_center()
            .rounded(&t.radius, "full")
            .font_weight(600)
            .transition("all 150ms ease")
            .whitespace_nowrap()
            .select_none();
            
        // Size
        let sized = match size {
            BadgeSize::Sm => base.px(&t.spacing, "sm").py_px(2).font_size(11),
            BadgeSize::Md => base.px(&t.spacing, "md").py_px(4).font_size(12),
            BadgeSize::Lg => base.px(&t.spacing, "md").py(&t.spacing, "xs").font_size(14),
        };
        
        // Variant styles
        let styled = match variant {
            BadgeVariant::Default => {
                let bg = if is_hovered() && has_onclick {
                    t.colors.primary.darken(0.1)
                } else {
                    t.colors.primary.clone()
                };
                sized
                    .bg(&bg)
                    .text_color(&t.colors.primary_foreground)
            }
            BadgeVariant::Secondary => {
                let bg = if is_hovered() && has_onclick {
                    t.colors.secondary.darken(0.05)
                } else {
                    t.colors.secondary.clone()
                };
                sized
                    .bg(&bg)
                    .text_color(&t.colors.secondary_foreground)
            }
            BadgeVariant::Success => {
                let bg = t.colors.success.clone();
                let fg = if is_dark_color(&bg) {
                    Color::new(255, 255, 255)
                } else {
                    Color::new(0, 0, 0)
                };
                sized
                    .bg(&bg)
                    .text_color(&fg)
            }
            BadgeVariant::Warning => {
                let bg = t.colors.warning.clone();
                let fg = Color::new(0, 0, 0);
                sized
                    .bg(&bg)
                    .text_color(&fg)
            }
            BadgeVariant::Destructive => {
                let bg = if is_hovered() && has_onclick {
                    t.colors.destructive.darken(0.1)
                } else {
                    t.colors.destructive.clone()
                };
                sized
                    .bg(&bg)
                    .text_color(&Color::new(255, 255, 255))
            }
            BadgeVariant::Outline => {
                let border_color = if is_hovered() && has_onclick {
                    t.colors.foreground.darken(0.2)
                } else {
                    t.colors.border.clone()
                };
                sized
                    .border(1, &border_color)
                    .text_color(&t.colors.foreground)
            }
            BadgeVariant::Ghost => {
                let bg = if is_hovered() && has_onclick {
                    t.colors.muted.clone()
                } else {
                    Color::new_rgba(0, 0, 0, 0.0)
                };
                sized
                    .bg(&bg)
                    .text_color(&t.colors.foreground)
            }
        };
        
        // Cursor
        if has_onclick {
            styled.cursor_pointer().build()
        } else {
            styled.build()
        }
    });
    
    let final_style = if let Some(custom) = &props.style {
        format!("{} {}", style(), custom)
    } else {
        style()
    };
    
    let class = props.class.clone().unwrap_or_default();
    let icon_element = props.icon.clone();
    let onclick_handler = props.onclick.clone();
    
    rsx! {
        span {
            style: "{final_style}",
            class: "{class}",
            onmouseenter: move |_| if has_onclick { is_hovered.set(true) },
            onmouseleave: move |_| is_hovered.set(false),
            onclick: move |e| {
                if let Some(handler) = &onclick_handler {
                    handler.call(e);
                }
            },
            
            if has_icon {
                span {
                    style: "margin-right: 4px; display: inline-flex;",
                    Icon {
                        name: icon_element.unwrap(),
                        size: match size {
                            BadgeSize::Sm => IconSize::ExtraSmall,
                            BadgeSize::Md => IconSize::Small,
                            BadgeSize::Lg => IconSize::Medium,
                        },
                        color: IconColor::Current,
                    }
                }
            }
            
            {props.children}
        }
    }
}

use crate::theme::tokens::Color;

/// Determine if a color is dark
fn is_dark_color(color: &Color) -> bool {
    let luminance = (0.299 * color.r as f32 + 0.587 * color.g as f32 + 0.114 * color.b as f32) / 255.0;
    luminance < 0.5
}

/// Status badge - convenience component for common status indicators
#[derive(Props, Clone, PartialEq)]
pub struct StatusBadgeProps {
    /// Status text
    pub status: String,
    /// Status type
    #[props(default)]
    pub status_type: StatusType,
}

/// Status types
#[derive(Default, Clone, PartialEq)]
pub enum StatusType {
    #[default]
    Default,
    Info,
    Success,
    Warning,
    Error,
}

/// Status Badge component
#[component]
pub fn StatusBadge(props: StatusBadgeProps) -> Element {
    let (variant, icon) = match props.status_type {
        StatusType::Default => (BadgeVariant::Default, None),
        StatusType::Info => (BadgeVariant::Secondary, Some("info".to_string())),
        StatusType::Success => (BadgeVariant::Success, Some("check".to_string())),
        StatusType::Warning => (BadgeVariant::Warning, Some("alert".to_string())),
        StatusType::Error => (BadgeVariant::Destructive, Some("x".to_string())),
    };
    
    rsx! {
        Badge {
            variant: variant,
            icon: icon,
            "{props.status}"
        }
    }
}
