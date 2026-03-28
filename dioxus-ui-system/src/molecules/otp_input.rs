//! OTP Input molecule component
//!
//! A configurable multi-digit code input with individual boxes for each character.
//! Supports auto-focus navigation, paste functionality, and masking.

use crate::styles::Style;
use crate::theme::use_style;
use dioxus::prelude::*;

/// OTP Input properties
#[derive(Props, Clone, PartialEq)]
pub struct OtpInputProps {
    /// Number of input boxes (default: 6)
    #[props(default = 6)]
    pub length: usize,
    /// Current value
    #[props(default)]
    pub value: String,
    /// Change handler called when the full code is entered or modified
    pub on_change: EventHandler<String>,
    /// Disabled state
    #[props(default)]
    pub disabled: bool,
    /// Error state styling
    #[props(default)]
    pub error: bool,
    /// Mask input (show • instead of numbers)
    #[props(default)]
    pub mask: bool,
    /// Auto-focus first input on mount
    #[props(default)]
    pub auto_focus: bool,
    /// Custom inline styles for the container
    #[props(default)]
    pub style: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
    /// Callback when input is completed (all digits filled)
    #[props(default)]
    pub on_complete: Option<EventHandler<String>>,
}

/// OTP Input molecule component
///
/// # Example
/// ```rust,ignore
/// use dioxus_ui_system::molecules::OtpInput;
///
/// let mut code = use_signal(|| String::new());
///
/// rsx! {
///     OtpInput {
///         length: 6,
///         value: code(),
///         on_change: move |v| code.set(v),
///         on_complete: Some(EventHandler::new(move |v: String| {
///             println!("Code complete: {}", v);
///         })),
///     }
/// }
/// ```
#[component]
pub fn OtpInput(props: OtpInputProps) -> Element {
    let length = props.length.max(1).min(12); // Clamp between 1 and 12
    let disabled = props.disabled;
    let error = props.error;
    let mask = props.mask;
    let auto_focus = props.auto_focus;

    // Track which input is focused (for styling)
    let mut focused_index = use_signal(|| Option::<usize>::None);

    // Container style
    let container_style = use_style(|_| {
        Style::new()
            .flex()
            .gap_px(8)
            .items_center()
            .justify_center()
            .build()
    });

    // Individual input box style
    let input_style = use_style(move |t| {
        let base = Style::new()
            .w_px(48)
            .h_px(56)
            .min_w_px(40)
            .rounded(&t.radius, "md")
            .border(
                1,
                if error {
                    &t.colors.destructive
                } else {
                    &t.colors.border
                },
            )
            .bg(&t.colors.background)
            .text_color(&t.colors.foreground)
            .text_center()
            .font_size(20)
            .font_weight(600)
            .transition("all 150ms cubic-bezier(0.4, 0, 0.2, 1)")
            .outline("none");

        // Responsive sizing
        let base = base
            .w("calc(40px + 2vw)")
            .h("calc(48px + 1vh)")
            .min_w_px(36)
            .min_h_px(44);

        // Disabled state
        let base = if disabled {
            base.cursor("not-allowed").opacity(0.5).bg(&t.colors.muted)
        } else {
            base.cursor("text")
        };

        // Error state shadow
        if error {
            Style {
                box_shadow: Some(format!("0 0 0 1px {}", t.colors.destructive.to_rgba())),
                ..base
            }
            .build()
        } else {
            base.build()
        }
    });

    // Focus style (applied dynamically)
    let get_focus_style = use_style(move |t| {
        Style::new()
            .border_color(&t.colors.ring)
            .shadow(&format!(
                "0 0 0 2px {}",
                t.colors
                    .ring
                    .to_rgba()
                    .replace(')', ", 0.3)")
                    .replace("rgba", "rgb")
            ))
            .build()
    });

    let custom_style = props.style.clone().unwrap_or_default();
    let class = props.class.clone().unwrap_or_default();
    let on_change = props.on_change.clone();
    let on_complete = props.on_complete.clone();
    let current_value = props.value.clone();

    // Helper to check if all digits are filled and call on_complete
    let check_complete = move |code: &str| {
        if code.len() == length && code.chars().all(|c| c.is_ascii_digit()) {
            if let Some(ref handler) = on_complete {
                handler.call(code.to_string());
            }
        }
    };

    rsx! {
        div {
            style: "{container_style} {custom_style}",
            class: "{class}",
            role: "group",
            "aria-label": "One-time code input",

            for index in 0..length {
                {
                    let idx = index;
                    let value = current_value.chars().nth(idx).unwrap_or(' ');
                    let display_value = if value.is_ascii_digit() {
                        if mask { "•".to_string() } else { value.to_string() }
                    } else {
                        String::new()
                    };

                    let is_focused = focused_index() == Some(idx);
                    let focus_style = if is_focused && !disabled { get_focus_style() } else { String::new() };
                    let should_auto_focus = auto_focus && idx == 0;

                    let aria_label = format!("Digit {} of {}", idx + 1, length);

                    // Clone values for closures
                    let on_change_clone = on_change.clone();
                    let on_change_clone2 = on_change.clone();
                    let current_value_clone = current_value.clone();
                    let current_value_clone2 = current_value.clone();

                    rsx! {
                        input {
                            key: "otp-{idx}",
                            id: "otp-input-{idx}",
                            r#type: "text",
                            inputmode: "numeric",
                            pattern: "[0-9]*",
                            autocomplete: if idx == 0 { "one-time-code" } else { "off" },
                            maxlength: "1",
                            disabled: disabled,
                            value: "{display_value}",
                            autofocus: should_auto_focus,
                            "aria-label": "{aria_label}",
                            "aria-disabled": disabled,
                            "aria-invalid": error,
                            style: "{input_style} {focus_style}",
                            oninput: move |e| {
                                let val = e.value();

                                // Helper to update value at specific index
                                let update_value_at = |index: usize, digit: char, base_value: &str| -> String {
                                    let mut current: Vec<char> = base_value.chars().collect();
                                    while current.len() < length {
                                        current.push(' ');
                                    }
                                    if index < length {
                                        current[index] = digit;
                                    }
                                    current.iter().take(length).collect::<String>().trim().to_string()
                                };

                                // Handle paste of multiple digits
                                let handle_paste = |start_index: usize, pasted: &str, base_value: &str| -> String {
                                    let digits: String = pasted.chars().filter(|c| c.is_ascii_digit()).collect();

                                    if !digits.is_empty() {
                                        let mut current: Vec<char> = base_value.chars().collect();
                                        while current.len() < length {
                                            current.push(' ');
                                        }

                                        // Fill from start index
                                        for (i, d) in digits.chars().enumerate() {
                                            let target_index = start_index + i;
                                            if target_index < length {
                                                current[target_index] = d;
                                            }
                                        }

                                        current.iter().take(length).collect::<String>().trim().to_string()
                                    } else {
                                        base_value.to_string()
                                    }
                                };

                                // Check if it looks like a paste (multiple characters)
                                let new_code = if val.len() > 1 {
                                    handle_paste(idx, &val, &current_value_clone)
                                } else {
                                    // Only accept single digit
                                    let digit = val.chars().filter(|c| c.is_ascii_digit()).next();
                                    if let Some(d) = digit {
                                        update_value_at(idx, d, &current_value_clone)
                                    } else {
                                        current_value_clone.clone()
                                    }
                                };

                                on_change_clone.call(new_code.clone());
                                check_complete(&new_code);
                            },
                            onkeydown: move |e: Event<dioxus::html::KeyboardData>| {
                                use dioxus::html::input_data::keyboard_types::Key;

                                // Helper to clear value at specific index
                                let clear_value_at = |index: usize, base_value: &str| -> String {
                                    let mut current: Vec<char> = base_value.chars().collect();
                                    while current.len() < length {
                                        current.push(' ');
                                    }
                                    if index < length {
                                        current[index] = ' ';
                                    }
                                    current.iter().take(length).collect::<String>().trim().to_string()
                                };

                                match e.key() {
                                    Key::Backspace => {
                                        let current_value_char = current_value_clone2.chars().nth(idx).unwrap_or(' ');

                                        if current_value_char != ' ' && current_value_char != '\0' {
                                            // Clear current value
                                            let new_code = clear_value_at(idx, &current_value_clone2);
                                            on_change_clone2.call(new_code);
                                        }
                                    }
                                    _ => {}
                                }
                            },
                            onfocus: move |_| {
                                focused_index.set(Some(idx));
                            },
                            onblur: move |_| {
                                if focused_index() == Some(idx) {
                                    focused_index.set(None);
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}
