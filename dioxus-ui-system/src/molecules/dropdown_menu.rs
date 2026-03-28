//! Dropdown Menu molecule component
//!
//! Displays a menu to the user—such as a set of actions or functions—triggered by a button.
//! Uses fixed positioning with portal-like behavior to escape parent overflow clipping.

use crate::atoms::{Icon, IconColor, IconSize};
use crate::styles::Style;
use crate::theme::{use_style, use_theme};
use dioxus::prelude::*;

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
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
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
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
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
///
/// Uses a portal-like approach with fixed positioning to avoid being clipped by parent containers.
/// The menu is rendered with position:fixed at calculated coordinates based on the trigger's position.
#[component]
pub fn DropdownMenu(props: DropdownMenuProps) -> Element {
    let _theme = use_theme();
    let mut is_open = use_signal(|| false);
    let mut menu_position = use_signal(|| (0i32, 0i32));

    let menu_base_style = use_style(|t| {
        Style::new()
            .rounded(&t.radius, "md")
            .border(1, &t.colors.border)
            .bg(&t.colors.popover)
            .shadow(&t.shadows.lg)
            .flex()
            .flex_col()
            .py(&t.spacing, "xs")
            .z_index(9999)
            .build()
    });

    // Store alignment for use in click handler
    let align = props.align.clone();

    let handle_trigger_click = move |event: Event<MouseData>| {
        if !is_open() {
            // Get the click coordinates in viewport space
            let coords = event.data().page_coordinates();
            let click_x = coords.x as i32;
            let click_y = coords.y as i32;

            // The menu width (used for alignment calculations)
            let menu_width = 180;

            // Calculate menu position based on alignment
            // We position relative to the click, with some offset to show below the trigger
            let (menu_x, menu_y) = match align {
                DropdownAlign::Start => (click_x - 20, click_y + 20), // Click is somewhere in trigger, offset left
                DropdownAlign::End => (click_x - menu_width + 20, click_y + 20), // Offset right
                DropdownAlign::Center => (click_x - menu_width / 2, click_y + 20), // Center on click
            };

            // Ensure menu stays within viewport (with some padding)
            let padding = 8;
            let final_x = menu_x.max(padding);
            let final_y = menu_y.max(padding);

            menu_position.set((final_x, final_y));
        }
        is_open.toggle();
    };

    let (menu_x, menu_y) = menu_position();
    let position_style = format!(
        "position: fixed; left: {}px; top: {}px; width: 180px;",
        menu_x, menu_y
    );
    let custom_style = props.style.clone().unwrap_or_default();

    rsx! {
        div {
            style: "position: relative; display: inline-block;",

            // Trigger
            div {
                onclick: handle_trigger_click,
                {props.trigger}
            }

            // Overlay to close on outside click
            if is_open() {
                div {
                    style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; z-index: 9998;",
                    onclick: move |_| is_open.set(false),
                }
            }

            // Menu - rendered with fixed positioning to escape clipping
            if is_open() {
                div {
                    style: "{menu_base_style} {position_style} {custom_style}",
                    onclick: move |e| e.stop_propagation(),

                    for item in props.items.clone() {
                        DropdownMenuItemView {
                            key: "{item.value}",
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
            .gap(&t.spacing, "sm")
            .px(&t.spacing, "sm")
            .py(&t.spacing, "sm")
            .rounded(&t.radius, "sm")
            .text(&t.typography, "sm")
            .cursor(if props.item.disabled {
                "not-allowed"
            } else {
                "pointer"
            })
            .opacity(if props.item.disabled { 0.5 } else { 1.0 });

        if is_hovered() && !props.item.disabled {
            base.bg(&t.colors.accent).build()
        } else {
            base.build()
        }
    });

    let value = props.item.value.clone();
    let on_select = props.on_select.clone();
    let on_close = props.on_close.clone();

    rsx! {
        div {
            style: "{item_style}",
            onmouseenter: move |_| is_hovered.set(true),
            onmouseleave: move |_| is_hovered.set(false),
            onclick: move |_| {
                if !props.item.disabled {
                    on_select.call(value.clone());
                    on_close.call(());
                }
            },

            // Label and icon
            div {
                style: "display: flex; align-items: center; gap: 8px;",
                if let Some(icon) = &props.item.icon {
                    Icon { name: icon.clone(), size: IconSize::Small, color: IconColor::Muted }
                }
                span { "{props.item.label}" }
            }

            // Shortcut
            if let Some(shortcut) = &props.item.shortcut {
                span {
                    style: "font-size: 12px; color: rgb(148,163,184); margin-left: 24px;",
                    "{shortcut}"
                }
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
