//! Date Range Picker organism component
//!
//! A date picker for selecting date ranges with two side-by-side calendars.

use dioxus::prelude::*;
use crate::theme::use_theme;

/// Date range preset definition
#[derive(Clone, PartialEq)]
pub struct DateRangePreset {
    /// Display label for the preset
    pub label: String,
    /// Function that returns (start_date, end_date) in YYYY-MM-DD format
    pub get_range: fn() -> (String, String),
}

impl DateRangePreset {
    /// Create a new preset
    pub fn new(label: impl Into<String>, get_range: fn() -> (String, String)) -> Self {
        Self {
            label: label.into(),
            get_range,
        }
    }
}

/// Date range picker properties
#[derive(Props, Clone, PartialEq)]
pub struct DateRangePickerProps {
    /// Selected start date (YYYY-MM-DD format)
    #[props(default)]
    pub start_date: Option<String>,
    /// Selected end date (YYYY-MM-DD format)
    #[props(default)]
    pub end_date: Option<String>,
    /// Start date change handler
    #[props(default)]
    pub on_start_date_change: Option<EventHandler<Option<String>>>,
    /// End date change handler
    #[props(default)]
    pub on_end_date_change: Option<EventHandler<Option<String>>>,
    /// Range change handler (called when both dates are selected)
    #[props(default)]
    pub on_range_change: Option<EventHandler<(Option<String>, Option<String>)>>,
    /// Minimum selectable date (YYYY-MM-DD)
    #[props(default)]
    pub min_date: Option<String>,
    /// Maximum selectable date (YYYY-MM-DD)
    #[props(default)]
    pub max_date: Option<String>,
    /// Placeholder text
    #[props(default = "Select date range...".to_string())]
    pub placeholder: String,
    /// Whether the picker is disabled
    #[props(default = false)]
    pub disabled: bool,
    /// Preset date ranges
    #[props(default = Vec::new())]
    pub presets: Vec<DateRangePreset>,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Date range picker component
#[component]
pub fn DateRangePicker(props: DateRangePickerProps) -> Element {
    let theme = use_theme();
    let mut is_open = use_signal(|| false);
    let mut selecting_start = use_signal(|| true);
    
    let class_css = props.class.as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();
    
    // Format display value
    let display_value = format_date_range(&props.start_date, &props.end_date);
    
    // Handle date selection
    let handle_select = {
        let start_date = props.start_date.clone();
        let end_date = props.end_date.clone();
        let on_start_date_change = props.on_start_date_change.clone();
        let on_end_date_change = props.on_end_date_change.clone();
        let on_range_change = props.on_range_change.clone();
        let mut selecting_start = selecting_start.clone();
        
        move |date: String| {
            if selecting_start() {
                // Selecting start date
                if let Some(ref handler) = on_start_date_change {
                    handler.call(Some(date.clone()));
                }
                // If new start is after current end, clear end
                if let Some(ref end) = end_date {
                    if let (Some(d), Some(e)) = (parse_date(&date), parse_date(end)) {
                        if d > e {
                            if let Some(ref handler) = on_end_date_change {
                                handler.call(None);
                            }
                        }
                    }
                }
                selecting_start.set(false);
            } else {
                // Selecting end date
                let should_close = true;
                
                // Validate end is not before start
                if let Some(ref start) = start_date {
                    if let (Some(s), Some(d)) = (parse_date(start), parse_date(&date)) {
                        if d >= s {
                            if let Some(ref handler) = on_end_date_change {
                                handler.call(Some(date.clone()));
                            }
                            if let Some(ref handler) = on_range_change {
                                handler.call((Some(start.clone()), Some(date)));
                            }
                        } else {
                            // End before start - swap or reset
                            if let Some(ref handler) = on_start_date_change {
                                handler.call(Some(date));
                            }
                            if let Some(ref handler) = on_end_date_change {
                                handler.call(Some(start.clone()));
                            }
                            if let Some(ref handler) = on_range_change {
                                handler.call((Some(start_date.clone().unwrap()), Some(start.clone())));
                            }
                        }
                    }
                } else {
                    // No start date selected, make this the start
                    if let Some(ref handler) = on_start_date_change {
                        handler.call(Some(date));
                    }
                    if let Some(ref handler) = on_end_date_change {
                        handler.call(None);
                    }
                }
                
                if should_close {
                    selecting_start.set(true);
                    is_open.set(false);
                }
            }
        }
    };
    
    // Handle preset selection
    let handle_preset = {
        let on_start_date_change = props.on_start_date_change.clone();
        let on_end_date_change = props.on_end_date_change.clone();
        let on_range_change = props.on_range_change.clone();
        let mut is_open = is_open.clone();
        
        move |(start, end): (String, String)| {
            if let Some(ref handler) = on_start_date_change {
                handler.call(Some(start.clone()));
            }
            if let Some(ref handler) = on_end_date_change {
                handler.call(Some(end.clone()));
            }
            if let Some(ref handler) = on_range_change {
                handler.call((Some(start), Some(end)));
            }
            
            is_open.set(false);
        }
    };
    
    // Handle clear
    let handle_clear = {
        let on_start_date_change = props.on_start_date_change.clone();
        let on_end_date_change = props.on_end_date_change.clone();
        let on_range_change = props.on_range_change.clone();
        let mut selecting_start = selecting_start.clone();
        
        move |e: Event<dioxus::html::MouseData>| {
            e.stop_propagation();
            
            if let Some(ref handler) = on_start_date_change {
                handler.call(None);
            }
            if let Some(ref handler) = on_end_date_change {
                handler.call(None);
            }
            if let Some(ref handler) = on_range_change {
                handler.call((None, None));
            }
            
            selecting_start.set(true);
        }
    };
    
    // Close dropdown
    let close_dropdown = {
        let mut is_open = is_open.clone();
        let mut selecting_start = selecting_start.clone();
        
        move || {
            is_open.set(false);
            selecting_start.set(true);
        }
    };
    
    rsx! {
        div {
            class: "date-range-picker{class_css}",
            style: "display: flex; flex-direction: column; gap: 6px; position: relative;",
            
            div {
                style: "position: relative;",
                
                // Trigger button
                button {
                    type: "button",
                    class: "date-range-picker-trigger",
                    style: "width: 100%; padding: 10px 14px; font-size: 16px; border: 1px solid {theme.tokens.read().colors.border.to_rgba()}; border-radius: 8px; background: white; color: {theme.tokens.read().colors.foreground.to_rgba()}; cursor: pointer; display: flex; align-items: center; justify-content: space-between;",
                    disabled: props.disabled,
                    onclick: move |_| {
                        if !props.disabled {
                            is_open.toggle();
                            if is_open() {
                                selecting_start.set(true);
                            }
                        }
                    },
                    
                    if props.start_date.is_some() || props.end_date.is_some() {
                        span { "{display_value}" }
                    } else {
                        span {
                            style: "color: {theme.tokens.read().colors.muted.to_rgba()};",
                            "{props.placeholder}"
                        }
                    }
                    
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        
                        // Clear button
                        if props.start_date.is_some() || props.end_date.is_some() {
                            button {
                                type: "button",
                                style: "background: none; border: none; cursor: pointer; font-size: 14px; color: {theme.tokens.read().colors.muted.to_rgba()}; padding: 2px;",
                                onclick: handle_clear,
                                "✕"
                            }
                        }
                        
                        // Calendar icon
                        span {
                            style: "font-size: 14px;",
                            "📅"
                        }
                    }
                }
                
                // Dropdown
                if is_open() && !props.disabled {
                    DateRangePickerDropdown {
                        start_date: props.start_date.clone(),
                        end_date: props.end_date.clone(),
                        min_date: props.min_date.clone(),
                        max_date: props.max_date.clone(),
                        presets: props.presets.clone(),
                        selecting_start: selecting_start(),
                        on_select: handle_select,
                        on_preset_select: handle_preset,
                        on_close: close_dropdown,
                    }
                }
            }
        }
    }
}

/// Date range picker dropdown component
#[derive(Props, Clone, PartialEq)]
pub struct DateRangePickerDropdownProps {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub min_date: Option<String>,
    pub max_date: Option<String>,
    pub presets: Vec<DateRangePreset>,
    pub selecting_start: bool,
    pub on_select: EventHandler<String>,
    pub on_preset_select: EventHandler<(String, String)>,
    pub on_close: EventHandler<()>,
}

#[component]
fn DateRangePickerDropdown(props: DateRangePickerDropdownProps) -> Element {
    // Parse dates for view state
    let today = get_today();
    let start = props.start_date.as_ref().and_then(|v| parse_date(v));
    
    // View months (current and next month)
    let initial_view = start.clone().unwrap_or_else(|| today.clone());
    let left_view = use_signal(move || initial_view.clone());
    
    // Right view is always one month after left
    let right_view = {
        let left = left_view();
        let (year, month) = if left.month == 12 {
            (left.year + 1, 1)
        } else {
            (left.year, left.month + 1)
        };
        Date { year, month, day: 1 }
    };
    
    // Click container to stop propagation
    let onclick_container = move |e: Event<dioxus::html::MouseData>| {
        e.stop_propagation();
    };
    
    rsx! {
        // Backdrop
        div {
            class: "date-range-picker-backdrop",
            style: "position: fixed; inset: 0; z-index: 40;",
            onclick: move |_| props.on_close.call(()),
        }
        
        // Dropdown
        div {
            class: "date-range-picker-dropdown",
            style: get_dropdown_style(),
            onclick: onclick_container,
            
            // Presets section
            if !props.presets.is_empty() {
                DateRangePresets {
                    presets: props.presets.clone(),
                    on_preset_select: props.on_preset_select.clone(),
                }
            }
            
            // Two calendars
            div {
                style: "display: flex; gap: 24px;",
                
                // Left calendar
                Calendar {
                    view_date: left_view(),
                    start_date: props.start_date.clone(),
                    end_date: props.end_date.clone(),
                    min_date: props.min_date.clone(),
                    max_date: props.max_date.clone(),
                    selecting_start: props.selecting_start,
                    show_prev_nav: true,
                    show_next_nav: true,
                    on_navigate_prev: {
                        let mut left_view = left_view.clone();
                        move || {
                            let current = left_view();
                            let new_month = if current.month == 1 { 12 } else { current.month - 1 };
                            let new_year = if current.month == 1 { current.year - 1 } else { current.year };
                            left_view.set(Date { year: new_year, month: new_month, day: 1 });
                        }
                    },
                    on_navigate_next: {
                        let mut left_view = left_view.clone();
                        move || {
                            let current = left_view();
                            let new_month = if current.month == 12 { 1 } else { current.month + 1 };
                            let new_year = if current.month == 12 { current.year + 1 } else { current.year };
                            left_view.set(Date { year: new_year, month: new_month, day: 1 });
                        }
                    },
                    on_select: props.on_select.clone(),
                }
                
                // Right calendar
                Calendar {
                    view_date: right_view.clone(),
                    start_date: props.start_date.clone(),
                    end_date: props.end_date.clone(),
                    min_date: props.min_date.clone(),
                    max_date: props.max_date.clone(),
                    selecting_start: props.selecting_start,
                    show_prev_nav: false,
                    show_next_nav: true,
                    on_navigate_prev: move || {},
                    on_navigate_next: {
                        let mut left_view = left_view.clone();
                        move || {
                            let current = left_view();
                            let new_month = if current.month == 12 { 1 } else { current.month + 1 };
                            let new_year = if current.month == 12 { current.year + 1 } else { current.year };
                            left_view.set(Date { year: new_year, month: new_month, day: 1 });
                        }
                    },
                    on_select: props.on_select.clone(),
                }
            }
            
            // Footer with quick actions
            DateRangeFooter {
                today: today.clone(),
                on_select: props.on_select.clone(),
                on_close: props.on_close.clone(),
            }
        }
    }
}

fn get_dropdown_style() -> String {
    "position: absolute; top: calc(100% + 4px); left: 0; z-index: 50; background: white; border: 1px solid #e5e7eb; border-radius: 12px; box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1); padding: 16px;".to_string()
}

/// Presets component
#[derive(Props, Clone, PartialEq)]
pub struct DateRangePresetsProps {
    pub presets: Vec<DateRangePreset>,
    pub on_preset_select: EventHandler<(String, String)>,
}

#[component]
fn DateRangePresets(props: DateRangePresetsProps) -> Element {
    let theme = use_theme();
    
    rsx! {
        div {
            class: "date-range-picker-presets",
            style: "display: flex; flex-wrap: wrap; gap: 8px; margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid {theme.tokens.read().colors.border.to_rgba()};",
            
            for preset in props.presets.clone() {
                PresetButton {
                    key: "{preset.label}",
                    preset: preset.clone(),
                    on_preset_select: props.on_preset_select.clone(),
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct PresetButtonProps {
    pub preset: DateRangePreset,
    pub on_preset_select: EventHandler<(String, String)>,
}

#[component]
fn PresetButton(props: PresetButtonProps) -> Element {
    let theme = use_theme();
    let preset = props.preset.clone();
    
    rsx! {
        button {
            type: "button",
            style: "padding: 6px 12px; font-size: 12px; font-weight: 500; color: {theme.tokens.read().colors.primary.to_rgba()}; background: {theme.tokens.read().colors.primary.to_rgba()}15; border: none; border-radius: 6px; cursor: pointer; transition: all 0.15s ease;",
            onclick: move |_| {
                let (start, end) = (preset.get_range)();
                props.on_preset_select.call((start, end));
            },
            "{props.preset.label}"
        }
    }
}

/// Footer component
#[derive(Props, Clone, PartialEq)]
pub struct DateRangeFooterProps {
    pub today: Date,
    pub on_select: EventHandler<String>,
    pub on_close: EventHandler<()>,
}

#[component]
fn DateRangeFooter(props: DateRangeFooterProps) -> Element {
    let theme = use_theme();
    let today_str = format!("{:04}-{:02}-{:02}", props.today.year, props.today.month, props.today.day);
    
    rsx! {
        div {
            style: "display: flex; justify-content: space-between; margin-top: 16px; padding-top: 12px; border-top: 1px solid {theme.tokens.read().colors.border.to_rgba()};",
            
            // Today button
            button {
                type: "button",
                style: "font-size: 12px; color: {theme.tokens.read().colors.muted.to_rgba()}; background: none; border: none; cursor: pointer;",
                onclick: move |_| props.on_select.call(today_str.clone()),
                "Today"
            }
            
            // Close button
            button {
                type: "button",
                style: "font-size: 12px; color: {theme.tokens.read().colors.foreground.to_rgba()}; background: none; border: none; cursor: pointer; font-weight: 500;",
                onclick: move |_| props.on_close.call(()),
                "Done"
            }
        }
    }
}

/// Calendar sub-component
#[derive(Props, Clone, PartialEq)]
pub struct CalendarProps {
    pub view_date: Date,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub min_date: Option<String>,
    pub max_date: Option<String>,
    pub selecting_start: bool,
    pub show_prev_nav: bool,
    pub show_next_nav: bool,
    pub on_navigate_prev: EventHandler<()>,
    pub on_navigate_next: EventHandler<()>,
    pub on_select: EventHandler<String>,
}

#[component]
fn Calendar(props: CalendarProps) -> Element {
    let theme = use_theme();
    
    let year = props.view_date.year;
    let month = props.view_date.month;
    let days_in_month = get_days_in_month(year, month);
    let first_day_of_week = get_first_day_of_week(year, month);
    let month_name = month_name(month);
    
    let today = get_today();
    let start = props.start_date.as_ref().and_then(|v| parse_date(v));
    let end = props.end_date.as_ref().and_then(|v| parse_date(v));
    
    rsx! {
        div {
            class: "date-range-picker-calendar",
            style: "min-width: 280px;",
            
            // Header
            div {
                style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 16px;",
                
                if props.show_prev_nav {
                    button {
                        type: "button",
                        style: "background: none; border: none; cursor: pointer; font-size: 16px; padding: 4px 8px; border-radius: 4px;",
                        onclick: move |_| props.on_navigate_prev.call(()),
                        "◀"
                    }
                } else {
                    div { style: "width: 32px;", "" }
                }
                
                span {
                    style: "font-weight: 600; font-size: 16px;",
                    "{month_name} {year}"
                }
                
                if props.show_next_nav {
                    button {
                        type: "button",
                        style: "background: none; border: none; cursor: pointer; font-size: 16px; padding: 4px 8px; border-radius: 4px;",
                        onclick: move |_| props.on_navigate_next.call(()),
                        "▶"
                    }
                } else {
                    div { style: "width: 32px;", "" }
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
                style: "display: grid; grid-template-columns: repeat(7, 1fr); gap: 2px;",
                
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
                        let is_today = date == today;
                        let is_start = start.as_ref() == Some(&date);
                        let is_end = end.as_ref() == Some(&date);
                        let is_in_range = if let (Some(ref s), Some(ref e)) = (&start, &end) {
                            date > *s && date < *e
                        } else {
                            false
                        };
                        let is_disabled = is_date_disabled(&date, props.min_date.as_ref(), props.max_date.as_ref());
                        
                        let bg_color = if is_start || is_end {
                            theme.tokens.read().colors.primary.to_rgba()
                        } else if is_in_range {
                            format!("{}30", theme.tokens.read().colors.primary.to_rgba().trim_start_matches('#'))
                        } else if is_today {
                            theme.tokens.read().colors.muted.to_rgba()
                        } else {
                            "transparent".to_string()
                        };
                        
                        let text_color = if is_start || is_end {
                            "white".to_string()
                        } else if is_disabled {
                            theme.tokens.read().colors.muted.to_rgba()
                        } else {
                            theme.tokens.read().colors.foreground.to_rgba()
                        };
                        
                        let border_radius = if is_start {
                            "8px 0 0 8px".to_string()
                        } else if is_end {
                            "0 8px 8px 0".to_string()
                        } else if is_in_range {
                            "0".to_string()
                        } else {
                            "8px".to_string()
                        };
                        
                        let cursor = if is_disabled { "not-allowed" } else { "pointer" };
                        let font_weight = if is_start || is_end || is_today { "600" } else { "400" };
                        
                        let day_value = day;
                        let year_value = year;
                        let month_value = month;
                        let on_select = props.on_select.clone();
                        
                        rsx! {
                            button {
                                key: "{day}",
                                type: "button",
                                style: "aspect-ratio: 1; display: flex; align-items: center; justify-content: center; font-size: 14px; border-radius: {border_radius}; background: {bg_color}; color: {text_color}; cursor: {cursor}; border: none; font-weight: {font_weight};",
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
        }
    }
}

// ============================================================================
// Date utilities (shared with date_picker.rs)
// ============================================================================

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

fn format_date(date: &Date) -> String {
    format!("{:02}/{:02}/{}", date.month, date.day, date.year)
}

fn format_short_date(date: &Date) -> String {
    let month_abbr = month_abbr(date.month);
    format!("{} {}", month_abbr, date.day)
}

fn format_date_range(start: &Option<String>, end: &Option<String>) -> String {
    match (start.as_ref().and_then(|s| parse_date(s)), end.as_ref().and_then(|e| parse_date(e))) {
        (Some(s), Some(e)) => {
            if s.year == e.year {
                let s_abbr = month_abbr(s.month);
                let e_abbr = month_abbr(e.month);
                if s.month == e.month {
                    format!("{} {}-{}", s_abbr, s.day, e.day)
                } else {
                    format!("{} {} - {} {}", s_abbr, s.day, e_abbr, e.day)
                }
            } else {
                format!("{} - {}", format_short_date(&s), format_short_date(&e))
            }
        }
        (Some(s), None) => format_short_date(&s),
        (None, Some(e)) => format_short_date(&e),
        (None, None) => "Select date range...".to_string(),
    }
}

fn month_abbr(month: u8) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "",
    }
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

fn get_today() -> Date {
    // Simplified - in a real app, use the system date
    Date { year: 2024, month: 3, day: 19 }
}

fn get_days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap_year(year) { 29 } else { 28 },
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

// ============================================================================
// Default presets
// ============================================================================

/// Create default date range presets
pub fn default_presets() -> Vec<DateRangePreset> {
    vec![
        DateRangePreset::new("Today", || {
            let today = get_today();
            let s = format!("{:04}-{:02}-{:02}", today.year, today.month, today.day);
            (s.clone(), s)
        }),
        DateRangePreset::new("Yesterday", || {
            let yesterday = add_days(&get_today(), -1);
            let s = format!("{:04}-{:02}-{:02}", yesterday.year, yesterday.month, yesterday.day);
            (s.clone(), s)
        }),
        DateRangePreset::new("Last 7 days", || {
            let end = get_today();
            let start = add_days(&end, -6);
            let s = format!("{:04}-{:02}-{:02}", start.year, start.month, start.day);
            let e = format!("{:04}-{:02}-{:02}", end.year, end.month, end.day);
            (s, e)
        }),
        DateRangePreset::new("Last 30 days", || {
            let end = get_today();
            let start = add_days(&end, -29);
            let s = format!("{:04}-{:02}-{:02}", start.year, start.month, start.day);
            let e = format!("{:04}-{:02}-{:02}", end.year, end.month, end.day);
            (s, e)
        }),
        DateRangePreset::new("This month", || {
            let today = get_today();
            let start = Date { year: today.year, month: today.month, day: 1 };
            let s = format!("{:04}-{:02}-{:02}", start.year, start.month, start.day);
            let e = format!("{:04}-{:02}-{:02}", today.year, today.month, today.day);
            (s, e)
        }),
        DateRangePreset::new("Last month", || {
            let today = get_today();
            let (year, month) = if today.month == 1 {
                (today.year - 1, 12)
            } else {
                (today.year, today.month - 1)
            };
            let days = get_days_in_month(year, month);
            let start = Date { year, month, day: 1 };
            let end = Date { year, month, day: days };
            let s = format!("{:04}-{:02}-{:02}", start.year, start.month, start.day);
            let e = format!("{:04}-{:02}-{:02}", end.year, end.month, end.day);
            (s, e)
        }),
    ]
}

fn add_days(date: &Date, days: i32) -> Date {
    // Simplified date arithmetic
    let mut result = date.clone();
    let days_in_month = get_days_in_month(result.year, result.month) as i32;
    
    let new_day = result.day as i32 + days;
    
    if new_day > 0 && new_day <= days_in_month {
        result.day = new_day as u8;
    } else if new_day > days_in_month {
        // Move to next month
        result.day = (new_day - days_in_month) as u8;
        if result.month == 12 {
            result.month = 1;
            result.year += 1;
        } else {
            result.month += 1;
        }
    } else {
        // Move to previous month
        if result.month == 1 {
            result.month = 12;
            result.year -= 1;
        } else {
            result.month -= 1;
        }
        let prev_days = get_days_in_month(result.year, result.month) as i32;
        result.day = (prev_days + new_day) as u8;
    }
    
    result
}
