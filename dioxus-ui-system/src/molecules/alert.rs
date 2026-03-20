//! Alert molecule component
//!
//! Displays a callout for user attention.

use dioxus::prelude::*;
use crate::theme::{use_theme, use_style};
use crate::styles::Style;
use crate::atoms::Box;

/// Alert variants
#[derive(Default, Clone, PartialEq)]
pub enum AlertVariant {
    /// Default alert
    #[default]
    Default,
    /// Destructive alert
    Destructive,
    /// Success alert
    Success,
    /// Warning alert
    Warning,
    /// Info alert
    Info,
}

/// Alert properties
#[derive(Props, Clone, PartialEq)]
pub struct AlertProps {
    /// Alert content
    pub children: Element,
    /// Alert variant
    #[props(default)]
    pub variant: AlertVariant,
    /// Optional title
    #[props(default)]
    pub title: Option<String>,
    /// Optional icon name
    #[props(default)]
    pub icon: Option<String>,
    /// Whether alert is dismissible
    #[props(default)]
    pub dismissible: bool,
    /// Callback when dismissed
    #[props(default)]
    pub on_dismiss: Option<EventHandler<()>>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Alert molecule component
#[component]
pub fn Alert(props: AlertProps) -> Element {
    let _theme = use_theme();
    let mut is_visible = use_signal(|| true);
    
    if !is_visible() {
        return rsx! {};
    }
    
    let variant = props.variant.clone();
    
    let alert_style = use_style(move |t| {
        let (bg_color, border_color, _text_color) = match variant {
            AlertVariant::Default => (
                &t.colors.background,
                &t.colors.border,
                &t.colors.foreground,
            ),
            AlertVariant::Destructive => (
                &t.colors.destructive.lighten(0.9),
                &t.colors.destructive,
                &t.colors.destructive,
            ),
            AlertVariant::Success => (
                &t.colors.success.lighten(0.9),
                &t.colors.success,
                &t.colors.success.darken(0.2),
            ),
            AlertVariant::Warning => (
                &t.colors.warning.lighten(0.9),
                &t.colors.warning,
                &t.colors.warning.darken(0.2),
            ),
            AlertVariant::Info => (
                &t.colors.primary.lighten(0.9),
                &t.colors.primary,
                &t.colors.primary,
            ),
        };
        
        Style::new()
            .w_full()
            .rounded(&t.radius, "lg")
            .border(1, border_color)
            .bg(bg_color)
            .p(&t.spacing, "md")
            .build()
    });
    
    let icon_style = use_style(|_t| {
        Style::new()
            .w_px(20)
            .h_px(20)
            .flex_shrink(0)
            .build()
    });
    
    let mut handle_dismiss = move || {
        is_visible.set(false);
        if let Some(handler) = &props.on_dismiss {
            handler.call(());
        }
    };
    
    // Default icons based on variant
    let default_icon = match props.variant {
        AlertVariant::Default => None,
        AlertVariant::Destructive => Some("alert-triangle"),
        AlertVariant::Success => Some("check-circle"),
        AlertVariant::Warning => Some("alert-triangle"),
        AlertVariant::Info => Some("info"),
    };
    
    let icon_name = props.icon.as_deref().or(default_icon);
    let custom_style = props.style.clone().unwrap_or_default();
    let custom_class = props.class.clone().unwrap_or_default();
    
    rsx! {
        div {
            role: "alert",
            style: "{alert_style} {custom_style} display: flex; align-items: flex-start; gap: 12px;",
            class: "{custom_class}",
            
            if let Some(icon) = icon_name {
                AlertIcon { name: icon.to_string(), style: icon_style() }
            }
            
            div {
                style: "flex: 1;",
                
                if let Some(title) = props.title {
                    h5 {
                        style: "margin: 0 0 4px 0; font-size: 14px; font-weight: 600;",
                        "{title}"
                    }
                }
                
                Box {
                    style: "font-size: 14px; line-height: 1.5;",
                    {props.children}
                }
            }
            
            if props.dismissible {
                button {
                    style: "background: none; border: none; cursor: pointer; padding: 4px; opacity: 0.5; transition: opacity 150ms;",
                    onmouseenter: move |e| e.stop_propagation(),
                    onclick: move |_| handle_dismiss(),
                    "✕"
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct AlertIconProps {
    name: String,
    style: String,
}

#[component]
fn AlertIcon(props: AlertIconProps) -> Element {
    // Map icon names to SVG paths
    let svg_content = match props.name.as_str() {
        "alert-triangle" => rsx! {
            svg {
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                style: "{props.style}",
                path { d: "m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z" }
                line { x1: "12", y1: "9", x2: "12", y2: "13" }
                line { x1: "12", y1: "17", x2: "12.01", y2: "17" }
            }
        },
        "check-circle" => rsx! {
            svg {
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                style: "{props.style}",
                path { d: "M22 11.08V12a10 10 0 1 1-5.93-9.14" }
                polyline { points: "22 4 12 14.01 9 11.01" }
            }
        },
        "info" => rsx! {
            svg {
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                style: "{props.style}",
                circle { cx: "12", cy: "12", r: "10" }
                line { x1: "12", y1: "16", x2: "12", y2: "12" }
                line { x1: "12", y1: "8", x2: "12.01", y2: "8" }
            }
        },
        _ => rsx! {}
    };
    
    svg_content
}
