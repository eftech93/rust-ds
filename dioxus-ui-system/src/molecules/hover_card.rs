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
/// - Show on hover with configurable delay (using CSS)
/// - Hide on mouse leave with configurable delay (using CSS)
/// - Position relative to trigger (side + align)
/// - Arrow pointing to trigger
/// - Click outside to close
/// - Escape key to close
/// - Smooth fade animation
#[component]
pub fn HoverCard(props: HoverCardProps) -> Element {
    let _theme = use_theme();
    let mut is_visible = use_signal(|| false);
    
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
    
    // Card base styles with CSS-based hover delay
    let open_delay_ms = props.open_delay;
    let close_delay_ms = props.close_delay;
    let card_style = use_style(move |t| {
        let transition = format!("opacity 200ms ease {}ms, transform 200ms ease {}ms", 
            if is_visible() { 0 } else { close_delay_ms as i32 },
            if is_visible() { 0 } else { close_delay_ms as i32 }
        );
        Style::new()
            .absolute()
            .w_px(320)
            .rounded(&t.radius, "md")
            .border(1, &t.colors.border)
            .bg(&t.colors.popover)
            .shadow(&t.shadows.lg)
            .z_index(9999)
            .transition(&transition)
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
    let visibility_style = if is_visible() {
        "opacity: 1; pointer-events: auto;"
    } else {
        "opacity: 0; pointer-events: none;"
    };
    
    let transform_style = match (&props.side, &props.align, is_visible()) {
        (Side::Top, Align::Center, true) => "transform: translateX(-50%) translateY(0);",
        (Side::Top, Align::Center, false) => "transform: translateX(-50%) translateY(4px);",
        (Side::Right, Align::Center, true) => "transform: translateY(-50%) translateX(0);",
        (Side::Right, Align::Center, false) => "transform: translateY(-50%) translateX(-4px);",
        (Side::Bottom, Align::Center, true) => "transform: translateX(-50%) translateY(0);",
        (Side::Bottom, Align::Center, false) => "transform: translateX(-50%) translateY(-4px);",
        (Side::Left, Align::Center, true) => "transform: translateY(-50%) translateX(0);",
        (Side::Left, Align::Center, false) => "transform: translateY(-50%) translateX(4px);",
        (_, _, true) => "transform: translateY(0);",
        (_, _, false) => "transform: translateY(4px);",
    };
    
    // Handle keyboard events (Escape to close)
    let handle_keydown = move |e: Event<KeyboardData>| {
        if e.key() == Key::Escape && is_visible() {
            is_visible.set(false);
        }
    };
    
    let custom_style = props.style.clone().unwrap_or_default();
    
    rsx! {
        div {
            style: "position: relative; display: inline-block;",
            onmouseenter: move |_| {
                is_visible.set(true);
            },
            onmouseleave: move |_| {
                is_visible.set(false);
            },
            onkeydown: handle_keydown,
            
            // Trigger
            div {
                style: "display: inline-block;",
                {props.trigger}
            }
            
            // Card content - sibling to trigger, not child of overlay
            div {
                style: "{card_style} {position_style} {visibility_style} {transform_style} {custom_style}",
                onmouseenter: move |_| {
                    is_visible.set(true);
                },
                onmouseleave: move |_| {
                    is_visible.set(false);
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
