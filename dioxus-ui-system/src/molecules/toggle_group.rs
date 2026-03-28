//! Toggle Group molecule component
//!
//! A group of two-state toggle buttons. Supports single selection (like text alignment)
//! or multiple selection (like bold/italic/underline).

use crate::styles::Style;
use crate::theme::{use_style, use_theme};
use dioxus::prelude::*;

/// Toggle group selection type
#[derive(Default, Clone, PartialEq, Debug)]
pub enum ToggleGroupType {
    /// Only one toggle can be selected at a time (like radio buttons)
    #[default]
    Single,
    /// Multiple toggles can be selected (like checkboxes)
    Multiple,
}

/// Toggle group properties
#[derive(Props, Clone, PartialEq)]
pub struct ToggleGroupProps {
    /// Selection type - Single or Multiple
    #[props(default)]
    pub group_type: ToggleGroupType,
    /// Currently selected values
    #[props(default)]
    pub value: Vec<String>,
    /// Callback when selection changes
    #[props(default)]
    pub on_value_change: Option<EventHandler<Vec<String>>>,
    /// Toggle group children (ToggleItem components)
    pub children: Element,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Toggle item properties (for use within ToggleGroup)
#[derive(Props, Clone, PartialEq)]
pub struct ToggleItemProps {
    /// Toggle item content
    pub children: Element,
    /// Unique value for this toggle item
    pub value: String,
    /// Whether this item is disabled
    #[props(default)]
    pub disabled: bool,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Toggle group context for managing state
#[derive(Clone)]
pub struct ToggleGroupContext {
    /// Currently selected values
    pub selected_values: Signal<Vec<String>>,
    /// Selection type
    pub group_type: ToggleGroupType,
    /// Callback when selection changes
    pub on_value_change: Option<EventHandler<Vec<String>>>,
}

impl ToggleGroupContext {
    /// Check if a value is selected
    pub fn is_selected(&self, value: &str) -> bool {
        self.selected_values.read().iter().any(|v| v == value)
    }

    /// Toggle a value
    pub fn toggle_value(&mut self, value: &str) {
        let mut values = self.selected_values.write();

        match self.group_type {
            ToggleGroupType::Single => {
                // In single mode, select only this value (or deselect if already selected)
                if values.iter().any(|v| v == value) {
                    values.clear();
                } else {
                    values.clear();
                    values.push(value.to_string());
                }
            }
            ToggleGroupType::Multiple => {
                // In multiple mode, toggle the value
                if let Some(pos) = values.iter().position(|v| v == value) {
                    values.remove(pos);
                } else {
                    values.push(value.to_string());
                }
            }
        }

        // Notify parent of change
        if let Some(handler) = &self.on_value_change {
            handler.call(values.clone());
        }
    }
}

/// Toggle group molecule component
///
/// Manages a group of toggle buttons with single or multiple selection.
///
/// # Example
/// ```rust,ignore
/// use dioxus::prelude::*;
/// use dioxus_ui_system::molecules::{ToggleGroup, ToggleItem, ToggleGroupType};
///
/// rsx! {
///     ToggleGroup {
///         group_type: ToggleGroupType::Single,
///         value: vec!["left".to_string()],
///         on_value_change: move |values| println!("Selected: {:?}", values),
///         ToggleItem { value: "left", "Left" }
///         ToggleItem { value: "center", "Center" }
///         ToggleItem { value: "right", "Right" }
///     }
/// }
/// ```
#[component]
pub fn ToggleGroup(props: ToggleGroupProps) -> Element {
    let _theme = use_theme();

    let mut selected_values = use_signal(|| props.value.clone());

    // Sync with prop changes
    use_effect(move || {
        selected_values.set(props.value.clone());
    });

    // Container style
    let container_style = use_style(move |t| {
        Style::new()
            .inline_flex()
            .items_center()
            .gap(&t.spacing, "xs")
            .build()
    });

    // Combine with custom styles
    let final_style = if let Some(custom) = &props.style {
        format!("{} {}", container_style(), custom)
    } else {
        container_style()
    };

    let class = props.class.clone().unwrap_or_default();

    // Provide context to children
    use_context_provider(|| ToggleGroupContext {
        selected_values,
        group_type: props.group_type.clone(),
        on_value_change: props.on_value_change.clone(),
    });

    rsx! {
        div {
            role: "group",
            style: "{final_style}",
            class: "{class}",
            {props.children}
        }
    }
}

/// Toggle item component for use within ToggleGroup
///
/// A single toggle button that participates in a ToggleGroup.
/// Must be used within a ToggleGroup component.
///
/// # Example
/// ```rust,ignore
/// use dioxus::prelude::*;
/// use dioxus_ui_system::molecules::{ToggleGroup, ToggleItem};
///
/// rsx! {
///     ToggleGroup {
///         ToggleItem { value: "bold", "B" }
///         ToggleItem { value: "italic", "I" }
///     }
/// }
/// ```
#[component]
pub fn ToggleItem(props: ToggleItemProps) -> Element {
    let _theme = use_theme();

    // Get context from parent ToggleGroup
    let mut context = use_context::<ToggleGroupContext>();

    let mut is_hovered = use_signal(|| false);
    let mut is_focused = use_signal(|| false);

    let value = props.value.clone();
    let disabled = props.disabled;

    // Check if this item is selected from context
    let is_selected = context.is_selected(&value);

    // Memoized styles
    let style = use_style(move |t| {
        let base = Style::new()
            .inline_flex()
            .items_center()
            .justify_center()
            .px(&t.spacing, "md")
            .h_px(40)
            .rounded(&t.radius, "md")
            .text(&t.typography, "sm")
            .font_weight(500)
            .line_height(1.0)
            .transition("all 150ms cubic-bezier(0.4, 0, 0.2, 1)")
            .select_none()
            .whitespace_nowrap()
            .cursor(if disabled { "not-allowed" } else { "pointer" });

        // Apply opacity for disabled state
        let base = if disabled {
            base.opacity(0.5)
        } else {
            base.opacity(1.0)
        };

        // Style based on selected state
        let (bg, fg, border) = if is_selected {
            let bg = if is_hovered() && !disabled {
                t.colors.primary.darken(0.1)
            } else {
                t.colors.primary.clone()
            };
            (bg, t.colors.primary_foreground.clone(), None)
        } else {
            let bg = if is_hovered() && !disabled {
                t.colors.accent.clone()
            } else {
                Color::new_rgba(0, 0, 0, 0.0)
            };
            (
                bg,
                t.colors.foreground.clone(),
                Some(t.colors.border.clone()),
            )
        };

        let mut final_style = base.bg(&bg).text_color(&fg);

        // Add border for unselected state
        if let Some(border_color) = border {
            final_style = final_style.border(1, &border_color);
        }

        // Add focus ring
        if is_focused() && !disabled {
            final_style = Style {
                box_shadow: Some(format!("0 0 0 2px {}", t.colors.ring.to_rgba())),
                ..final_style
            };
        }

        final_style.build()
    });

    // Combine with custom styles
    let final_style = if let Some(custom) = &props.style {
        format!("{} {}", style(), custom)
    } else {
        style()
    };

    let class = props.class.clone().unwrap_or_default();

    let handle_click = move |_| {
        if !disabled {
            context.toggle_value(&value);
        }
    };

    rsx! {
        button {
            r#type: "button",
            role: "switch",
            aria_pressed: "{is_selected}",
            disabled: disabled,
            style: "{final_style}",
            class: "{class}",
            onclick: handle_click,
            onmouseenter: move |_| if !disabled { is_hovered.set(true) },
            onmouseleave: move |_| is_hovered.set(false),
            onfocus: move |_| is_focused.set(true),
            onblur: move |_| is_focused.set(false),
            {props.children}
        }
    }
}

/// Convenience component for a single-select toggle group with icon buttons
///
/// Similar to ToggleGroup but optimized for icon-only toggles.
#[component]
pub fn IconToggleGroup(
    #[props(default)] group_type: ToggleGroupType,
    #[props(default)] value: Vec<String>,
    #[props(default)] on_value_change: Option<EventHandler<Vec<String>>>,
    children: Element,
    #[props(default)] style: Option<String>,
    #[props(default)] class: Option<String>,
) -> Element {
    rsx! {
        ToggleGroup {
            group_type: group_type,
            value: value,
            on_value_change: on_value_change,
            style: style,
            class: class,
            {children}
        }
    }
}

/// Convenience component for an icon-only toggle item
///
/// Similar to ToggleItem but styled for icon-only content.
#[component]
pub fn IconToggleItem(
    value: String,
    icon: Element,
    #[props(default)] disabled: bool,
    #[props(default)] style: Option<String>,
    #[props(default)] class: Option<String>,
) -> Element {
    let _theme = use_theme();

    // Get context from parent ToggleGroup
    let mut context = use_context::<ToggleGroupContext>();

    let mut is_hovered = use_signal(|| false);
    let mut is_focused = use_signal(|| false);

    let is_selected = context.is_selected(&value);
    let item_disabled = disabled;

    // Memoized styles - square icon button style
    let item_style = use_style(move |t| {
        let base = Style::new()
            .inline_flex()
            .items_center()
            .justify_center()
            .w_px(40)
            .h_px(40)
            .rounded(&t.radius, "md")
            .text(&t.typography, "base")
            .font_weight(500)
            .transition("all 150ms cubic-bezier(0.4, 0, 0.2, 1)")
            .select_none()
            .cursor(if item_disabled {
                "not-allowed"
            } else {
                "pointer"
            });

        // Apply opacity for disabled state
        let base = if item_disabled {
            base.opacity(0.5)
        } else {
            base.opacity(1.0)
        };

        // Style based on selected state
        let (bg, fg, border) = if is_selected {
            let bg = if is_hovered() && !item_disabled {
                t.colors.primary.darken(0.1)
            } else {
                t.colors.primary.clone()
            };
            (bg, t.colors.primary_foreground.clone(), None)
        } else {
            let bg = if is_hovered() && !item_disabled {
                t.colors.accent.clone()
            } else {
                Color::new_rgba(0, 0, 0, 0.0)
            };
            (
                bg,
                t.colors.foreground.clone(),
                Some(t.colors.border.clone()),
            )
        };

        let mut final_style = base.bg(&bg).text_color(&fg);

        // Add border for unselected state
        if let Some(border_color) = border {
            final_style = final_style.border(1, &border_color);
        }

        // Add focus ring
        if is_focused() && !item_disabled {
            final_style = Style {
                box_shadow: Some(format!("0 0 0 2px {}", t.colors.ring.to_rgba())),
                ..final_style
            };
        }

        final_style.build()
    });

    // Combine with custom styles
    let final_style = if let Some(custom) = &style {
        format!("{} {}", item_style(), custom)
    } else {
        item_style()
    };

    let class_name = class.clone().unwrap_or_default();

    let handle_click = move |_| {
        if !disabled {
            context.toggle_value(&value);
        }
    };

    rsx! {
        button {
            r#type: "button",
            role: "switch",
            aria_pressed: "{is_selected}",
            disabled: disabled,
            style: "{final_style}",
            class: "{class_name}",
            onclick: handle_click,
            onmouseenter: move |_| if !disabled { is_hovered.set(true) },
            onmouseleave: move |_| is_hovered.set(false),
            onfocus: move |_| is_focused.set(true),
            onblur: move |_| is_focused.set(false),
            {icon}
        }
    }
}

use crate::theme::tokens::Color;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_group_type_equality() {
        assert_eq!(ToggleGroupType::Single, ToggleGroupType::Single);
        assert_eq!(ToggleGroupType::Multiple, ToggleGroupType::Multiple);
        assert_ne!(ToggleGroupType::Single, ToggleGroupType::Multiple);
    }
}
