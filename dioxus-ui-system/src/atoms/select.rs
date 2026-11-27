//! Select atom component
//!
//! Displays a list of options for the user to pick from.

use crate::a11y::{next_index, use_unique_id, Orientation};
use crate::styles::Style;
use crate::theme::{use_style, use_theme};
use dioxus::prelude::*;

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
    pub on_change: Option<EventHandler<String>>,
    /// Available options
    pub options: Vec<SelectOption>,
    /// Placeholder text when no value selected
    #[props(default)]
    pub placeholder: Option<String>,
    /// Whether the select is disabled
    #[props(default)]
    pub disabled: bool,
    /// Error message; `None` means no error
    #[props(default)]
    pub error: Option<String>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
    /// Element id
    #[props(default)]
    pub id: Option<String>,
}

/// Select atom component (native HTML select with styling)
#[component]
pub fn Select(props: SelectProps) -> Element {
    let _theme = use_theme();
    let mut is_focused = use_signal(|| false);

    let disabled = props.disabled;
    let error = props.error.is_some();

    let select_style = use_style(move |t| {
        let base = Style::new()
            .w_full()
            .h_px(40)
            .px(&t.spacing, "md")
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
            base.opacity(0.5).bg(&t.colors.muted)
        } else {
            base
        };

        // Add dropdown arrow
        let style_str = base.build();
        format!("{} appearance: none; background-image: url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='16' height='16' viewBox='0 0 24 24' fill='none' stroke='%2364748b' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E\"); background-repeat: no-repeat; background-position: right 12px center; padding-right: 40px;", style_str)
    });

    let error_style = use_style(|t| {
        Style::new()
            .block()
            .mt(&t.spacing, "xs")
            .font_size(12)
            .text_color(&t.colors.destructive)
            .build()
    });

    let handle_change = move |e: Event<FormData>| {
        let data = e.data();
        let new_value = data.value();
        if let Some(handler) = &props.on_change {
            handler.call(new_value);
        }
    };

    rsx! {
        select {
            value: "{props.value}",
            disabled: disabled,
            style: "{select_style} {props.style.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            id: "{props.id.clone().unwrap_or_default()}",
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

        if let Some(error) = props.error.clone() {
            span {
                style: "{error_style}",
                "{error}"
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
    pub on_change: Option<EventHandler<Vec<String>>>,
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
    /// Accessible name of the combobox (`aria-label`); defaults to the
    /// placeholder text
    #[props(default)]
    pub aria_label: Option<String>,
}

/// What a key press does in a multi-select listbox.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ListboxAction {
    /// Open the listbox.
    Open,
    /// Close the listbox.
    Close,
    /// Move the highlight to this option index.
    Move(usize),
    /// Toggle selection of the option at this index.
    Toggle(usize),
}

/// Map a key press to a [`ListboxAction`] (`None` means "not handled").
///
/// Closed: Enter, Space, ArrowDown and ArrowUp open. Open: Escape closes,
/// ArrowUp/ArrowDown/Home/End move the highlight, Enter/Space toggle it.
fn listbox_key_action(
    key: &Key,
    open: bool,
    highlighted: usize,
    len: usize,
) -> Option<ListboxAction> {
    let is_space = matches!(key, Key::Character(c) if c == " ");
    if !open {
        return match key {
            Key::Enter | Key::ArrowDown | Key::ArrowUp => Some(ListboxAction::Open),
            _ if is_space => Some(ListboxAction::Open),
            _ => None,
        };
    }
    match key {
        Key::Escape => Some(ListboxAction::Close),
        Key::Enter if len > 0 => Some(ListboxAction::Toggle(highlighted.min(len - 1))),
        _ if is_space && len > 0 => Some(ListboxAction::Toggle(highlighted.min(len - 1))),
        _ => next_index(key, highlighted, len, Orientation::Vertical).map(ListboxAction::Move),
    }
}

/// Multi-select component with tags
///
/// # Accessibility
/// The trigger is a focusable `role="combobox"` with `aria-haspopup="listbox"`,
/// `aria-expanded`, `aria-controls` and `aria-activedescendant` pointing at the
/// highlighted option. The dropdown is a `role="listbox"` with
/// `aria-multiselectable="true"`; each item is a `role="option"` with
/// `aria-selected`. Keys: Enter/Space/ArrowDown open; ArrowUp/ArrowDown/Home/End
/// move the highlight; Enter/Space toggle the highlighted option; Escape closes.
#[component]
pub fn MultiSelect(props: MultiSelectProps) -> Element {
    let _theme = use_theme();
    let mut selected = use_signal(|| props.values.clone());
    let mut is_open = use_signal(|| false);
    let mut highlighted = use_signal(|| 0usize);
    let listbox_id = use_unique_id("multi-select-listbox");

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

    let on_change_clone = props.on_change;
    let max_selections = props.max_selections;

    let remove_selected = move |value: String| {
        let on_change = on_change_clone;
        selected.with_mut(|s| {
            s.retain(|v| v != &value);
            if let Some(h) = on_change {
                h.call(s.clone());
            }
        });
    };

    let add_selected = move |value: String| {
        let on_change = on_change_clone;
        selected.with_mut(|s| {
            if !s.contains(&value) {
                if let Some(max) = max_selections {
                    if s.len() >= max {
                        return;
                    }
                }
                s.push(value);
                if let Some(h) = on_change {
                    h.call(s.clone());
                }
            }
        });
    };

    let disabled = props.disabled;
    let key_options = props.options.clone();
    let handle_key_down = move |e: KeyboardEvent| {
        if disabled {
            return;
        }
        let len = key_options.len();
        let Some(action) = listbox_key_action(&e.key(), is_open(), highlighted(), len) else {
            return;
        };
        e.prevent_default();
        match action {
            ListboxAction::Open => is_open.set(true),
            ListboxAction::Close => is_open.set(false),
            ListboxAction::Move(i) => highlighted.set(i),
            ListboxAction::Toggle(i) => {
                if let Some(option) = key_options.get(i).filter(|o| !o.disabled) {
                    let (mut remove, mut add) = (remove_selected, add_selected);
                    if selected.read().contains(&option.value) {
                        remove(option.value.clone());
                    } else {
                        add(option.value.clone());
                    }
                }
            }
        }
    };

    let placeholder = props
        .placeholder
        .clone()
        .unwrap_or_else(|| "Select options...".to_string());
    let aria_label = props
        .aria_label
        .clone()
        .unwrap_or_else(|| placeholder.clone());
    let active_descendant = (is_open() && !props.options.is_empty()).then(|| {
        option_id(
            &listbox_id,
            highlighted().min(props.options.len().saturating_sub(1)),
        )
    });
    let tabindex = if disabled { "-1" } else { "0" };
    let aria_disabled = disabled.then_some("true");

    let selected_labels: Vec<_> = selected()
        .iter()
        .filter_map(|v| {
            props
                .options
                .iter()
                .find(|o| o.value == *v)
                .map(|o| (v.clone(), o.label.clone()))
        })
        .collect();

    rsx! {
        div {
            style: "position: relative;",

            // Selected tags container (combobox)
            div {
                style: "{container_style}",
                role: "combobox",
                tabindex: "{tabindex}",
                "aria-haspopup": "listbox",
                "aria-expanded": "{is_open()}",
                "aria-controls": "{listbox_id}",
                "aria-label": "{aria_label}",
                "aria-disabled": aria_disabled,
                "aria-activedescendant": active_descendant,
                onclick: move |_| if !props.disabled { is_open.toggle() },
                onkeydown: handle_key_down,

                if selected_labels.is_empty() {
                    span {
                        style: "color: #64748b; font-size: 14px;",
                        "{placeholder}"
                    }
                }

                for (value, label) in selected_labels {
                    MultiSelectTag {
                        key: "{value}",
                        value: value.clone(),
                        label: label.clone(),
                        tag_style: tag_style,
                        on_remove: remove_selected,
                    }
                }

                // Dropdown arrow
                span {
                    "aria-hidden": "true",
                    style: "margin-left: auto; color: #64748b;",
                    if is_open() { "▲" } else { "▼" }
                }
            }

            // Dropdown
            if is_open() && !props.disabled {
                MultiSelectDropdown {
                    id: listbox_id.clone(),
                    highlighted: highlighted(),
                    options: props.options.clone(),
                    selected: selected(),
                    on_select: add_selected,
                    on_close: move |_| is_open.set(false),
                }
            }
        }
    }
}

/// Element id of the option at `index` inside the listbox `listbox_id`.
fn option_id(listbox_id: &str, index: usize) -> String {
    format!("{listbox_id}-option-{index}")
}

#[derive(Props, Clone, PartialEq)]
struct MultiSelectDropdownProps {
    id: String,
    highlighted: usize,
    options: Vec<SelectOption>,
    selected: Vec<String>,
    #[props(default)]
    on_select: Option<EventHandler<String>>,
    #[props(default)]
    on_close: Option<EventHandler<()>>,
}

#[derive(Props, Clone, PartialEq)]
struct CheckBoxIndicatorProps {
    is_selected: bool,
}

#[component]
fn CheckBoxIndicator(props: CheckBoxIndicatorProps) -> Element {
    let bg_color = if props.is_selected {
        "#0f172a"
    } else {
        "white"
    };
    rsx! {
        div {
            "aria-hidden": "true",
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
    #[props(default)]
    label: Option<String>,
    tag_style: String,
    #[props(default)]
    on_remove: Option<EventHandler<String>>,
}

#[component]
fn MultiSelectTag(props: MultiSelectTagProps) -> Element {
    let value = props.value.clone();
    let remove_label = format!(
        "Remove {}",
        props.label.clone().unwrap_or_else(|| props.value.clone())
    );
    rsx! {
        span {
            style: "{props.tag_style}",
            if let Some(label) = props.label.clone() {
                "{label}"
            }
            button {
                r#type: "button",
                tabindex: "-1",
                "aria-label": "{remove_label}",
                style: "background: none; border: none; cursor: pointer; padding: 0; margin: 0; display: flex; align-items: center;",
                onclick: move |e: Event<MouseData>| {
                    e.stop_propagation();
                    if let Some(h) = &props.on_remove {
                        h.call(value.clone());
                    }
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
    let highlighted_bg = use_style(|t| t.colors.muted.to_rgba());

    rsx! {
        div {
            id: "{props.id}",
            role: "listbox",
            "aria-multiselectable": "true",
            style: "{dropdown_style}",

            for (index, option) in props.options.iter().cloned().enumerate() {
                DropdownOptionItem {
                    key: "{option.value}",
                    id: option_id(&props.id, index),
                    option: option.clone(),
                    item_style: if index == props.highlighted {
                        format!("{item_style}; background: {highlighted_bg};")
                    } else {
                        item_style()
                    },
                    is_selected: props.selected.contains(&option.value),
                    on_select: props.on_select,
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct DropdownOptionItemProps {
    id: String,
    option: SelectOption,
    item_style: String,
    is_selected: bool,
    #[props(default)]
    on_select: Option<EventHandler<String>>,
}

#[component]
fn DropdownOptionItem(props: DropdownOptionItemProps) -> Element {
    let value = props.option.value.clone();
    rsx! {
        button {
            r#type: "button",
            id: "{props.id}",
            role: "option",
            tabindex: "-1",
            "aria-selected": "{props.is_selected}",
            "aria-disabled": props.option.disabled.then_some("true"),
            style: "{props.item_style}",
            disabled: props.option.disabled,
            onclick: move |_| {
                if let Some(h) = &props.on_select {
                    h.call(value.clone());
                }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn render(app: fn() -> Element) -> String {
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
        dioxus_ssr::render(&dom)
    }

    fn opts() -> Vec<SelectOption> {
        vec![
            SelectOption::new("a", "Alpha"),
            SelectOption::new("b", "Beta"),
        ]
    }

    #[test]
    fn listbox_keys_open_move_toggle_close() {
        let space = Key::Character(" ".to_string());
        assert_eq!(
            listbox_key_action(&Key::ArrowDown, false, 0, 3),
            Some(ListboxAction::Open)
        );
        assert_eq!(
            listbox_key_action(&space, false, 0, 3),
            Some(ListboxAction::Open)
        );
        assert_eq!(listbox_key_action(&Key::Escape, false, 0, 3), None);
        assert_eq!(
            listbox_key_action(&Key::ArrowDown, true, 2, 3),
            Some(ListboxAction::Move(0))
        );
        assert_eq!(
            listbox_key_action(&Key::ArrowUp, true, 0, 3),
            Some(ListboxAction::Move(2))
        );
        assert_eq!(
            listbox_key_action(&Key::End, true, 0, 3),
            Some(ListboxAction::Move(2))
        );
        assert_eq!(
            listbox_key_action(&Key::Enter, true, 9, 3),
            Some(ListboxAction::Toggle(2))
        );
        assert_eq!(
            listbox_key_action(&space, true, 1, 3),
            Some(ListboxAction::Toggle(1))
        );
        assert_eq!(
            listbox_key_action(&Key::Escape, true, 0, 3),
            Some(ListboxAction::Close)
        );
        assert_eq!(listbox_key_action(&Key::Enter, true, 0, 0), None);
    }

    #[test]
    fn multi_select_trigger_is_collapsed_combobox() {
        let html = render(|| {
            rsx! { MultiSelect { options: opts(), placeholder: "Pick letters" } }
        });
        assert!(html.contains("role=\"combobox\""), "{html}");
        assert!(html.contains("aria-expanded=\"false\""), "{html}");
        assert!(html.contains("aria-haspopup=\"listbox\""), "{html}");
        assert!(
            html.contains("aria-controls=\"multi-select-listbox-"),
            "{html}"
        );
        assert!(html.contains("aria-label=\"Pick letters\""), "{html}");
        assert!(html.contains("tabindex=\"0\""), "{html}");
        assert!(!html.contains("aria-activedescendant"), "{html}");
        assert!(!html.contains("role=\"listbox\""), "{html}");
    }

    #[test]
    fn multi_select_dropdown_is_multiselectable_listbox() {
        let html = render(|| {
            rsx! {
                MultiSelectDropdown {
                    id: "lb".to_string(),
                    highlighted: 1,
                    options: opts(),
                    selected: vec!["a".to_string()],
                }
            }
        });
        assert!(html.contains("role=\"listbox\""), "{html}");
        assert!(html.contains("id=\"lb\""), "{html}");
        assert!(html.contains("aria-multiselectable=\"true\""), "{html}");
        assert_eq!(html.matches("role=\"option\"").count(), 2, "{html}");
        assert!(html.contains("id=\"lb-option-0\""), "{html}");
        assert!(html.contains("aria-selected=\"true\""), "{html}");
        assert!(html.contains("aria-selected=\"false\""), "{html}");
    }
}
