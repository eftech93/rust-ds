//! Calendar organism component
//!
//! A full calendar view (like Google Calendar mini-view or shadcn Calendar).
//! Supports single date, multiple dates, and range selection modes.

use dioxus::prelude::*;
use crate::theme::{use_theme, use_style};
use crate::styles::Style;

/// Calendar selection mode
#[derive(Clone, PartialEq, Default, Debug)]
pub enum CalendarMode {
    /// Single date selection (default)
    #[default]
    Single,
    /// Multiple date selection
    Multiple,
    /// Date range selection
    Range,
}

/// Calendar properties
#[derive(Props, Clone, PartialEq)]
pub struct CalendarProps {
    /// Selected date (YYYY-MM-DD format) for Single mode
    /// For Multiple mode, this is not used - use on_change to track multiple dates
    /// For Range mode, this represents the start date
    #[props(default)]
    pub value: Option<String>,
    /// Change handler - called when selection changes
    /// For Single mode: returns the selected date or None if cleared
    /// For Range mode: returns the start date when selecting, or end date format when complete
    #[props(default)]
    pub on_change: EventHandler<Option<String>>,
    /// Currently displayed month (YYYY-MM format) - for controlled mode
    #[props(default)]
    pub month: Option<String>,
    /// Callback when the displayed month changes
    #[props(default)]
    pub on_month_change: EventHandler<String>,
    /// List of disabled dates (YYYY-MM-DD format)
    #[props(default)]
    pub disabled_dates: Vec<String>,
    /// Minimum selectable date (YYYY-MM-DD format)
    #[props(default)]
    pub min_date: Option<String>,
    /// Maximum selectable date (YYYY-MM-DD format)
    #[props(default)]
    pub max_date: Option<String>,
    /// Show week numbers on the left side
    #[props(default = false)]
    pub show_week_numbers: bool,
    /// Selection mode
    #[props(default)]
    pub mode: CalendarMode,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Calendar organism component
#[component]
pub fn Calendar(props: CalendarProps) -> Element {
    let theme = use_theme();
    
    // Parse controlled month or default to current month
    let today = get_today();
    let initial_view_date = props.month.as_ref()
        .and_then(|m| parse_month_year(m))
        .unwrap_or_else(|| (today.year, today.month));
    
    let mut view_year = use_signal(|| initial_view_date.0);
    let mut view_month = use_signal(|| initial_view_date.1);
    
    // Sync with controlled month prop
    use_effect(move || {
        if let Some(month_str) = props.month.clone() {
            if let Some((year, month)) = parse_month_year(&month_str) {
                view_year.set(year);
                view_month.set(month);
            }
        }
    });
    
    // Selection state for multiple and range modes
    let mut selected_dates = use_signal(|| {
        if let Some(val) = props.value.clone() {
            vec![val]
        } else {
            Vec::new()
        }
    });
    
    // Range selection state
    let mut range_start = use_signal(|| None::<String>);
    
    let year = view_year();
    let month = view_month();
    
    let days_in_month = get_days_in_month(year, month);
    let first_day_of_week = get_first_day_of_week(year, month);
    let days_in_prev_month = if month == 1 {
        get_days_in_month(year - 1, 12)
    } else {
        get_days_in_month(year, month - 1)
    };
    
    let month_name = month_name_str(month);
    
    // Navigate to previous month
    let mut go_to_prev_month = move || {
        let new_month = if month == 1 { 12 } else { month - 1 };
        let new_year = if month == 1 { year - 1 } else { year };
        view_month.set(new_month);
        view_year.set(new_year);
        props.on_month_change.call(format!("{:04}-{:02}", new_year, new_month));
    };
    
    // Navigate to next month
    let mut go_to_next_month = move || {
        let new_month = if month == 12 { 1 } else { month + 1 };
        let new_year = if month == 12 { year + 1 } else { year };
        view_month.set(new_month);
        view_year.set(new_year);
        props.on_month_change.call(format!("{:04}-{:02}", new_year, new_month));
    };
    
    // Go to today
    let go_to_today = move || {
        let today = get_today();
        view_month.set(today.month);
        view_year.set(today.year);
        props.on_month_change.call(format!("{:04}-{:02}", today.year, today.month));
    };
    
    // Container style
    let container_style = use_style(|t| {
        Style::new()
            .inline_flex()
            .flex_col()
            .p(&t.spacing, "md")
            .rounded(&t.radius, "lg")
            .border(1, &t.colors.border)
            .bg(&t.colors.background)
            .gap(&t.spacing, "sm")
            .build()
    });
    
    // Header style
    let header_style = use_style(|t| {
        Style::new()
            .flex()
            .items_center()
            .justify_between()
            .px(&t.spacing, "sm")
            .build()
    });
    
    // Navigation button style
    let nav_button_style = use_style(move |t| {
        Style::new()
            .w("28px")
            .h("28px")
            .inline_flex()
            .items_center()
            .justify_center()
            .rounded(&t.radius, "md")
            .border(0, &t.colors.border)
            .bg_transparent()
            .cursor("pointer")
            .text_color(&t.colors.foreground)
            .transition("all 150ms ease")
            .build()
    });
    
    // Title style
    let title_style = use_style(|t| {
        Style::new()
            .font_weight(600)
            .text(&t.typography, "base")
            .text_color(&t.colors.foreground)
            .build()
    });
    
    // Weekday header style
    let weekday_style = use_style(|t| {
        Style::new()
            .w("36px")
            .h("32px")
            .inline_flex()
            .items_center()
            .justify_center()
            .text(&t.typography, "xs")
            .font_weight(500)
            .text_color(&t.colors.muted_foreground)
            .build()
    });
    
    // Weekday headers grid style
    let weekday_grid_style = use_style(|t| {
        Style::new()
            .grid()
            .gap(&t.spacing, "xs")
            .build()
    });
    
    let col_count = if props.show_week_numbers { 8 } else { 7 };
    
    rsx! {
        div {
            style: "{container_style} {props.style.clone().unwrap_or_default()}",
            
            // Header with navigation
            div {
                style: "{header_style}",
                
                // Previous month button
                button {
                    type: "button",
                    style: "{nav_button_style}",
                    onclick: move |_| go_to_prev_month(),
                    aria_label: "Previous month",
                    
                    CalendarChevron { direction: ChevronDirection::Left }
                }
                
                // Month/Year display with dropdowns
                div {
                    style: "display: flex; align-items: center; gap: 8px;",
                    
                    // Month selector
                    select {
                        style: "{title_style} border: none; background: transparent; cursor: pointer; padding: 4px;",
                        value: "{month}",
                        onchange: move |e: Event<dioxus::html::FormData>| {
                            if let Ok(new_month) = e.value().parse::<u8>() {
                                view_month.set(new_month);
                                props.on_month_change.call(format!("{:04}-{:02}", view_year(), new_month));
                            }
                        },
                        
                        for m in 1..=12 {
                            option {
                                value: "{m}",
                                selected: m == month,
                                "{month_name_str(m)}"
                            }
                        }
                    }
                    
                    // Year selector
                    select {
                        style: "{title_style} border: none; background: transparent; cursor: pointer; padding: 4px;",
                        value: "{year}",
                        onchange: move |e: Event<dioxus::html::FormData>| {
                            if let Ok(new_year) = e.value().parse::<i32>() {
                                view_year.set(new_year);
                                props.on_month_change.call(format!("{:04}-{:02}", new_year, view_month()));
                            }
                        },
                        
                        for y in (year - 10)..=(year + 10) {
                            option {
                                value: "{y}",
                                selected: y == year,
                                "{y}"
                            }
                        }
                    }
                }
                
                // Next month button
                button {
                    type: "button",
                    style: "{nav_button_style}",
                    onclick: move |_| go_to_next_month(),
                    aria_label: "Next month",
                    
                    CalendarChevron { direction: ChevronDirection::Right }
                }
            }
            
            // Weekday headers
            div {
                style: "{weekday_grid_style} grid-template-columns: repeat({col_count}, 1fr); margin-bottom: 4px;",
                
                if props.show_week_numbers {
                    div {
                        style: "{weekday_style} width: 40px;",
                        "Wk"
                    }
                }
                
                for day in ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"] {
                    div {
                        key: "{day}",
                        style: "{weekday_style}",
                        "{day}"
                    }
                }
            }
            
            // Days grid
            CalendarGrid {
                year,
                month,
                days_in_month,
                first_day_of_week,
                days_in_prev_month,
                today: today.clone(),
                selected_value: props.value.clone(),
                selected_dates,
                range_start,
                disabled_dates: props.disabled_dates.clone(),
                min_date: props.min_date.clone(),
                max_date: props.max_date.clone(),
                show_week_numbers: props.show_week_numbers,
                mode: props.mode.clone(),
                on_change: props.on_change.clone(),
                go_to_today,
            }
        }
    }
}

/// Calendar grid component
#[derive(Props, Clone, PartialEq)]
struct CalendarGridProps {
    year: i32,
    month: u8,
    days_in_month: u8,
    first_day_of_week: u8,
    days_in_prev_month: u8,
    today: Date,
    selected_value: Option<String>,
    selected_dates: Signal<Vec<String>>,
    range_start: Signal<Option<String>>,
    disabled_dates: Vec<String>,
    min_date: Option<String>,
    max_date: Option<String>,
    show_week_numbers: bool,
    mode: CalendarMode,
    on_change: EventHandler<Option<String>>,
    go_to_today: EventHandler<()>,
}

#[component]
fn CalendarGrid(props: CalendarGridProps) -> Element {
    let theme = use_theme();
    
    // Parse selected date
    let selected = props.selected_value.as_ref().and_then(|v| parse_date(v));
    
    // Calculate week numbers for the displayed days
    let mut days_data: Vec<DayData> = Vec::new();
    
    // Previous month days (padding)
    for i in 0..props.first_day_of_week {
        let day = props.days_in_prev_month - props.first_day_of_week + i + 1;
        let prev_month = if props.month == 1 { 12 } else { props.month - 1 };
        let prev_year = if props.month == 1 { props.year - 1 } else { props.year };
        let date = Date { year: prev_year, month: prev_month, day };
        let week_num = get_week_number(&date);
        days_data.push(DayData {
            date,
            is_current_month: false,
            week_number: week_num,
        });
    }
    
    // Current month days
    for day in 1..=props.days_in_month {
        let date = Date { year: props.year, month: props.month, day };
        days_data.push(DayData {
            date: date.clone(),
            is_current_month: true,
            week_number: get_week_number(&date),
        });
    }
    
    // Next month days (padding) to complete the grid
    let remaining_cells = (7 - (days_data.len() % 7)) % 7;
    for day in 1..=remaining_cells as u8 {
        let next_month = if props.month == 12 { 1 } else { props.month + 1 };
        let next_year = if props.month == 12 { props.year + 1 } else { props.year };
        let date = Date { year: next_year, month: next_month, day };
        let week_num = get_week_number(&date);
        days_data.push(DayData {
            date,
            is_current_month: false,
            week_number: week_num,
        });
    }
    
    // Group by weeks for week numbers
    let weeks: Vec<&[DayData]> = days_data.chunks(7).collect();
    
    let grid_style = use_style(|t| {
        Style::new()
            .grid()
            .gap(&t.spacing, "xs")
            .build()
    });
    
    let col_count = if props.show_week_numbers { 8 } else { 7 };
    
    rsx! {
        div {
            style: "{grid_style} grid-template-columns: repeat({col_count}, 1fr);",
            
            for (week_idx, week) in weeks.iter().enumerate() {
                // Week number
                if props.show_week_numbers {
                    div {
                        key: "wk-{week_idx}",
                        style: use_style(|t| {
                            Style::new()
                                .w("40px")
                                .h("36px")
                                .inline_flex()
                                .items_center()
                                .justify_center()
                                .text(&t.typography, "xs")
                                .text_color(&t.colors.muted_foreground)
                                .font_weight(500)
                                .build()
                        }),
                        "{week[0].week_number}"
                    }
                }
                
                // Days in this week
                for day_data in week.iter() {
                    CalendarDay {
                        key: "{day_data.date.year}-{day_data.date.month}-{day_data.date.day}",
                        date: day_data.date.clone(),
                        is_current_month: day_data.is_current_month,
                        is_today: day_data.date == props.today,
                        is_selected: selected.as_ref() == Some(&day_data.date),
                        is_disabled: is_date_disabled(
                            &day_data.date,
                            props.disabled_dates.clone(),
                            props.min_date.clone(),
                            props.max_date.clone(),
                        ),
                        selected_dates: props.selected_dates.clone(),
                        range_start: props.range_start.clone(),
                        mode: props.mode.clone(),
                        on_change: props.on_change.clone(),
                    }
                }
            }
        }
    }
}

/// Data for a calendar day
#[derive(Clone)]
struct DayData {
    date: Date,
    is_current_month: bool,
    week_number: u8,
}

/// Individual day cell component
#[derive(Props, Clone, PartialEq)]
struct CalendarDayProps {
    date: Date,
    is_current_month: bool,
    is_today: bool,
    is_selected: bool,
    is_disabled: bool,
    selected_dates: Signal<Vec<String>>,
    range_start: Signal<Option<String>>,
    mode: CalendarMode,
    on_change: EventHandler<Option<String>>,
}

#[component]
fn CalendarDay(mut props: CalendarDayProps) -> Element {
    let theme = use_theme();
    let mut is_hovered = use_signal(|| false);
    
    let date_str = format!("{:04}-{:02}-{:02}", props.date.year, props.date.month, props.date.day);
    let date_str_clone = date_str.clone();
    let date_clone = props.date.clone();
    
    // Check if in selected dates (for multiple mode)
    let is_in_selected = props.selected_dates.read().contains(&date_str);
    
    // Check if in range
    let range_start_read = props.range_start.read();
    let is_in_range = if let Some(start_str) = range_start_read.as_ref() {
        if let Some(start_date) = parse_date(start_str) {
            let start_cmp = (start_date.year, start_date.month, start_date.day);
            let current_cmp = (props.date.year, props.date.month, props.date.day);
            current_cmp > start_cmp
        } else {
            false
        }
    } else {
        false
    };
    drop(range_start_read);
    
    let is_selected = props.is_selected || is_in_selected || (props.mode == CalendarMode::Range && is_in_range);
    
    // Day cell style
    let day_style = use_style(move |t| {
        let mut style = Style::new()
            .w("36px")
            .h("36px")
            .inline_flex()
            .items_center()
            .justify_center()
            .text(&t.typography, "sm")
            .rounded(&t.radius, "md")
            .border(0, &t.colors.border)
            .cursor(if props.is_disabled { "not-allowed" } else { "pointer" })
            .transition("all 150ms ease")
            .bg_transparent();
        
        // Text color
        if props.is_disabled {
            style = style.text_color(&t.colors.muted);
        } else if props.is_current_month {
            style = style.text_color(&t.colors.foreground);
        } else {
            // Other month days
            style = style.text_color(&t.colors.muted);
        }
        
        // Background for selected/today states
        if is_selected {
            style = style
                .bg(&t.colors.primary)
                .text_color(&t.colors.primary_foreground);
        } else if props.is_today {
            style = style
                .bg(&t.colors.accent)
                .text_color(&t.colors.accent_foreground);
        } else if is_hovered() && !props.is_disabled {
            style = style.bg(&t.colors.muted);
        }
        
        style.build()
    });
    
    let handle_click = move |_| {
        if props.is_disabled {
            return;
        }
        
        match props.mode {
            CalendarMode::Single => {
                let date_str = format!("{:04}-{:02}-{:02}", date_clone.year, date_clone.month, date_clone.day);
                props.on_change.call(Some(date_str));
            }
            CalendarMode::Multiple => {
                let mut dates = props.selected_dates.read().clone();
                let date_str = format!("{:04}-{:02}-{:02}", date_clone.year, date_clone.month, date_clone.day);
                
                if dates.contains(&date_str) {
                    dates.retain(|d| d != &date_str);
                } else {
                    dates.push(date_str.clone());
                }
                
                props.selected_dates.set(dates);
                props.on_change.call(Some(date_str));
            }
            CalendarMode::Range => {
                let date_str = format!("{:04}-{:02}-{:02}", date_clone.year, date_clone.month, date_clone.day);
                
                let range_start_read = props.range_start.read();
                let has_start = range_start_read.is_some();
                let start_opt = range_start_read.clone();
                drop(range_start_read);
                
                if has_start {
                    if let Some(start) = start_opt {
                        // Selecting end date
                        let start_date = parse_date(&start).unwrap();
                        let end_date = date_clone.clone();
                        
                        // Ensure start is before end
                        let (actual_start, actual_end) = if start_date > end_date {
                            (end_date.clone(), start_date)
                        } else {
                            (start_date, end_date)
                        };
                        
                        let range_str = format!(
                            "{:04}-{:02}-{:02}/{:04}-{:02}-{:02}",
                            actual_start.year, actual_start.month, actual_start.day,
                            actual_end.year, actual_end.month, actual_end.day
                        );
                        
                        props.range_start.set(None);
                        props.on_change.call(Some(range_str));
                    }
                } else {
                    // Selecting start date
                    props.range_start.set(Some(date_str.clone()));
                    props.on_change.call(Some(date_str));
                }
            }
        }
    };
    
    rsx! {
        button {
            type: "button",
            style: "{day_style}",
            disabled: props.is_disabled,
            aria_label: "{date_str_clone}",
            aria_selected: "{is_selected}",
            onclick: handle_click,
            onmouseenter: move |_| if !props.is_disabled { is_hovered.set(true) },
            onmouseleave: move |_| is_hovered.set(false),
            
            "{props.date.day}"
        }
    }
}

/// Chevron direction
#[derive(Clone, PartialEq)]
enum ChevronDirection {
    Left,
    Right,
}

/// Chevron icon component
#[derive(Props, Clone, PartialEq)]
struct CalendarChevronProps {
    direction: ChevronDirection,
}

#[component]
fn CalendarChevron(props: CalendarChevronProps) -> Element {
    let transform = match props.direction {
        ChevronDirection::Left => "rotate(180deg)",
        ChevronDirection::Right => "rotate(0deg)",
    };
    
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            style: "width: 16px; height: 16px; transform: {transform};",
            
            polyline { points: "9 18 15 12 9 6" }
        }
    }
}

// ============================================================================
// Date utilities
// ============================================================================

/// Date struct
#[derive(Clone, PartialEq)]
struct Date {
    year: i32,
    month: u8,
    day: u8,
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((self.year, self.month, self.day).cmp(&(other.year, other.month, other.day)))
    }
}

/// Parse date from YYYY-MM-DD format
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

/// Parse month-year from YYYY-MM format
fn parse_month_year(s: &str) -> Option<(i32, u8)> {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 2 {
        return None;
    }
    
    let year = parts[0].parse().ok()?;
    let month = parts[1].parse().ok()?;
    
    Some((year, month))
}

/// Get today's date
fn get_today() -> Date {
    // In a real implementation, use system time
    // For now, return a fixed date
    Date { year: 2024, month: 3, day: 19 }
}

/// Get number of days in a month
fn get_days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap_year(year) { 29 } else { 28 },
        _ => 30,
    }
}

/// Check if a year is a leap year
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Get the first day of the week for a month (0 = Sunday)
fn get_first_day_of_week(year: i32, month: u8) -> u8 {
    // Using Zeller's congruence
    let mut y = year;
    let mut m = month as i32;
    
    if m < 3 {
        m += 12;
        y -= 1;
    }
    
    let k = y % 100;
    let j = y / 100;
    
    let day = (1 + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 + 5 * j) % 7;
    ((day + 5) % 7) as u8
}

/// Get ISO week number for a date
fn get_week_number(date: &Date) -> u8 {
    // Simplified week number calculation
    // In a real implementation, use proper ISO week calculation
    let day_of_year = get_day_of_year(date);
    ((day_of_year + 3) / 7).min(53) as u8
}

/// Get day of year (1-366)
fn get_day_of_year(date: &Date) -> u16 {
    let mut day_of_year = date.day as u16;
    
    for m in 1..date.month {
        day_of_year += get_days_in_month(date.year, m) as u16;
    }
    
    day_of_year
}

/// Get month name string
fn month_name_str(month: u8) -> &'static str {
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

/// Check if a date is disabled
fn is_date_disabled(
    date: &Date,
    disabled_dates: Vec<String>,
    min_date: Option<String>,
    max_date: Option<String>,
) -> bool {
    let date_str = format!("{:04}-{:02}-{:02}", date.year, date.month, date.day);
    
    // Check explicit disabled dates
    if disabled_dates.contains(&date_str) {
        return true;
    }
    
    // Check min date
    if let Some(min_str) = min_date {
        if let Some(min_date) = parse_date(&min_str) {
            if *date < min_date {
                return true;
            }
        }
    }
    
    // Check max date
    if let Some(max_str) = max_date {
        if let Some(max_date) = parse_date(&max_str) {
            if *date > max_date {
                return true;
            }
        }
    }
    
    false
}
