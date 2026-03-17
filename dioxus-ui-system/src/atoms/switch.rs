//! Switch atom component
//!
//! A control that allows the user to toggle between checked and not checked.

use dioxus::prelude::*;
use crate::theme::{use_theme, use_style};
use crate::styles::Style;

/// Switch properties
#[derive(Props, Clone, PartialEq)]
pub struct SwitchProps {
    /// Whether the switch is checked
    #[props(default)]
    pub checked: bool,
    /// Callback when checked state changes
    #[props(default)]
    pub onchange: Option<EventHandler<bool>>,
    /// Whether the switch is disabled
    #[props(default)]
    pub disabled: bool,
    /// Switch label
    #[props(default)]
    pub label: Option<String>,
    /// Switch size
    #[props(default)]
    pub size: SwitchSize,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Switch sizes
#[derive(Default, Clone, PartialEq)]
pub enum SwitchSize {
    /// Small switch
    Sm,
    /// Medium (default) switch
    #[default]
    Md,
    /// Large switch
    Lg,
}

/// Switch atom component
#[component]
pub fn Switch(props: SwitchProps) -> Element {
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
    let size = props.size.clone();
    
    // Size configurations
    let (width, height, thumb_size) = match size {
        SwitchSize::Sm => (32, 18, 14),
        SwitchSize::Md => (44, 24, 20),
        SwitchSize::Lg => (56, 30, 26),
    };
    
    let switch_style = use_style(move |t| {
        let base = Style::new()
            .w_px(width)
            .h_px(height)
            .rounded_full()
            .relative()
            .cursor(if disabled { "not-allowed" } else { "pointer" })
            .transition("all 150ms ease")
            .border(0, &t.colors.border);
        
        let styled = if checked {
            base.bg(&t.colors.primary)
        } else {
            base.bg(&t.colors.muted)
        };
        
        let styled = if is_hovered() && !disabled {
            if checked {
                styled.bg(&t.colors.primary.darken(0.1))
            } else {
                styled.bg(&t.colors.muted.darken(0.1))
            }
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
    
    let _thumb_offset = if checked { width - height + 2 } else { 2 };
    
    let thumb_style = use_style(move |t| {
        Style::new()
            .absolute()
            .top("2px")
            .left("{thumb_offset}px")
            .w_px(thumb_size)
            .h_px(thumb_size)
            .rounded_full()
            .bg(&t.colors.background)
            .transition("all 150ms ease")
            .shadow(&t.shadows.sm)
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
    
    let switch_element = rsx! {
        button {
            r#type: "button",
            role: "switch",
            aria_checked: "{checked}",
            disabled: disabled,
            style: "{switch_style} {props.style.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            onclick: handle_click,
            onmouseenter: move |_| if !disabled { is_hovered.set(true) },
            onmouseleave: move |_| is_hovered.set(false),
            onfocus: move |_| is_focused.set(true),
            onblur: move |_| is_focused.set(false),
            
            span {
                style: "{thumb_style}",
            }
        }
    };
    
    let label_element = if let Some(label_text) = props.label.clone() {
        rsx! {
            label {
                style: "display: flex; align-items: center; gap: 12px; cursor: {cursor_style};",
                {switch_element}
                span { 
                    style: "opacity: {opacity_style};",
                    "{label_text}" 
                }
            }
        }
    } else {
        switch_element
    };
    
    rsx! {
        {label_element}
    }
}
