//! Tooltip molecule component
//!
//! A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it.

use dioxus::prelude::*;
use crate::theme::{use_theme, use_style};
use crate::styles::Style;


/// Tooltip properties
#[derive(Props, Clone, PartialEq)]
pub struct TooltipProps {
    /// Trigger element (the element that shows the tooltip)
    pub children: Element,
    /// Tooltip content
    pub content: String,
    /// Tooltip placement
    #[props(default)]
    pub placement: TooltipPlacement,
    /// Delay before showing tooltip (in ms)
    #[props(default = 200)]
    pub delay: u64,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Tooltip placement options
#[derive(Default, Clone, PartialEq)]
pub enum TooltipPlacement {
    /// Top placement (default)
    #[default]
    Top,
    /// Top-start placement
    TopStart,
    /// Top-end placement
    TopEnd,
    /// Right placement
    Right,
    /// Right-start placement
    RightStart,
    /// Right-end placement
    RightEnd,
    /// Bottom placement
    Bottom,
    /// Bottom-start placement
    BottomStart,
    /// Bottom-end placement
    BottomEnd,
    /// Left placement
    Left,
    /// Left-start placement
    LeftStart,
    /// Left-end placement
    LeftEnd,
}

/// Tooltip component
#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    let _theme = use_theme();
    let mut is_visible = use_signal(|| false);
    let show_timeout = use_signal(|| None::<i32>);
    
    let position_style = match props.placement {
        TooltipPlacement::Top => "bottom: calc(100% + 6px); left: 50%; transform: translateX(-50%);",
        TooltipPlacement::TopStart => "bottom: calc(100% + 6px); left: 0;",
        TooltipPlacement::TopEnd => "bottom: calc(100% + 6px); right: 0;",
        TooltipPlacement::Right => "left: calc(100% + 6px); top: 50%; transform: translateY(-50%);",
        TooltipPlacement::RightStart => "left: calc(100% + 6px); top: 0;",
        TooltipPlacement::RightEnd => "left: calc(100% + 6px); bottom: 0;",
        TooltipPlacement::Bottom => "top: calc(100% + 6px); left: 50%; transform: translateX(-50%);",
        TooltipPlacement::BottomStart => "top: calc(100% + 6px); left: 0;",
        TooltipPlacement::BottomEnd => "top: calc(100% + 6px); right: 0;",
        TooltipPlacement::Left => "right: calc(100% + 6px); top: 50%; transform: translateY(-50%);",
        TooltipPlacement::LeftStart => "right: calc(100% + 6px); top: 0;",
        TooltipPlacement::LeftEnd => "right: calc(100% + 6px); bottom: 0;",
    };
    
    let tooltip_style = use_style(|t| {
        Style::new()
            .absolute()
            .px(&t.spacing, "sm")
            .py(&t.spacing, "xs")
            .rounded(&t.radius, "md")
            .bg(&t.colors.foreground)
            .text_color(&t.colors.background)
            .font_size(12)
            .font_weight(500)
            .whitespace_nowrap()
            .z_index(100)
            .build()
    });
    
    let mut show_tooltip = move || {
        // Use set_timeout for delay (simplified)
        // In a real implementation, you'd use web_sys::set_timeout_with_callback
        // For now, we show immediately
        is_visible.set(true);
    };
    
    let mut hide_tooltip = move || {
        if let Some(timeout) = show_timeout() {
            // Clear timeout would go here
            let _ = timeout;
        }
        is_visible.set(false);
    };
    
    rsx! {
        span {
            style: "position: relative; display: inline-block;",
            onmouseenter: move |_| show_tooltip(),
            onmouseleave: move |_| hide_tooltip(),
            onfocus: move |_| show_tooltip(),
            onblur: move |_| hide_tooltip(),
            
            {props.children}
            
            if is_visible() {
                div {
                    role: "tooltip",
                    style: "{tooltip_style} {position_style} {props.style.clone().unwrap_or_default()}",
                    "{props.content}"
                }
            }
        }
    }
}

/// Simple tooltip that wraps any element
#[derive(Props, Clone, PartialEq)]
pub struct SimpleTooltipProps {
    /// Element to wrap
    pub children: Element,
    /// Tooltip text
    pub text: String,
    /// Tooltip placement
    #[props(default)]
    pub placement: TooltipPlacement,
}

/// Simple tooltip wrapper
#[component]
pub fn SimpleTooltip(props: SimpleTooltipProps) -> Element {
    rsx! {
        Tooltip {
            content: props.text,
            placement: props.placement,
            {props.children}
        }
    }
}
