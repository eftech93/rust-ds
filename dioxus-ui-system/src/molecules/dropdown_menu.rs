//! Dropdown Menu molecule component
//!
//! Displays a menu to the user—such as a set of actions or functions—triggered by a button.
//! Uses fixed positioning with portal-like behavior to escape parent overflow clipping.

use crate::a11y::next_enabled_index;
use crate::a11y::{focus_element, use_unique_id, Orientation};
use crate::atoms::{Icon, IconColor, IconSize};
use crate::styles::Style;
use crate::theme::{use_style, use_theme};
use dioxus::prelude::*;

/// Menu width in pixels (also used for alignment calculations).
const MENU_WIDTH: i32 = 180;
/// Minimum distance between the menu and the viewport edge.
const VIEWPORT_PADDING: i32 = 8;

/// Fixed-position `(x, y)` for a menu opened from the keyboard, computed from
/// the trigger's client rect (`left`, `width`, `bottom`).
fn keyboard_menu_position(align: &DropdownAlign, left: f64, width: f64, bottom: f64) -> (i32, i32) {
    let x = match align {
        DropdownAlign::Start => left,
        DropdownAlign::End => left + width - MENU_WIDTH as f64,
        DropdownAlign::Center => left + width / 2.0 - MENU_WIDTH as f64 / 2.0,
    };
    (
        (x as i32).max(VIEWPORT_PADDING),
        (bottom as i32 + 4).max(VIEWPORT_PADDING),
    )
}

/// Item index to focus when the menu opens: first enabled item, or the last
/// one when opened with `ArrowUp`.
fn initial_item_index(disabled: &[bool], from_end: bool) -> Option<usize> {
    let key = if from_end { Key::End } else { Key::Home };
    next_enabled_index(&key, 0, disabled, Orientation::Vertical)
}

/// Run a JavaScript snippet against the trigger: `w` is the trigger wrapper and
/// `t` its first focusable descendant (or the wrapper itself).
fn with_trigger(wrapper_id: &str, action: &str) {
    let _ = document::eval(&format!(
        "requestAnimationFrame(() => {{ const w = document.getElementById({wrapper_id:?}); \
         if (!w) return; \
         const t = w.querySelector('button,a[href],input,select,textarea,[tabindex]') || w; \
         {action} }});"
    ));
}

fn is_space(key: &Key) -> bool {
    matches!(key, Key::Character(c) if c == " ")
}

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
    #[props(default)]
    pub on_select: Option<EventHandler<String>>,
    /// Menu alignment
    #[props(default)]
    pub align: DropdownAlign,
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
///
/// # Accessibility
///
/// Follows the WAI-ARIA menu button pattern:
/// - The trigger wrapper (and, at runtime, the trigger's first focusable element)
///   gets `aria-haspopup="menu"`, `aria-expanded` and `aria-controls`.
/// - The popup has `role="menu"` (labelled by the trigger); items have
///   `role="menuitem"` and `aria-disabled="true"` when disabled.
/// - `Enter`, `Space` or `ArrowDown` on the trigger opens the menu and focuses the
///   first enabled item (`ArrowUp` focuses the last one).
/// - In the menu, `ArrowUp`/`ArrowDown` (wrapping), `Home` and `End` move focus,
///   skipping disabled items; `Enter`/`Space` selects; `Escape` closes and returns
///   focus to the trigger; `Tab` closes the menu.
/// - Element ids derive from the `id` prop when set, otherwise a generated id.
#[component]
pub fn DropdownMenu(props: DropdownMenuProps) -> Element {
    let _theme = use_theme();
    let mut is_open = use_signal(|| false);
    let mut menu_position = use_signal(|| (0i32, 0i32));
    let mut active = use_signal(|| None::<usize>);
    let mut trigger_el = use_signal(|| None::<MountedEvent>);

    let generated_id = use_unique_id("dropdown-menu");
    let base_id = props.id.clone().unwrap_or(generated_id);
    let trigger_id = format!("{base_id}-trigger");
    let menu_id = format!("{base_id}-content");
    let item_ids: Vec<String> = (0..props.items.len())
        .map(|i| format!("{base_id}-item-{i}"))
        .collect();
    let disabled: Vec<bool> = props.items.iter().map(|i| i.disabled).collect();

    let highlight_style = use_style(|t| Style::new().bg(&t.colors.accent).build());

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

    // Keep the trigger's ARIA state in sync (also on the inner focusable element,
    // which is what assistive tech announces) and move focus into the open menu.
    {
        let trigger_id = trigger_id.clone();
        let menu_id = menu_id.clone();
        let item_ids = item_ids.clone();
        use_effect(move || {
            let open = is_open();
            let current = active();
            with_trigger(
                &trigger_id,
                &format!(
                    "if (t !== w) {{ t.setAttribute('aria-haspopup', 'menu'); \
                     t.setAttribute('aria-expanded', '{open}'); \
                     t.setAttribute('aria-controls', {menu_id:?}); }}"
                ),
            );
            if open {
                match current.and_then(|i| item_ids.get(i)) {
                    Some(item_id) => focus_element(item_id),
                    None => focus_element(&menu_id),
                }
            }
        });
    }

    // Close the menu and move focus back to the trigger.
    let close = {
        let trigger_id = trigger_id.clone();
        use_callback(move |()| {
            is_open.set(false);
            active.set(None);
            with_trigger(&trigger_id, "t.focus();");
        })
    };

    // Store alignment for use in click handler
    let align = props.align.clone();
    let initial_disabled = disabled.clone();

    let handle_trigger_click = move |event: Event<MouseData>| {
        // Keyboard activation of a native button also fires `click` (with zeroed
        // coordinates); the keydown handler already handled it.
        let screen = event.data().screen_coordinates();
        let client = event.data().client_coordinates();
        if screen.x == 0.0 && screen.y == 0.0 && client.x == 0.0 && client.y == 0.0 {
            return;
        }
        if is_open() {
            close.call(());
            return;
        }
        {
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
        active.set(initial_item_index(&initial_disabled, false));
        is_open.set(true);
    };

    let key_align = props.align.clone();
    let key_disabled = disabled.clone();
    let handle_trigger_keydown = move |e: KeyboardEvent| {
        let key = e.key();
        let opens = matches!(key, Key::Enter | Key::ArrowDown | Key::ArrowUp) || is_space(&key);
        if !opens {
            return;
        }
        e.prevent_default();
        if is_open() {
            if !matches!(key, Key::ArrowDown | Key::ArrowUp) {
                close.call(());
            }
            return;
        }
        let from_end = key == Key::ArrowUp;
        let first = initial_item_index(&key_disabled, from_end);
        let align = key_align.clone();
        let mounted = trigger_el.cloned();
        spawn(async move {
            if let Some(el) = mounted {
                if let Ok(rect) = el.data().get_client_rect().await {
                    menu_position.set(keyboard_menu_position(
                        &align,
                        rect.origin.x,
                        rect.size.width,
                        rect.origin.y + rect.size.height,
                    ));
                }
            }
            active.set(first);
            is_open.set(true);
        });
    };

    let items = props.items.clone();
    let on_select = props.on_select;
    let menu_disabled = disabled.clone();
    let handle_menu_keydown = move |e: KeyboardEvent| {
        let key = e.key();
        if key == Key::Escape {
            e.prevent_default();
            e.stop_propagation();
            close.call(());
        } else if key == Key::Tab {
            is_open.set(false);
            active.set(None);
        } else if key == Key::Enter || is_space(&key) {
            e.prevent_default();
            if let Some(item) = active().and_then(|i| items.get(i)) {
                if !item.disabled {
                    if let Some(handler) = &on_select {
                        handler.call(item.value.clone());
                    }
                    close.call(());
                }
            }
        } else {
            // With nothing focused yet, ArrowDown starts at the top and ArrowUp at the bottom.
            let key = match (active(), key) {
                (None, Key::ArrowDown) => Key::Home,
                (None, Key::ArrowUp) => Key::End,
                (_, key) => key,
            };
            let current = active().unwrap_or(0);
            if let Some(next) =
                next_enabled_index(&key, current, &menu_disabled, Orientation::Vertical)
            {
                e.prevent_default();
                active.set(Some(next));
            }
        }
    };

    let (menu_x, menu_y) = menu_position();
    let position_style = format!(
        "position: fixed; left: {}px; top: {}px; width: {MENU_WIDTH}px;",
        menu_x, menu_y
    );
    let custom_style = props.style.clone().unwrap_or_default();
    let expanded = if is_open() { "true" } else { "false" };
    let current = active();
    let highlights: Vec<String> = disabled
        .iter()
        .enumerate()
        .map(|(i, &off)| {
            if current == Some(i) && !off {
                highlight_style()
            } else {
                String::new()
            }
        })
        .collect();

    rsx! {
        div {
            id: "{props.id.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            style: "position: relative; display: inline-block;",

            // Trigger
            div {
                id: "{trigger_id}",
                aria_haspopup: "menu",
                aria_expanded: expanded,
                aria_controls: "{menu_id}",
                onclick: handle_trigger_click,
                onkeydown: handle_trigger_keydown,
                onmounted: move |e| trigger_el.set(Some(e)),
                {props.trigger}
            }

            // Overlay to close on outside click
            if is_open() {
                div {
                    style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; z-index: 9998;",
                    onclick: move |_| close.call(()),
                }
            }

            // Menu - rendered with fixed positioning to escape clipping
            if is_open() {
                div {
                    id: "{menu_id}",
                    role: "menu",
                    aria_labelledby: "{trigger_id}",
                    aria_orientation: "vertical",
                    tabindex: "-1",
                    style: "{menu_base_style} {position_style} outline: none; {custom_style}",
                    onclick: move |e| e.stop_propagation(),
                    onkeydown: handle_menu_keydown,

                    for (index, item) in props.items.iter().cloned().enumerate() {
                        DropdownMenuItemView {
                            key: "{item.value}",
                            id: Some(item_ids[index].clone()),
                            highlight: highlights[index].clone(),
                            item: item.clone(),
                            on_select: props.on_select,
                            on_close: move |_| close.call(()),
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
    #[props(default)]
    on_select: Option<EventHandler<String>>,
    #[props(default)]
    on_close: Option<EventHandler<()>>,
    /// Extra style for the keyboard-active item (empty when not active)
    #[props(default)]
    highlight: String,
    /// Element id
    #[props(default)]
    pub id: Option<String>,
    /// Custom CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// A single menu entry: `role="menuitem"`, `tabindex="-1"` (focus is managed by
/// the menu) and `aria-disabled="true"` when disabled.
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
    let on_select = props.on_select;
    let on_close = props.on_close;
    let aria_disabled = props.item.disabled.then_some("true");

    rsx! {
        div {
            id: "{props.id.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            role: "menuitem",
            tabindex: "-1",
            aria_disabled: aria_disabled,
            style: "{item_style} {props.highlight}",
            onmouseenter: move |_| is_hovered.set(true),
            onmouseleave: move |_| is_hovered.set(false),
            onclick: move |_| {
                if !props.item.disabled {
                    if let Some(handler) = &on_select {
                        handler.call(value.clone());
                    }
                    if let Some(handler) = &on_close {
                        handler.call(());
                    }
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
                    style: "font-size: 12px; color: rgb(148,163,184); margin-left: 24px; {props.style.clone().unwrap_or_default()}",
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
            role: "separator",
            style: "{separator_style}",
        }
    }
}

/// Dropdown menu label
#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuLabelProps {
    pub children: Element,
    /// Element id
    #[props(default)]
    pub id: Option<String>,
    /// Custom CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
}

#[component]
pub fn DropdownMenuLabel(props: DropdownMenuLabelProps) -> Element {
    rsx! {
        div {
            id: "{props.id.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            style: "padding: 6px 8px; font-size: 12px; font-weight: 500; color: #64748b; {props.style.clone().unwrap_or_default()}",
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
    fn closed_trigger_exposes_menu_button_semantics() {
        let html = render(|| {
            rsx! {
                DropdownMenu {
                    id: "dd",
                    trigger: rsx! { button { "Open" } },
                    items: vec![DropdownMenuItem::new("a", "A")],
                }
            }
        });
        assert!(html.contains("id=\"dd-trigger\""), "{html}");
        assert!(html.contains("aria-haspopup=\"menu\""), "{html}");
        assert!(html.contains("aria-expanded=\"false\""), "{html}");
        assert!(html.contains("aria-controls=\"dd-content\""), "{html}");
        assert!(!html.contains("role=\"menu\""), "{html}");
    }

    #[test]
    fn items_have_menuitem_role_and_disabled_state() {
        let html = render(|| {
            rsx! {
                DropdownMenuItemView { item: DropdownMenuItem::new("a", "A"), id: "i0" }
                DropdownMenuItemView { item: DropdownMenuItem::new("b", "B").disabled(), id: "i1" }
            }
        });
        assert_eq!(html.matches("role=\"menuitem\"").count(), 2, "{html}");
        assert_eq!(html.matches("tabindex=\"-1\"").count(), 2, "{html}");
        assert_eq!(html.matches("aria-disabled=\"true\"").count(), 1, "{html}");
        assert!(html.contains("id=\"i1\""), "{html}");
    }

    #[test]
    fn initial_item_skips_disabled() {
        assert_eq!(initial_item_index(&[true, false, false], false), Some(1));
        assert_eq!(initial_item_index(&[false, false, true], true), Some(1));
        assert_eq!(initial_item_index(&[true, true], false), None);
        assert_eq!(initial_item_index(&[], false), None);
    }

    #[test]
    fn keyboard_position_follows_alignment_and_clamps() {
        assert_eq!(
            keyboard_menu_position(&DropdownAlign::Start, 100.0, 80.0, 40.0),
            (100, 44)
        );
        assert_eq!(
            keyboard_menu_position(&DropdownAlign::End, 300.0, 80.0, 40.0),
            (200, 44)
        );
        assert_eq!(
            keyboard_menu_position(&DropdownAlign::Center, 200.0, 80.0, 40.0),
            (150, 44)
        );
        assert_eq!(
            keyboard_menu_position(&DropdownAlign::End, 10.0, 80.0, 0.0),
            (VIEWPORT_PADDING, VIEWPORT_PADDING)
        );
    }

    #[test]
    fn space_detection() {
        assert!(is_space(&Key::Character(" ".to_string())));
        assert!(!is_space(&Key::Character("a".to_string())));
        assert!(!is_space(&Key::Enter));
    }
}
