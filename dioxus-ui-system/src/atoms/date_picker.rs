//! Date picker atom component
//!
//! Date and date range selection input.

use crate::theme::use_theme;
use dioxus::prelude::*;

/// Date picker properties
#[derive(Props, Clone, PartialEq)]
pub struct DatePickerProps {
    /// Selected date (YYYY-MM-DD format)
    #[props(default)]
    pub value: Option<String>,
    /// Change handler
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,
    /// Minimum date (YYYY-MM-DD)
    #[props(default)]
    pub min: Option<String>,
    /// Maximum date (YYYY-MM-DD)
    #[props(default)]
    pub max: Option<String>,
    /// Placeholder text
    #[props(default)]
    pub placeholder: Option<String>,
    /// Label
    #[props(default)]
    pub label: Option<String>,
    /// Whether the field is required
    #[props(default = false)]
    pub required: bool,
    /// Whether the field is disabled
    #[props(default = false)]
    pub disabled: bool,
    /// Error message
    #[props(default)]
    pub error: Option<String>,
    /// Size variant
    #[props(default = DatePickerSize::Md)]
    pub size: DatePickerSize,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Date picker size
#[derive(Default, Clone, PartialEq, Debug)]
pub enum DatePickerSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl DatePickerSize {
    fn to_padding(&self) -> &'static str {
        match self {
            DatePickerSize::Sm => "6px 10px",
            DatePickerSize::Md => "10px 14px",
            DatePickerSize::Lg => "14px 18px",
        }
    }

    fn to_font_size(&self) -> &'static str {
        match self {
            DatePickerSize::Sm => "14px",
            DatePickerSize::Md => "16px",
            DatePickerSize::Lg => "18px",
        }
    }
}

/// Date picker component
#[component]
pub fn DatePicker(props: DatePickerProps) -> Element {
    let theme = use_theme();
    let mut is_open = use_signal(|| false);

    let class_css = props
        .class
        .as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();

    let border_color = if props.error.is_some() {
        theme.tokens.read().colors.destructive.to_rgba()
    } else {
        theme.tokens.read().colors.border.to_rgba()
    };

    let padding = props.size.to_padding();
    let font_size = props.size.to_font_size();

    // Parse current value for display
    let display_value = props
        .value
        .as_ref()
        .and_then(|v| parse_date(v))
        .map(|d| format!("{:02}/{:02}/{}", d.month, d.day, d.year))
        .unwrap_or_else(|| {
            props
                .placeholder
                .clone()
                .unwrap_or_else(|| "Select date".to_string())
        });

    rsx! {
        div {
            class: "date-picker{class_css}",
            style: "display: flex; flex-direction: column; gap: 6px; position: relative;",

            if let Some(label) = props.label {
                label {
                    class: "date-picker-label",
                    style: "font-size: 14px; font-weight: 500; color: {theme.tokens.read().colors.foreground.to_rgba()};",
                    "{label}"
                    if props.required {
                        span {
                            style: "color: {theme.tokens.read().colors.destructive.to_rgba()}; margin-left: 4px;",
                            "*"
                        }
                    }
                }
            }

            div {
                style: "position: relative;",

                button {
                    type: "button",
                    class: "date-picker-trigger",
                    style: "width: 100%; padding: {padding}; font-size: {font_size}; border: 1px solid {border_color}; border-radius: 8px; background: white; color: {theme.tokens.read().colors.foreground.to_rgba()}; cursor: pointer; display: flex; align-items: center; justify-content: space-between;",
                    disabled: props.disabled,
                    onclick: move |_| is_open.toggle(),

                    if props.value.is_some() {
                        span { "{display_value}" }
                    } else {
                        span {
                            style: "color: {theme.tokens.read().colors.muted.to_rgba()};",
                            "{props.placeholder.clone().unwrap_or_else(|| \"Select date\".to_string())}"
                        }
                    }

                    span {
                        style: "font-size: 14px;",
                        "📅"
                    }
                }

                if is_open() && !props.disabled {
                    DatePickerCalendar {
                        value: props.value.clone(),
                        min: props.min.clone(),
                        max: props.max.clone(),
                        on_select: move |date: String| {
                            if let Some(handler) = &props.on_change {
                                handler.call(date);
                            }
                            is_open.set(false);
                        },
                        on_close: move || is_open.set(false),
                    }
                }
            }

            if let Some(error) = props.error {
                span {
                    class: "date-picker-error",
                    style: "font-size: 12px; color: {theme.tokens.read().colors.destructive.to_rgba()};",
                    "{error}"
                }
            }
        }
    }
}

/// Calendar popup component
#[derive(Props, Clone, PartialEq)]
pub struct DatePickerCalendarProps {
    pub value: Option<String>,
    pub min: Option<String>,
    pub max: Option<String>,
    pub on_select: EventHandler<String>,
    pub on_close: EventHandler<()>,
}

#[component]
fn DatePickerCalendar(props: DatePickerCalendarProps) -> Element {
    let theme = use_theme();

    // Parse current date or default to today
    let today = get_today();
    let selected = props.value.as_ref().and_then(|v| parse_date(v));
    let selected_for_view = selected.clone();
    let today_for_view = today.clone();
    let mut view_date = use_signal(move || selected_for_view.unwrap_or(today_for_view));

    let year = view_date().year;
    let month = view_date().month;

    let days_in_month = get_days_in_month(year, month);
    let first_day_of_week = get_first_day_of_week(year, month);

    let month_name = month_name(month);

    // Click outside to close
    let onclick_container = move |e: Event<dioxus::html::MouseData>| {
        e.stop_propagation();
    };

    let onclick_outside = move || {
        props.on_close.call(());
    };

    rsx! {
        // Backdrop
        div {
            class: "date-picker-backdrop",
            style: "position: fixed; inset: 0; z-index: 40;",
            onclick: move |_| onclick_outside(),
        }

        // Calendar popup
        div {
            class: "date-picker-calendar",
            style: "position: absolute; top: calc(100% + 4px); left: 0; z-index: 50; background: white; border: 1px solid {theme.tokens.read().colors.border.to_rgba()}; border-radius: 12px; box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1); padding: 16px; min-width: 280px;",
            onclick: onclick_container,

            // Header
            div {
                style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 16px;",

                button {
                    type: "button",
                    style: "background: none; border: none; cursor: pointer; font-size: 16px; padding: 4px 8px; border-radius: 4px;",
                    onclick: move |_| {
                        let new_month = if month == 1 { 12 } else { month - 1 };
                        let new_year = if month == 1 { year - 1 } else { year };
                        view_date.set(Date { year: new_year, month: new_month, day: 1 });
                    },
                    "◀"
                }

                span {
                    style: "font-weight: 600; font-size: 16px;",
                    "{month_name} {year}"
                }

                button {
                    type: "button",
                    style: "background: none; border: none; cursor: pointer; font-size: 16px; padding: 4px 8px; border-radius: 4px;",
                    onclick: move |_| {
                        let new_month = if month == 12 { 1 } else { month + 1 };
                        let new_year = if month == 12 { year + 1 } else { year };
                        view_date.set(Date { year: new_year, month: new_month, day: 1 });
                    },
                    "▶"
                }
            }

            // Weekday headers
            div {
                style: "display: grid; grid-template-columns: repeat(7, 1fr); gap: 4px; margin-bottom: 8px;",

                for day in ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"] {
                    div {
                        key: "{day}",
                        style: "text-align: center; font-size: 12px; font-weight: 600; color: {theme.tokens.read().colors.muted.to_rgba()}; padding: 4px;",
                        "{day}"
                    }
                }
            }

            // Days grid
            div {
                style: "display: grid; grid-template-columns: repeat(7, 1fr); gap: 4px;",

                // Empty cells for days before month starts
                for _ in 0..first_day_of_week {
                    div {
                        style: "aspect-ratio: 1;",
                    }
                }

                // Days of the month
                for day in 1..=days_in_month {
                    {
                        let date = Date { year, month, day };
                        let is_selected = selected.as_ref() == Some(&date);
                        let is_today = date == today;
                        let is_disabled = is_date_disabled(&date, props.min.as_ref(), props.max.as_ref());

                        let bg_color = if is_selected {
                            theme.tokens.read().colors.primary.to_rgba()
                        } else if is_today {
                            theme.tokens.read().colors.muted.to_rgba()
                        } else {
                            "transparent".to_string()
                        };

                        let text_color = if is_selected {
                            "white".to_string()
                        } else if is_disabled {
                            theme.tokens.read().colors.muted.to_rgba()
                        } else {
                            theme.tokens.read().colors.foreground.to_rgba()
                        };

                        let cursor = if is_disabled { "not-allowed" } else { "pointer" };

                        let day_value = day;
                        let year_value = year;
                        let month_value = month;
                        let on_select = props.on_select.clone();

                        rsx! {
                            button {
                                key: "{day}",
                                type: "button",
                                style: "aspect-ratio: 1; display: flex; align-items: center; justify-content: center; font-size: 14px; border-radius: 8px; background: {bg_color}; color: {text_color}; cursor: {cursor}; border: none;",
                                disabled: is_disabled,
                                onclick: move |_| {
                                    on_select.call(format!("{:04}-{:02}-{:02}", year_value, month_value, day_value));
                                },
                                "{day}"
                            }
                        }
                    }
                }
            }

            // Footer
            div {
                style: "display: flex; justify-content: space-between; margin-top: 16px; padding-top: 12px; border-top: 1px solid {theme.tokens.read().colors.border.to_rgba()};",

                {
                    let today_clone = today.clone();
                    rsx! {
                        button {
                            type: "button",
                            style: "font-size: 12px; color: {theme.tokens.read().colors.muted.to_rgba()}; background: none; border: none; cursor: pointer;",
                            onclick: move |_| {
                                let today_str = format!("{:04}-{:02}-{:02}", today_clone.year, today_clone.month, today_clone.day);
                                props.on_select.call(today_str);
                            },
                            "Today"
                        }
                    }
                }

                if selected.is_some() {
                    button {
                        type: "button",
                        style: "font-size: 12px; color: {theme.tokens.read().colors.destructive.to_rgba()}; background: none; border: none; cursor: pointer;",
                        onclick: move |_| {
                            // Clear selection - you might want to handle this differently
                            props.on_close.call(());
                        },
                        "Clear"
                    }
                }
            }
        }
    }
}

// Date utilities
#[derive(Clone, PartialEq)]
struct Date {
    year: i32,
    month: u8,
    day: u8,
}

fn parse_date(s: &str) -> Option<Date> {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 3 {
        return None;
    }

    let year = parts[0].parse().ok()?;
    let month = parts[1].parse().ok()?;
    let day = parts[2].parse().ok()?;

    Some(Date { year, month, day })
}

fn get_today() -> Date {
    // Simplified - in a real app, use the system date
    Date {
        year: 2024,
        month: 3,
        day: 19,
    }
}

fn get_days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn get_first_day_of_week(year: i32, month: u8) -> u8 {
    // Simplified Zeller's congruence
    // Returns 0 = Sunday, 1 = Monday, etc.
    let mut y = year;
    let mut m = month as i32;

    if m < 3 {
        m += 12;
        y -= 1;
    }

    let k = y % 100;
    let j = y / 100;

    let day = (1 + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 + 5 * j) % 7;
    ((day + 5) % 7) as u8 // Adjust to make 0 = Sunday
}

fn month_name(month: u8) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "",
    }
}

fn is_date_disabled(date: &Date, min: Option<&String>, max: Option<&String>) -> bool {
    if let Some(min_str) = min {
        if let Some(min_date) = parse_date(min_str) {
            if *date < min_date {
                return true;
            }
        }
    }

    if let Some(max_str) = max {
        if let Some(max_date) = parse_date(max_str) {
            if *date > max_date {
                return true;
            }
        }
    }

    false
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((self.year, self.month, self.day).cmp(&(other.year, other.month, other.day)))
    }
}
