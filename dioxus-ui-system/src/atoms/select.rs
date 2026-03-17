//! Select atom component
//!
//! Displays a list of options for the user to pick from.

use dioxus::prelude::*;
use crate::theme::{use_theme, use_style};
use crate::styles::Style;

/// Select option
#[derive(Clone, PartialEq, Debug)]
pub struct SelectOption {
    /// Option value
    pub value: String,
    /// Option label
    pub label: String,
    /// Whether this option is disabled
    pub disabled: bool,
}

impl SelectOption {
    /// Create a new select option
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }
    
    /// Create a new disabled select option
    pub fn disabled(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: true,
        }
    }
}

/// Select properties
#[derive(Props, Clone, PartialEq)]
pub struct SelectProps {
    /// Currently selected value
    #[props(default)]
    pub value: String,
    /// Callback when selection changes
    #[props(default)]
    pub onchange: Option<EventHandler<String>>,
    /// Available options
    pub options: Vec<SelectOption>,
    /// Placeholder text when no value selected
    #[props(default)]
    pub placeholder: Option<String>,
    /// Whether the select is disabled
    #[props(default)]
    pub disabled: bool,
    /// Whether the select has an error
    #[props(default)]
    pub error: bool,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Select atom component (native HTML select with styling)
#[component]
pub fn Select(props: SelectProps) -> Element {
    let _theme = use_theme();
    let mut is_focused = use_signal(|| false);
    
    let disabled = props.disabled;
    let error = props.error;
    
    let select_style = use_style(move |t| {
        let base = Style::new()
            .w_full()
            .h_px(40)
            .px(&t.spacing, "md")
            .rounded(&t.radius, "md")
            .border(1, if error { &t.colors.destructive } else { &t.colors.border })
            .bg(&t.colors.background)
            .text_color(&t.colors.foreground)
            .font_size(14)
            .cursor(if disabled { "not-allowed" } else { "pointer" })
            .transition("all 150ms ease")
            .outline("none");
        
        let base = if is_focused() {
            base.border_color(&t.colors.ring)
                .shadow(&format!("0 0 0 1px {}", t.colors.ring.to_rgba()))
        } else {
            base
        };
        
        let base = if disabled {
            base.opacity(0.5)
                .bg(&t.colors.muted)
        } else {
            base
        };
        
        // Add dropdown arrow
        let style_str = base.build();
        format!("{} appearance: none; background-image: url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='16' height='16' viewBox='0 0 24 24' fill='none' stroke='%2364748b' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E\"); background-repeat: no-repeat; background-position: right 12px center; padding-right: 40px;", style_str)
    });
    
    let handle_change = move |e: Event<FormData>| {
        let data = e.data();
        let new_value = data.value();
        if let Some(handler) = &props.onchange {
            handler.call(new_value);
        }
    };
    
    rsx! {
        select {
            value: "{props.value}",
            disabled: disabled,
            style: "{select_style} {props.style.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            onchange: handle_change,
            onfocus: move |_| is_focused.set(true),
            onblur: move |_| is_focused.set(false),
            
            if let Some(placeholder) = props.placeholder.clone() {
                if props.value.is_empty() {
                    option {
                        value: "",
                        disabled: true,
                        selected: true,
                        "{placeholder}"
                    }
                }
            }
            
            for option in props.options {
                option {
                    key: "{option.value}",
                    value: "{option.value}",
                    disabled: option.disabled,
                    selected: props.value == option.value,
                    "{option.label}"
                }
            }
        }
    }
}

/// Multi-select properties
#[derive(Props, Clone, PartialEq)]
pub struct MultiSelectProps {
    /// Currently selected values
    #[props(default)]
    pub values: Vec<String>,
    /// Callback when selection changes
    #[props(default)]
    pub onchange: Option<EventHandler<Vec<String>>>,
    /// Available options
    pub options: Vec<SelectOption>,
    /// Placeholder text
    #[props(default)]
    pub placeholder: Option<String>,
    /// Whether the select is disabled
    #[props(default)]
    pub disabled: bool,
    /// Maximum number of selections
    #[props(default)]
    pub max_selections: Option<usize>,
}

/// Multi-select component with tags
#[component]
pub fn MultiSelect(props: MultiSelectProps) -> Element {
    let _theme = use_theme();
    let mut selected = use_signal(|| props.values.clone());
    let mut is_open = use_signal(|| false);
    
    // Sync with props
    use_effect(move || {
        selected.set(props.values.clone());
    });
    
    let container_style = use_style(|t| {
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
            .cursor("pointer")
            .relative()
            .build()
    });
    
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
    
    let onchange_clone = props.onchange.clone();
    let max_selections = props.max_selections;
    
    let mut remove_selected = move |value: String| {
        let onchange = onchange_clone.clone();
        selected.with_mut(|s| {
            s.retain(|v| v != &value);
            if let Some(h) = onchange {
                h.call(s.clone());
            }
        });
    };
    
    let add_selected = move |value: String| {
        let onchange = onchange_clone.clone();
        selected.with_mut(|s| {
            if !s.contains(&value) {
                if let Some(max) = max_selections {
                    if s.len() >= max {
                        return;
                    }
                }
                s.push(value);
                if let Some(h) = onchange {
                    h.call(s.clone());
                }
            }
        });
    };
    
    let selected_labels: Vec<_> = selected()
        .iter()
        .filter_map(|v| {
            props.options.iter().find(|o| o.value == *v).map(|o| (v.clone(), o.label.clone()))
        })
        .collect();
    
    rsx! {
        div {
            style: "position: relative;",
            
            // Selected tags container
            div {
                style: "{container_style}",
                onclick: move |_| if !props.disabled { is_open.toggle() },
                
                if selected_labels.is_empty() {
                    span {
                        style: "color: #64748b; font-size: 14px;",
                        "{props.placeholder.clone().unwrap_or_else(|| \"Select options...\".to_string())}"
                    }
                }
                
                for (value, label) in selected_labels {
                    MultiSelectTag {
                        key: "{value}",
                        value: value.clone(),
                        label: label.clone(),
                        tag_style: tag_style.clone(),
                        on_remove: remove_selected,
                    }
                }
                
                // Dropdown arrow
                span {
                    style: "margin-left: auto; color: #64748b;",
                    if is_open() { "▲" } else { "▼" }
                }
            }
            
            // Dropdown
            if is_open() && !props.disabled {
                MultiSelectDropdown {
                    options: props.options.clone(),
                    selected: selected(),
                    on_select: add_selected,
                    on_close: move || is_open.set(false),
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct MultiSelectDropdownProps {
    options: Vec<SelectOption>,
    selected: Vec<String>,
    on_select: EventHandler<String>,
    on_close: EventHandler<()>,
}

#[derive(Props, Clone, PartialEq)]
struct CheckBoxIndicatorProps {
    is_selected: bool,
}

#[component]
fn CheckBoxIndicator(props: CheckBoxIndicatorProps) -> Element {
    let bg_color = if props.is_selected { "#0f172a" } else { "white" };
    rsx! {
        div {
            style: "width: 16px; height: 16px; border: 1px solid #cbd5e1; border-radius: 4px; display: flex; align-items: center; justify-content: center; background: {bg_color}; color: white;",
            if props.is_selected {
                "✓"
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct MultiSelectTagProps {
    value: String,
    label: String,
    tag_style: String,
    on_remove: EventHandler<String>,
}

#[component]
fn MultiSelectTag(props: MultiSelectTagProps) -> Element {
    let value = props.value.clone();
    rsx! {
        span {
            style: "{props.tag_style}",
            "{props.label}"
            button {
                style: "background: none; border: none; cursor: pointer; padding: 0; margin: 0; display: flex; align-items: center;",
                onclick: move |e: Event<MouseData>| {
                    e.stop_propagation();
                    props.on_remove.call(value.clone());
                },
                "×"
            }
        }
    }
}

#[component]
fn MultiSelectDropdown(props: MultiSelectDropdownProps) -> Element {
    let _theme = use_theme();
    
    let dropdown_style = use_style(|t| {
        Style::new()
            .absolute()
            .top("calc(100% + 4px)")
            .left("0")
            .w_full()
            .max_h_px(200)
            .rounded(&t.radius, "md")
            .border(1, &t.colors.border)
            .bg(&t.colors.popover)
            .shadow(&t.shadows.lg)
            .overflow_auto()
            .z_index(50)
            .build()
    });
    
    let item_style = use_style(|t| {
        Style::new()
            .w_full()
            .px(&t.spacing, "md")
            .py(&t.spacing, "sm")
            .text_left()
            .cursor("pointer")
            .transition("all 100ms ease")
            .build()
    });
    
    rsx! {
        div {
            style: "{dropdown_style}",
            
            for option in props.options.iter().cloned().collect::<Vec<_>>() {
                DropdownOptionItem {
                    key: "{option.value}",
                    option: option.clone(),
                    item_style: item_style.clone(),
                    is_selected: props.selected.contains(&option.value),
                    on_select: props.on_select,
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct DropdownOptionItemProps {
    option: SelectOption,
    item_style: String,
    is_selected: bool,
    on_select: EventHandler<String>,
}

#[component]
fn DropdownOptionItem(props: DropdownOptionItemProps) -> Element {
    let value = props.option.value.clone();
    rsx! {
        button {
            style: "{props.item_style}",
            disabled: props.option.disabled,
            onclick: move |_| {
                props.on_select.call(value.clone());
            },
            
            div {
                style: "display: flex; align-items: center; gap: 8px;",
                
                // Checkbox
                CheckBoxIndicator { is_selected: props.is_selected }
                
                span {
                    style: if props.option.disabled { "opacity: 0.5;" } else { "" },
                    "{props.option.label}"
                }
            }
        }
    }
}
