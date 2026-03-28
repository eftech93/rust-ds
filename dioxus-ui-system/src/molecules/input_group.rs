//! Input Group molecule component
//!
//! Combines Label, Input, and helper text/error messages into a cohesive form field.

use crate::atoms::{AlignItems, Input, InputType, Label, SpacingSize, TextColor, TextSize, VStack};
use crate::styles::Style;
use crate::theme::use_style;
use dioxus::prelude::*;

/// Input Group properties
#[derive(Props, Clone, PartialEq)]
pub struct InputGroupProps {
    /// Field label
    pub label: String,
    /// Current input value
    #[props(default)]
    pub value: String,
    /// Placeholder text
    #[props(default)]
    pub placeholder: Option<String>,
    /// Input type
    #[props(default)]
    pub input_type: InputType,
    /// Error message (shows error state when Some)
    #[props(default)]
    pub error: Option<String>,
    /// Help/hint text
    #[props(default)]
    pub hint: Option<String>,
    /// Required field indicator
    #[props(default)]
    pub required: bool,
    /// Disabled state
    #[props(default)]
    pub disabled: bool,
    /// Change handler
    #[props(default)]
    pub onchange: Option<EventHandler<String>>,
    /// Optional icon before input
    #[props(default)]
    pub leading_icon: Option<Element>,
    /// Optional icon after input
    #[props(default)]
    pub trailing_icon: Option<Element>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Input Group molecule component
///
/// # Example
/// ```rust,ignore
/// use dioxus_ui_system::molecules::InputGroup;
///
/// let mut email = use_signal(|| String::new());
///
/// rsx! {
///     InputGroup {
///         label: "Email",
///         value: email(),
///         placeholder: "you@example.com",
///         hint: "We'll never share your email.",
///         onchange: move |v| email.set(v),
///     }
/// }
/// ```
#[component]
pub fn InputGroup(props: InputGroupProps) -> Element {
    let label = props.label.clone();
    let value = props.value.clone();
    let placeholder = props.placeholder.clone();
    let input_type = props.input_type.clone();
    let error = props.error.clone();
    let hint = props.hint.clone();
    let required = props.required;
    let disabled = props.disabled;

    let container_style = use_style(|_t| Style::new().w_full().build());

    let input_wrapper_style = use_style(|_| Style::new().relative().build());

    let label_element = if required {
        rsx! {
            Label {
                size: TextSize::Small,
                weight: crate::atoms::label::TextWeight::Medium,
                "{label}"
                Label {
                    size: TextSize::Small,
                    color: TextColor::Destructive,
                    " *"
                }
            }
        }
    } else {
        rsx! {
            Label {
                size: TextSize::Small,
                weight: crate::atoms::label::TextWeight::Medium,
                "{label}"
            }
        }
    };

    let helper_text = if let Some(err) = error {
        Some(rsx! {
            Label {
                size: TextSize::ExtraSmall,
                color: TextColor::Destructive,
                "{err}"
            }
        })
    } else if let Some(h) = hint {
        Some(rsx! {
            Label {
                size: TextSize::ExtraSmall,
                color: TextColor::Muted,
                "{h}"
            }
        })
    } else {
        None
    };

    let custom_style = props.style.clone().unwrap_or_default();

    rsx! {
        div {
            style: "{container_style} {custom_style}",

            VStack {
                gap: SpacingSize::Xs,
                align: AlignItems::Stretch,

                {label_element}

                div {
                    style: "{input_wrapper_style} display: flex; align-items: center;",

                    if props.leading_icon.is_some() {
                        div {
                            style: "position: absolute; left: 12px; z-index: 1;",
                            {props.leading_icon.unwrap()}
                        }
                    }

                    Input {
                        value: value,
                        placeholder: placeholder,
                        input_type: input_type,
                        disabled: disabled,
                        onchange: props.onchange.clone(),
                    }

                    if props.trailing_icon.is_some() {
                        div {
                            style: "position: absolute; right: 12px; z-index: 1;",
                            {props.trailing_icon.unwrap()}
                        }
                    }
                }

                {helper_text}
            }
        }
    }
}
