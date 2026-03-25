//! Hover Card molecule component
//!
//! A card that appears when hovering over a trigger element.
//! Similar to GitHub's user preview cards.

use dioxus::prelude::*;
use crate::theme::{use_theme, use_style};
use crate::styles::Style;

/// Side options for hover card placement
#[derive(Default, Clone, PartialEq)]
pub enum Side {
    /// Top placement
    Top,
    /// Right placement
    Right,
    /// Bottom placement (default)
    #[default]
    Bottom,
    /// Left placement
    Left,
}

/// Alignment options for hover card
#[derive(Default, Clone, PartialEq)]
pub enum Align {
    /// Start alignment
    Start,
    /// Center alignment (default)
    #[default]
    Center,
    /// End alignment
    End,
}

/// Hover Card properties
#[derive(Props, Clone, PartialEq)]
pub struct HoverCardProps {
    /// Trigger element (the element that triggers the hover card)
    pub trigger: Element,
    /// Card content
    pub children: Element,
    /// Delay before showing the card (in ms)
    #[props(default = 200)]
    pub open_delay: u64,
    /// Delay before hiding the card (in ms)
    #[props(default = 100)]
    pub close_delay: u64,
    /// Card placement side
    #[props(default)]
    pub side: Side,
    /// Card alignment
    #[props(default)]
    pub align: Align,
    /// Custom inline styles for the card
    #[props(default)]
    pub style: Option<String>,
}

/// Hover Card component
///
/// A card that appears when hovering over a trigger element.
/// Features:
/// - Show on hover with configurable delay
/// - Hide on mouse leave with configurable delay
/// - Position relative to trigger (side + align)
/// - Arrow pointing to trigger
/// - Click outside to close
/// - Escape key to close
/// - Smooth fade animation
#[component]
pub fn HoverCard(props: HoverCardProps) -> Element {
    let _theme = use_theme();
    let mut is_visible = use_signal(|| false);
    let mut is_hovered = use_signal(|| false);
    
    // Use generation counter to track state changes for delay handling
    let mut generation = use_signal(|| 0u64);
    
    // Calculate position styles based on side and align
    let position_style = match (&props.side, &props.align) {
        (Side::Top, Align::Start) => "bottom: calc(100% + 8px); left: 0;",
        (Side::Top, Align::Center) => "bottom: calc(100% + 8px); left: 50%; transform: translateX(-50%);",
        (Side::Top, Align::End) => "bottom: calc(100% + 8px); right: 0;",
        (Side::Right, Align::Start) => "left: calc(100% + 8px); top: 0;",
        (Side::Right, Align::Center) => "left: calc(100% + 8px); top: 50%; transform: translateY(-50%);",
        (Side::Right, Align::End) => "left: calc(100% + 8px); bottom: 0;",
        (Side::Bottom, Align::Start) => "top: calc(100% + 8px); left: 0;",
        (Side::Bottom, Align::Center) => "top: calc(100% + 8px); left: 50%; transform: translateX(-50%);",
        (Side::Bottom, Align::End) => "top: calc(100% + 8px); right: 0;",
        (Side::Left, Align::Start) => "right: calc(100% + 8px); top: 0;",
        (Side::Left, Align::Center) => "right: calc(100% + 8px); top: 50%; transform: translateY(-50%);",
        (Side::Left, Align::End) => "right: calc(100% + 8px); bottom: 0;",
    };
    
    // Card base styles
    let card_style = use_style(|t| {
        Style::new()
            .absolute()
            .w_px(320)
            .rounded(&t.radius, "md")
            .border(1, &t.colors.border)
            .bg(&t.colors.popover)
            .shadow(&t.shadows.lg)
            .z_index(50)
            .transition("opacity 150ms ease-in-out, transform 150ms ease-in-out")
            .build()
    });
    
    // Arrow styles based on side
    let arrow_style = use_style(|t| {
        Style::new()
            .absolute()
            .w_px(8)
            .h_px(8)
            .bg(&t.colors.popover)
            .border(1, &t.colors.border)
            .build()
    });
    
    let arrow_position = match &props.side {
        Side::Top => "bottom: -4px; left: 50%; transform: translateX(-50%) rotate(45deg); border-top: none; border-left: none;",
        Side::Right => "left: -4px; top: 50%; transform: translateY(-50%) rotate(45deg); border-right: none; border-bottom: none;",
        Side::Bottom => "top: -4px; left: 50%; transform: translateX(-50%) rotate(45deg); border-bottom: none; border-right: none;",
        Side::Left => "right: -4px; top: 50%; transform: translateY(-50%) rotate(45deg); border-left: none; border-top: none;",
    };
    
    // Visibility styles with animation
    let visibility_style = match (&props.side, &props.align, is_visible()) {
        (Side::Top, Align::Center, true) => "opacity: 1; transform: translateX(-50%) translateY(0); pointer-events: auto;",
        (Side::Top, Align::Center, false) => "opacity: 0; transform: translateX(-50%) translateY(4px); pointer-events: none;",
        (Side::Right, Align::Center, true) => "opacity: 1; transform: translateY(-50%) translateX(0); pointer-events: auto;",
        (Side::Right, Align::Center, false) => "opacity: 0; transform: translateY(-50%) translateX(-4px); pointer-events: none;",
        (Side::Bottom, Align::Center, true) => "opacity: 1; transform: translateX(-50%) translateY(0); pointer-events: auto;",
        (Side::Bottom, Align::Center, false) => "opacity: 0; transform: translateX(-50%) translateY(-4px); pointer-events: none;",
        (Side::Left, Align::Center, true) => "opacity: 1; transform: translateY(-50%) translateX(0); pointer-events: auto;",
        (Side::Left, Align::Center, false) => "opacity: 0; transform: translateY(-50%) translateX(4px); pointer-events: none;",
        (_, _, true) => "opacity: 1; transform: translateY(0); pointer-events: auto;",
        (_, _, false) => "opacity: 0; transform: translateY(-4px); pointer-events: none;",
    };
    
    // Handle keyboard events (Escape to close)
    let handle_keydown = move |e: Event<KeyboardData>| {
        if e.key() == Key::Escape && is_visible() {
            is_visible.set(false);
            is_hovered.set(false);
            generation.set(generation() + 1);
        }
    };
    
    let custom_style = props.style.clone().unwrap_or_default();
    let open_delay = props.open_delay;
    let close_delay = props.close_delay;
    
    rsx! {
        div {
            style: "position: relative; display: inline-block;",
            onmouseenter: move |_| {
                is_hovered.set(true);
                let current_gen = generation() + 1;
                generation.set(current_gen);
                let open_delay = open_delay;
                let gen_signal = generation;
                let mut vis_signal = is_visible;
                let hover_signal = is_hovered;
                spawn(async move {
                    // Simple delay using a counter-based approach
                    let target = gen_signal();
                    let mut count = 0u64;
                    loop {
                        // Check if generation changed (means cancel)
                        if gen_signal() != target {
                            return;
                        }
                        // ~16ms per iteration for 60fps
                        for _ in 0..100 { /* small delay */ }
                        count += 1;
                        // Approximate delay (count * loop time)
                        if count * 16 >= open_delay {
                            break;
                        }
                    }
                    // Check again after delay
                    if gen_signal() == target && hover_signal() {
                        vis_signal.set(true);
                    }
                });
            },
            onmouseleave: move |_| {
                is_hovered.set(false);
                let current_gen = generation() + 1;
                generation.set(current_gen);
                let close_delay = close_delay;
                let gen_signal = generation;
                let mut vis_signal = is_visible;
                let hover_signal = is_hovered;
                spawn(async move {
                    let target = gen_signal();
                    let mut count = 0u64;
                    loop {
                        if gen_signal() != target {
                            return;
                        }
                        for _ in 0..100 {}
                        count += 1;
                        if count * 16 >= close_delay {
                            break;
                        }
                    }
                    if gen_signal() == target && !hover_signal() {
                        vis_signal.set(false);
                    }
                });
            },
            onkeydown: handle_keydown,
            
            // Trigger
            div {
                style: "display: inline-block;",
                {props.trigger}
            }
            
            // Overlay for click outside to close
            if is_visible() {
                div {
                    style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; z-index: 49;",
                    onclick: move |_| {
                        is_visible.set(false);
                        is_hovered.set(false);
                        generation.set(generation() + 1);
                    },
                }
            }
            
            // Card content
            div {
                style: "{card_style} {position_style} {visibility_style} {custom_style}",
                onmouseenter: move |_| {
                    is_hovered.set(true);
                    generation.set(generation() + 1);
                },
                onmouseleave: move |_| {
                    is_hovered.set(false);
                    let current_gen = generation() + 1;
                    generation.set(current_gen);
                    let close_delay = close_delay;
                    let gen_signal = generation;
                    let mut vis_signal = is_visible;
                    let hover_signal = is_hovered;
                    spawn(async move {
                        let target = gen_signal();
                        let mut count = 0u64;
                        loop {
                            if gen_signal() != target {
                                return;
                            }
                            for _ in 0..100 {}
                            count += 1;
                            if count * 16 >= close_delay {
                                break;
                            }
                        }
                        if gen_signal() == target && !hover_signal() {
                            vis_signal.set(false);
                        }
                    });
                },
                
                // Arrow
                div {
                    style: "{arrow_style} {arrow_position}",
                }
                
                // Content wrapper
                div {
                    style: "position: relative; z-index: 1;",
                    {props.children}
                }
            }
        }
    }
}

/// Hover Card Header component
#[derive(Props, Clone, PartialEq)]
pub struct HoverCardHeaderProps {
    /// Header title
    pub title: String,
    /// Optional description
    #[props(default)]
    pub description: Option<String>,
}

#[component]
pub fn HoverCardHeader(props: HoverCardHeaderProps) -> Element {
    let _theme = use_theme();
    
    let header_style = use_style(|t| {
        Style::new()
            .pb(&t.spacing, "md")
            .mb(&t.spacing, "sm")
            .border_bottom(1, &t.colors.border)
            .build()
    });
    
    rsx! {
        div {
            style: "{header_style}",
            
            h4 {
                style: "margin: 0; font-size: 16px; font-weight: 600;",
                "{props.title}"
            }
            
            if let Some(description) = props.description {
                p {
                    style: "margin: 4px 0 0 0; font-size: 13px; color: #64748b;",
                    "{description}"
                }
            }
        }
    }
}

/// Hover Card Content component
#[derive(Props, Clone, PartialEq)]
pub struct HoverCardContentProps {
    /// Content to display
    pub children: Element,
}

#[component]
pub fn HoverCardContent(props: HoverCardContentProps) -> Element {
    let _theme = use_theme();
    
    let content_style = use_style(|t| {
        Style::new()
            .p(&t.spacing, "md")
            .build()
    });
    
    rsx! {
        div {
            style: "{content_style}",
            {props.children}
        }
    }
}

/// Hover Card Footer component
#[derive(Props, Clone, PartialEq)]
pub struct HoverCardFooterProps {
    /// Footer content
    pub children: Element,
}

#[component]
pub fn HoverCardFooter(props: HoverCardFooterProps) -> Element {
    let _theme = use_theme();
    
    let footer_style = use_style(|t| {
        Style::new()
            .pt(&t.spacing, "md")
            .mt(&t.spacing, "sm")
            .border_top(1, &t.colors.border)
            .flex()
            .justify_end()
            .items_center()
            .gap(&t.spacing, "sm")
            .build()
    });
    
    rsx! {
        div {
            style: "{footer_style}",
            {props.children}
        }
    }
}

/// Hover Card Avatar component for user previews
#[derive(Props, Clone, PartialEq)]
pub struct HoverCardAvatarProps {
    /// Avatar image URL
    pub src: String,
    /// Avatar alt text
    #[props(default)]
    pub alt: Option<String>,
    /// Avatar size in pixels
    #[props(default = 40)]
    pub size: u16,
}

#[component]
pub fn HoverCardAvatar(props: HoverCardAvatarProps) -> Element {
    let size = props.size;
    let alt = props.alt.clone().unwrap_or_default();
    
    rsx! {
        img {
            src: "{props.src}",
            alt: "{alt}",
            style: "width: {size}px; height: {size}px; border-radius: 50%; object-fit: cover;",
        }
    }
}

/// Hover Card User Info component for GitHub-style user previews
#[derive(Props, Clone, PartialEq)]
pub struct HoverCardUserInfoProps {
    /// User name
    pub name: String,
    /// User handle/username
    pub handle: String,
    /// Optional avatar URL
    #[props(default)]
    pub avatar_url: Option<String>,
    /// Optional bio/description
    #[props(default)]
    pub bio: Option<String>,
    /// Optional stats (e.g., "10 repos · 50 followers")
    #[props(default)]
    pub stats: Option<String>,
}

#[component]
pub fn HoverCardUserInfo(props: HoverCardUserInfoProps) -> Element {
    let _theme = use_theme();
    
    let container_style = use_style(|t| {
        Style::new()
            .flex()
            .items_start()
            .gap(&t.spacing, "md")
            .build()
    });
    
    rsx! {
        div {
            style: "{container_style}",
            
            if let Some(avatar_url) = props.avatar_url {
                HoverCardAvatar {
                    src: avatar_url,
                    alt: Some(props.name.clone()),
                    size: 48,
                }
            }
            
            div {
                style: "flex: 1; min-width: 0;",
                
                div {
                    style: "font-weight: 600; font-size: 15px;",
                    "{props.name}"
                }
                
                div {
                    style: "font-size: 13px; color: #64748b;",
                    "{props.handle}"
                }
                
                if let Some(bio) = props.bio {
                    p {
                        style: "margin: 8px 0 0 0; font-size: 13px; line-height: 1.4;",
                        "{bio}"
                    }
                }
                
                if let Some(stats) = props.stats {
                    div {
                        style: "margin-top: 8px; font-size: 12px; color: #64748b;",
                        "{stats}"
                    }
                }
            }
        }
    }
}
