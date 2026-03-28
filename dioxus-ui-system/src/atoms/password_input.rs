//! Password Input atom component
//!
//! A password input field with show/hide toggle and optional strength indicator.

use crate::styles::Style;
use crate::theme::{use_style, use_theme};
use dioxus::prelude::*;

/// Password strength levels
#[derive(Clone, PartialEq, Debug)]
pub enum PasswordStrength {
    Weak,
    Medium,
    Strong,
}

impl PasswordStrength {
    /// Get the label text for the strength level
    pub fn label(&self) -> &'static str {
        match self {
            PasswordStrength::Weak => "Weak",
            PasswordStrength::Medium => "Medium",
            PasswordStrength::Strong => "Strong",
        }
    }

    /// Get the color for the strength indicator
    pub fn color(&self) -> &'static str {
        match self {
            PasswordStrength::Weak => "#ef4444",   // red-500
            PasswordStrength::Medium => "#f59e0b", // amber-500
            PasswordStrength::Strong => "#22c55e", // green-500
        }
    }
}

/// Calculate password strength based on common criteria
fn calculate_strength(password: &str) -> PasswordStrength {
    let length = password.len();

    if length < 6 {
        return PasswordStrength::Weak;
    }

    let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
    let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| !c.is_alphanumeric());

    let criteria_count = [has_lowercase, has_uppercase, has_digit, has_special]
        .iter()
        .filter(|&&x| x)
        .count();

    match criteria_count {
        0..=1 => PasswordStrength::Weak,
        2 => PasswordStrength::Medium,
        _ => PasswordStrength::Strong,
    }
}

/// Password Input properties
#[derive(Props, Clone, PartialEq)]
pub struct PasswordInputProps {
    /// Current value
    #[props(default)]
    pub value: String,
    /// Callback when value changes
    pub on_change: EventHandler<String>,
    /// Placeholder text
    #[props(default)]
    pub placeholder: Option<String>,
    /// Disabled state
    #[props(default)]
    pub disabled: bool,
    /// Error message to display
    #[props(default)]
    pub error: Option<String>,
    /// Label text
    #[props(default)]
    pub label: Option<String>,
    /// Required field indicator
    #[props(default)]
    pub required: bool,
    /// Show password strength indicator
    #[props(default)]
    pub strength_indicator: bool,
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
    /// Autofocus on mount
    #[props(default)]
    pub autofocus: bool,
}

/// Password Input atom component
///
/// # Example
/// ```rust,ignore
/// use dioxus_ui_system::atoms::PasswordInput;
///
/// let mut value = use_signal(|| String::new());
///
/// rsx! {
///     PasswordInput {
///         value: value(),
///         placeholder: "Enter password...",
///         on_change: move |v| value.set(v),
///         strength_indicator: true,
///     }
/// }
/// ```
#[component]
pub fn PasswordInput(props: PasswordInputProps) -> Element {
    let _theme = use_theme();
    let mut is_visible = use_signal(|| false);
    let mut is_focused = use_signal(|| false);
    let mut is_hovered = use_signal(|| false);

    let disabled = props.disabled;
    let value = props.value.clone();
    let has_error = props.error.is_some();
    let error_clone = props.error.clone();
    let strength = calculate_strength(&value);

    // Container style (full width flex column)
    let container_style = use_style(move |t| {
        Style::new()
            .flex()
            .flex_col()
            .w_full()
            .gap(&t.spacing, "xs")
            .build()
    });

    // Input wrapper style (for positioning the toggle button)
    let wrapper_style = use_style(move |_t| Style::new().relative().w_full().build());

    // Input style
    let input_style = use_style(move |t| {
        let base = Style::new()
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
            .pr_px(40) // Extra padding for toggle button
            .text(&t.typography, "sm")
            .transition("all 150ms cubic-bezier(0.4, 0, 0.2, 1)")
            .outline("none");

        // Disabled state
        let base = if disabled {
            base.cursor("not-allowed").opacity(0.5).bg(&t.colors.muted)
        } else {
            base.cursor("text")
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

        // Error state
        let base = if has_error {
            base.border_color(&t.colors.destructive)
        } else {
            base
        };

        base.build()
    });

    // Toggle button style
    let toggle_style = use_style(move |t| {
        Style::new()
            .absolute()
            .right("8px")
            .top("50%")
            .transform("translateY(-50%)")
            .w_px(32)
            .h_px(32)
            .flex()
            .items_center()
            .justify_center()
            .rounded(&t.radius, "sm")
            .border(0, &t.colors.border)
            .bg_transparent()
            .text_color(&t.colors.muted_foreground)
            .cursor(if disabled { "not-allowed" } else { "pointer" })
            .opacity(if disabled { 0.5 } else { 1.0 })
            .transition("all 150ms ease")
            .build()
    });

    // Label style
    let label_style = use_style(move |t| {
        Style::new()
            .text(&t.typography, "sm")
            .text_color(&t.colors.foreground)
            .font_weight(500)
            .build()
    });

    // Error message style
    let error_style = use_style(move |t| {
        Style::new()
            .text(&t.typography, "xs")
            .text_color(&t.colors.destructive)
            .mt(&t.spacing, "xs")
            .build()
    });

    // Strength indicator container style
    let strength_container_style = use_style(move |t| {
        Style::new()
            .flex()
            .flex_col()
            .gap(&t.spacing, "xs")
            .mt(&t.spacing, "sm")
            .build()
    });

    // Strength bar background style
    let strength_bar_bg_style = use_style(move |t| {
        Style::new()
            .w_full()
            .h_px(4)
            .rounded(&t.radius, "full")
            .bg(&t.colors.muted)
            .overflow_hidden()
            .build()
    });

    // Strength bar fill width and color based on strength
    let (strength_width, strength_color) = match strength {
        PasswordStrength::Weak => ("33%", PasswordStrength::Weak.color()),
        PasswordStrength::Medium => ("66%", PasswordStrength::Medium.color()),
        PasswordStrength::Strong => ("100%", PasswordStrength::Strong.color()),
    };

    // Strength bar fill style
    let strength_bar_fill_style =
        use_style(move |_t| Style::new().h_full().transition("all 300ms ease").build());

    // Strength label style
    let strength_label_style = use_style(move |t| {
        Style::new()
            .text(&t.typography, "xs")
            .text_color(&t.colors.muted_foreground)
            .build()
    });

    // Combine with custom styles
    let final_input_style = if let Some(custom) = &props.style {
        format!("{} {}", input_style(), custom)
    } else {
        input_style()
    };

    let class = props.class.clone().unwrap_or_default();
    let placeholder = props.placeholder.clone();
    let name = props.name.clone();
    let id = props.id.clone();
    let autofocus = props.autofocus;
    let input_type = if is_visible() { "text" } else { "password" };

    // Eye icon SVG (show password)
    let eye_icon = rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            width: "18",
            height: "18",
            path { d: "M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z" }
            circle { cx: "12", cy: "12", r: "3" }
        }
    };

    // EyeOff icon SVG (hide password)
    let eye_off_icon = rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            width: "18",
            height: "18",
            path { d: "M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24" }
            line { x1: "1", y1: "1", x2: "23", y2: "23" }
        }
    };

    rsx! {
        div {
            style: "{container_style}",

            // Label
            if let Some(label_text) = &props.label {
                label {
                    style: "{label_style}",
                    r#for: id.clone(),
                    "{label_text}"
                    if props.required {
                        span { style: "margin-left: 4px; color: #ef4444;", "*" }
                    }
                }
            }

            // Input wrapper with toggle button
            div {
                style: "{wrapper_style}",

                input {
                    r#type: "{input_type}",
                    style: "{final_input_style}",
                    class: "{class}",
                    value: "{value}",
                    placeholder: placeholder,
                    name: name,
                    id: id,
                    disabled: disabled,
                    autofocus: autofocus,
                    required: props.required,
                    onmouseenter: move |_| is_hovered.set(true),
                    onmouseleave: move |_| is_hovered.set(false),
                    onfocus: move |_| is_focused.set(true),
                    onblur: move |_| is_focused.set(false),
                    oninput: move |e| {
                        props.on_change.call(e.value());
                    },
                }

                // Toggle visibility button
                button {
                    r#type: "button",
                    style: "{toggle_style}",
                    disabled: disabled,
                    aria_label: if is_visible() { "Hide password" } else { "Show password" },
                    onclick: move |_| {
                        if !disabled {
                            is_visible.toggle();
                        }
                    },
                    if is_visible() {
                        {eye_off_icon}
                    } else {
                        {eye_icon}
                    }
                }
            }

            // Error message
            if let Some(error_msg) = error_clone {
                span {
                    style: "{error_style}",
                    "{error_msg}"
                }
            }

            // Strength indicator
            if props.strength_indicator && !value.is_empty() {
                div {
                    style: "{strength_container_style}",

                    // Strength bar
                    div {
                        style: "{strength_bar_bg_style}",
                        div {
                            style: "{strength_bar_fill_style} width: {strength_width}; background-color: {strength_color};",
                        }
                    }

                    // Strength label
                    span {
                        style: "{strength_label_style}",
                        "Password strength: {strength.label()}"
                    }
                }
            }
        }
    }
}
