//! Breadcrumb molecule component
//!
//! Displays the path to the current resource using a hierarchy of links.

use dioxus::prelude::*;
use crate::theme::{use_theme, use_style};
use crate::styles::Style;

/// Breadcrumb item
#[derive(Clone, PartialEq)]
pub struct BreadcrumbItem {
    /// Item label
    pub label: String,
    /// Item href (None for current page)
    pub href: Option<String>,
    /// Item icon (optional)
    pub icon: Option<String>,
}

impl BreadcrumbItem {
    /// Create a new breadcrumb item
    pub fn new(label: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: Some(href.into()),
            icon: None,
        }
    }
    
    /// Create a new breadcrumb item without link (current page)
    pub fn current(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: None,
            icon: None,
        }
    }
    
    /// Add an icon to the breadcrumb item
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

/// Breadcrumb properties
#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbProps {
    /// Breadcrumb items
    pub items: Vec<BreadcrumbItem>,
    /// Custom separator (default: /)
    #[props(default)]
    pub separator: Option<String>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Breadcrumb molecule component
#[component]
pub fn Breadcrumb(props: BreadcrumbProps) -> Element {
    let _theme = use_theme();
    
    let breadcrumb_style = use_style(|t| {
        Style::new()
            .flex()
            .flex_wrap()
            .items_center()
            .gap(&t.spacing, "sm")
            .text(&t.typography, "sm")
            .build()
    });
    
    let separator = props.separator.unwrap_or_else(|| "›".to_string());
    let items_count = props.items.len();
    
    rsx! {
        nav {
            aria_label: "breadcrumb",
            style: "{breadcrumb_style} {props.style.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            
            ol {
                style: "display: flex; flex-wrap: wrap; align-items: center; gap: 8px; list-style: none; margin: 0; padding: 0;",
                
                for (index, item) in props.items.iter().enumerate() {
                    li {
                        key: "{item.label}",
                        style: "display: flex; align-items: center; gap: 8px;",
                        
                        if index > 0 {
                            span {
                                style: "color: #94a3b8; user-select: none;",
                                "{separator}"
                            }
                        }
                        
                        BreadcrumbLink {
                            item: item.clone(),
                            is_last: index == items_count - 1,
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct BreadcrumbLinkProps {
    item: BreadcrumbItem,
    is_last: bool,
}

#[component]
fn BreadcrumbLink(props: BreadcrumbLinkProps) -> Element {
    let _theme = use_theme();
    
    let link_style = use_style(move |t| {
        let base = Style::new()
            .flex()
            .items_center()
            .gap(&t.spacing, "xs")
            .transition("color 150ms ease");
        
        if props.is_last {
            base.text_color(&t.colors.foreground)
                .font_weight(500)
        } else {
            base.text_color(&t.colors.muted_foreground)
                .no_underline()
        }.build()
    });
    
    let has_icon = props.item.icon.is_some();
    
    if props.is_last || props.item.href.is_none() {
        rsx! {
            span {
                style: "{link_style} cursor: default;",
                aria_current: "page",
                
                if has_icon {
                    BreadcrumbIcon { name: props.item.icon.clone().unwrap() }
                }
                
                "{props.item.label}"
            }
        }
    } else {
        rsx! {
            a {
                href: "{props.item.href.clone().unwrap()}",
                style: "{link_style}",
                onmouseenter: move |e| {
                    let _ = e;
                    // Could add hover effect here
                },
                
                if has_icon {
                    BreadcrumbIcon { name: props.item.icon.clone().unwrap() }
                }
                
                "{props.item.label}"
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct BreadcrumbIconProps {
    name: String,
}

#[component]
fn BreadcrumbIcon(props: BreadcrumbIconProps) -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            style: "width: 16px; height: 16px;",
            
            match props.name.as_str() {
                "home" => rsx! {
                    path { d: "m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" }
                    polyline { points: "9 22 9 12 15 12 15 22" }
                },
                "folder" => rsx! {
                    path { d: "M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z" }
                },
                _ => rsx! {
                    circle { cx: "12", cy: "12", r: "10" }
                },
            }
        }
    }
}
