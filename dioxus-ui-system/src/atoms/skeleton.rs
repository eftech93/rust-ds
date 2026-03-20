//! Skeleton loader atom component
//!
//! Loading placeholders that mimic content structure.

use dioxus::prelude::*;
use crate::theme::use_theme;

/// Skeleton shape variant
#[derive(Default, Clone, PartialEq, Debug)]
pub enum SkeletonShape {
    #[default]
    Rectangle,
    Circle,
    Text,
    Rounded,
}

/// Skeleton properties
#[derive(Props, Clone, PartialEq)]
pub struct SkeletonProps {
    /// Shape variant
    #[props(default = SkeletonShape::Rectangle)]
    pub shape: SkeletonShape,
    /// Width (CSS value or "auto")
    /// Width (CSS value or "auto")
    #[props(default)]
    pub width: Option<String>,
    /// Height (CSS value or "auto")
    #[props(default)]
    pub height: Option<String>,
    /// Enable animation
    #[props(default = true)]
    pub animated: bool,
    /// Animation variant
    #[props(default = SkeletonAnimation::Shimmer)]
    pub animation: SkeletonAnimation,
    /// Base color
    pub color: Option<String>,
    /// Highlight color for animation
    pub highlight_color: Option<String>,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Skeleton animation type
#[derive(Default, Clone, PartialEq, Debug)]
pub enum SkeletonAnimation {
    #[default]
    Shimmer,
    Pulse,
    Wave,
}

/// Skeleton loader component
#[component]
pub fn Skeleton(props: SkeletonProps) -> Element {
    let theme = use_theme();
    
    let base_color = props.color.unwrap_or_else(|| {
        theme.tokens.read().colors.muted.to_rgba()
    });
    
    let highlight = props.highlight_color.unwrap_or_else(|| {
        // Slightly lighter version of base color
        theme.tokens.read().colors.background.to_rgba()
    });
    
    let class_css = props.class.as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();
    
    let border_radius = match props.shape {
        SkeletonShape::Circle => "50%",
        SkeletonShape::Rounded => "8px",
        SkeletonShape::Text => "4px",
        SkeletonShape::Rectangle => "0px",
    };
    
    let animation_css = if props.animated {
        match props.animation {
            SkeletonAnimation::Shimmer => format!(
                "background: linear-gradient(90deg, {base_color} 25%, {highlight} 50%, {base_color} 75%); background-size: 200% 100%; animation: shimmer 1.5s infinite;",
            ),
            SkeletonAnimation::Pulse => format!(
                "background: {base_color}; animation: pulse 1.5s ease-in-out infinite;",
            ),
            SkeletonAnimation::Wave => format!(
                "background: {base_color}; animation: wave 1.5s ease-in-out infinite;",
            ),
        }
    } else {
        format!("background: {base_color};")
    };
    
    let width = props.width.as_deref().unwrap_or("100%");
    let height = props.height.as_deref().unwrap_or("16px");
    
    rsx! {
        div {
            class: "skeleton{class_css}",
            style: "width: {width}; height: {height}; border-radius: {border_radius}; {animation_css}",
        }
        

    }
}

/// Skeleton text properties (multiple lines)
#[derive(Props, Clone, PartialEq)]
pub struct SkeletonTextProps {
    /// Number of lines
    #[props(default = 3)]
    pub lines: u8,
    /// Line height
    #[props(default = 1.5)]
    pub line_height: f32,
    /// Enable animation
    #[props(default = true)]
    pub animated: bool,
    /// Last line width (as percentage)
    #[props(default = 80)]
    pub last_line_width: u8,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Skeleton text loader (multiple lines)
#[component]
pub fn SkeletonText(props: SkeletonTextProps) -> Element {
    let class_css = props.class.as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();
    
    rsx! {
        div {
            class: "skeleton-text{class_css}",
            style: "display: flex; flex-direction: column; gap: 8px;",
            
            for i in 0..props.lines {
                {
                    let is_last = i == props.lines - 1;
                    let width = if is_last { format!("{}%", props.last_line_width) } else { "100%".to_string() };
                    
                    rsx! {
                        Skeleton {
                            key: "{i}",
                            shape: SkeletonShape::Text,
                            width: width,
                            height: "1em",
                            animated: props.animated,
                        }
                    }
                }
            }
        }
    }
}

/// Skeleton card properties
#[derive(Props, Clone, PartialEq)]
pub struct SkeletonCardProps {
    /// Show image area
    #[props(default = true)]
    pub show_image: bool,
    /// Image height
    #[props(default)]
    pub image_height: Option<String>,
    /// Number of text lines
    #[props(default = 3)]
    pub text_lines: u8,
    /// Show action buttons
    #[props(default = true)]
    pub show_actions: bool,
    /// Enable animation
    #[props(default = true)]
    pub animated: bool,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Skeleton card loader
#[component]
pub fn SkeletonCard(props: SkeletonCardProps) -> Element {
    let class_css = props.class.as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();
    
    rsx! {
        div {
            class: "skeleton-card{class_css}",
            style: "border: 1px solid #e2e8f0; border-radius: 12px; overflow: hidden; background: white;",
            
            if props.show_image {
                Skeleton {
                    shape: SkeletonShape::Rectangle,
                    width: Some("100%".to_string()),
                    height: Some(props.image_height.clone().unwrap_or_else(|| "200px".to_string())),
                    animated: props.animated,
                }
            }
            
            div {
                style: "padding: 16px;",
                
                SkeletonText {
                    lines: props.text_lines,
                    animated: props.animated,
                }
                
                if props.show_actions {
                    div {
                        style: "display: flex; gap: 8px; margin-top: 16px;",
                        
                        Skeleton {
                            shape: SkeletonShape::Rounded,
                            width: "80px",
                            height: "36px",
                            animated: props.animated,
                        }
                        
                        Skeleton {
                            shape: SkeletonShape::Rounded,
                            width: "80px",
                            height: "36px",
                            animated: props.animated,
                        }
                    }
                }
            }
        }
    }
}

/// Skeleton avatar properties
#[derive(Props, Clone, PartialEq)]
pub struct SkeletonAvatarProps {
    /// Size
    #[props(default = AvatarSize::Md)]
    pub size: AvatarSize,
    /// Enable animation
    #[props(default = true)]
    pub animated: bool,
}

/// Avatar size
#[derive(Clone, PartialEq, Debug)]
pub enum AvatarSize {
    Xs,  // 24px
    Sm,  // 32px
    Md,  // 40px
    Lg,  // 48px
    Xl,  // 64px
    Xxl, // 96px
}

impl Default for AvatarSize {
    fn default() -> Self {
        AvatarSize::Md
    }
}

impl AvatarSize {
    fn to_px(&self) -> u16 {
        match self {
            AvatarSize::Xs => 24,
            AvatarSize::Sm => 32,
            AvatarSize::Md => 40,
            AvatarSize::Lg => 48,
            AvatarSize::Xl => 64,
            AvatarSize::Xxl => 96,
        }
    }
}

/// Skeleton avatar loader
#[component]
pub fn SkeletonAvatar(props: SkeletonAvatarProps) -> Element {
    let size = props.size.to_px();
    
    rsx! {
        Skeleton {
            shape: SkeletonShape::Circle,
            width: "{size}px",
            height: "{size}px",
            animated: props.animated,
        }
    }
}

/// Skeleton list properties
#[derive(Props, Clone, PartialEq)]
pub struct SkeletonListProps {
    /// Number of items
    #[props(default = 5)]
    pub items: u8,
    /// Show avatar
    #[props(default = true)]
    pub show_avatar: bool,
    /// Number of text lines per item
    #[props(default = 2)]
    pub lines: u8,
    /// Enable animation
    #[props(default = true)]
    pub animated: bool,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Skeleton list loader
#[component]
pub fn SkeletonList(props: SkeletonListProps) -> Element {
    let class_css = props.class.as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();
    
    rsx! {
        div {
            class: "skeleton-list{class_css}",
            style: "display: flex; flex-direction: column;",
            
            for i in 0..props.items {
                div {
                    key: "{i}",
                    style: "display: flex; align-items: center; gap: 12px; padding: 12px 0; border-bottom: 1px solid #e2e8f0;",
                    
                    if props.show_avatar {
                        SkeletonAvatar {
                            size: AvatarSize::Md,
                            animated: props.animated,
                        }
                    }
                    
                    div {
                        style: "flex: 1;",
                        
                        SkeletonText {
                            lines: props.lines,
                            animated: props.animated,
                        }
                    }
                }
            }
        }
    }
}
