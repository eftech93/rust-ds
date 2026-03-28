//! Context Menu molecule component
//!
//! A right-click context menu that displays actions when the user right-clicks on an element.
//! Uses fixed positioning with portal-like behavior to escape parent overflow clipping.

use crate::styles::Style;
use crate::theme::{use_style, use_theme};
use dioxus::prelude::*;

/// Context Menu properties
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuProps {
    /// Child elements (should include ContextMenuTrigger and ContextMenuContent)
    pub children: Element,
}

/// Context menu container component
///
/// Provides the context for child trigger and content components.
#[component]
pub fn ContextMenu(props: ContextMenuProps) -> Element {
    let is_open = use_signal(|| false);
    let menu_position = use_signal(|| (0i32, 0i32));
    let focused_index = use_signal(|| 0usize);

    use_context_provider(|| ContextMenuContext {
        is_open,
        menu_position,
        focused_index,
    });

    rsx! {
        div {
            style: "display: inline-block;",
            {props.children}
        }
    }
}

/// Context for sharing state between context menu components
#[derive(Clone, Copy)]
struct ContextMenuContext {
    is_open: Signal<bool>,
    menu_position: Signal<(i32, i32)>,
    focused_index: Signal<usize>,
}

/// Context Menu Trigger properties
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuTriggerProps {
    /// Child element that triggers the context menu on right-click
    pub children: Element,
}

/// Context menu trigger component
///
/// Wraps the element that will trigger the context menu on right-click.
#[component]
pub fn ContextMenuTrigger(props: ContextMenuTriggerProps) -> Element {
    let mut ctx: ContextMenuContext = use_context();

    let handle_context_menu = move |event: Event<MouseData>| {
        event.prevent_default();

        // Get click coordinates
        let coords = event.data().page_coordinates();
        let click_x = coords.x as i32;
        let click_y = coords.y as i32;

        // Basic padding to ensure menu is not at the very edge
        let padding = 8;

        // Position with basic bounds checking
        let menu_x = click_x.max(padding);
        let menu_y = click_y.max(padding);

        ctx.menu_position.set((menu_x, menu_y));
        ctx.is_open.set(true);
        ctx.focused_index.set(0);
    };

    rsx! {
        div {
            oncontextmenu: handle_context_menu,
            {props.children}
        }
    }
}

/// Context Menu Content properties
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuContentProps {
    /// Menu items and content
    pub children: Element,
}

/// Context menu content component
///
/// The actual menu that appears on right-click, positioned at cursor location.
#[component]
pub fn ContextMenuContent(props: ContextMenuContentProps) -> Element {
    let _theme = use_theme();
    let mut ctx: ContextMenuContext = use_context();

    let menu_base_style = use_style(|t| {
        Style::new()
            .rounded(&t.radius, "md")
            .border(1, &t.colors.border)
            .bg(&t.colors.popover)
            .shadow(&t.shadows.lg)
            .flex()
            .flex_col()
            .py(&t.spacing, "xs")
            .min_w_px(160)
            .z_index(9999)
            .outline("none")
            .build()
    });

    let menu_x = ctx.menu_position.read().0;
    let menu_y = ctx.menu_position.read().1;
    let position_style = format!("position: fixed; left: {}px; top: {}px;", menu_x, menu_y);

    // Handle keyboard navigation
    let handle_keydown = move |event: Event<KeyboardData>| {
        use dioxus::html::input_data::keyboard_types::Key;
        let key = event.key();
        if key == Key::Escape {
            ctx.is_open.set(false);
        } else if key == Key::ArrowDown {
            event.prevent_default();
            ctx.focused_index.with_mut(|i| *i = i.saturating_add(1));
        } else if key == Key::ArrowUp {
            event.prevent_default();
            ctx.focused_index.with_mut(|i| *i = i.saturating_sub(1));
        }
    };

    // Close when clicking outside
    let handle_overlay_click = move |_| {
        ctx.is_open.set(false);
    };

    rsx! {
        if *ctx.is_open.read() {
            // Overlay for outside click
            div {
                style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; z-index: 9998;",
                onclick: handle_overlay_click,
            }

            // Menu content
            div {
                style: "{menu_base_style} {position_style}",
                tabindex: "0",
                role: "menu",
                onkeydown: handle_keydown,
                onclick: move |e| e.stop_propagation(),
                {props.children}
            }
        }
    }
}

/// Context Menu Item properties
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuItemProps {
    /// Item content (typically text)
    pub children: Element,
    /// Callback when item is clicked
    #[props(default)]
    pub on_click: Option<EventHandler<()>>,
    /// Whether the item is disabled
    #[props(default = false)]
    pub disabled: bool,
    /// Optional keyboard shortcut display
    #[props(default)]
    pub shortcut: Option<String>,
    /// Optional icon element
    #[props(default)]
    pub icon: Option<Element>,
}

/// Context menu item component
///
/// A clickable item within the context menu.
#[component]
pub fn ContextMenuItem(props: ContextMenuItemProps) -> Element {
    let _theme = use_theme();
    let mut is_hovered = use_signal(|| false);
    let mut ctx: ContextMenuContext = use_context();

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
            .cursor(if props.disabled {
                "not-allowed"
            } else {
                "pointer"
            })
            .opacity(if props.disabled { 0.5 } else { 1.0 })
            .outline("none");

        if is_hovered() && !props.disabled {
            base.bg(&t.colors.accent).build()
        } else {
            base.build()
        }
    });

    let handle_click = move |_| {
        if !props.disabled {
            if let Some(ref handler) = props.on_click {
                handler.call(());
            }
            ctx.is_open.set(false);
        }
    };

    let shortcut_style = use_style(|t| {
        Style::new()
            .text(&t.typography, "xs")
            .text_color(&t.colors.muted_foreground)
            .build()
    });

    rsx! {
        div {
            style: "{item_style} user-select: none;",
            role: "menuitem",
            aria_disabled: props.disabled,
            onmouseenter: move |_| is_hovered.set(true),
            onmouseleave: move |_| is_hovered.set(false),
            onclick: handle_click,

            // Left section: icon and content
            div {
                style: "display: flex; align-items: center; gap: 8px;",
                if let Some(icon) = props.icon.clone() {
                    {icon}
                }
                {props.children}
            }

            // Right section: shortcut
            if let Some(shortcut) = &props.shortcut {
                span {
                    style: "{shortcut_style} margin-left: auto;",
                    "{shortcut}"
                }
            }
        }
    }
}

/// Context menu separator component
///
/// A horizontal divider between menu items.
#[component]
pub fn ContextMenuSeparator() -> Element {
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
            role: "separator",
        }
    }
}

/// Context Menu Label properties
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuLabelProps {
    /// Label text content
    pub children: Element,
}

/// Context menu label component
///
/// A non-clickable label for grouping menu items.
#[component]
pub fn ContextMenuLabel(props: ContextMenuLabelProps) -> Element {
    let _theme = use_theme();

    let label_style = use_style(|t| {
        Style::new()
            .px(&t.spacing, "sm")
            .py(&t.spacing, "xs")
            .text(&t.typography, "xs")
            .font_weight(500)
            .text_color(&t.colors.muted_foreground)
            .build()
    });

    rsx! {
        div {
            style: "{label_style} user-select: none;",
            {props.children}
        }
    }
}

/// Context Menu Checkbox Item properties
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuCheckboxItemProps {
    /// Item label
    pub children: Element,
    /// Whether the checkbox is checked
    #[props(default = false)]
    pub checked: bool,
    /// Callback when checked state changes
    #[props(default)]
    pub on_checked_change: Option<EventHandler<bool>>,
    /// Whether the item is disabled
    #[props(default = false)]
    pub disabled: bool,
    /// Optional keyboard shortcut display
    #[props(default)]
    pub shortcut: Option<String>,
}

/// Context menu checkbox item component
///
/// An item with a checkbox that can be toggled.
#[component]
pub fn ContextMenuCheckboxItem(props: ContextMenuCheckboxItemProps) -> Element {
    let _theme = use_theme();
    let mut is_hovered = use_signal(|| false);
    let _ctx: ContextMenuContext = use_context();

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
            .cursor(if props.disabled {
                "not-allowed"
            } else {
                "pointer"
            })
            .opacity(if props.disabled { 0.5 } else { 1.0 })
            .outline("none");

        if is_hovered() && !props.disabled {
            base.bg(&t.colors.accent).build()
        } else {
            base.build()
        }
    });

    let handle_click = move |_| {
        if !props.disabled {
            if let Some(ref handler) = props.on_checked_change {
                handler.call(!props.checked);
            }
        }
    };

    let checkbox_style = use_style(|t| {
        Style::new()
            .w_px(16)
            .h_px(16)
            .flex()
            .items_center()
            .justify_center()
            .rounded(&t.radius, "sm")
            .border(1, &t.colors.border)
            .build()
    });

    let check_icon_style = use_style(|t| {
        Style::new()
            .text(&t.typography, "xs")
            .text_color(&t.colors.primary)
            .build()
    });

    let shortcut_style = use_style(|t| {
        Style::new()
            .text(&t.typography, "xs")
            .text_color(&t.colors.muted_foreground)
            .build()
    });

    rsx! {
        div {
            style: "{item_style} user-select: none;",
            role: "menuitemcheckbox",
            aria_checked: props.checked,
            aria_disabled: props.disabled,
            onmouseenter: move |_| is_hovered.set(true),
            onmouseleave: move |_| is_hovered.set(false),
            onclick: handle_click,

            // Left section: checkbox and content
            div {
                style: "display: flex; align-items: center;",

                // Checkbox indicator
                div {
                    style: "{checkbox_style} margin-right: 8px;",
                    if props.checked {
                        span {
                            style: "{check_icon_style}",
                            "✓"
                        }
                    }
                }

                {props.children}
            }

            // Right section: shortcut
            if let Some(shortcut) = &props.shortcut {
                span {
                    style: "{shortcut_style} margin-left: auto;",
                    "{shortcut}"
                }
            }
        }
    }
}

/// Context Menu Submenu properties
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuSubProps {
    /// Trigger element that opens the submenu
    pub trigger: Element,
    /// Submenu content
    pub children: Element,
}

/// Context menu submenu component
///
/// A nested menu that appears when hovering over a parent item.
#[component]
pub fn ContextMenuSub(props: ContextMenuSubProps) -> Element {
    let _theme = use_theme();
    let mut is_open = use_signal(|| false);
    let mut menu_position = use_signal(|| (0i32, 0i32));

    let submenu_style = use_style(|t| {
        Style::new()
            .rounded(&t.radius, "md")
            .border(1, &t.colors.border)
            .bg(&t.colors.popover)
            .shadow(&t.shadows.lg)
            .flex()
            .flex_col()
            .py(&t.spacing, "xs")
            .min_w_px(160)
            .z_index(10000)
            .outline("none")
            .build()
    });

    rsx! {
        div {
            style: "position: relative;",

            // Trigger with hover handling
            div {
                onmouseenter: move |e: Event<MouseData>| {
                    let coords = e.data().page_coordinates();
                    menu_position.set((coords.x as i32 + 160, coords.y as i32));
                    is_open.set(true);
                },
                {props.trigger}
            }

            // Submenu content
            if is_open() {
                div {
                    onmouseleave: move |_| is_open.set(false),

                    // Submenu overlay
                    div {
                        style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; z-index: 9997;",
                        onmouseenter: move |_| is_open.set(false),
                    }

                    // Submenu
                    div {
                        style: "{submenu_style} position: fixed; left: {menu_position.read().0}px; top: {menu_position.read().1}px;",
                        role: "menu",
                        onclick: move |e| e.stop_propagation(),
                        {props.children}
                    }
                }
            }
        }
    }
}

/// Context Menu Sub Trigger properties
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuSubTriggerProps {
    /// Trigger content
    pub children: Element,
    /// Whether the item is disabled
    #[props(default = false)]
    pub disabled: bool,
    /// Optional keyboard shortcut display
    #[props(default)]
    pub shortcut: Option<String>,
}

/// Context menu sub trigger component
///
/// An item that opens a submenu on hover.
#[component]
pub fn ContextMenuSubTrigger(props: ContextMenuSubTriggerProps) -> Element {
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
            .cursor(if props.disabled {
                "not-allowed"
            } else {
                "pointer"
            })
            .opacity(if props.disabled { 0.5 } else { 1.0 })
            .outline("none");

        if is_hovered() && !props.disabled {
            base.bg(&t.colors.accent).build()
        } else {
            base.build()
        }
    });

    let shortcut_style = use_style(|t| {
        Style::new()
            .text(&t.typography, "xs")
            .text_color(&t.colors.muted_foreground)
            .flex()
            .items_center()
            .gap(&t.spacing, "xs")
            .build()
    });

    rsx! {
        div {
            style: "{item_style} user-select: none;",
            role: "menuitem",
            aria_disabled: props.disabled,
            aria_haspopup: "menu",
            onmouseenter: move |_| is_hovered.set(true),
            onmouseleave: move |_| is_hovered.set(false),

            // Content
            div {
                style: "display: flex; align-items: center; gap: 8px;",
                {props.children}
            }

            // Shortcut and chevron
            div {
                style: "{shortcut_style} margin-left: auto;",
                if let Some(shortcut) = &props.shortcut {
                    span { "{shortcut}" }
                }
                span { "›" }
            }
        }
    }
}
