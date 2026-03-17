//! Checkbox atom component
//!
//! A control that allows the user to toggle between checked and not checked.

use dioxus::prelude::*;
use crate::theme::{use_theme, use_style};
use crate::styles::Style;

/// Checkbox properties
#[derive(Props, Clone, PartialEq)]
pub struct CheckboxProps {
    /// Whether the checkbox is checked
    #[props(default)]
    pub checked: bool,
    /// Callback when checked state changes
    #[props(default)]
    pub onchange: Option<EventHandler<bool>>,
    /// Whether the checkbox is disabled
    #[props(default)]
    pub disabled: bool,
    /// Checkbox label
    #[props(default)]
    pub label: Option<String>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Checkbox atom component
#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    let _theme = use_theme();
    let mut is_checked = use_signal(|| props.checked);
    let mut is_hovered = use_signal(|| false);
    let mut is_focused = use_signal(|| false);
    
    // Sync with prop changes
    use_effect(move || {
        is_checked.set(props.checked);
    });
    
    let checked = is_checked();
    let disabled = props.disabled;
    let cursor_style = if disabled { "not-allowed" } else { "pointer" };
    let opacity_style = if disabled { "0.5" } else { "1" };
    
    let checkbox_style = use_style(move |t| {
        let base = Style::new()
            .w_px(20)
            .h_px(20)
            .rounded(&t.radius, "sm")
            .border(1, &t.colors.border)
            .flex()
            .items_center()
            .justify_center()
            .cursor("pointer")
            .transition("all 150ms ease");
        
        let styled = if checked {
            base.bg(&t.colors.primary)
                .border_color(&t.colors.primary)
        } else {
            base.bg(&t.colors.background)
        };
        
        let styled = if is_hovered() && !disabled && !checked {
            styled.border_color(&t.colors.primary)
        } else {
            styled
        };
        
        let styled = if is_focused() && !disabled {
            Style {
                box_shadow: Some(format!("0 0 0 2px {}", t.colors.ring.to_rgba())),
                ..styled
            }
        } else {
            styled
        };
        
        let styled = if disabled {
            styled.opacity(0.5)
        } else {
            styled
        };
        
        styled.build()
    });
    
    let checkmark_style = use_style(|t| {
        Style::new()
            .w_px(12)
            .h_px(12)
            .text_color(&t.colors.primary_foreground)
            .build()
    });
    
    let handle_click = move |_| {
        if !disabled {
            let new_checked = !is_checked();
            is_checked.set(new_checked);
            if let Some(handler) = &props.onchange {
                handler.call(new_checked);
            }
        }
    };
    
    let checkbox_element = rsx! {
        button {
            r#type: "button",
            role: "checkbox",
            aria_checked: "{checked}",
            disabled: disabled,
            style: "{checkbox_style} {props.style.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            onclick: handle_click,
            onmouseenter: move |_| if !disabled { is_hovered.set(true) },
            onmouseleave: move |_| is_hovered.set(false),
            onfocus: move |_| is_focused.set(true),
            onblur: move |_| is_focused.set(false),
            
            if checked {
                // Checkmark icon
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "3",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    style: "{checkmark_style}",
                    polyline { points: "20 6 9 17 4 12" }
                }
            }
        }
    };
    
    let label_element = if let Some(label_text) = props.label.clone() {
        rsx! {
            label {
                style: "display: flex; align-items: center; gap: 8px; cursor: {cursor_style}; opacity: {opacity_style};",
                {checkbox_element}
                span { "{label_text}" }
            }
        }
    } else {
        checkbox_element
    };
    
    rsx! {
        {label_element}
    }
}
