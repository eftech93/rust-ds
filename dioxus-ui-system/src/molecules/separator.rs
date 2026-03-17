//! Separator molecule component
//!
//! Visually or semantically separates content.

use dioxus::prelude::*;
use crate::theme::{use_theme, use_style};
use crate::styles::Style;

/// Separator orientation
#[derive(Default, Clone, PartialEq)]
pub enum SeparatorOrientation {
    /// Horizontal separator (default)
    #[default]
    Horizontal,
    /// Vertical separator
    Vertical,
}

/// Separator properties
#[derive(Props, Clone, PartialEq)]
pub struct SeparatorProps {
    /// Separator orientation
    #[props(default)]
    pub orientation: SeparatorOrientation,
    /// Whether the separator is decorative only
    #[props(default = true)]
    pub decorative: bool,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Separator molecule component
#[component]
pub fn Separator(props: SeparatorProps) -> Element {
    let _theme = use_theme();
    
    let is_horizontal = props.orientation == SeparatorOrientation::Horizontal;
    
    let separator_style = use_style(move |t| {
        if is_horizontal {
            Style::new()
                .w_full()
                .h_px(1)
                .bg(&t.colors.border)
                .my(&t.spacing, "md")
                .build()
        } else {
            Style::new()
                .w_px(1)
                .h_full()
                .bg(&t.colors.border)
                .mx(&t.spacing, "md")
                .build()
        }
    });
    
    let aria_props = if props.decorative {
        "aria-hidden: true;"
    } else {
        "role: separator;"
    };
    
    let orientation_attr = if is_horizontal { "horizontal" } else { "vertical" };
    
    rsx! {
        div {
            style: "{separator_style} {props.style.clone().unwrap_or_default()} {aria_props}",
            class: "{props.class.clone().unwrap_or_default()}",
            aria_orientation: "{orientation_attr}",
        }
    }
}
