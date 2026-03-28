//! Menubar organism component
//!
//! An application menu bar (File, Edit, View, etc.) with nested menus.
//! Uses a compound component pattern for flexibility.

use crate::styles::Style;
use crate::theme::{use_style, use_theme};
use dioxus::prelude::*;

// ============================================================================
// Context
// ============================================================================

/// Shared context for menubar state
#[derive(Clone, Copy)]
struct MenubarContext {
    /// Currently active/open menu index (if any)
    active_menu: Signal<Option<usize>>,
    /// Whether the menubar is currently "open" mode (a menu is active)
    is_open: Signal<bool>,
    /// Number of menus in the bar
    menu_count: Signal<usize>,
    /// Whether to loop navigation
    loop_navigation: Signal<bool>,
}

/// Context for individual menu state
#[derive(Clone, Copy)]
struct MenuContext {
    /// Index of this menu
    index: usize,
}

/// Context for submenu state
#[derive(Clone, Copy)]
struct SubmenuContext {
    /// Whether this submenu is open
    is_open: Signal<bool>,
}

// ============================================================================
// Menubar
// ============================================================================

/// Menubar properties
#[derive(Props, Clone, PartialEq)]
pub struct MenubarProps {
    /// Child elements (MenubarMenu components)
    pub children: Element,
    /// Whether to loop navigation (cycle through menus), default: true
    #[props(default = true)]
    pub loop_navigation: bool,
}

/// Menubar container component
///
/// The root component that provides context for all menu components.
/// Renders a horizontal bar of menu triggers.
///
/// # Example
/// ```rust,ignore
/// Menubar {
///     loop_navigation: true,
///     MenubarMenu {
///         MenubarTrigger { label: "File" }
///         MenubarContent {
///             MenubarItem { "New", on_click: move |_| {} }
///             MenubarItem { "Open", on_click: move |_| {} }
///             MenubarSeparator {}
///             MenubarItem { "Exit", on_click: move |_| {} }
///         }
///     }
/// }
/// ```
#[component]
pub fn Menubar(props: MenubarProps) -> Element {
    let _theme = use_theme();

    let active_menu = use_signal(|| None);
    let is_open = use_signal(|| false);
    let menu_count = use_signal(|| 0);
    let loop_navigation = use_signal(|| props.loop_navigation);

    use_context_provider(|| MenubarContext {
        active_menu,
        is_open,
        menu_count,
        loop_navigation,
    });

    let bar_style = use_style(|t| {
        Style::new()
            .flex()
            .items_center()
            .gap(&t.spacing, "xs")
            .px(&t.spacing, "sm")
            .py(&t.spacing, "xs")
            .bg(&t.colors.background)
            .border_bottom(1, &t.colors.border)
            .w_full()
            .build()
    });

    rsx! {
        div {
            style: "{bar_style}",
            role: "menubar",
            {props.children}
        }
    }
}

// ============================================================================
// MenubarMenu
// ============================================================================

/// MenubarMenu properties
#[derive(Props, Clone, PartialEq)]
pub struct MenubarMenuProps {
    /// Child elements (MenubarTrigger and MenubarContent)
    pub children: Element,
}

/// Menubar menu component
///
/// Wraps a single top-level menu (trigger + content).
/// Manages the active state for this specific menu.
#[component]
pub fn MenubarMenu(props: MenubarMenuProps) -> Element {
    let ctx: MenubarContext = use_context();

    // Get the index for this menu
    let mut count_sig = ctx.menu_count;
    let index = *count_sig.read();
    count_sig.set(index + 1);

    use_context_provider(|| MenuContext { index });

    rsx! {
        div {
            style: "position: relative; display: inline-block;",
            role: "none",
            {props.children}
        }
    }
}

// ============================================================================
// MenubarTrigger
// ============================================================================

/// MenubarTrigger properties
#[derive(Props, Clone, PartialEq)]
pub struct MenubarTriggerProps {
    /// The label text for this menu
    pub label: String,
    /// Whether this trigger is disabled
    #[props(default = false)]
    pub disabled: bool,
}

/// Menubar trigger component
///
/// The clickable label that opens a menu.
/// Click to toggle, or hover when another menu is open.
#[component]
pub fn MenubarTrigger(props: MenubarTriggerProps) -> Element {
    let _theme = use_theme();
    let menubar_ctx: MenubarContext = use_context();
    let menu_ctx: MenuContext = use_context();
    let mut is_hovered = use_signal(|| false);

    // Check if this menu is active
    let is_active = (*menubar_ctx.active_menu.read()) == Some(menu_ctx.index);

    let trigger_style = use_style(move |t| {
        let base = Style::new()
            .px(&t.spacing, "sm")
            .py(&t.spacing, "xs")
            .rounded(&t.radius, "sm")
            .text(&t.typography, "sm")
            .font_weight(500)
            .cursor(if props.disabled {
                "not-allowed"
            } else {
                "pointer"
            })
            .opacity(if props.disabled { 0.5 } else { 1.0 })
            .select_none();

        let hovered = *is_hovered.read();

        if is_active {
            base.bg(&t.colors.accent)
                .text_color(&t.colors.accent_foreground)
                .build()
        } else if hovered && !props.disabled {
            base.bg(&t.colors.muted)
                .text_color(&t.colors.muted_foreground)
                .build()
        } else {
            base.text_color(&t.colors.foreground).build()
        }
    });

    // Clone signals for use in closures (signals are Copy)
    let mut active_menu_sig = menubar_ctx.active_menu;
    let mut is_open_sig = menubar_ctx.is_open;
    let menu_idx = menu_ctx.index;
    let is_open_val = *is_open_sig.read();

    let handle_click = move |_| {
        if props.disabled {
            return;
        }

        if is_active {
            // Close this menu
            active_menu_sig.set(None);
            is_open_sig.set(false);
        } else {
            // Open this menu
            active_menu_sig.set(Some(menu_idx));
            is_open_sig.set(true);
        }
    };

    let handle_mouse_enter = move |_| {
        if props.disabled {
            return;
        }

        is_hovered.set(true);

        // If menubar is in "open" mode, switch to this menu
        if is_open_val && !is_active {
            active_menu_sig.set(Some(menu_idx));
        }
    };

    let handle_mouse_leave = move |_| {
        is_hovered.set(false);
    };

    rsx! {
        button {
            style: "{trigger_style} border: none; background: transparent;",
            role: "menuitem",
            aria_haspopup: "true",
            aria_expanded: is_active,
            disabled: props.disabled,
            onmouseenter: handle_mouse_enter,
            onmouseleave: handle_mouse_leave,
            onclick: handle_click,
            "{props.label}"
        }
    }
}

// ============================================================================
// MenubarContent
// ============================================================================

/// MenubarContent properties
#[derive(Props, Clone, PartialEq)]
pub struct MenubarContentProps {
    /// Menu items and content
    pub children: Element,
    /// Optional alignment
    #[props(default)]
    pub align: MenubarAlign,
}

/// Menubar content alignment
#[derive(Default, Clone, PartialEq)]
pub enum MenubarAlign {
    /// Align to start (left)
    #[default]
    Start,
    /// Align to end (right)
    End,
    /// Center alignment
    Center,
}

/// Menubar content component
///
/// The dropdown menu content that appears below the trigger.
#[component]
pub fn MenubarContent(props: MenubarContentProps) -> Element {
    let _theme = use_theme();
    let menubar_ctx: MenubarContext = use_context();
    let menu_ctx: MenuContext = use_context();

    // Check if this menu is active
    let is_open = (*menubar_ctx.active_menu.read()) == Some(menu_ctx.index);

    let content_style = use_style(|t| {
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

    // Clone signals for use in closures
    let mut active_menu_sig = menubar_ctx.active_menu;
    let mut is_open_sig = menubar_ctx.is_open;
    let loop_nav_sig = menubar_ctx.loop_navigation;
    let menu_count_sig = menubar_ctx.menu_count;
    let current_idx = menu_ctx.index;

    // Handle keyboard navigation
    let handle_keydown = move |event: Event<KeyboardData>| {
        use dioxus::html::input_data::keyboard_types::Key;
        let key = event.key();
        let loop_nav = *loop_nav_sig.read();
        let count = *menu_count_sig.read();

        if key == Key::Escape {
            event.stop_propagation();
            active_menu_sig.set(None);
            is_open_sig.set(false);
        } else if key == Key::ArrowLeft {
            event.prevent_default();

            let new_index = if current_idx == 0 {
                if loop_nav {
                    count.saturating_sub(1)
                } else {
                    0
                }
            } else {
                current_idx.saturating_sub(1)
            };

            if new_index != current_idx {
                active_menu_sig.set(Some(new_index));
            }
        } else if key == Key::ArrowRight {
            event.prevent_default();

            let new_index = if current_idx >= count.saturating_sub(1) {
                if loop_nav {
                    0
                } else {
                    current_idx
                }
            } else {
                current_idx + 1
            };

            if new_index != current_idx {
                active_menu_sig.set(Some(new_index));
            }
        }
    };

    // Close when clicking outside
    let handle_overlay_click = move |_| {
        active_menu_sig.set(None);
        is_open_sig.set(false);
    };

    let align_offset = match props.align {
        MenubarAlign::Start => 0,
        MenubarAlign::End => -80,
        MenubarAlign::Center => -80,
    };

    let menu_y = 8i32; // Offset below trigger

    rsx! {
        if is_open {
            // Overlay for outside click
            div {
                style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; z-index: 9998;",
                onclick: handle_overlay_click,
            }

            // Menu content
            div {
                style: "{content_style} position: absolute; top: 100%; left: {align_offset}px; margin-top: {menu_y}px;",
                role: "menu",
                tabindex: "0",
                onkeydown: handle_keydown,
                onclick: move |e| e.stop_propagation(),
                {props.children}
            }
        }
    }
}

// ============================================================================
// MenubarItem
// ============================================================================

/// MenubarItem properties
#[derive(Props, Clone, PartialEq)]
pub struct MenubarItemProps {
    /// Item content
    pub children: Element,
    /// Callback when item is clicked
    pub on_click: EventHandler<()>,
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

/// Menubar item component
///
/// A clickable item within the menu.
#[component]
pub fn MenubarItem(props: MenubarItemProps) -> Element {
    let _theme = use_theme();
    let menubar_ctx: MenubarContext = use_context();
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
            base.bg(&t.colors.accent)
                .text_color(&t.colors.accent_foreground)
                .build()
        } else {
            base.text_color(&t.colors.foreground).build()
        }
    });

    // Clone signals for use in closures
    let mut active_menu_sig = menubar_ctx.active_menu;
    let mut is_open_sig = menubar_ctx.is_open;

    let handle_click = move |_| {
        if !props.disabled {
            props.on_click.call(());
            // Close the menu
            active_menu_sig.set(None);
            is_open_sig.set(false);
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

// ============================================================================
// MenubarSeparator
// ============================================================================

/// Menubar separator component
///
/// A horizontal divider between menu items.
#[component]
pub fn MenubarSeparator() -> Element {
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

// ============================================================================
// MenubarSub (Nested submenu)
// ============================================================================

/// MenubarSub properties
#[derive(Props, Clone, PartialEq)]
pub struct MenubarSubProps {
    /// Child elements (MenubarSubTrigger and MenubarSubContent)
    pub children: Element,
}

/// Menubar submenu component
///
/// A nested menu within a menu item.
#[component]
pub fn MenubarSub(props: MenubarSubProps) -> Element {
    let is_open = use_signal(|| false);

    use_context_provider(|| SubmenuContext { is_open });

    rsx! {
        div {
            style: "position: relative;",
            role: "none",
            {props.children}
        }
    }
}

// ============================================================================
// MenubarSubTrigger
// ============================================================================

/// MenubarSubTrigger properties
#[derive(Props, Clone, PartialEq)]
pub struct MenubarSubTriggerProps {
    /// Trigger content
    pub children: Element,
    /// Whether the item is disabled
    #[props(default = false)]
    pub disabled: bool,
    /// Optional keyboard shortcut display
    #[props(default)]
    pub shortcut: Option<String>,
}

/// Menubar submenu trigger component
///
/// An item that opens a submenu on hover or click.
#[component]
pub fn MenubarSubTrigger(props: MenubarSubTriggerProps) -> Element {
    let _theme = use_theme();
    let mut submenu_ctx: SubmenuContext = use_context();
    let mut is_hovered = use_signal(|| false);

    let trigger_style = use_style(move |t| {
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
            base.bg(&t.colors.accent)
                .text_color(&t.colors.accent_foreground)
                .build()
        } else {
            base.text_color(&t.colors.foreground).build()
        }
    });

    let handle_mouse_enter = move |_| {
        if !props.disabled {
            is_hovered.set(true);
            submenu_ctx.is_open.set(true);
        }
    };

    let handle_mouse_leave = move |_| {
        is_hovered.set(false);
    };

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
            style: "{trigger_style} user-select: none;",
            role: "menuitem",
            aria_haspopup: "true",
            aria_expanded: *submenu_ctx.is_open.read(),
            aria_disabled: props.disabled,
            onmouseenter: handle_mouse_enter,
            onmouseleave: handle_mouse_leave,

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

// ============================================================================
// MenubarSubContent
// ============================================================================

/// MenubarSubContent properties
#[derive(Props, Clone, PartialEq)]
pub struct MenubarSubContentProps {
    /// Submenu content
    pub children: Element,
}

/// Menubar submenu content component
///
/// The content of a nested submenu, positioned to the right of the trigger.
#[component]
pub fn MenubarSubContent(props: MenubarSubContentProps) -> Element {
    let _theme = use_theme();
    let mut submenu_ctx: SubmenuContext = use_context();

    let is_open = *submenu_ctx.is_open.read();

    let content_style = use_style(|t| {
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

    let handle_mouse_leave = move |_| {
        submenu_ctx.is_open.set(false);
    };

    rsx! {
        if is_open {
            div {
                style: "position: relative;",
                onmouseleave: handle_mouse_leave,

                // Submenu overlay (to close when moving away)
                div {
                    style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; z-index: 9997;",
                    onmouseenter: move |_| submenu_ctx.is_open.set(false),
                }

                // Submenu content
                div {
                    style: "{content_style} position: absolute; left: 100%; top: 0; margin-left: 4px;",
                    role: "menu",
                    onclick: move |e| e.stop_propagation(),
                    {props.children}
                }
            }
        }
    }
}

// ============================================================================
// MenubarLabel
// ============================================================================

/// MenubarLabel properties
#[derive(Props, Clone, PartialEq)]
pub struct MenubarLabelProps {
    /// Label text content
    pub children: Element,
}

/// Menubar label component
///
/// A non-clickable label for grouping menu items.
#[component]
pub fn MenubarLabel(props: MenubarLabelProps) -> Element {
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

// ============================================================================
// MenubarCheckboxItem
// ============================================================================

/// MenubarCheckboxItem properties
#[derive(Props, Clone, PartialEq)]
pub struct MenubarCheckboxItemProps {
    /// Item label
    pub children: Element,
    /// Whether the checkbox is checked
    #[props(default = false)]
    pub checked: bool,
    /// Callback when checked state changes
    pub on_checked_change: EventHandler<bool>,
    /// Whether the item is disabled
    #[props(default = false)]
    pub disabled: bool,
    /// Optional keyboard shortcut display
    #[props(default)]
    pub shortcut: Option<String>,
}

/// Menubar checkbox item component
///
/// An item with a checkbox that can be toggled.
#[component]
pub fn MenubarCheckboxItem(props: MenubarCheckboxItemProps) -> Element {
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
            base.bg(&t.colors.accent)
                .text_color(&t.colors.accent_foreground)
                .build()
        } else {
            base.text_color(&t.colors.foreground).build()
        }
    });

    let handle_click = move |_| {
        if !props.disabled {
            props.on_checked_change.call(!props.checked);
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
            .mr(&t.spacing, "sm")
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
                    style: "{checkbox_style}",
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
