//! Icon atom component
//!
//! SVG icon component with theme integration and sizing options.

use crate::styles::Style;
use crate::theme::tokens::Color;
use crate::theme::{use_style, use_theme};
use dioxus::prelude::*;

/// Icon sizes
#[derive(Default, Copy, Clone, PartialEq)]
pub enum IconSize {
    ExtraSmall, // 12px
    Small,      // 16px
    #[default]
    Medium, // 20px
    Large,      // 24px
    ExtraLarge, // 32px
}

impl IconSize {
    pub fn to_px(&self) -> u16 {
        match self {
            IconSize::ExtraSmall => 12,
            IconSize::Small => 16,
            IconSize::Medium => 20,
            IconSize::Large => 24,
            IconSize::ExtraLarge => 32,
        }
    }
}

/// Icon color options
#[derive(Default, Clone, PartialEq)]
pub enum IconColor {
    #[default]
    Current, // Uses currentColor (inherits from parent)
    Primary,
    Secondary,
    Muted,
    Destructive,
    Success,
    Warning,
    Inverse,
    Custom(Color),
}

/// Icon properties
#[derive(Props, Clone, PartialEq)]
pub struct IconProps {
    /// SVG path data or icon name
    pub name: String,
    /// Icon size
    #[props(default)]
    pub size: IconSize,
    /// Icon color
    #[props(default)]
    pub color: IconColor,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
    /// Accessibility label
    #[props(default)]
    pub aria_label: Option<String>,
    /// Flip horizontally
    #[props(default)]
    pub flip_h: bool,
    /// Flip vertically
    #[props(default)]
    pub flip_v: bool,
    /// Rotate degrees (0, 90, 180, 270)
    #[props(default)]
    pub rotate: u16,
}

/// Icon atom component
///
/// Renders an SVG icon. The icon name can be:
/// - A preset icon name (e.g., "check", "x", "arrow-right")
/// - Raw SVG path data
///
/// # Example
/// ```rust,ignore
/// use dioxus_ui_system::atoms::{Icon, IconSize, IconColor};
///
/// rsx! {
///     Icon {
///         name: "check".to_string(),
///         size: IconSize::Medium,
///         color: IconColor::Success,
///     }
/// }
/// ```
#[component]
pub fn Icon(props: IconProps) -> Element {
    let _theme = use_theme();

    let size = props.size.clone();
    let color = props.color.clone();
    let flip_h = props.flip_h;
    let flip_v = props.flip_v;
    let rotate = props.rotate;

    // Memoized styles
    let style = use_style(move |t| {
        let base = Style::new()
            .inline_flex()
            .items_center()
            .justify_center()
            .w_px(size.to_px())
            .h_px(size.to_px())
            .transition("color 150ms ease");

        // Apply color
        let base = match &color {
            IconColor::Current => base,
            IconColor::Primary => base.text_color(&t.colors.primary),
            IconColor::Secondary => base.text_color(&t.colors.secondary_foreground),
            IconColor::Muted => base.text_color(&t.colors.muted_foreground),
            IconColor::Destructive => base.text_color(&t.colors.destructive),
            IconColor::Success => base.text_color(&t.colors.success),
            IconColor::Warning => base.text_color(&t.colors.warning),
            IconColor::Inverse => base.text_color(&t.colors.background),
            IconColor::Custom(c) => base.text_color(c),
        };

        // Transforms
        let mut transform = String::new();

        if flip_h {
            transform.push_str("scaleX(-1) ");
        }
        if flip_v {
            transform.push_str("scaleY(-1) ");
        }
        if rotate != 0 {
            transform.push_str(&format!("rotate({}deg)", rotate));
        }

        if !transform.is_empty() {
            Style {
                transform: Some(transform.trim().to_string()),
                ..base
            }
            .build()
        } else {
            base.build()
        }
    });

    // Combine with custom styles
    let final_style = if let Some(custom) = &props.style {
        format!("{} {}", style(), custom)
    } else {
        style()
    };

    let class = props.class.clone().unwrap_or_default();
    let aria_label = props.aria_label.clone();

    // Get SVG content
    let svg_content = get_icon_svg(&props.name);
    let px = size.to_px();
    let view_box = get_icon_viewbox(&props.name);

    rsx! {
        svg {
            style: "{final_style}",
            class: "{class}",
            width: "{px}",
            height: "{px}",
            view_box: "{view_box}",
            fill: "currentColor",
            role: if aria_label.is_some() { "img" } else { "presentation" },
            dangerous_inner_html: "{svg_content}",
        }
    }
}

/// Get SVG path data for preset icons
fn get_icon_svg(name: &str) -> String {
    match name {
        // Common icons
        "check" => r#"<path d="M20 6L9 17l-5-5" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "x" | "close" => r#"<path d="M18 6L6 18M6 6l12 12" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "plus" => r#"<path d="M12 5v14M5 12h14" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "minus" => r#"<path d="M5 12h14" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "arrow-left" => r#"<path d="M19 12H5M12 19l-7-7 7-7" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "arrow-right" => r#"<path d="M5 12h14M12 5l7 7-7 7" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "arrow-up" => r#"<path d="M12 19V5M5 12l7-7 7 7" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "arrow-down" => r#"<path d="M12 5v14M19 12l-7 7-7-7" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "chevron-left" => r#"<path d="M15 18l-6-6 6-6" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "chevron-right" => r#"<path d="M9 18l6-6-6-6" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "chevron-up" => r#"<path d="M18 15l-6-6-6 6" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "chevron-down" => r#"<path d="M6 9l6 6 6-6" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "menu" => r#"<path d="M3 12h18M3 6h18M3 18h18" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "search" => r#"<circle cx="11" cy="11" r="8" stroke="currentColor" stroke-width="2" fill="none"/><path d="M21 21l-4.35-4.35" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "user" => r#"<path d="M20 21v-2a4 4 0 00-4-4H8a4 4 0 00-4 4v2" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><circle cx="12" cy="7" r="4" stroke="currentColor" stroke-width="2" fill="none"/>"#,
        "settings" => r#"<circle cx="12" cy="12" r="3" stroke="currentColor" stroke-width="2" fill="none"/><path d="M19.4 15a1.65 1.65 0 00.33 1.82l.06.06a2 2 0 010 2.83 2 2 0 01-2.83 0l-.06-.06a1.65 1.65 0 00-1.82-.33 1.65 1.65 0 00-1 1.51V21a2 2 0 01-2 2 2 2 0 01-2-2v-.09A1.65 1.65 0 009 19.4a1.65 1.65 0 00-1.82.33l-.06.06a2 2 0 01-2.83 0 2 2 0 010-2.83l.06-.06a1.65 1.65 0 00.33-1.82 1.65 1.65 0 00-1.51-1H3a2 2 0 01-2-2 2 2 0 012-2h.09A1.65 1.65 0 004.6 9a1.65 1.65 0 00-.33-1.82l-.06-.06a2 2 0 010-2.83 2 2 0 012.83 0l.06.06a1.65 1.65 0 001.82.33H9a1.65 1.65 0 001-1.51V3a2 2 0 012-2 2 2 0 012 2v.09a1.65 1.65 0 001 1.51 1.65 1.65 0 001.82-.33l.06-.06a2 2 0 012.83 0 2 2 0 010 2.83l-.06.06a1.65 1.65 0 00-.33 1.82V9a1.65 1.65 0 001.51 1H21a2 2 0 012 2 2 2 0 01-2 2h-.09a1.65 1.65 0 00-1.51 1z" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "home" => r#"<path d="M3 9l9-7 9 7v11a2 2 0 01-2 2H5a2 2 0 01-2-2z" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><polyline points="9 22 9 12 15 12 15 22" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "bell" => r#"<path d="M18 8A6 6 0 006 8c0 7-3 9-3 9h18s-3-2-3-9" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><path d="M13.73 21a2 2 0 01-3.46 0" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "heart" => r#"<path d="M20.84 4.61a5.5 5.5 0 00-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 00-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 000-7.78z" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "star" => r#"<polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "trash" => r#"<polyline points="3 6 5 6 21 6" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><path d="M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "edit" => r#"<path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "copy" => r#"<rect x="9" y="9" width="13" height="13" rx="2" ry="2" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "external-link" => r#"<path d="M18 13v6a2 2 0 01-2 2H5a2 2 0 01-2-2V8a2 2 0 012-2h6" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><polyline points="15 3 21 3 21 9" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><line x1="10" y1="14" x2="21" y2="3" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "loading" | "spinner" => r#"<path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "info" => r#"<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><line x1="12" y1="16" x2="12" y2="12" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><line x1="12" y1="8" x2="12.01" y2="8" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "warning" | "alert" => r#"<path d="M10.29 3.86L1.82 18a2 2 0 001.71 3h16.94a2 2 0 001.71-3L13.71 3.86a2 2 0 00-3.42 0z" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><line x1="12" y1="9" x2="12" y2="13" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><line x1="12" y1="17" x2="12.01" y2="17" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "error" | "alert-circle" => r#"<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><line x1="15" y1="9" x2="9" y2="15" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><line x1="9" y1="9" x2="15" y2="15" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "moon" => r#"<path d="M21 12.79A9 9 0 1111.21 3 7 7 0 0021 12.79z" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "sun" => r#"<circle cx="12" cy="12" r="5" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><line x1="12" y1="1" x2="12" y2="3" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><line x1="12" y1="21" x2="12" y2="23" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><line x1="4.22" y1="4.22" x2="5.64" y2="5.64" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><line x1="18.36" y1="18.36" x2="19.78" y2="19.78" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><line x1="1" y1="12" x2="3" y2="12" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><line x1="21" y1="12" x2="23" y2="12" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><line x1="4.22" y1="19.78" x2="5.64" y2="18.36" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        // Layout icons
        "book" => r#"<path d="M4 19.5A2.5 2.5 0 016.5 17H20" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><path d="M6.5 2H20v20H6.5A2.5 2.5 0 014 19.5v-15A2.5 2.5 0 016.5 2z" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "layout" => r#"<rect x="3" y="3" width="18" height="18" rx="2" ry="2" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><line x1="3" y1="9" x2="21" y2="9" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><line x1="9" y1="21" x2="9" y2="9" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "sidebar" => r#"<rect x="3" y="3" width="18" height="18" rx="2" ry="2" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><line x1="9" y1="3" x2="9" y2="21" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "maximize" => r#"<path d="M8 3H5a2 2 0 00-2 2v3m18 0V5a2 2 0 00-2-2h-3m0 18h3a2 2 0 002-2v-3M3 16v3a2 2 0 002 2h3" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "box" => r#"<path d="M21 16V8a2 2 0 00-1-1.73l-7-4a2 2 0 00-2 0l-7 4A2 2 0 003 8v8a2 2 0 001 1.73l7 4a2 2 0 002 0l7-4A2 2 0 0021 16z" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><polyline points="3.27 6.96 12 12.01 20.73 6.96" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><line x1="12" y1="22.08" x2="12" y2="12" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "smartphone" => r#"<rect x="5" y="2" width="14" height="20" rx="2" ry="2" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><line x1="12" y1="18" x2="12.01" y2="18" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "palette" => r#"<circle cx="13.5" cy="6.5" r=".5" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><circle cx="17.5" cy="10.5" r=".5" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><circle cx="8.5" cy="7.5" r=".5" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><circle cx="6.5" cy="12.5" r=".5" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><path d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.926 0 1.648-.746 1.648-1.688 0-.437-.18-.835-.437-1.125-.29-.289-.438-.652-.438-1.042a1.66 1.66 0 011.668-1.668h1.996c3.051 0 5.555-2.503 5.555-5.554C21.965 6.01 17.461 2 12 2z" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "share" => r#"<path d="M4 12v8a2 2 0 002 2h12a2 2 0 002-2v-8" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><polyline points="16 6 12 2 8 6" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><line x1="12" y1="2" x2="12" y2="15" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "x-circle" => r#"<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><line x1="15" y1="9" x2="9" y2="15" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><line x1="9" y1="9" x2="15" y2="15" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        "play-circle" => r#"<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/><polygon points="10 8 16 12 10 16 10 8" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"#,
        // Default: return the name as raw SVG path data
        _ => name,
    }.to_string()
}

/// Get viewBox for preset icons
fn get_icon_viewbox(name: &str) -> &'static str {
    match name {
        "check" | "x" | "close" | "plus" | "minus" | "arrow-left" | "arrow-right" | "arrow-up"
        | "arrow-down" | "chevron-left" | "chevron-right" | "chevron-up" | "chevron-down"
        | "menu" | "search" | "user" | "settings" | "home" | "bell" | "heart" | "star"
        | "trash" | "edit" | "copy" | "external-link" | "loading" | "spinner" | "info"
        | "warning" | "alert" | "error" | "alert-circle" | "moon" | "sun" | "book" | "layout"
        | "sidebar" | "maximize" | "box" | "smartphone" | "palette" => "0 0 24 24",
        // Default
        _ => "0 0 24 24",
    }
}

/// Icon button component (combines Icon with Button behavior)
#[derive(Props, Clone, PartialEq)]
pub struct IconButtonProps {
    pub icon: String,
    #[props(default)]
    pub size: IconSize,
    #[props(default)]
    pub color: IconColor,
    #[props(default)]
    pub disabled: bool,
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    pub aria_label: String,
    #[props(default)]
    pub style: Option<String>,
    #[props(default)]
    pub class: Option<String>,
}

#[component]
pub fn IconButton(props: IconButtonProps) -> Element {
    let class = format!("icon-button {}", props.class.clone().unwrap_or_default());

    rsx! {
        button {
            class: "{class}",
            style: props.style.clone().unwrap_or_default(),
            disabled: props.disabled,
            aria_label: props.aria_label.clone(),
            onclick: move |e| {
                if let Some(handler) = &props.onclick {
                    if !props.disabled {
                        handler.call(e);
                    }
                }
            },
            Icon {
                name: props.icon.clone(),
                size: props.size.clone(),
                color: props.color.clone(),
            }
        }
    }
}
