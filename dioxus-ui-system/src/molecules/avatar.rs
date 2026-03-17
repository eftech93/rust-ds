//! Avatar molecule component
//!
//! An image element with a fallback for representing the user.

use dioxus::prelude::*;
use crate::theme::{use_theme, use_style};
use crate::styles::Style;

/// Avatar sizes
#[derive(Clone, PartialEq)]
pub enum AvatarSize {
    /// Extra small (24px)
    Xs,
    /// Small (32px)
    Sm,
    /// Medium (40px)
    Md,
    /// Large (48px)
    Lg,
    /// Extra large (64px)
    Xl,
}

impl Default for AvatarSize {
    fn default() -> Self {
        AvatarSize::Md
    }
}

/// Avatar properties
#[derive(Props, Clone, PartialEq)]
pub struct AvatarProps {
    /// Image source URL
    #[props(default)]
    pub src: Option<String>,
    /// Alt text for the image
    #[props(default)]
    pub alt: String,
    /// User name (used for fallback initials)
    #[props(default)]
    pub name: Option<String>,
    /// Avatar size
    #[props(default)]
    pub size: AvatarSize,
    /// Custom fallback content
    #[props(default)]
    pub fallback: Option<Element>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Avatar molecule component
#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    let _theme = use_theme();
    let mut image_error = use_signal(|| false);
    
    let size_px = match props.size {
        AvatarSize::Xs => 24,
        AvatarSize::Sm => 32,
        AvatarSize::Md => 40,
        AvatarSize::Lg => 48,
        AvatarSize::Xl => 64,
    };
    
    let font_size = match props.size {
        AvatarSize::Xs => 10,
        AvatarSize::Sm => 12,
        AvatarSize::Md => 14,
        AvatarSize::Lg => 16,
        AvatarSize::Xl => 20,
    };
    
    let avatar_style = use_style(move |t| {
        Style::new()
            .w_px(size_px)
            .h_px(size_px)
            .rounded_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(&t.colors.muted)
            .text_color(&t.colors.foreground)
            .font_size(font_size)
            .font_weight(500)
            .overflow_hidden()
            .border(0, &t.colors.border)
            .build()
    });
    
    let show_image = props.src.is_some() && !image_error();
    let src_clone = props.src.clone();
    
    rsx! {
        div {
            style: "{avatar_style} {props.style.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            
            if show_image {
                img {
                    src: "{src_clone.clone().unwrap()}",
                    alt: "{props.alt}",
                    style: "width: 100%; height: 100%; object-fit: cover;",
                    onerror: move |_| image_error.set(true),
                }
            } else if let Some(fallback) = props.fallback.clone() {
                {fallback}
            } else if let Some(name) = props.name.clone() {
                AvatarFallbackInitials { name: name, font_size: font_size }
            } else {
                AvatarFallbackIcon { size: size_px / 2 }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct AvatarFallbackInitialsProps {
    name: String,
    font_size: u16,
}

#[component]
fn AvatarFallbackInitials(props: AvatarFallbackInitialsProps) -> Element {
    let initials: String = props
        .name
        .split_whitespace()
        .filter_map(|s| s.chars().next())
        .collect::<String>()
        .to_uppercase()
        .chars()
        .take(2)
        .collect();
    
    rsx! {
        span {
            style: "font-size: {props.font_size}px; user-select: none;",
            "{initials}"
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct AvatarFallbackIconProps {
    size: u16,
}

#[component]
fn AvatarFallbackIcon(props: AvatarFallbackIconProps) -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            style: "width: {props.size}px; height: {props.size}px;",
            path { d: "M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2" }
            circle { cx: "12", cy: "7", r: "4" }
        }
    }
}

/// Avatar group component for displaying multiple avatars
#[derive(Props, Clone, PartialEq)]
pub struct AvatarGroupProps {
    /// Maximum number of avatars to show
    #[props(default = 4)]
    pub max: usize,
    /// Size of all avatars in the group
    #[props(default)]
    pub size: AvatarSize,
    /// Avatar children
    pub children: Element,
}

/// Avatar group component
#[component]
pub fn AvatarGroup(props: AvatarGroupProps) -> Element {
    let _size_px = match props.size {
        AvatarSize::Xs => 24,
        AvatarSize::Sm => 32,
        AvatarSize::Md => 40,
        AvatarSize::Lg => 48,
        AvatarSize::Xl => 64,
    };
    
    let _overlap = _size_px / 4;
    
    rsx! {
        div {
            style: "display: flex; align-items: center;",
            
            div {
                style: "display: flex;",
                
                // Note: In a real implementation, we'd use context to manage the overlapping
                // For now, this is a simplified version
                {props.children}
            }
        }
    }
}
