//! Combobox molecule component
//!
//! Autocomplete input with dropdown suggestions.

use dioxus::prelude::*;
use crate::theme::use_theme;

/// Combobox option
#[derive(Clone, PartialEq, Debug)]
pub struct ComboboxOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl ComboboxOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }
    
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Combobox properties
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxProps {
    /// Available options
    pub options: Vec<ComboboxOption>,
    /// Currently selected value
    #[props(default)]
    pub value: Option<String>,
    /// Change handler
    pub on_change: EventHandler<String>,
    /// Placeholder text
    #[props(default)]
    pub placeholder: Option<String>,
    /// Label
    #[props(default)]
    pub label: Option<String>,
    /// Whether the field is disabled
    #[props(default = false)]
    pub disabled: bool,
    /// Error message
    #[props(default)]
    pub error: Option<String>,
    /// Allow creating new options
    #[props(default = false)]
    pub creatable: bool,
    /// Created option handler
    #[props(default)]
    pub on_create: Option<EventHandler<String>>,
    /// Clearable
    #[props(default = true)]
    pub clearable: bool,
    /// Loading state
    #[props(default = false)]
    pub loading: bool,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Combobox component (autocomplete dropdown)
#[component]
pub fn Combobox(props: ComboboxProps) -> Element {
    let theme = use_theme();
    let mut is_open = use_signal(|| false);
    let mut input_value = use_signal(|| {
        props.value.as_ref().and_then(|v| {
            props.options.iter().find(|o| o.value == *v).map(|o| o.label.clone())
        }).unwrap_or_default()
    });
    let mut highlighted_index = use_signal(|| 0usize);
    
    let class_css = props.class.as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();
    
    // Filter options based on input
    let filtered_options: Vec<_> = props.options.iter()
        .filter(|o| {
            let input = input_value().to_lowercase();
            o.label.to_lowercase().contains(&input) || o.value.to_lowercase().contains(&input)
        })
        .cloned()
        .collect();
    
    let _selected_label = props.value.as_ref().and_then(|v| {
        props.options.iter().find(|o| o.value == *v).map(|o| o.label.clone())
    });
    
    let border_color = if props.error.is_some() {
        theme.tokens.read().colors.destructive.to_rgba()
    } else if is_open() {
        theme.tokens.read().colors.primary.to_rgba()
    } else {
        theme.tokens.read().colors.border.to_rgba()
    };
    
    let mut select_option = {
        let on_change = props.on_change.clone();
        move |option: ComboboxOption| {
            input_value.set(option.label.clone());
            on_change.call(option.value);
            is_open.set(false);
        }
    };
    
    let filtered_options_for_keys = filtered_options.clone();
    let handle_key_down = move |e: Event<dioxus::html::KeyboardData>| {
        use dioxus::html::input_data::keyboard_types::Key;
        match e.key() {
            Key::ArrowDown => {
                if !is_open() {
                    is_open.set(true);
                }
                let max = filtered_options_for_keys.len().saturating_sub(1);
                highlighted_index.with_mut(|i| *i = (*i + 1).min(max));
            }
            Key::ArrowUp => {
                highlighted_index.with_mut(|i| *i = i.saturating_sub(1));
            }
            Key::Enter => {
                if is_open() {
                    if let Some(option) = filtered_options_for_keys.get(highlighted_index()) {
                        if !option.disabled {
                            select_option(option.clone());
                        }
                    } else if props.creatable && !input_value().is_empty() {
                        if let Some(handler) = &props.on_create {
                            handler.call(input_value());
                        }
                        is_open.set(false);
                    }
                }
            }
            Key::Escape => {
                is_open.set(false);
            }
            _ => {}
        }
    };
    
    rsx! {
        div {
            class: "combobox{class_css}",
            style: "position: relative; display: flex; flex-direction: column; gap: 6px;",
            
            if let Some(label) = props.label {
                label {
                    class: "combobox-label",
                    style: "font-size: 14px; font-weight: 500; color: {theme.tokens.read().colors.foreground.to_rgba()};",
                    "{label}"
                }
            }
            
            div {
                class: "combobox-input-wrapper",
                style: "position: relative;",
                
                input {
                    type: "text",
                    class: "combobox-input",
                    style: "width: 100%; padding: 10px 40px 10px 14px; font-size: 16px; border: 1px solid {border_color}; border-radius: 8px; background: white; color: {theme.tokens.read().colors.foreground.to_rgba()}; outline: none;",
                    placeholder: "{props.placeholder.clone().unwrap_or_else(|| \"Search...\".to_string())}",
                    value: "{input_value}",
                    disabled: props.disabled,
                    onfocus: move |_| is_open.set(true),
                    oninput: move |e: Event<FormData>| {
                        input_value.set(e.value());
                        highlighted_index.set(0);
                        is_open.set(true);
                    },
                    onkeydown: handle_key_down,
                }
                
                // Dropdown arrow / Clear button
                div {
                    class: "combobox-controls",
                    style: "position: absolute; right: 12px; top: 50%; transform: translateY(-50%); display: flex; align-items: center; gap: 4px;",
                    
                    if props.loading {
                        span {
                            style: "font-size: 12px; animation: spin 1s linear infinite;",
                            "⟳"
                        }
                    } else if props.clearable && !input_value().is_empty() {
                        button {
                            type: "button",
                            style: "background: none; border: none; cursor: pointer; font-size: 14px; color: #9ca3af; padding: 2px;",
                            onclick: move |_| {
                                input_value.set(String::new());
                                is_open.set(true);
                            },
                            "✕"
                        }
                    }
                    
                    span {
                        style: "font-size: 12px; color: #9ca3af; transition: transform 0.2s;",
                        style: if is_open() { "transform: rotate(180deg);" } else { "" },
                        "▼"
                    }
                }
                
                // Dropdown
                if is_open() && !props.disabled {
                    div {
                        class: "combobox-dropdown",
                        style: "position: absolute; top: calc(100% + 4px); left: 0; right: 0; background: white; border: 1px solid {theme.tokens.read().colors.border.to_rgba()}; border-radius: 8px; box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1); max-height: 250px; overflow-y: auto; z-index: 50;",
                        
                        if filtered_options.is_empty() && !props.creatable {
                            div {
                                style: "padding: 12px; text-align: center; color: #9ca3af; font-size: 14px;",
                                "No options found"
                            }
                        } else {
                            for (index, option) in filtered_options.iter().enumerate() {
                                {
                                    let is_highlighted = index == highlighted_index();
                                    let is_selected = props.value.as_ref() == Some(&option.value);
                                    let bg_color = if is_highlighted {
                                        theme.tokens.read().colors.muted.to_rgba()
                                    } else {
                                        "transparent".to_string()
                                    };
                                    
                                    let text_color = if is_selected {
                                        theme.tokens.read().colors.primary.to_rgba()
                                    } else if option.disabled {
                                        theme.tokens.read().colors.muted.to_rgba()
                                    } else {
                                        theme.tokens.read().colors.foreground.to_rgba()
                                    };
                                    
                                    let cursor = if option.disabled { "not-allowed" } else { "pointer" };
                                    let option_clone = option.clone();
                                    
                                    rsx! {
                                        div {
                                            key: "{option.value}",
                                            class: "combobox-option",
                                            style: "padding: 10px 14px; font-size: 14px; cursor: {cursor}; background: {bg_color}; color: {text_color}; display: flex; align-items: center; justify-content: space-between;",
                                            onclick: move |_| {
                                                if !option_clone.disabled {
                                                    select_option(option_clone.clone());
                                                }
                                            },
                                            onmouseenter: move |_| highlighted_index.set(index),
                                            
                                            span {
                                                "{option.label}"
                                            }
                                            
                                            if is_selected {
                                                span {
                                                    style: "color: {theme.tokens.read().colors.primary.to_rgba()};",
                                                    "✓"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            
                            // Create option
                            if props.creatable && !input_value().is_empty() && !filtered_options.iter().any(|o| o.label.to_lowercase() == input_value().to_lowercase()) {
                                div {
                                    class: "combobox-create-option",
                                    style: "padding: 10px 14px; font-size: 14px; cursor: pointer; background: {theme.tokens.read().colors.muted.to_rgba()}; color: {theme.tokens.read().colors.foreground.to_rgba()}; border-top: 1px solid {theme.tokens.read().colors.border.to_rgba()};",
                                    onclick: move |_| {
                                        if let Some(handler) = &props.on_create {
                                            handler.call(input_value());
                                        }
                                        is_open.set(false);
                                    },
                                    "Create \"{input_value}\""
                                }
                            }
                        }
                    }
                }
            }
            
            if let Some(error) = props.error {
                span {
                    class: "combobox-error",
                    style: "font-size: 12px; color: {theme.tokens.read().colors.destructive.to_rgba()};",
                    "{error}"
                }
            }
        }
        

    }
}

/// Multi-select combobox properties
#[derive(Props, Clone, PartialEq)]
pub struct MultiComboboxProps {
    /// Available options
    pub options: Vec<ComboboxOption>,
    /// Selected values
    #[props(default)]
    pub values: Vec<String>,
    /// Change handler
    pub on_change: EventHandler<Vec<String>>,
    /// Placeholder text
    #[props(default)]
    pub placeholder: Option<String>,
    /// Label
    #[props(default)]
    pub label: Option<String>,
    /// Maximum selections
    #[props(default = usize::MAX)]
    pub max: usize,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Multi-select combobox component
#[component]
pub fn MultiCombobox(props: MultiComboboxProps) -> Element {
    let theme = use_theme();
    let mut is_open = use_signal(|| false);
    let mut input_value = use_signal(|| String::new());
    
    let class_css = props.class.as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();
    
    let selected_options: Vec<_> = props.options.iter()
        .filter(|o| props.values.contains(&o.value))
        .cloned()
        .collect();
    
    rsx! {
        div {
            class: "multi-combobox{class_css}",
            style: "position: relative; display: flex; flex-direction: column; gap: 6px;",
            
            if let Some(label) = props.label {
                label {
                    class: "multi-combobox-label",
                    style: "font-size: 14px; font-weight: 500;",
                    "{label}"
                }
            }
            
            div {
                class: "multi-combobox-input",
                style: "min-height: 42px; padding: 6px; border: 1px solid {theme.tokens.read().colors.border.to_rgba()}; border-radius: 8px; background: white; display: flex; flex-wrap: wrap; gap: 6px; cursor: text;",
                onclick: move |_| is_open.set(true),
                
                // Selected tags
                for option in selected_options.clone() {
                    {
                        let option_value = option.value.clone();
                        let on_change = props.on_change.clone();
                        let values = props.values.clone();
                        rsx! {
                            span {
                                key: "{option.value}",
                                style: "display: inline-flex; align-items: center; gap: 4px; padding: 4px 10px; background: {theme.tokens.read().colors.primary.to_rgba()}; color: white; border-radius: 9999px; font-size: 13px;",
                                
                                "{option.label}"
                                
                                button {
                                    type: "button",
                                    style: "background: none; border: none; cursor: pointer; color: white; font-size: 14px; padding: 0; width: 16px; height: 16px; display: flex; align-items: center; justify-content: center;",
                                    onclick: move |e: Event<dioxus::html::MouseData>| {
                                        e.stop_propagation();
                                        let mut new_values = values.clone();
                                        new_values.retain(|v| v != &option_value);
                                        on_change.call(new_values);
                                    },
                                    "✕"
                                }
                            }
                        }
                    }
                }
                
                // Input for searching
                if props.values.len() < props.max {
                    input {
                        type: "text",
                        style: "flex: 1; min-width: 80px; border: none; outline: none; font-size: 14px; padding: 4px;",
                        placeholder: if selected_options.clone().is_empty() { props.placeholder.clone().unwrap_or_else(|| "Select items...".to_string()) } else { "" },
                        value: "{input_value}",
                        oninput: move |e: Event<FormData>| {
                            input_value.set(e.value());
                            is_open.set(true);
                        },
                        onfocus: move |_| is_open.set(true),
                    }
                }
            }
            
            // Dropdown
            if is_open() {
                div {
                    class: "multi-combobox-dropdown",
                    style: "position: absolute; top: calc(100% + 4px); left: 0; right: 0; background: white; border: 1px solid {theme.tokens.read().colors.border.to_rgba()}; border-radius: 8px; box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1); max-height: 200px; overflow-y: auto; z-index: 50;",
                    
                    for option in props.options.iter().filter(|o| !props.values.contains(&o.value)) {
                        {
                            let option = option.clone();
                            let values = props.values.clone();
                            let on_change = props.on_change.clone();
                            rsx! {
                                div {
                                    key: "{option.value}",
                                    class: "multi-combobox-option",
                                    style: "padding: 10px 14px; font-size: 14px; cursor: pointer;",
                                    onclick: move |_| {
                                        let mut new_values = values.clone();
                                        new_values.push(option.value.clone());
                                        on_change.call(new_values);
                                        input_value.set(String::new());
                                    },
                                    "{option.label}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
