//! Input atom component
//!
//! Text input field with full theme integration and state management.

use crate::styles::Style;
use crate::theme::use_style;
use dioxus::prelude::*;

/// Input types
#[derive(Default, Clone, PartialEq)]
pub enum InputType {
    #[default]
    Text,
    Password,
    Email,
    Number,
    Tel,
    Url,
    Search,
    Date,
    Time,
    DatetimeLocal,
    Month,
    Week,
    Color,
}

impl InputType {
    pub fn as_str(&self) -> &'static str {
        match self {
            InputType::Text => "text",
            InputType::Password => "password",
            InputType::Email => "email",
            InputType::Number => "number",
            InputType::Tel => "tel",
            InputType::Url => "url",
            InputType::Search => "search",
            InputType::Date => "date",
            InputType::Time => "time",
            InputType::DatetimeLocal => "datetime-local",
            InputType::Month => "month",
            InputType::Week => "week",
            InputType::Color => "color",
        }
    }
}

/// Input properties
#[derive(Props, Clone, PartialEq)]
pub struct InputProps {
    /// Current value
    #[props(default)]
    pub value: String,
    /// Placeholder text
    #[props(default)]
    pub placeholder: Option<String>,
    /// Input type
    #[props(default)]
    pub input_type: InputType,
    /// Disabled state
    #[props(default)]
    pub disabled: bool,
    /// Read-only state
    #[props(default)]
    pub readonly: bool,
    /// Required field
    #[props(default)]
    pub required: bool,
    /// Autofocus on mount
    #[props(default)]
    pub autofocus: bool,
    /// Change handler
    #[props(default)]
    pub onchange: Option<EventHandler<String>>,
    /// Focus handler
    #[props(default)]
    pub onfocus: Option<EventHandler<FocusEvent>>,
    /// Blur handler
    #[props(default)]
    pub onblur: Option<EventHandler<FocusEvent>>,
    /// Input handler (real-time)
    #[props(default)]
    pub oninput: Option<EventHandler<FormEvent>>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
    /// Input name attribute
    #[props(default)]
    pub name: Option<String>,
    /// Input id attribute
    #[props(default)]
    pub id: Option<String>,
}

/// Input atom component
///
/// # Example
/// ```rust,ignore
/// use dioxus_ui_system::atoms::Input;
///
/// let mut value = use_signal(|| String::new());
///
/// rsx! {
///     Input {
///         value: value(),
///         placeholder: "Enter text...",
///         onchange: move |v| value.set(v),
///     }
/// }
/// ```
#[component]
pub fn Input(props: InputProps) -> Element {
    let disabled = props.disabled;
    let readonly = props.readonly;

    // Interactive states
    let mut is_focused = use_signal(|| false);
    let mut is_hovered = use_signal(|| false);

    // Memoized styles
    let style = use_style(move |t| {
        let base = Style::new()
            .flex()
            .w_full()
            .h_px(40)
            .rounded(&t.radius, "md")
            .border(
                1,
                if is_focused() {
                    &t.colors.ring
                } else {
                    &t.colors.border
                },
            )
            .bg(&t.colors.background)
            .text_color(&t.colors.foreground)
            .px(&t.spacing, "md")
            .text(&t.typography, "sm")
            .transition("all 150ms cubic-bezier(0.4, 0, 0.2, 1)")
            .outline("none");

        // Disabled state
        let base = if disabled || readonly {
            base.cursor("not-allowed").opacity(0.5).bg(&t.colors.muted)
        } else {
            base.cursor("text").opacity(1.0)
        };

        // Focus ring effect
        let base = if is_focused() && !disabled {
            Style {
                box_shadow: Some(format!("0 0 0 1px {}", t.colors.ring.to_rgba())),
                ..base
            }
        } else {
            base
        };

        // Hover effect (only when not focused)
        let base = if is_hovered() && !is_focused() && !disabled {
            base.border_color(&t.colors.foreground.darken(0.2))
        } else {
            base
        };

        base.build()
    });

    // Combine with custom styles
    let final_style = if let Some(custom) = &props.style {
        format!("{} {}", style(), custom)
    } else {
        style()
    };

    let class = props.class.clone().unwrap_or_default();
    let input_type = props.input_type.clone();
    let placeholder = props.placeholder.clone();
    let name = props.name.clone();
    let id = props.id.clone();
    let value = props.value.clone();
    let required = props.required;
    let autofocus = props.autofocus;
    let readonly = props.readonly;
    let disabled = props.disabled;

    rsx! {
        input {
            r#type: input_type.as_str(),
            style: "{final_style}",
            class: "{class}",
            value: "{value}",
            placeholder: placeholder,
            name: name,
            id: id,
            required: required,
            autofocus: autofocus,
            readonly: readonly,
            disabled: disabled,
            onmouseenter: move |_| is_hovered.set(true),
            onmouseleave: move |_| is_hovered.set(false),
            onfocus: move |e| {
                is_focused.set(true);
                if let Some(handler) = &props.onfocus {
                    handler.call(e);
                }
            },
            onblur: move |e| {
                is_focused.set(false);
                if let Some(handler) = &props.onblur {
                    handler.call(e);
                }
            },
            oninput: move |e| {
                if let Some(handler) = &props.oninput {
                    handler.call(e);
                }
            },
            onchange: move |e| {
                if let Some(handler) = &props.onchange {
                    handler.call(e.value());
                }
            },
        }
    }
}

// Note: outline method is now in the main Style builder
