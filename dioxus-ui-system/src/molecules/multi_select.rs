//! MultiSelect molecule component
//!
//! A dropdown that allows selecting multiple items with tag/chip display.

use dioxus::prelude::*;
use crate::atoms::select::SelectOption;
use crate::theme::{use_theme, use_style};
use crate::styles::Style;

/// MultiSelect properties
#[derive(Props, Clone, PartialEq)]
pub struct MultiSelectProps {
    /// Available options
    pub options: Vec<SelectOption>,
    /// Currently selected values
    #[props(default)]
    pub value: Vec<String>,
    /// Callback when selection changes
    pub on_change: EventHandler<Vec<String>>,
    /// Placeholder text when no value selected
    #[props(default)]
    pub placeholder: Option<String>,
    /// Whether the select is disabled
    #[props(default = false)]
    pub disabled: bool,
    /// Maximum number of selections allowed
    #[props(default)]
    pub max_selected: Option<usize>,
    /// Allow creating new options
    #[props(default = false)]
    pub creatable: bool,
    /// Enable search/filter functionality
    #[props(default = true)]
    pub searchable: bool,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// MultiSelect molecule component
#[component]
pub fn MultiSelect(props: MultiSelectProps) -> Element {
    let _theme = use_theme();
    let mut is_open = use_signal(|| false);
    let mut search_value = use_signal(|| String::new());
    let mut highlighted_index = use_signal(|| 0usize);
    let mut selected_values = use_signal(|| props.value.clone());
    let options = props.options.clone();

    // Sync with props
    use_effect(move || {
        selected_values.set(props.value.clone());
    });

    let class_css = props.class.as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();

    // Filter options based on search
    let filtered_options: Memo<Vec<SelectOption>> = use_memo({
        let options = options.clone();
        let searchable = props.searchable;
        move || {
            let search = search_value().to_lowercase();
            if search.is_empty() || !searchable {
                options.clone()
            } else {
                options
                    .iter()
                    .filter(|o| {
                        o.label.to_lowercase().contains(&search) || 
                        o.value.to_lowercase().contains(&search)
                    })
                    .cloned()
                    .collect()
            }
        }
    });

    // Check if at max selection
    let is_at_max = move || {
        props.max_selected.map(|max| selected_values().len() >= max).unwrap_or(false)
    };

    // Toggle option selection
    let toggle_option = use_callback({
        let mut selected_values = selected_values.clone();
        let on_change = props.on_change.clone();
        move |value: String| {
            let mut new_values = selected_values();
            if new_values.contains(&value) {
                new_values.retain(|v| v != &value);
            } else if !is_at_max() {
                new_values.push(value);
            }
            selected_values.set(new_values.clone());
            on_change.call(new_values);
        }
    });

    // Remove a selected value
    let remove_value = use_callback({
        let mut selected_values = selected_values.clone();
        let on_change = props.on_change.clone();
        move |value: String| {
            let mut new_values = selected_values();
            new_values.retain(|v| v != &value);
            selected_values.set(new_values.clone());
            on_change.call(new_values);
        }
    });

    // Select all options
    let select_all = use_callback({
        let mut selected_values = selected_values.clone();
        let on_change = props.on_change.clone();
        let options = options.clone();
        let max_selected = props.max_selected;
        move |()| {
            let mut new_values: Vec<String> = options
                .iter()
                .filter(|o| !o.disabled)
                .map(|o| o.value.clone())
                .collect();
            
            // Respect max_selected limit
            if let Some(max) = max_selected {
                new_values.truncate(max);
            }
            
            selected_values.set(new_values.clone());
            on_change.call(new_values);
        }
    });

    // Clear all selections
    let clear_all = use_callback({
        let mut selected_values = selected_values.clone();
        let on_change = props.on_change.clone();
        move |()| {
            selected_values.set(Vec::new());
            on_change.call(Vec::new());
        }
    });

    // Create new option
    let create_option = use_callback({
        let mut selected_values = selected_values.clone();
        let mut search_value = search_value.clone();
        let on_change = props.on_change.clone();
        move |()| {
            let value = search_value().trim().to_string();
            if !value.is_empty() && !is_at_max() {
                let mut new_values = selected_values();
                if !new_values.contains(&value) {
                    new_values.push(value);
                    selected_values.set(new_values.clone());
                    on_change.call(new_values);
                }
                search_value.set(String::new());
            }
        }
    });

    // Handle keyboard navigation
    let handle_key_down = {
        let filtered_options = filtered_options.clone();
        let creatable = props.creatable;
        move |e: Event<dioxus::html::KeyboardData>| {
            use dioxus::html::input_data::keyboard_types::Key;
            let filtered = filtered_options();
            
            match e.key() {
                Key::ArrowDown => {
                    e.prevent_default();
                    if !is_open() {
                        is_open.set(true);
                    }
                    let max = if creatable && !search_value().trim().is_empty() {
                        filtered.len()
                    } else {
                        filtered.len().saturating_sub(1)
                    };
                    highlighted_index.with_mut(|i| *i = (*i + 1).min(max));
                }
                Key::ArrowUp => {
                    e.prevent_default();
                    highlighted_index.with_mut(|i| *i = i.saturating_sub(1));
                }
                Key::Enter => {
                    e.prevent_default();
                    if is_open() {
                        if let Some(option) = filtered.get(highlighted_index()) {
                            if !option.disabled {
                                toggle_option.call(option.value.clone());
                            }
                        } else if creatable && !search_value().trim().is_empty() && highlighted_index() == filtered.len() {
                            create_option.call(());
                        }
                    } else {
                        is_open.set(true);
                    }
                }
                Key::Escape => {
                    is_open.set(false);
                }
                Key::Backspace => {
                    if search_value().is_empty() && !selected_values().is_empty() {
                        if let Some(last) = selected_values().last() {
                            let last_value = last.clone();
                            remove_value.call(last_value);
                        }
                    }
                }
                _ => {}
            }
        }
    };

    // Container styles
    let container_style = use_style(move |t| {
        let border_color = if is_open() {
            t.colors.ring.to_rgba()
        } else {
            t.colors.border.to_rgba()
        };
        
        Style::new()
            .w_full()
            .min_h_px(40)
            .px(&t.spacing, "sm")
            .py(&t.spacing, "xs")
            .rounded(&t.radius, "md")
            .border(1, &t.colors.border)
            .bg(&t.colors.background)
            .flex()
            .flex_wrap()
            .items_center()
            .gap_px(6)
            .cursor(if props.disabled { "not-allowed" } else { "pointer" })
            .transition("all 150ms ease")
            .build() + &format!("; border-color: {}", border_color)
    });

    // Container focus shadow style
    let container_shadow_style = use_style(move |t| {
        if is_open() {
            format!("box-shadow: 0 0 0 1px {}", t.colors.ring.to_rgba())
        } else {
            String::new()
        }
    });

    // Tag styles for selected items
    let tag_style = use_style(|t| {
        Style::new()
            .inline_flex()
            .items_center()
            .gap_px(4)
            .px(&t.spacing, "sm")
            .py(&t.spacing, "xs")
            .rounded(&t.radius, "sm")
            .bg(&t.colors.secondary)
            .text_color(&t.colors.secondary_foreground)
            .font_size(12)
            .build()
    });

    // Dropdown styles - uses absolute positioning with high z-index
    let dropdown_style = use_style(|t| {
        Style::new()
            .absolute()
            .top("calc(100% + 4px)")
            .left("0")
            .w_full()
            .max_h_px(250)
            .rounded(&t.radius, "md")
            .border(1, &t.colors.border)
            .bg(&t.colors.popover)
            .shadow(&t.shadows.lg)
            .overflow_auto()
            .z_index(9999)
            .build()
    });

    // Dropdown item styles
    let item_style_base = use_style(|t| {
        Style::new()
            .w_full()
            .px(&t.spacing, "md")
            .py(&t.spacing, "sm")
            .flex()
            .items_center()
            .gap_px(8)
            .cursor("pointer")
            .transition("all 100ms ease")
            .build()
    });

    // Get label for a value
    let get_label = use_callback({
        let options = options.clone();
        move |value: String| -> String {
            options
                .iter()
                .find(|o| o.value == value)
                .map(|o| o.label.clone())
                .unwrap_or_else(|| value.to_string())
        }
    });

    // Check if all filtered options are selected
    let all_selected = move || {
        let filtered = filtered_options();
        let selected = selected_values();
        !filtered.is_empty() && 
        filtered.iter().all(|o| o.disabled || selected.contains(&o.value))
    };

    // Check if any option is selected
    let has_selection = move || !selected_values().is_empty();

    // Can create new option from search
    let can_create = move || {
        props.creatable && 
        !search_value().trim().is_empty() &&
        !options.iter().any(|o| o.label.to_lowercase() == search_value().trim().to_lowercase())
    };

    let placeholder_text = props.placeholder.clone().unwrap_or_else(|| "Select items...".to_string());

    // Colors for styling
    let muted_color = use_style(|t| t.colors.muted.to_rgba());
    let border_color = use_style(|t| t.colors.border.to_rgba());
    let primary_color = use_style(|t| t.colors.primary.to_rgba());
    let destructive_color = use_style(|t| t.colors.destructive.to_rgba());
    let foreground_color = use_style(|t| t.colors.foreground.to_rgba());

    // Get highlighted background color
    let highlighted_bg = use_style(|t| t.colors.muted.to_rgba());

    rsx! {
        div {
            class: "multi-select{class_css}",
            style: "position: relative;",

            // Selected tags container
            div {
                class: "multi-select-container",
                style: "{container_style}; {container_shadow_style}",
                onclick: move |_| {
                    if !props.disabled {
                        is_open.toggle();
                    }
                },

                // Selected tags
                for value in selected_values().iter() {
                    {
                        let label = get_label.call(value.clone());
                        let value_clone = value.clone();
                        rsx! {
                            span {
                                key: "{value_clone}",
                                class: "multi-select-tag",
                                style: "{tag_style}",

                                "{label}"

                                button {
                                    r#type: "button",
                                    class: "multi-select-tag-remove",
                                    style: "display: inline-flex; align-items: center; justify-content: center; margin-left: 4px; padding: 2px; background: none; border: none; cursor: pointer; font-size: 12px; color: inherit; opacity: 0.7; border-radius: 50%; transition: opacity 0.15s;",
                                    onclick: move |e: Event<dioxus::html::MouseData>| {
                                        e.stop_propagation();
                                        remove_value.call(value_clone.clone());
                                    },
                                    "✕"
                                }
                            }
                        }
                    }
                }

                // Search input or placeholder
                if props.searchable && !props.disabled {
                    input {
                        r#type: "text",
                        class: "multi-select-input",
                        style: "flex: 1; min-width: 80px; border: none; outline: none; font-size: 14px; padding: 4px; background: transparent;",
                        placeholder: if selected_values().is_empty() { "{placeholder_text}" } else { "" },
                        value: "{search_value}",
                        disabled: props.disabled,
                        oninput: move |e: Event<FormData>| {
                            search_value.set(e.value());
                            highlighted_index.set(0);
                            is_open.set(true);
                        },
                        onkeydown: handle_key_down,
                        onclick: move |e: Event<dioxus::html::MouseData>| {
                            e.stop_propagation();
                        },
                    }
                } else if selected_values().is_empty() {
                    span {
                        class: "multi-select-placeholder",
                        style: "color: {muted_color}; font-size: 14px;",
                        "{placeholder_text}"
                    }
                }

                // Dropdown arrow
                span {
                    class: "multi-select-arrow",
                    style: "margin-left: auto; color: {muted_color}; transition: transform 0.2s;",
                    style: if is_open() { "transform: rotate(180deg);" } else { "" },
                    "▼"
                }
            }

            // Dropdown with overlay for click-outside
            if is_open() && !props.disabled {
                // Overlay to capture clicks outside
                div {
                    style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; z-index: 9998;",
                    onclick: move |_| is_open.set(false),
                }
                div {
                    class: "multi-select-dropdown",
                    style: "{dropdown_style}",

                    // Search bar in dropdown (when not searchable in input)
                    if !props.searchable {
                        div {
                            class: "multi-select-dropdown-search",
                            style: "padding: 8px 12px; border-bottom: 1px solid {border_color};",

                            input {
                                r#type: "text",
                                class: "multi-select-search-input",
                                style: "width: 100%; padding: 8px 12px; border: 1px solid {border_color}; border-radius: 6px; font-size: 14px; outline: none;",
                                placeholder: "Search...",
                                value: "{search_value}",
                                oninput: move |e: Event<FormData>| {
                                    search_value.set(e.value());
                                    highlighted_index.set(0);
                                },
                            }
                        }
                    }

                    // Actions bar
                    div {
                        class: "multi-select-actions",
                        style: "display: flex; justify-content: space-between; padding: 8px 12px; border-bottom: 1px solid {border_color};",

                        button {
                            r#type: "button",
                            class: "multi-select-select-all",
                            style: "font-size: 12px; color: {primary_color}; background: none; border: none; cursor: pointer; padding: 4px 8px; border-radius: 4px;",
                            disabled: all_selected() || is_at_max(),
                            onclick: move |e: Event<dioxus::html::MouseData>| {
                                e.stop_propagation();
                                select_all.call(());
                            },
                            "Select All"
                        }

                        button {
                            r#type: "button",
                            class: "multi-select-clear-all",
                            style: "font-size: 12px; color: {destructive_color}; background: none; border: none; cursor: pointer; padding: 4px 8px; border-radius: 4px;",
                            disabled: !has_selection(),
                            onclick: move |e: Event<dioxus::html::MouseData>| {
                                e.stop_propagation();
                                clear_all.call(());
                            },
                            "Clear All"
                        }
                    }

                    // Options list
                    div {
                        class: "multi-select-options",
                        style: "max-height: 200px; overflow-y: auto;",

                        if filtered_options().is_empty() && !can_create() {
                            div {
                                class: "multi-select-empty",
                                style: "padding: 16px; text-align: center; color: {muted_color}; font-size: 14px;",
                                "No options found"
                            }
                        } else {
                            for (index, option) in filtered_options().iter().enumerate() {
                                MultiSelectOptionItem {
                                    key: "{option.value}",
                                    option: option.clone(),
                                    is_selected: selected_values().contains(&option.value),
                                    is_highlighted: index == highlighted_index(),
                                    is_disabled: option.disabled || (!selected_values().contains(&option.value) && is_at_max()),
                                    item_style_base: item_style_base.clone(),
                                    primary_color: primary_color.clone(),
                                    border_color: border_color.clone(),
                                    foreground_color: foreground_color.clone(),
                                    highlighted_bg: highlighted_bg.clone(),
                                    on_toggle: toggle_option.clone(),
                                    set_highlighted_index: {
                                        let mut idx = highlighted_index.clone();
                                        Callback::new(move |i: usize| idx.set(i))
                                    },
                                    index,
                                }
                            }

                            // Create option
                            if can_create() {
                                MultiSelectCreateOptionItem {
                                    search_value: search_value().trim().to_string(),
                                    is_highlighted: highlighted_index() == filtered_options().len(),
                                    item_style_base: item_style_base.clone(),
                                    primary_color: primary_color.clone(),
                                    highlighted_bg: highlighted_bg.clone(),
                                    border_color: border_color.clone(),
                                    on_create: create_option.clone(),
                                    set_highlighted_index: {
                                        let idx = filtered_options().len();
                                        let mut hi = highlighted_index.clone();
                                        Callback::new(move |_: ()| hi.set(idx))
                                    },
                                }
                            }
                        }
                    }

                    // Selection count footer
                    if props.max_selected.is_some() || has_selection() {
                        div {
                            class: "multi-select-footer",
                            style: "padding: 8px 12px; border-top: 1px solid {border_color}; font-size: 12px; color: {muted_color}; text-align: center;",

                            if let Some(max) = props.max_selected {
                                "{selected_values().len()} / {max} selected"
                            } else {
                                "{selected_values().len()} selected"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct MultiSelectOptionItemProps {
    option: SelectOption,
    is_selected: bool,
    is_highlighted: bool,
    is_disabled: bool,
    item_style_base: String,
    primary_color: String,
    border_color: String,
    foreground_color: String,
    highlighted_bg: String,
    on_toggle: Callback<String>,
    set_highlighted_index: Callback<usize>,
    index: usize,
}

#[component]
fn MultiSelectOptionItem(props: MultiSelectOptionItemProps) -> Element {
    let bg_color = if props.is_highlighted {
        props.highlighted_bg.clone()
    } else {
        "transparent".to_string()
    };
    
    let opacity = if props.is_disabled { "0.5" } else { "1" };
    let checkbox_border = if props.is_selected {
        props.primary_color.clone()
    } else {
        props.border_color.clone()
    };
    let checkbox_bg = if props.is_selected {
        props.primary_color.clone()
    } else {
        "transparent".to_string()
    };
    let value = props.option.value.clone();
    let idx = props.index;
    
    rsx! {
        div {
            class: "multi-select-option",
            style: "{props.item_style_base}; background: {bg_color}; opacity: {opacity};",
            onclick: move |_| {
                if !props.is_disabled {
                    props.on_toggle.call(value.clone());
                }
            },
            onmouseenter: move |_| props.set_highlighted_index.call(idx),

            // Checkbox indicator
            div {
                class: "multi-select-checkbox",
                style: "width: 16px; height: 16px; border: 1px solid {checkbox_border}; border-radius: 4px; display: flex; align-items: center; justify-content: center; background: {checkbox_bg}; transition: all 0.15s;",

                if props.is_selected {
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "white",
                        stroke_width: "3",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        style: "width: 12px; height: 12px;",
                        polyline { points: "20 6 9 17 4 12" }
                    }
                }
            }

            span {
                class: "multi-select-option-label",
                style: "flex: 1; font-size: 14px; color: {props.foreground_color};",
                "{props.option.label}"
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct MultiSelectCreateOptionItemProps {
    search_value: String,
    is_highlighted: bool,
    item_style_base: String,
    primary_color: String,
    highlighted_bg: String,
    border_color: String,
    on_create: Callback<()>,
    set_highlighted_index: Callback<()>,
}

#[component]
fn MultiSelectCreateOptionItem(props: MultiSelectCreateOptionItemProps) -> Element {
    let bg_color = if props.is_highlighted {
        props.highlighted_bg.clone()
    } else {
        "transparent".to_string()
    };
    
    rsx! {
        div {
            class: "multi-select-create",
            style: "{props.item_style_base}; background: {bg_color}; border-top: 1px solid {props.border_color};",
            onclick: move |_| props.on_create.call(()),
            onmouseenter: move |_| props.set_highlighted_index.call(()),

            span {
                style: "font-size: 14px; color: {props.primary_color};",
                "+ Create \"{props.search_value}\""
            }
        }
    }
}
