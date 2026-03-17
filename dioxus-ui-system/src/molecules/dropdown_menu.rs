//! Dropdown Menu molecule component
//!
//! Displays a menu to the user—such as a set of actions or functions—triggered by a button.

use dioxus::prelude::*;
use crate::theme::{use_theme, use_style};
use crate::styles::Style;

/// Dropdown menu item
#[derive(Clone, PartialEq)]
pub struct DropdownMenuItem {
    /// Item label
    pub label: String,
    /// Item value
    pub value: String,
    /// Optional icon
    pub icon: Option<String>,
    /// Whether item is disabled
    pub disabled: bool,
    /// Keyboard shortcut (optional)
    pub shortcut: Option<String>,
}

impl DropdownMenuItem {
    /// Create a new dropdown menu item
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            icon: None,
            disabled: false,
            shortcut: None,
        }
    }
    
    /// Add an icon
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
    
    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    
    /// Add keyboard shortcut
    pub fn with_shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }
}

/// Dropdown menu properties
#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuProps {
    /// Trigger element
    pub trigger: Element,
    /// Menu items
    pub items: Vec<DropdownMenuItem>,
    /// Callback when an item is selected
    pub on_select: EventHandler<String>,
    /// Menu alignment
    #[props(default)]
    pub align: DropdownAlign,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Dropdown alignment
#[derive(Default, Clone, PartialEq)]
pub enum DropdownAlign {
    /// Align to start (left)
    #[default]
    Start,
    /// Align to end (right)
    End,
    /// Center alignment
    Center,
}

/// Dropdown menu component
#[component]
pub fn DropdownMenu(props: DropdownMenuProps) -> Element {
    let _theme = use_theme();
    let mut is_open = use_signal(|| false);
    
    let position = match props.align {
        DropdownAlign::Start => "left: 0;",
        DropdownAlign::End => "right: 0;",
        DropdownAlign::Center => "left: 50%; transform: translateX(-50%);",
    };
    
    let menu_style = use_style(|t| {
        Style::new()
            .absolute()
            .top("calc(100% + 4px)")
            .min_w_px(160)
            .max_w_px(280)
            .rounded(&t.radius, "md")
            .border(1, &t.colors.border)
            .bg(&t.colors.popover)
            .shadow(&t.shadows.lg)
            .flex()
            .flex_col()
            .p(&t.spacing, "xs")
            .z_index(50)
            .build()
    });
    
    rsx! {
        div {
            style: "position: relative; display: inline-block;",
            
            // Trigger
            div {
                onclick: move |_| is_open.toggle(),
                {props.trigger}
            }
            
            // Menu
            if is_open() {
                // Overlay to close on outside click
                div {
                    style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; z-index: 40;",
                    onclick: move |_| is_open.set(false),
                }
                
                div {
                    style: "{menu_style} {position} {props.style.clone().unwrap_or_default()}",
                    onclick: move |e| e.stop_propagation(),
                    
                    for item in props.items.clone() {
                        DropdownMenuItemView {
                            item: item.clone(),
                            on_select: props.on_select.clone(),
                            on_close: move || is_open.set(false),
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct DropdownMenuItemViewProps {
    item: DropdownMenuItem,
    on_select: EventHandler<String>,
    on_close: EventHandler<()>,
}

#[component]
fn DropdownMenuItemView(props: DropdownMenuItemViewProps) -> Element {
    let _theme = use_theme();
    let mut is_hovered = use_signal(|| false);
    
    let item_style = use_style(move |t| {
        let base = Style::new()
            .w_full()
            .flex()
            .items_center()
            .justify_between()
            .px(&t.spacing, "sm")
            .py(&t.spacing, "sm")
            .rounded(&t.radius, "sm")
            .text(&t.typography, "sm")
            .cursor(if props.item.disabled { "not-allowed" } else { "pointer" })
            .transition("all 100ms ease");
        
        if is_hovered() && !props.item.disabled {
            base.bg(&t.colors.accent)
                .text_color(&t.colors.accent_foreground)
        } else {
            base
        }.build()
    });
    
    let handle_click = move |_| {
        if !props.item.disabled {
            props.on_select.call(props.item.value.clone());
            props.on_close.call(());
        }
    };
    
    rsx! {
        button {
            style: "{item_style} background: none; border: none; text-align: left; color: inherit;",
            disabled: props.item.disabled,
            onclick: handle_click,
            onmouseenter: move |_| if !props.item.disabled { is_hovered.set(true) },
            onmouseleave: move |_| is_hovered.set(false),
            
            div {
                style: "display: flex; align-items: center; gap: 8px;",
                
                if let Some(icon) = props.item.icon.clone() {
                    DropdownIcon { name: icon }
                }
                
                span {
                    style: if props.item.disabled { "opacity: 0.5;" } else { "" },
                    "{props.item.label}"
                }
            }
            
            if let Some(shortcut) = props.item.shortcut.clone() {
                span {
                    style: "font-size: 11px; color: #94a3b8; margin-left: 24px;",
                    "{shortcut}"
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct DropdownIconProps {
    name: String,
}

#[component]
fn DropdownIcon(props: DropdownIconProps) -> Element {
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
                "edit" => rsx! {
                    path { d: "M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" }
                    path { d: "M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z" }
                },
                "copy" => rsx! {
                    rect { x: "9", y: "9", width: "13", height: "13", rx: "2", ry: "2" }
                    path { d: "M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" }
                },
                "trash" => rsx! {
                    polyline { points: "3 6 5 6 21 6" }
                    path { d: "M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" }
                },
                _ => rsx! {
                    circle { cx: "12", cy: "12", r: "10" }
                },
            }
        }
    }
}

/// Dropdown menu separator
#[component]
pub fn DropdownMenuSeparator() -> Element {
    let _theme = use_theme();
    
    let separator_style = use_style(|t| {
        Style::new()
            .h_px(1)
            .mx(&t.spacing, "sm")
            .my(&t.spacing, "xs")
            .bg(&t.colors.border)
            .build()
    });
    
    rsx! {
        div {
            style: "{separator_style}",
        }
    }
}

/// Dropdown menu label
#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuLabelProps {
    pub children: Element,
}

#[component]
pub fn DropdownMenuLabel(props: DropdownMenuLabelProps) -> Element {
    rsx! {
        div {
            style: "padding: 6px 8px; font-size: 12px; font-weight: 500; color: #64748b;",
            {props.children}
        }
    }
}
