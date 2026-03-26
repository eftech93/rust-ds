//! Time Picker molecule component
//!
//! A time selection component with scrollable columns for hours, minutes,
//! and optionally seconds. Supports 12h (AM/PM) and 24h formats.

use dioxus::prelude::*;
use crate::theme::{use_theme, use_style};
use crate::styles::Style;

/// Time picker properties
#[derive(Props, Clone, PartialEq)]
pub struct TimePickerProps {
    /// Current time value (HH:MM or HH:MM:SS format)
    #[props(default)]
    pub value: Option<String>,
    /// Change handler called when time changes
    pub on_change: EventHandler<Option<String>>,
    /// Use 24-hour format (default: true, false for AM/PM)
    #[props(default = true)]
    pub use_24h: bool,
    /// Show seconds column
    #[props(default = false)]
    pub show_seconds: bool,
    /// Disabled state
    #[props(default = false)]
    pub disabled: bool,
    /// Minute step interval (default: 1, e.g., 15 for quarters)
    #[props(default = 1)]
    pub minute_step: u32,
    /// Placeholder text
    #[props(default)]
    pub placeholder: Option<String>,
    /// Label text
    #[props(default)]
    pub label: Option<String>,
    /// Error message
    #[props(default)]
    pub error: Option<String>,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Parsed time components
#[derive(Clone, PartialEq, Debug)]
struct TimeValue {
    hour: u32,
    minute: u32,
    second: u32,
    is_pm: bool,
}

impl TimeValue {
    fn from_string(value: &str, use_24h: bool) -> Option<Self> {
        let parts: Vec<&str> = value.split(':').collect();
        if parts.len() < 2 {
            return None;
        }

        let hour = parts[0].parse().ok()?;
        let minute = parts[1].parse().ok()?;
        let second = if parts.len() > 2 {
            parts[2].parse().ok()?
        } else {
            0
        };

        let (hour, is_pm) = if use_24h {
            (hour, false)
        } else {
            // Convert from 24h to 12h
            if hour == 0 {
                (12, false) // 12 AM
            } else if hour == 12 {
                (12, true) // 12 PM
            } else if hour > 12 {
                (hour - 12, true)
            } else {
                (hour, false)
            }
        };

        Some(TimeValue {
            hour,
            minute,
            second,
            is_pm,
        })
    }

    fn to_string(&self, use_24h: bool, show_seconds: bool) -> String {
        let hour = if use_24h {
            if self.is_pm && self.hour != 12 {
                self.hour + 12
            } else if !self.is_pm && self.hour == 12 {
                0
            } else {
                self.hour
            }
        } else {
            self.hour
        };

        if show_seconds {
            format!("{:02}:{:02}:{:02}", hour, self.minute, self.second)
        } else {
            format!("{:02}:{:02}", hour, self.minute)
        }
    }

    fn now(use_24h: bool) -> Self {
        // Get current time - in a real app, use system time
        // For now, default to 12:00
        TimeValue {
            hour: if use_24h { 12 } else { 12 },
            minute: 0,
            second: 0,
            is_pm: true,
        }
    }
}

/// Time picker molecule component
///
/// # Example
/// ```rust,ignore
/// use dioxus_ui_system::molecules::TimePicker;
///
/// let mut time = use_signal(|| None::<String>);
///
/// rsx! {
///     TimePicker {
///         value: time(),
///         on_change: move |t| time.set(t),
///         use_24h: true,
///         show_seconds: false,
///         minute_step: 15,
///     }
/// }
/// ```
#[component]
pub fn TimePicker(props: TimePickerProps) -> Element {
    let theme = use_theme();
    let mut is_open = use_signal(|| false);

    // Parse current value
    let current_time = props.value.as_ref()
        .and_then(|v| TimeValue::from_string(v, props.use_24h));

    let class_css = props.class.as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();

    let border_color = if props.error.is_some() {
        theme.tokens.read().colors.destructive.to_rgba()
    } else if is_open() {
        theme.tokens.read().colors.primary.to_rgba()
    } else {
        theme.tokens.read().colors.border.to_rgba()
    };

    // Display format
    let display_value = current_time.as_ref().map(|t| {
        if props.use_24h {
            if props.show_seconds {
                format!("{:02}:{:02}:{:02}", t.hour, t.minute, t.second)
            } else {
                format!("{:02}:{:02}", t.hour, t.minute)
            }
        } else {
            let am_pm = if t.is_pm { "PM" } else { "AM" };
            if props.show_seconds {
                format!("{:02}:{:02}:{:02} {}", t.hour, t.minute, t.second, am_pm)
            } else {
                format!("{:02}:{:02} {}", t.hour, t.minute, am_pm)
            }
        }
    });

    let has_error = props.error.is_some();
    let is_disabled = props.disabled;
    let trigger_style = use_style(move |t| {
        Style::new()
            .w_full()
            .h_px(40)
            .px(&t.spacing, "md")
            .rounded(&t.radius, "md")
            .border(1, if has_error { &t.colors.destructive } else { &t.colors.border })
            .bg(&t.colors.background)
            .text_color(&t.colors.foreground)
            .font_size(14)
            .cursor(if is_disabled { "not-allowed" } else { "pointer" })
            .inline_flex()
            .items_center()
            .justify_between()
            .transition("all 150ms ease")
            .build()
    });

    let on_select_time = {
        let on_change = props.on_change.clone();
        let use_24h = props.use_24h;
        let show_seconds = props.show_seconds;
        move |time: TimeValue| {
            let value = time.to_string(use_24h, show_seconds);
            on_change.call(Some(value));
            is_open.set(false);
        }
    };

    let on_clear = {
        let on_change = props.on_change.clone();
        move |e: Event<MouseData>| {
            e.stop_propagation();
            on_change.call(None);
        }
    };

    let disabled_style = if props.disabled { "opacity: 0.5;" } else { "" };
    let placeholder_text = props.placeholder.clone().unwrap_or_else(|| "Select time".to_string());

    rsx! {
        div {
            class: "time-picker{class_css}",
            style: "display: flex; flex-direction: column; gap: 6px; position: relative;",

            if let Some(label) = props.label.clone() {
                label {
                    class: "time-picker-label",
                    style: "font-size: 14px; font-weight: 500; color: {theme.tokens.read().colors.foreground.to_rgba()};",
                    "{label}"
                }
            }

            div {
                style: "position: relative;",

                button {
                    type: "button",
                    class: "time-picker-trigger",
                    style: "{trigger_style} border-color: {border_color}; {disabled_style}",
                    disabled: props.disabled,
                    onclick: move |_| if !props.disabled { is_open.toggle() },

                    if let Some(value) = display_value.clone() {
                        span { "{value}" }
                    } else {
                        span {
                            style: "color: {theme.tokens.read().colors.muted.to_rgba()};",
                            "{placeholder_text}"
                        }
                    }

                    div {
                        style: "display: flex; align-items: center; gap: 8px;",

                        if props.value.is_some() {
                            button {
                                type: "button",
                                style: "background: none; border: none; cursor: pointer; font-size: 12px; color: {theme.tokens.read().colors.muted.to_rgba()}; padding: 2px; display: flex; align-items: center; justify-content: center;",
                                onclick: on_clear,
                                "✕"
                            }
                        }

                        span {
                            style: "font-size: 14px; color: {theme.tokens.read().colors.muted.to_rgba()}; transition: transform 0.2s;",
                            style: if is_open() { "transform: rotate(180deg);" } else { "" },
                            "▼"
                        }
                    }
                }

                if is_open() && !props.disabled {
                    TimePickerDropdown {
                        value: current_time.clone(),
                        use_24h: props.use_24h,
                        show_seconds: props.show_seconds,
                        minute_step: props.minute_step,
                        on_select: on_select_time,
                        on_close: move || is_open.set(false),
                    }
                }
            }

            if let Some(error) = props.error.clone() {
                span {
                    class: "time-picker-error",
                    style: "font-size: 12px; color: {theme.tokens.read().colors.destructive.to_rgba()};",
                    "{error}"
                }
            }
        }
    }
}

/// Time picker dropdown component
#[derive(Props, Clone, PartialEq)]
struct TimePickerDropdownProps {
    value: Option<TimeValue>,
    use_24h: bool,
    show_seconds: bool,
    minute_step: u32,
    on_select: EventHandler<TimeValue>,
    on_close: EventHandler<()>,
}

#[component]
fn TimePickerDropdown(props: TimePickerDropdownProps) -> Element {
    let theme = use_theme();

    // Use signals for each component
    let initial = props.value.clone().unwrap_or_else(|| TimeValue {
        hour: if props.use_24h { 12 } else { 12 },
        minute: 0,
        second: 0,
        is_pm: false,
    });

    let mut hour = use_signal(|| initial.hour);
    let mut minute = use_signal(|| initial.minute);
    let mut second = use_signal(|| initial.second);
    let mut is_pm = use_signal(|| initial.is_pm);

    // Sync with props
    use_effect(move || {
        if let Some(v) = &props.value {
            hour.set(v.hour);
            minute.set(v.minute);
            second.set(v.second);
            is_pm.set(v.is_pm);
        }
    });

    let dropdown_style = use_style(|t| {
        Style::new()
            .absolute()
            .top("calc(100% + 4px)")
            .left("0")
            .rounded(&t.radius, "md")
            .border(1, &t.colors.border)
            .bg(&t.colors.popover)
            .shadow(&t.shadows.lg)
            .z_index(9999)
            .p_px(12)
            .build()
    });

    let columns_container_style = use_style(|_| {
        Style::new()
            .flex()
            .gap_px(8)
            .build()
    });

    let column_style = use_style(|_| {
        Style::new()
            .flex()
            .flex_col()
            .items_center()
            .gap_px(4)
            .build()
    });

    let header_style = use_style(|t| {
        Style::new()
            .font_size(11)
            .font_weight(600)
            .text_color(&t.colors.muted)
            .pb_px(4)
            .build()
    });

    let scroll_container_style = use_style(|t| {
        Style::new()
            .h_px(200)
            .overflow_auto()
            .flex()
            .flex_col()
            .gap_px(2)
            .build()
            + " scrollbar-width: thin; &::-webkit-scrollbar { width: 4px; } &::-webkit-scrollbar-thumb { background: "
            + &t.colors.border.to_rgba()
            + "; border-radius: 2px; }"
    });

    // Generate hour options
    let hour_options: Vec<u32> = if props.use_24h {
        (0..24).collect()
    } else {
        (1..13).collect()
    };

    // Generate minute options based on step
    let minute_options: Vec<u32> = (0..60)
        .step_by(props.minute_step as usize)
        .collect();

    // Generate second options
    let second_options: Vec<u32> = (0..60).step_by(5).collect();

    let on_select = {
        let on_select = props.on_select.clone();
        move || {
            on_select.call(TimeValue {
                hour: hour(),
                minute: minute(),
                second: second(),
                is_pm: is_pm(),
            });
        }
    };

    let on_now = {
        let mut hour = hour.clone();
        let mut minute = minute.clone();
        let mut second = second.clone();
        let mut is_pm = is_pm.clone();
        let on_select = props.on_select.clone();
        let use_24h = props.use_24h;
        move || {
            let now = TimeValue::now(use_24h);
            hour.set(now.hour);
            minute.set(now.minute);
            second.set(now.second);
            is_pm.set(now.is_pm);
            on_select.call(now);
        }
    };

    rsx! {
        // Backdrop to close on outside click - high z-index to capture clicks but below dropdown
        div {
            class: "time-picker-backdrop",
            style: "position: fixed; inset: 0; z-index: 9998;",
            onclick: move |_| props.on_close.call(()),
        }

        div {
            class: "time-picker-dropdown",
            style: "{dropdown_style}",
            onclick: move |e: Event<MouseData>| e.stop_propagation(),

            // Columns container
            div {
                style: "{columns_container_style}",

                // Hours column
                div {
                    style: "{column_style}",

                    span {
                        style: "{header_style}",
                        "HH"
                    }

                    div {
                        style: "{scroll_container_style}",

                        for h in hour_options {
                            {
                                let is_selected = hour() == h;
                                let bg_color = if is_selected {
                                    theme.tokens.read().colors.primary.to_rgba()
                                } else {
                                    "transparent".to_string()
                                };
                                let text_color = if is_selected {
                                    "white".to_string()
                                } else {
                                    theme.tokens.read().colors.foreground.to_rgba()
                                };

                                rsx! {
                                    button {
                                        key: "hour-{h}",
                                        type: "button",
                                        style: "width: 48px; height: 32px; border: none; border-radius: 6px; background: {bg_color}; color: {text_color}; font-size: 14px; cursor: pointer; transition: all 100ms ease;",
                                        onclick: move |_| {
                                            hour.set(h);
                                            on_select();
                                        },
                                        "{h:02}"
                                    }
                                }
                            }
                        }
                    }
                }

                // Separator
                div {
                    style: "display: flex; flex-direction: column; justify-content: center; padding-top: 20px;",
                    span {
                        style: "font-size: 14px; font-weight: 600; color: {theme.tokens.read().colors.muted.to_rgba()};",
                        ":"
                    }
                }

                // Minutes column
                div {
                    style: "{column_style}",

                    span {
                        style: "{header_style}",
                        "MM"
                    }

                    div {
                        style: "{scroll_container_style}",

                        for m in minute_options {
                            {
                                let is_selected = minute() == m;
                                let bg_color = if is_selected {
                                    theme.tokens.read().colors.primary.to_rgba()
                                } else {
                                    "transparent".to_string()
                                };
                                let text_color = if is_selected {
                                    "white".to_string()
                                } else {
                                    theme.tokens.read().colors.foreground.to_rgba()
                                };

                                rsx! {
                                    button {
                                        key: "minute-{m}",
                                        type: "button",
                                        style: "width: 48px; height: 32px; border: none; border-radius: 6px; background: {bg_color}; color: {text_color}; font-size: 14px; cursor: pointer; transition: all 100ms ease;",
                                        onclick: move |_| {
                                            minute.set(m);
                                            on_select();
                                        },
                                        "{m:02}"
                                    }
                                }
                            }
                        }
                    }
                }

                // Seconds column (optional)
                if props.show_seconds {
                    // Separator
                    div {
                        style: "display: flex; flex-direction: column; justify-content: center; padding-top: 20px;",
                        span {
                            style: "font-size: 14px; font-weight: 600; color: {theme.tokens.read().colors.muted.to_rgba()};",
                            ":"
                        }
                    }

                    div {
                        style: "{column_style}",

                        span {
                            style: "{header_style}",
                            "SS"
                        }

                        div {
                            style: "{scroll_container_style}",

                            for s in second_options {
                                {
                                    let is_selected = second() == s;
                                    let bg_color = if is_selected {
                                        theme.tokens.read().colors.primary.to_rgba()
                                    } else {
                                        "transparent".to_string()
                                    };
                                    let text_color = if is_selected {
                                        "white".to_string()
                                    } else {
                                        theme.tokens.read().colors.foreground.to_rgba()
                                    };

                                    rsx! {
                                        button {
                                            key: "second-{s}",
                                            type: "button",
                                            style: "width: 48px; height: 32px; border: none; border-radius: 6px; background: {bg_color}; color: {text_color}; font-size: 14px; cursor: pointer; transition: all 100ms ease;",
                                            onclick: move |_| {
                                                second.set(s);
                                                on_select();
                                            },
                                            "{s:02}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // AM/PM column (for 12h format)
                if !props.use_24h {
                    div {
                        style: "{column_style} margin-left: 4px;",

                        span {
                            style: "{header_style}",
                            ""
                        }

                        div {
                            style: "{scroll_container_style}",

                            for (label, value) in [("AM", false), ("PM", true)] {
                                {
                                    let is_selected = is_pm() == value;
                                    let bg_color = if is_selected {
                                        theme.tokens.read().colors.primary.to_rgba()
                                    } else {
                                        "transparent".to_string()
                                    };
                                    let text_color = if is_selected {
                                        "white".to_string()
                                    } else {
                                        theme.tokens.read().colors.foreground.to_rgba()
                                    };

                                    rsx! {
                                        button {
                                            key: "ampm-{label}",
                                            type: "button",
                                            style: "width: 48px; height: 32px; border: none; border-radius: 6px; background: {bg_color}; color: {text_color}; font-size: 12px; cursor: pointer; transition: all 100ms ease; font-weight: 500;",
                                            onclick: move |_| {
                                                is_pm.set(value);
                                                on_select();
                                            },
                                            "{label}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Footer with "Now" button
            TimePickerNowButton {
                on_click: on_now,
            }
        }
    }
}

/// Now button component with hover state
#[derive(Props, Clone, PartialEq)]
struct TimePickerNowButtonProps {
    on_click: EventHandler<()>,
}

#[component]
fn TimePickerNowButton(props: TimePickerNowButtonProps) -> Element {
    let theme = use_theme();
    let mut is_hovered = use_signal(|| false);
    
    let bg_color = if is_hovered() {
        theme.tokens.read().colors.muted.to_rgba()
    } else {
        "transparent".to_string()
    };
    
    rsx! {
        div {
            style: "display: flex; justify-content: center; margin-top: 12px; padding-top: 12px; border-top: 1px solid {theme.tokens.read().colors.border.to_rgba()};",
            
            button {
                type: "button",
                style: "font-size: 13px; font-weight: 500; color: {theme.tokens.read().colors.primary.to_rgba()}; background: {bg_color}; border: none; cursor: pointer; padding: 6px 12px; border-radius: 6px; transition: all 100ms ease;",
                onmouseenter: move |_| is_hovered.set(true),
                onmouseleave: move |_| is_hovered.set(false),
                onclick: move |_| props.on_click.call(()),
                "Now"
            }
        }
    }
}

/// Simple time input component with mask
#[derive(Props, Clone, PartialEq)]
pub struct TimeInputProps {
    /// Current time value (HH:MM or HH:MM:SS format)
    #[props(default)]
    pub value: Option<String>,
    /// Change handler
    pub on_change: EventHandler<Option<String>>,
    /// Use 24-hour format
    #[props(default = true)]
    pub use_24h: bool,
    /// Show seconds
    #[props(default = false)]
    pub show_seconds: bool,
    /// Disabled state
    #[props(default = false)]
    pub disabled: bool,
    /// Placeholder text
    #[props(default)]
    pub placeholder: Option<String>,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Time input component with manual text entry
#[component]
pub fn TimeInput(props: TimeInputProps) -> Element {
    let _theme = use_theme();
    let mut input_value = use_signal(|| props.value.clone().unwrap_or_default());

    // Sync with props
    use_effect(move || {
        input_value.set(props.value.clone().unwrap_or_default());
    });

    let class_css = props.class.as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();

    let input_style = use_style(move |t| {
        Style::new()
            .w_full()
            .h_px(40)
            .px(&t.spacing, "md")
            .rounded(&t.radius, "md")
            .border(1, &t.colors.border)
            .bg(&t.colors.background)
            .text_color(&t.colors.foreground)
            .font_size(14)
            .cursor(if props.disabled { "not-allowed" } else { "text" })
            .transition("all 150ms ease")
            .outline("none")
            .build()
    });

    let handle_input = move |e: Event<FormData>| {
        let value = e.value();

        // Basic validation - allow only digits and colons
        let filtered: String = value.chars()
            .filter(|c| c.is_ascii_digit() || *c == ':')
            .collect();

        // Auto-format as user types
        let formatted = format_time_input(&filtered, props.show_seconds);
        input_value.set(formatted.clone());

        // Validate and emit if valid
        if is_valid_time(&formatted, props.show_seconds) {
            props.on_change.call(Some(formatted));
        }
    };

    let placeholder = if props.show_seconds {
        props.placeholder.clone().unwrap_or_else(|| "HH:MM:SS".to_string())
    } else {
        props.placeholder.clone().unwrap_or_else(|| "HH:MM".to_string())
    };

    rsx! {
        input {
            r#type: "text",
            class: "time-input{class_css}",
            style: "{input_style}",
            placeholder: "{placeholder}",
            value: "{input_value}",
            disabled: props.disabled,
            oninput: handle_input,
        }
    }
}

/// Format time input as user types
fn format_time_input(input: &str, show_seconds: bool) -> String {
    let digits: String = input.chars().filter(|c| c.is_ascii_digit()).collect();

    if show_seconds {
        match digits.len() {
            0..=2 => digits,
            3..=4 => format!("{}:{}", &digits[..2], &digits[2..]),
            _ => format!("{}:{}:{}", &digits[..2], &digits[2..4], &digits[4..6.min(digits.len())]),
        }
    } else {
        match digits.len() {
            0..=2 => digits,
            _ => format!("{}:{}", &digits[..2], &digits[2..4.min(digits.len())]),
        }
    }
}

/// Validate time string
fn is_valid_time(input: &str, show_seconds: bool) -> bool {
    let parts: Vec<&str> = input.split(':').collect();

    if show_seconds {
        if parts.len() != 3 {
            return false;
        }
        if let (Ok(h), Ok(m), Ok(s)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>(), parts[2].parse::<u32>()) {
            h <= 23 && m <= 59 && s <= 59
        } else {
            false
        }
    } else {
        if parts.len() != 2 {
            return false;
        }
        if let (Ok(h), Ok(m)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
            h <= 23 && m <= 59
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_value_parse() {
        let time = TimeValue::from_string("14:30", true).unwrap();
        assert_eq!(time.hour, 14);
        assert_eq!(time.minute, 30);
        assert_eq!(time.second, 0);
        assert!(!time.is_pm);
    }

    #[test]
    fn test_time_value_parse_12h() {
        let time = TimeValue::from_string("14:30", false).unwrap();
        assert_eq!(time.hour, 2);
        assert_eq!(time.minute, 30);
        assert!(time.is_pm);
    }

    #[test]
    fn test_time_value_to_string() {
        let time = TimeValue { hour: 2, minute: 30, second: 0, is_pm: true };
        assert_eq!(time.to_string(true, false), "14:30");
        assert_eq!(time.to_string(false, false), "02:30");
    }

    #[test]
    fn test_format_time_input() {
        assert_eq!(format_time_input("123", false), "12:3");
        assert_eq!(format_time_input("1234", false), "12:34");
        assert_eq!(format_time_input("123456", true), "12:34:56");
    }

    #[test]
    fn test_is_valid_time() {
        assert!(is_valid_time("12:30", false));
        assert!(is_valid_time("23:59", false));
        assert!(!is_valid_time("24:00", false));
        assert!(!is_valid_time("12:60", false));
        assert!(is_valid_time("12:30:45", true));
        assert!(!is_valid_time("12:30", true)); // missing seconds
    }
}
