//! Accordion organism component
//!
//! A vertically stacked set of interactive headings that each reveal a section of content.

use crate::a11y::{focus_element, next_index, use_unique_id, Orientation};
use crate::styles::Style;
use crate::theme::{use_style, use_theme};
use dioxus::prelude::*;

/// Accordion item definition
#[derive(Clone, PartialEq)]
pub struct AccordionItem {
    /// Item ID
    pub id: String,
    /// Item title/heading
    pub title: String,
    /// Item content
    pub content: String,
    /// Whether item is disabled
    pub disabled: bool,
}

impl AccordionItem {
    /// Create a new accordion item
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            content: content.into(),
            disabled: false,
        }
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Accordion properties
#[derive(Props, Clone, PartialEq)]
pub struct AccordionProps {
    /// Accordion items
    pub items: Vec<AccordionItem>,
    /// Currently expanded item IDs (for single: one item, for multiple: many)
    pub expanded: Vec<String>,
    /// Callback when expanded items change
    #[props(default)]
    pub on_change: Option<EventHandler<Vec<String>>>,
    /// Whether multiple items can be expanded
    #[props(default)]
    pub multiple: bool,
    /// Whether items can be collapsed
    #[props(default = true)]
    pub collapsible: bool,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Element id
    #[props(default)]
    pub id: Option<String>,
    /// Custom CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Accordion organism component
///
/// # Accessibility
///
/// Follows the WAI-ARIA accordion pattern: each header is a `button` inside an
/// `h3` with `aria-expanded` and `aria-controls`; each panel has
/// `role="region"` and `aria-labelledby` pointing at its header. Enter/Space
/// toggle a section (native button behaviour), Up/Down move focus between
/// enabled headers (wrapping) and Home/End jump to the first/last header.
/// Element ids derive from `id` when given, otherwise from a generated id.
#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    let _theme = use_theme();
    let generated_id = use_unique_id("accordion");
    let base_id = props.id.clone().unwrap_or(generated_id);
    let disabled: Vec<bool> = props.items.iter().map(|item| item.disabled).collect();

    let accordion_style = use_style(|t| {
        Style::new()
            .w_full()
            .rounded(&t.radius, "lg")
            .border(1, &t.colors.border)
            .bg(&t.colors.background)
            .flex()
            .flex_col()
            .build()
    });

    rsx! {
        div {
            id: "{props.id.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            style: "{accordion_style} {props.style.clone().unwrap_or_default()}",

            for (index, item) in props.items.iter().enumerate() {
                AccordionSection {
                    key: "{item.id}",
                    item: item.clone(),
                    index: index,
                    base_id: base_id.clone(),
                    disabled_items: disabled.clone(),
                    is_expanded: props.expanded.contains(&item.id),
                    is_last: index == props.items.len() - 1,
                    multiple: props.multiple,
                    collapsible: props.collapsible,
                    on_toggle: props.on_change,
                    all_expanded: props.expanded.clone(),
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct AccordionSectionProps {
    item: AccordionItem,
    index: usize,
    base_id: String,
    disabled_items: Vec<bool>,
    is_expanded: bool,
    is_last: bool,
    multiple: bool,
    collapsible: bool,
    #[props(default)]
    on_toggle: Option<EventHandler<Vec<String>>>,
    all_expanded: Vec<String>,
}

#[component]
fn AccordionSection(props: AccordionSectionProps) -> Element {
    let _theme = use_theme();
    let mut is_hovered = use_signal(|| false);

    let is_expanded = props.is_expanded;
    let is_disabled = props.item.disabled;

    let trigger_style = use_style(move |t| {
        let base = Style::new()
            .w_full()
            .flex()
            .items_center()
            .justify_between()
            .px(&t.spacing, "lg")
            .py(&t.spacing, "md")
            .text(&t.typography, "base")
            .font_weight(500)
            .cursor(if is_disabled {
                "not-allowed"
            } else {
                "pointer"
            })
            .transition("all 150ms ease")
            .bg_transparent()
            .border(0, &t.colors.border)
            .outline("none")
            .text_color(&t.colors.foreground);

        if !props.is_last || is_expanded {
            base.border_bottom(1, &t.colors.border)
        } else {
            base
        }
        .build()
    });

    let content_style = use_style(|t| {
        Style::new()
            .px(&t.spacing, "lg")
            .py(&t.spacing, "md")
            .text(&t.typography, "sm")
            .text_color(&t.colors.muted_foreground)
            .line_height(1.6)
            .build()
    });

    let handle_toggle = move |_| {
        if is_disabled {
            return;
        }

        let new_expanded = toggle_expanded(
            &props.all_expanded,
            &props.item.id,
            props.multiple,
            props.collapsible,
        );

        if let Some(h) = &props.on_toggle {
            h.call(new_expanded);
        }
    };

    let header_id = accordion_header_id(&props.base_id, props.index);
    let panel_id = accordion_panel_id(&props.base_id, props.index);

    let handle_keydown = {
        let base_id = props.base_id.clone();
        let disabled_items = props.disabled_items.clone();
        let index = props.index;
        move |e: KeyboardEvent| {
            if let Some(target) = next_header(&e.key(), index, &disabled_items) {
                e.prevent_default();
                focus_element(&accordion_header_id(&base_id, target));
            }
        }
    };

    let chevron_rotation = if is_expanded {
        "rotate(180deg)"
    } else {
        "rotate(0deg)"
    };

    rsx! {
        div {
            h3 {
                style: "margin: 0;",

                button {
                    id: "{header_id}",
                    style: "{trigger_style}",
                    type: "button",
                    aria_expanded: "{is_expanded}",
                    "aria-controls": "{panel_id}",
                    disabled: is_disabled,
                    onclick: handle_toggle,
                    onkeydown: handle_keydown,
                    onmouseenter: move |_| if !is_disabled { is_hovered.set(true) },
                    onmouseleave: move |_| is_hovered.set(false),

                    "{props.item.title}"

                    span {
                        style: "transform: {chevron_rotation}; transition: transform 200ms ease;",

                        AccordionChevron {}
                    }
                }
            }

            div {
                id: "{panel_id}",
                role: "region",
                "aria-labelledby": "{header_id}",
                hidden: !is_expanded,
                style: "{content_style}",
                if is_expanded {
                    "{props.item.content}"
                }
            }
        }
    }
}

/// Element id of the header button at `index`.
fn accordion_header_id(base_id: &str, index: usize) -> String {
    format!("{base_id}-header-{index}")
}

/// Element id of the panel at `index`.
fn accordion_panel_id(base_id: &str, index: usize) -> String {
    format!("{base_id}-panel-{index}")
}

/// Index of the header that should receive focus for `key`, skipping disabled
/// headers (they cannot be focused). Up/Down wrap; Home/End jump.
fn next_header(key: &Key, current: usize, disabled: &[bool]) -> Option<usize> {
    let enabled: Vec<usize> = (0..disabled.len()).filter(|&i| !disabled[i]).collect();
    let position = enabled.iter().position(|&i| i == current)?;
    let next = next_index(key, position, enabled.len(), Orientation::Vertical)?;
    Some(enabled[next])
}

/// New list of expanded ids after toggling `item_id`.
fn toggle_expanded(
    expanded: &[String],
    item_id: &str,
    multiple: bool,
    collapsible: bool,
) -> Vec<String> {
    let mut new_expanded = expanded.to_vec();
    let is_open = new_expanded.iter().any(|id| id == item_id);
    if is_open {
        if collapsible {
            if multiple {
                new_expanded.retain(|id| id != item_id);
            } else {
                new_expanded.clear();
            }
        }
    } else {
        if !multiple {
            new_expanded.clear();
        }
        new_expanded.push(item_id.to_string());
    }
    new_expanded
}

#[component]
fn AccordionChevron() -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            style: "width: 16px; height: 16px;",
            polyline { points: "6 9 12 15 18 9" }
        }
    }
}

/// Simplified accordion item with custom content
#[derive(Props, Clone, PartialEq)]
pub struct AccordionItem2Props {
    /// Item title
    #[props(default)]
    pub title: Option<String>,
    /// Item content
    pub children: Element,
    /// Whether item is expanded
    #[props(default)]
    pub default_expanded: bool,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Element id
    #[props(default)]
    pub id: Option<String>,
    /// Custom CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Single accordion item that manages its own state
///
/// # Accessibility
///
/// The header is a `button` with `aria-expanded` and `aria-controls`; the panel
/// has `role="region"` and `aria-labelledby`. Enter/Space toggle it. Arrow-key
/// navigation between headers is provided by [`Accordion`], not by standalone
/// items.
#[component]
pub fn AccordionItem2(props: AccordionItem2Props) -> Element {
    let _theme = use_theme();
    let generated_id = use_unique_id("accordion-item");
    let base_id = props.id.clone().unwrap_or(generated_id);
    let header_id = format!("{base_id}-header");
    let panel_id = format!("{base_id}-panel");
    let mut is_expanded = use_signal(|| props.default_expanded);
    let mut is_hovered = use_signal(|| false);

    let trigger_style = use_style(move |t| {
        Style::new()
            .w_full()
            .flex()
            .items_center()
            .justify_between()
            .px(&t.spacing, "lg")
            .py(&t.spacing, "md")
            .text(&t.typography, "base")
            .font_weight(500)
            .cursor("pointer")
            .transition("all 150ms ease")
            .bg_transparent()
            .border(0, &t.colors.border)
            .outline("none")
            .text_color(&t.colors.foreground)
            .border_bottom(1, &t.colors.border)
            .build()
    });

    let content_style = use_style(|t| {
        Style::new()
            .px(&t.spacing, "lg")
            .py(&t.spacing, "md")
            .build()
    });

    let expanded = is_expanded();
    let chevron_rotation = if expanded {
        "rotate(180deg)"
    } else {
        "rotate(0deg)"
    };

    rsx! {
        div {
            id: "{props.id.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            style: "{props.style.clone().unwrap_or_default()}",

            h3 {
                style: "margin: 0;",

                button {
                    style: "{trigger_style}",
                    type: "button",
                    id: "{header_id}",
                    aria_expanded: "{expanded}",
                    "aria-controls": "{panel_id}",
                    onclick: move |_| is_expanded.toggle(),
                    onmouseenter: move |_| is_hovered.set(true),
                    onmouseleave: move |_| is_hovered.set(false),

                    if let Some(title) = props.title.clone() {
                        "{title}"
                    }

                    span {
                        style: "transform: {chevron_rotation}; transition: transform 200ms ease;",
                        AccordionChevron {}
                    }
                }
            }

            div {
                id: "{panel_id}",
                role: "region",
                "aria-labelledby": "{header_id}",
                hidden: !expanded,
                style: "{content_style}",
                if expanded {
                    {props.children}
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

    #[test]
    fn accordion_wires_headers_and_panels() {
        let html = render(|| {
            rsx! {
                Accordion {
                    id: "acc",
                    items: vec![
                        AccordionItem::new("a", "First", "Alpha"),
                        AccordionItem::new("b", "Second", "Beta"),
                    ],
                    expanded: vec!["a".to_string()],
                }
            }
        });
        assert!(html.contains("id=\"acc-header-0\""), "{html}");
        assert!(html.contains("aria-controls=\"acc-panel-0\""), "{html}");
        assert!(html.contains("aria-controls=\"acc-panel-1\""), "{html}");
        assert!(html.contains("id=\"acc-panel-1\""), "{html}");
        assert!(html.contains("role=\"region\""), "{html}");
        assert!(html.contains("aria-labelledby=\"acc-header-0\""), "{html}");
        assert!(html.contains("aria-expanded=\"true\""), "{html}");
        assert!(html.contains("aria-expanded=\"false\""), "{html}");
        assert!(html.contains("Alpha"), "{html}");
        assert!(!html.contains("Beta"), "{html}");
    }

    #[test]
    fn accordion_item2_wires_header_and_panel() {
        let html = render(|| {
            rsx! {
                AccordionItem2 { id: "item", title: "Title", default_expanded: true, "Body" }
            }
        });
        assert!(html.contains("id=\"item-header\""), "{html}");
        assert!(html.contains("aria-controls=\"item-panel\""), "{html}");
        assert!(html.contains("aria-labelledby=\"item-header\""), "{html}");
        assert!(html.contains("role=\"region\""), "{html}");
        assert!(html.contains("Body"), "{html}");
    }

    #[test]
    fn next_header_skips_disabled_and_wraps() {
        let disabled = [false, true, false, false];
        assert_eq!(next_header(&Key::ArrowDown, 0, &disabled), Some(2));
        assert_eq!(next_header(&Key::ArrowUp, 2, &disabled), Some(0));
        assert_eq!(next_header(&Key::ArrowDown, 3, &disabled), Some(0));
        assert_eq!(next_header(&Key::ArrowUp, 0, &disabled), Some(3));
        assert_eq!(next_header(&Key::Home, 3, &disabled), Some(0));
        assert_eq!(next_header(&Key::End, 0, &disabled), Some(3));
        assert_eq!(next_header(&Key::ArrowLeft, 0, &disabled), None);
        assert_eq!(next_header(&Key::ArrowDown, 1, &disabled), None);
    }

    #[test]
    fn toggle_expanded_single_and_multiple() {
        let ids = |v: &[&str]| v.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(toggle_expanded(&ids(&["a"]), "b", false, true), ids(&["b"]));
        assert_eq!(toggle_expanded(&ids(&["a"]), "a", false, true), ids(&[]));
        assert_eq!(
            toggle_expanded(&ids(&["a"]), "a", false, false),
            ids(&["a"])
        );
        assert_eq!(
            toggle_expanded(&ids(&["a"]), "b", true, true),
            ids(&["a", "b"])
        );
        assert_eq!(
            toggle_expanded(&ids(&["a", "b"]), "a", true, true),
            ids(&["b"])
        );
        assert_eq!(toggle_expanded(&ids(&["a"]), "a", true, false), ids(&["a"]));
    }
}
