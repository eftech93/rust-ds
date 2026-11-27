//! Popover molecule component
//!
//! Displays rich content in a portal, triggered by a button.

use super::dialog::{is_dismiss_key, resolve_base_id, FocusTrapScope};
use crate::a11y::use_unique_id;
use crate::atoms::{AlignItems, Box, HStack, JustifyContent, SpacingSize, VStack};
use crate::styles::Style;
use crate::theme::{use_style, use_theme};
use dioxus::prelude::*;

/// Popover properties
#[derive(Props, Clone, PartialEq)]
pub struct PopoverProps {
    /// Trigger element
    pub trigger: Element,
    /// Popover content
    pub children: Element,
    /// Whether the popover is open
    #[props(default)]
    pub open: bool,
    /// Callback when open state changes
    #[props(default)]
    pub on_open_change: Option<EventHandler<bool>>,
    /// Popover placement
    #[props(default)]
    pub placement: PopoverPlacement,
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

/// Popover placement options
#[derive(Default, Clone, PartialEq)]
pub enum PopoverPlacement {
    /// Top placement
    Top,
    /// Top-start placement
    TopStart,
    /// Top-end placement
    TopEnd,
    /// Right placement
    Right,
    /// Right-start placement
    RightStart,
    /// Right-end placement
    RightEnd,
    /// Bottom placement (default)
    #[default]
    Bottom,
    /// Bottom-start placement
    BottomStart,
    /// Bottom-end placement
    BottomEnd,
    /// Left placement
    Left,
    /// Left-start placement
    LeftStart,
    /// Left-end placement
    LeftEnd,
}

/// Close the popover and notify the owner.
fn close_popover(mut is_open: Signal<bool>, on_open_change: Option<EventHandler<bool>>) {
    is_open.set(false);
    if let Some(handler) = &on_open_change {
        handler.call(false);
    }
}

/// Popover component
///
/// # Accessibility
///
/// - The trigger wrapper has `aria-haspopup="dialog"`, `aria-expanded` and,
///   while open, `aria-controls` pointing at the content.
/// - The content has `role="dialog"` and is labelled by the trigger. Ids
///   derive from the `id` prop when set.
/// - Escape closes the popover (calling `on_open_change(false)`).
/// - While open, focus moves into the content and Tab / Shift+Tab cycle inside
///   it; on close (Escape, outside click or toggle) focus returns to the
///   trigger element that opened it.
#[component]
pub fn Popover(props: PopoverProps) -> Element {
    let _theme = use_theme();
    let mut is_open = use_signal(|| props.open);
    let auto_id = use_unique_id("popover");

    // Sync with props
    use_effect(move || {
        is_open.set(props.open);
    });

    let position_style = match props.placement {
        PopoverPlacement::Top => {
            "bottom: calc(100% + 8px); left: 50%; transform: translateX(-50%);"
        }
        PopoverPlacement::TopStart => "bottom: calc(100% + 8px); left: 0;",
        PopoverPlacement::TopEnd => "bottom: calc(100% + 8px); right: 0;",
        PopoverPlacement::Right => "left: calc(100% + 8px); top: 50%; transform: translateY(-50%);",
        PopoverPlacement::RightStart => "left: calc(100% + 8px); top: 0;",
        PopoverPlacement::RightEnd => "left: calc(100% + 8px); bottom: 0;",
        PopoverPlacement::Bottom => {
            "top: calc(100% + 8px); left: 50%; transform: translateX(-50%);"
        }
        PopoverPlacement::BottomStart => "top: calc(100% + 8px); left: 0;",
        PopoverPlacement::BottomEnd => "top: calc(100% + 8px); right: 0;",
        PopoverPlacement::Left => "right: calc(100% + 8px); top: 50%; transform: translateY(-50%);",
        PopoverPlacement::LeftStart => "right: calc(100% + 8px); top: 0;",
        PopoverPlacement::LeftEnd => "right: calc(100% + 8px); bottom: 0;",
    };

    let popover_style = use_style(|t| {
        Style::new()
            .absolute()
            .w_px(320)
            .rounded(&t.radius, "md")
            .border(1, &t.colors.border)
            .bg(&t.colors.popover)
            .shadow(&t.shadows.lg)
            .z_index(50)
            .build()
    });

    let mut toggle = move || {
        let new_open = !is_open();
        is_open.set(new_open);
        if let Some(handler) = &props.on_open_change {
            handler.call(new_open);
        }
    };

    let custom_style = props.style.clone().unwrap_or_default();

    let base_id = resolve_base_id(props.id.as_deref(), &auto_id);
    let trigger_id = format!("{base_id}-trigger");
    let content_id = format!("{base_id}-content");
    let open = is_open();
    let controls = open.then(|| content_id.clone());
    let on_open_change = props.on_open_change;

    let handle_keydown = move |e: KeyboardEvent| {
        if is_open() && is_dismiss_key(&e.key()) {
            e.stop_propagation();
            close_popover(is_open, on_open_change);
        }
    };

    rsx! {
        div {
            id: "{props.id.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            style: "position: relative; display: inline-block;",
            onkeydown: handle_keydown,

            // Trigger
            div {
                id: "{trigger_id}",
                "aria-haspopup": "dialog",
                "aria-expanded": "{open}",
                "aria-controls": controls,
                onclick: move |_| toggle(),
                {props.trigger}
            }

            // Content
            if open {
                // Overlay to close on outside click
                div {
                    style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; z-index: 40;",
                    onclick: move |_| close_popover(is_open, on_open_change),
                }

                div {
                    id: "{content_id}",
                    role: "dialog",
                    "aria-labelledby": "{trigger_id}",
                    tabindex: "-1",
                    style: "{popover_style} {position_style} {custom_style}",
                    onclick: move |e: Event<MouseData>| e.stop_propagation(),

                    FocusTrapScope { container_id: content_id.clone() }

                    PopoverContent { children: props.children.clone() }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct PopoverContentProps {
    children: Element,
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
fn PopoverContent(props: PopoverContentProps) -> Element {
    let _theme = use_theme();

    let content_style = use_style(|t| Style::new().p(&t.spacing, "md").build());

    rsx! {
        Box {
            id: "{props.id.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            style: "{content_style} {props.style.clone().unwrap_or_default()}",
            {props.children}
        }
    }
}

/// Popover header component
#[derive(Props, Clone, PartialEq)]
pub struct PopoverHeaderProps {
    /// Header title
    #[props(default)]
    pub title: Option<String>,
    /// Optional description
    #[props(default)]
    pub description: Option<String>,
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
pub fn PopoverHeader(props: PopoverHeaderProps) -> Element {
    let _theme = use_theme();

    let header_style = use_style(|t| {
        Style::new()
            .pb(&t.spacing, "md")
            .mb(&t.spacing, "md")
            .border_bottom(1, &t.colors.border)
            .build()
    });

    rsx! {
        VStack {
            id: "{props.id.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            style: "{header_style}",
            gap: SpacingSize::Xs,
            align: AlignItems::Stretch,

            if let Some(title) = props.title {
                h4 {
                    style: "margin: 0; font-size: 16px; font-weight: 600;",
                    "{title}"
                }
            }

            if let Some(description) = props.description {
                p {
                    style: "margin: 0; font-size: 13px; color: #64748b; {props.style.clone().unwrap_or_default()}",
                    "{description}"
                }
            }
        }
    }
}

/// Popover footer component
#[derive(Props, Clone, PartialEq)]
pub struct PopoverFooterProps {
    /// Footer content
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
pub fn PopoverFooter(props: PopoverFooterProps) -> Element {
    let _theme = use_theme();

    let footer_style = use_style(|t| {
        Style::new()
            .pt(&t.spacing, "md")
            .mt(&t.spacing, "md")
            .border_top(1, &t.colors.border)
            .build()
    });

    rsx! {
        HStack {
            id: "{props.id.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            style: "{footer_style} {props.style.clone().unwrap_or_default()}",
            justify: JustifyContent::End,
            align: AlignItems::Center,
            gap: SpacingSize::Sm,
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
    fn closed_popover_trigger_advertises_dialog() {
        let html = render(|| {
            rsx! {
                Popover {
                    id: "pop",
                    trigger: rsx! { button { "Open" } },
                    "Content"
                }
            }
        });
        assert!(html.contains("id=\"pop-trigger\""), "{html}");
        assert!(html.contains("aria-haspopup=\"dialog\""), "{html}");
        assert!(html.contains("aria-expanded=\"false\""), "{html}");
        assert!(!html.contains("aria-controls"), "{html}");
        assert!(!html.contains("role=\"dialog\""), "{html}");
    }

    #[test]
    fn open_popover_content_is_dialog_controlled_by_trigger() {
        let html = render(|| {
            rsx! {
                Popover {
                    id: "pop",
                    open: true,
                    trigger: rsx! { button { "Open" } },
                    "Content"
                }
            }
        });
        assert!(html.contains("aria-expanded=\"true\""), "{html}");
        assert!(html.contains("aria-controls=\"pop-content\""), "{html}");
        assert!(html.contains("id=\"pop-content\""), "{html}");
        assert!(html.contains("role=\"dialog\""), "{html}");
        assert!(html.contains("aria-labelledby=\"pop-trigger\""), "{html}");
    }
}
