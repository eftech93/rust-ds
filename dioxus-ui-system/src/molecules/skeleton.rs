//! Skeleton molecule component
//!
//! Use to show a placeholder while content is loading.

use dioxus::prelude::*;
use crate::theme::{use_theme, use_style};
use crate::styles::Style;

/// Skeleton properties
#[derive(Props, Clone, PartialEq)]
pub struct SkeletonProps {
    /// Width of the skeleton
    #[props(default)]
    pub width: Option<String>,
    /// Height of the skeleton
    #[props(default)]
    pub height: Option<String>,
    /// Whether to show animation
    #[props(default = true)]
    pub animate: bool,
    /// Border radius
    #[props(default)]
    pub rounded: Option<String>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Skeleton molecule component
#[component]
pub fn Skeleton(props: SkeletonProps) -> Element {
    let _theme = use_theme();
    
    let width = props.width.unwrap_or_else(|| "100%".to_string());
    let height = props.height.unwrap_or_else(|| "20px".to_string());
    let rounded = props.rounded.unwrap_or_else(|| "4px".to_string());
    
    let skeleton_style = use_style(move |t| {
        Style::new()
            .bg(&t.colors.muted)
            .build()
    });
    
    let animation = if props.animate {
        "animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;"
    } else {
        ""
    };
    
    rsx! {
        div {
            style: "{skeleton_style} width: {width}; height: {height}; border-radius: {rounded}; {animation} {props.style.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
        }
    }
}

/// Skeleton circle variant
#[derive(Props, Clone, PartialEq)]
pub struct SkeletonCircleProps {
    /// Size of the circle (width and height)
    #[props(default = "40".to_string())]
    pub size: String,
    /// Whether to show animation
    #[props(default = true)]
    pub animate: bool,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Skeleton circle component
#[component]
pub fn SkeletonCircle(props: SkeletonCircleProps) -> Element {
    let _theme = use_theme();
    
    let skeleton_style = use_style(move |t| {
        Style::new()
            .bg(&t.colors.muted)
            .rounded_full()
            .build()
    });
    
    let animation = if props.animate {
        "animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;"
    } else {
        ""
    };
    
    rsx! {
        div {
            style: "{skeleton_style} width: {props.size}px; height: {props.size}px; {animation} {props.style.clone().unwrap_or_default()}",
        }
    }
}

/// Skeleton text variant with multiple lines
#[derive(Props, Clone, PartialEq)]
pub struct SkeletonTextProps {
    /// Number of lines
    #[props(default = 3)]
    pub lines: usize,
    /// Whether to show animation
    #[props(default = true)]
    pub animate: bool,
    /// Last line width (as percentage)
    #[props(default = 60)]
    pub last_line_width: u8,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Skeleton text component
#[component]
pub fn SkeletonText(props: SkeletonTextProps) -> Element {
    let _theme = use_theme();
    
    let container_style = use_style(|t| {
        Style::new()
            .flex()
            .flex_col()
            .gap(&t.spacing, "sm")
            .build()
    });
    
    rsx! {
        div {
            style: "{container_style} {props.style.clone().unwrap_or_default()}",
            
            for i in 0..props.lines {
                Skeleton {
                    width: if i == props.lines - 1 {
                        Some(format!("{}%", props.last_line_width))
                    } else {
                        Some("100%".to_string())
                    },
                    height: Some("12px".to_string()),
                    animate: props.animate,
                    rounded: Some("6px".to_string()),
                }
            }
        }
    }
}

/// Skeleton card variant
#[derive(Props, Clone, PartialEq)]
pub struct SkeletonCardProps {
    /// Whether to show animation
    #[props(default = true)]
    pub animate: bool,
    /// Show avatar placeholder
    #[props(default = true)]
    pub show_avatar: bool,
    /// Number of text lines
    #[props(default = 2)]
    pub text_lines: usize,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Skeleton card component
#[component]
pub fn SkeletonCard(props: SkeletonCardProps) -> Element {
    let _theme = use_theme();
    
    let card_style = use_style(|t| {
        Style::new()
            .w_full()
            .rounded(&t.radius, "lg")
            .border(1, &t.colors.border)
            .p(&t.spacing, "lg")
            .flex()
            .flex_col()
            .gap(&t.spacing, "md")
            .build()
    });
    
    rsx! {
        div {
            style: "{card_style} {props.style.clone().unwrap_or_default()}",
            
            if props.show_avatar {
                div {
                    style: "display: flex; align-items: center; gap: 12px;",
                    
                    SkeletonCircle {
                        size: "48".to_string(),
                        animate: props.animate,
                    }
                    
                    div {
                        style: "flex: 1;",
                        Skeleton {
                            width: Some("120px".to_string()),
                            height: Some("14px".to_string()),
                            animate: props.animate,
                            rounded: Some("4px".to_string()),
                        }
                    }
                }
            }
            
            SkeletonText {
                lines: props.text_lines,
                animate: props.animate,
                last_line_width: 80,
            }
        }
    }
}
