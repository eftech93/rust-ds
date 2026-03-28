//! Number Input atom component
//!
//! An input with +/- buttons for incrementing/decrementing numeric values.

use crate::styles::Style;
use crate::theme::{use_style, use_theme};
use dioxus::prelude::*;

/// Number input properties
#[derive(Props, Clone, PartialEq)]
pub struct NumberInputProps {
    /// Current value
    #[props(default)]
    pub value: f64,
    /// Callback when value changes
    pub on_change: EventHandler<f64>,
    /// Minimum value
    #[props(default)]
    pub min: Option<f64>,
    /// Maximum value
    #[props(default)]
    pub max: Option<f64>,
    /// Step increment
    #[props(default = 1.0)]
    pub step: f64,
    /// Decimal precision
    #[props(default)]
    pub precision: Option<usize>,
    /// Placeholder text
    #[props(default)]
    pub placeholder: Option<String>,
    /// Disabled state
    #[props(default)]
    pub disabled: bool,
    /// Label text
    #[props(default)]
    pub label: Option<String>,
    /// Error message
    #[props(default)]
    pub error: Option<String>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Number input component with increment/decrement buttons
#[component]
pub fn NumberInput(props: NumberInputProps) -> Element {
    let _theme = use_theme();

    let container_style = use_style(|_t| Style::new().flex().flex_col().w_full().build());

    let input_wrapper_style = use_style(|t| {
        Style::new()
            .flex()
            .items_center()
            .border(1, &t.colors.border)
            .rounded(&t.radius, "md")
            .bg(&t.colors.background)
            .overflow_hidden()
            .build()
    });

    let button_style = use_style(|t| {
        Style::new()
            .flex()
            .items_center()
            .justify_center()
            .w_px(32)
            .h_full()
            .bg(&t.colors.muted)
            .cursor_pointer()
            .text_color(&t.colors.foreground)
            .font_size(16)
            .transition("background 0.15s ease")
            .build()
    });

    let input_style = use_style(|t| {
        Style::new()
            .flex()
            .min_w_px(60)
            .p(&t.spacing, "sm")
            .text_color(&t.colors.foreground)
            .font_size(14)
            .text_align("center")
            .build()
    });

    let label_style = use_style(|t| {
        Style::new()
            .block()
            .mb(&t.spacing, "xs")
            .font_size(14)
            .font_weight(500)
            .text_color(&t.colors.foreground)
            .build()
    });

    let error_style = use_style(|t| {
        Style::new()
            .mt(&t.spacing, "xs")
            .font_size(12)
            .text_color(&t.colors.destructive)
            .build()
    });

    let handle_increment = move |_| {
        if props.disabled {
            return;
        }
        let new_value = props.value + props.step;
        let clamped = if let Some(max) = props.max {
            new_value.min(max)
        } else {
            new_value
        };
        let formatted = format_value(clamped, props.precision);
        props.on_change.call(formatted);
    };

    let handle_decrement = move |_| {
        if props.disabled {
            return;
        }
        let new_value = props.value - props.step;
        let clamped = if let Some(min) = props.min {
            new_value.max(min)
        } else {
            new_value
        };
        let formatted = format_value(clamped, props.precision);
        props.on_change.call(formatted);
    };

    let handle_input = move |e: Event<FormData>| {
        if props.disabled {
            return;
        }
        if let Ok(val) = e.value().parse::<f64>() {
            let clamped = clamp_value(val, props.min, props.max);
            let formatted = format_value(clamped, props.precision);
            props.on_change.call(formatted);
        }
    };

    let display_value = format_value(props.value, props.precision);

    rsx! {
        div {
            style: "{container_style} {props.style.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",

            if let Some(label) = props.label.clone() {
                label {
                    style: "{label_style}",
                    "{label}"
                }
            }

            div {
                style: "{input_wrapper_style}",
                opacity: if props.disabled { "0.5" } else { "1" },
                pointer_events: if props.disabled { "none" } else { "auto" },

                button {
                    style: "{button_style}",
                    onclick: handle_decrement,
                    disabled: props.disabled,
                    "−"
                }

                input {
                    style: "{input_style}",
                    r#type: "number",
                    value: "{display_value}",
                    placeholder: props.placeholder.clone().unwrap_or_default(),
                    min: props.min.map(|m| m.to_string()).unwrap_or_default(),
                    max: props.max.map(|m| m.to_string()).unwrap_or_default(),
                    step: props.step.to_string(),
                    disabled: props.disabled,
                    oninput: handle_input,
                }

                button {
                    style: "{button_style}",
                    onclick: handle_increment,
                    disabled: props.disabled,
                    "+"
                }
            }

            if let Some(error) = props.error.clone() {
                span {
                    style: "{error_style}",
                    "{error}"
                }
            }
        }
    }
}

fn clamp_value(value: f64, min: Option<f64>, max: Option<f64>) -> f64 {
    let mut result = value;
    if let Some(min) = min {
        result = result.max(min);
    }
    if let Some(max) = max {
        result = result.min(max);
    }
    result
}

fn format_value(value: f64, precision: Option<usize>) -> f64 {
    match precision {
        Some(p) => {
            let multiplier = 10f64.powi(p as i32);
            (value * multiplier).round() / multiplier
        }
        None => value,
    }
}
