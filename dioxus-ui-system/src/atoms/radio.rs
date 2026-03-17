//! Radio button atom component
//!
//! A set of checkable buttons—known as radio buttons—where no more than one of the buttons can be checked at a time.

use dioxus::prelude::*;
use crate::theme::use_style;
use crate::styles::Style;

/// Radio properties
#[derive(Props, Clone, PartialEq)]
pub struct RadioProps {
    /// Whether the radio is selected
    #[props(default)]
    pub checked: bool,
    /// Callback when selected state changes
    #[props(default)]
    pub onchange: Option<EventHandler<()>>,
    /// Whether the radio is disabled
    #[props(default)]
    pub disabled: bool,
    /// Radio label
    #[props(default)]
    pub label: Option<String>,
    /// Radio value
    #[props(default)]
    pub value: String,
    /// Name for grouping radios
    #[props(default)]
    pub name: String,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Radio atom component
#[component]
pub fn Radio(props: RadioProps) -> Element {
    let mut is_hovered = use_signal(|| false);
    let mut is_focused = use_signal(|| false);
    
    let checked = props.checked;
    let disabled = props.disabled;
    let cursor_style = if disabled { "not-allowed" } else { "pointer" };
    
    let radio_style = use_style(move |t| {
        let base = Style::new()
            .w_px(20)
            .h_px(20)
            .rounded_full()
            .border(1, &t.colors.border)
            .flex()
            .items_center()
            .justify_center()
            .cursor("pointer")
            .transition("all 150ms ease");
        
        let styled = if checked {
            base.border(4, &t.colors.primary)
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
    
    let opacity_style = if disabled { "0.5" } else { "1" };
    
    let label_text_element = if let Some(label_text) = props.label.clone() {
        rsx! {
            span { 
                style: "opacity: {opacity_style};",
                "{label_text}" 
            }
        }
    } else {
        rsx! {}
    };
    
    rsx! {
        label {
            style: "display: flex; align-items: center; gap: 8px; cursor: {cursor_style}; position: relative;",
            
            input {
                r#type: "radio",
                name: "{props.name}",
                value: "{props.value}",
                checked: checked,
                disabled: disabled,
                style: "position: absolute; opacity: 0; width: 0; height: 0;",
                onchange: move |_| {
                    if !disabled && !checked {
                        if let Some(handler) = &props.onchange {
                            handler.call(());
                        }
                    }
                },
            }
            span {
                style: "{radio_style} {props.style.clone().unwrap_or_default()}",
                class: "{props.class.clone().unwrap_or_default()}",
                onmouseenter: move |_| if !disabled { is_hovered.set(true) },
                onmouseleave: move |_| is_hovered.set(false),
                onfocus: move |_| is_focused.set(true),
                onblur: move |_| is_focused.set(false),
            }
            {label_text_element}
        }
    }
}

/// Radio group properties
#[derive(Props, Clone, PartialEq)]
pub struct RadioGroupProps {
    /// Radio group name
    pub name: String,
    /// Currently selected value
    pub value: String,
    /// Callback when selection changes
    pub onchange: EventHandler<String>,
    /// Radio group children (Radio components)
    pub children: Element,
    /// Layout direction
    #[props(default)]
    pub direction: RadioDirection,
}

/// Radio group layout direction
#[derive(Default, Clone, PartialEq)]
pub enum RadioDirection {
    /// Horizontal layout
    #[default]
    Horizontal,
    /// Vertical layout
    Vertical,
}

/// Radio group component for managing a set of radio buttons
#[component]
pub fn RadioGroup(props: RadioGroupProps) -> Element {
    let direction = match props.direction {
        RadioDirection::Horizontal => "row",
        RadioDirection::Vertical => "column",
    };
    
    rsx! {
        div {
            style: "display: flex; flex-direction: {direction}; gap: 12px;",
            {props.children}
        }
    }
}
