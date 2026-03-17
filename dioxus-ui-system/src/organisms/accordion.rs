//! Accordion organism component
//!
//! A vertically stacked set of interactive headings that each reveal a section of content.

use dioxus::prelude::*;
use crate::theme::{use_theme, use_style};
use crate::styles::Style;

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
    pub fn new(id: impl Into<String>, title: impl Into<String>, content: impl Into<String>) -> Self {
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
    pub on_change: EventHandler<Vec<String>>,
    /// Whether multiple items can be expanded
    #[props(default)]
    pub multiple: bool,
    /// Whether items can be collapsed
    #[props(default = true)]
    pub collapsible: bool,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Accordion organism component
#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    let _theme = use_theme();
    
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
            style: "{accordion_style} {props.style.clone().unwrap_or_default()}",
            
            for (index, item) in props.items.iter().enumerate() {
                AccordionSection {
                    key: "{item.id}",
                    item: item.clone(),
                    is_expanded: props.expanded.contains(&item.id),
                    is_last: index == props.items.len() - 1,
                    multiple: props.multiple,
                    collapsible: props.collapsible,
                    on_toggle: props.on_change.clone(),
                    all_expanded: props.expanded.clone(),
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct AccordionSectionProps {
    item: AccordionItem,
    is_expanded: bool,
    is_last: bool,
    multiple: bool,
    collapsible: bool,
    on_toggle: EventHandler<Vec<String>>,
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
            .cursor(if is_disabled { "not-allowed" } else { "pointer" })
            .transition("all 150ms ease")
            .bg_transparent()
            .border(0, &t.colors.border)
            .outline("none")
            .text_color(&t.colors.foreground);
        
        if !props.is_last || is_expanded {
            base.border_bottom(1, &t.colors.border)
        } else {
            base
        }.build()
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
        
        let mut new_expanded = props.all_expanded.clone();
        let item_id = props.item.id.clone();
        
        if props.multiple {
            // Toggle in multiple mode
            if new_expanded.contains(&item_id) {
                if props.collapsible {
                    new_expanded.retain(|id| id != &item_id);
                }
            } else {
                new_expanded.push(item_id);
            }
        } else {
            // Single mode
            if new_expanded.contains(&item_id) {
                if props.collapsible {
                    new_expanded.clear();
                }
            } else {
                new_expanded.clear();
                new_expanded.push(item_id);
            }
        }
        
        props.on_toggle.call(new_expanded);
    };
    
    let chevron_rotation = if is_expanded { "rotate(180deg)" } else { "rotate(0deg)" };
    
    rsx! {
        div {
            h3 {
                style: "margin: 0;",
                
                button {
                    style: "{trigger_style}",
                    type: "button",
                    aria_expanded: "{is_expanded}",
                    disabled: is_disabled,
                    onclick: handle_toggle,
                    onmouseenter: move |_| if !is_disabled { is_hovered.set(true) },
                    onmouseleave: move |_| is_hovered.set(false),
                    
                    "{props.item.title}"
                    
                    span {
                        style: "transform: {chevron_rotation}; transition: transform 200ms ease;",
                        
                        AccordionChevron {}
                    }
                }
            }
            
            if is_expanded {
                div {
                    style: "{content_style}",
                    "{props.item.content}"
                }
            }
        }
    }
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
    pub title: String,
    /// Item content
    pub children: Element,
    /// Whether item is expanded
    #[props(default)]
    pub default_expanded: bool,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Single accordion item that manages its own state
#[component]
pub fn AccordionItem2(props: AccordionItem2Props) -> Element {
    let _theme = use_theme();
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
    let chevron_rotation = if expanded { "rotate(180deg)" } else { "rotate(0deg)" };
    
    rsx! {
        div {
            style: "{props.style.clone().unwrap_or_default()}",
            
            h3 {
                style: "margin: 0;",
                
                button {
                    style: "{trigger_style}",
                    type: "button",
                    aria_expanded: "{expanded}",
                    onclick: move |_| is_expanded.toggle(),
                    onmouseenter: move |_| is_hovered.set(true),
                    onmouseleave: move |_| is_hovered.set(false),
                    
                    "{props.title}"
                    
                    span {
                        style: "transform: {chevron_rotation}; transition: transform 200ms ease;",
                        AccordionChevron {}
                    }
                }
            }
            
            if expanded {
                div {
                    style: "{content_style}",
                    {props.children}
                }
            }
        }
    }
}
