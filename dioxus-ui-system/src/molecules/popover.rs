//! Popover molecule component
//!
//! Displays rich content in a portal, triggered by a button.

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

/// Popover component
#[component]
pub fn Popover(props: PopoverProps) -> Element {
    let _theme = use_theme();
    let mut is_open = use_signal(|| props.open);

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

    rsx! {
        div {
            style: "position: relative; display: inline-block;",

            // Trigger
            div {
                onclick: move |_| toggle(),
                {props.trigger}
            }

            // Content
            if is_open() {
                // Overlay to close on outside click
                div {
                    style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; z-index: 40;",
                    onclick: move |_| {
                        is_open.set(false);
                        if let Some(handler) = &props.on_open_change {
                            handler.call(false);
                        }
                    },
                }

                div {
                    style: "{popover_style} {position_style} {custom_style}",
                    onclick: move |e: Event<MouseData>| e.stop_propagation(),

                    PopoverContent { children: props.children.clone() }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct PopoverContentProps {
    children: Element,
}

#[component]
fn PopoverContent(props: PopoverContentProps) -> Element {
    let _theme = use_theme();

    let content_style = use_style(|t| Style::new().p(&t.spacing, "md").build());

    rsx! {
        Box {
            style: "{content_style}",
            {props.children}
        }
    }
}

/// Popover header component
#[derive(Props, Clone, PartialEq)]
pub struct PopoverHeaderProps {
    /// Header title
    pub title: String,
    /// Optional description
    #[props(default)]
    pub description: Option<String>,
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
            style: "{header_style}",
            gap: SpacingSize::Xs,
            align: AlignItems::Stretch,

            h4 {
                style: "margin: 0; font-size: 16px; font-weight: 600;",
                "{props.title}"
            }

            if let Some(description) = props.description {
                p {
                    style: "margin: 0; font-size: 13px; color: #64748b;",
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
            style: "{footer_style}",
            justify: JustifyContent::End,
            align: AlignItems::Center,
            gap: SpacingSize::Sm,
            {props.children}
        }
    }
}
