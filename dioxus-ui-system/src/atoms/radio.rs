//! Radio button atom component
//!
//! A set of checkable buttons—known as radio buttons—where no more than one of the buttons can be checked at a time.

use crate::a11y::use_unique_id;
use crate::styles::Style;
use crate::theme::use_style;
use dioxus::prelude::*;

/// Radio properties
#[derive(Props, Clone, PartialEq)]
pub struct RadioProps {
    /// Whether the radio is selected
    #[props(default)]
    pub checked: bool,
    /// Called with this radio's `value` when it becomes selected
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,
    /// Whether the radio is disabled
    #[props(default)]
    pub disabled: bool,
    /// Radio label
    #[props(default)]
    pub label: Option<String>,
    /// Radio value
    #[props(default)]
    pub value: String,
    /// Name for grouping radios
    #[props(default)]
    pub name: String,
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

/// Radio atom component
#[component]
pub fn Radio(props: RadioProps) -> Element {
    let mut is_hovered = use_signal(|| false);
    let mut is_focused = use_signal(|| false);

    let checked = props.checked;
    let disabled = props.disabled;
    let cursor_style = if disabled { "not-allowed" } else { "pointer" };

    let radio_style = use_style(move |t| {
        let base = Style::new()
            .w_px(20)
            .h_px(20)
            .rounded_full()
            .border(1, &t.colors.border)
            .flex()
            .items_center()
            .justify_center()
            .cursor("pointer")
            .transition("all 150ms ease");

        let styled = if checked {
            base.border(4, &t.colors.primary)
        } else {
            base.bg(&t.colors.background)
        };

        let styled = if is_hovered() && !disabled && !checked {
            styled.border_color(&t.colors.primary)
        } else {
            styled
        };

        let styled = if is_focused() && !disabled {
            Style {
                box_shadow: Some(format!("0 0 0 2px {}", t.colors.ring.to_rgba())),
                ..styled
            }
        } else {
            styled
        };

        let styled = if disabled {
            styled.opacity(0.5)
        } else {
            styled
        };

        styled.build()
    });

    let opacity_style = if disabled { "0.5" } else { "1" };

    let label_text_element = if let Some(label_text) = props.label.clone() {
        rsx! {
            span {
                style: "opacity: {opacity_style};",
                "{label_text}"
            }
        }
    } else {
        rsx! {}
    };

    let id_css = props.id.clone().unwrap_or_default();

    rsx! {
        label {
            style: "display: flex; align-items: center; gap: 8px; cursor: {cursor_style}; position: relative;",
            id: "{id_css}",

            input {
                r#type: "radio",
                name: "{props.name}",
                value: "{props.value}",
                checked: checked,
                disabled: disabled,
                style: "position: absolute; opacity: 0; width: 0; height: 0;",
                onchange: move |_| {
                    if !disabled && !checked {
                        if let Some(handler) = &props.on_change {
                            handler.call(props.value.clone());
                        }
                    }
                },
            }
            span {
                style: "{radio_style} {props.style.clone().unwrap_or_default()}",
                class: "{props.class.clone().unwrap_or_default()}",
                onmouseenter: move |_| if !disabled { is_hovered.set(true) },
                onmouseleave: move |_| is_hovered.set(false),
                onfocus: move |_| is_focused.set(true),
                onblur: move |_| is_focused.set(false),
            }
            {label_text_element}
        }
    }
}

/// Radio group properties
#[derive(Props, Clone, PartialEq)]
pub struct RadioGroupProps {
    /// Radio group name
    pub name: String,
    /// Currently selected value
    pub value: String,
    /// Callback when selection changes
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,
    /// Radio group children (Radio components)
    pub children: Element,
    /// Layout direction
    #[props(default)]
    pub direction: RadioDirection,
    /// Visible group label, rendered above the radios and referenced by
    /// `aria-labelledby`
    #[props(default)]
    pub label: Option<String>,
    /// Accessible name when there is no visible `label` (`aria-label`)
    #[props(default)]
    pub aria_label: Option<String>,
    /// Element id of the group (also used to derive the label id)
    #[props(default)]
    pub id: Option<String>,
}

/// Radio group layout direction
#[derive(Default, Clone, PartialEq)]
pub enum RadioDirection {
    /// Horizontal layout
    #[default]
    Horizontal,
    /// Vertical layout
    Vertical,
}

impl RadioDirection {
    /// Value for `aria-orientation`.
    fn aria_orientation(&self) -> &'static str {
        match self {
            RadioDirection::Horizontal => "horizontal",
            RadioDirection::Vertical => "vertical",
        }
    }
}

/// Radio group component for managing a set of radio buttons
///
/// # Accessibility
/// Renders `role="radiogroup"` with `aria-orientation` from `direction`. The
/// group is named by its visible `label` (via `aria-labelledby`) or by
/// `aria_label`. Radios are native `<input type="radio">` elements sharing
/// `name`, so the browser provides Tab into the group and arrow-key movement
/// between its radios.
#[component]
pub fn RadioGroup(props: RadioGroupProps) -> Element {
    let generated_id = use_unique_id("radio-group");
    let group_id = props.id.clone().unwrap_or(generated_id);
    let label_id = format!("{group_id}-label");

    let direction = match props.direction {
        RadioDirection::Horizontal => "row",
        RadioDirection::Vertical => "column",
    };
    let orientation = props.direction.aria_orientation();
    let labelledby = props.label.as_ref().map(|_| label_id.clone());
    let aria_label = if props.label.is_none() {
        props.aria_label.clone()
    } else {
        None
    };

    rsx! {
        if let Some(label) = props.label.clone() {
            span {
                id: "{label_id}",
                class: "radio-group-label",
                style: "display: block; margin-bottom: 8px; font-size: 14px; font-weight: 500;",
                "{label}"
            }
        }
        div {
            id: "{group_id}",
            role: "radiogroup",
            "aria-labelledby": labelledby,
            "aria-label": aria_label,
            "aria-orientation": "{orientation}",
            style: "display: flex; flex-direction: {direction}; gap: 12px;",
            {props.children}
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

    #[test]
    fn radio_group_is_labelled_by_visible_label() {
        let html = render(|| {
            rsx! {
                RadioGroup {
                    id: "plan",
                    name: "plan".to_string(),
                    value: "a".to_string(),
                    label: "Plan",
                    direction: RadioDirection::Vertical,
                    Radio { name: "plan".to_string(), value: "a".to_string(), checked: true }
                    Radio { name: "plan".to_string(), value: "b".to_string() }
                }
            }
        });
        assert!(html.contains("role=\"radiogroup\""), "{html}");
        assert!(html.contains("id=\"plan-label\""), "{html}");
        assert!(html.contains("aria-labelledby=\"plan-label\""), "{html}");
        assert!(html.contains("aria-orientation=\"vertical\""), "{html}");
        assert!(!html.contains("aria-label=\""), "{html}");
        assert!(html.contains("type=\"radio\""), "{html}");
    }

    #[test]
    fn radio_group_without_label_uses_aria_label() {
        let html = render(|| {
            rsx! {
                RadioGroup { name: "size".to_string(), value: "s".to_string(), aria_label: "Size",
                    Radio { name: "size".to_string(), value: "s".to_string() }
                }
            }
        });
        assert!(html.contains("role=\"radiogroup\""), "{html}");
        assert!(html.contains("aria-label=\"Size\""), "{html}");
        assert!(html.contains("aria-orientation=\"horizontal\""), "{html}");
        assert!(!html.contains("aria-labelledby"), "{html}");
    }
}
