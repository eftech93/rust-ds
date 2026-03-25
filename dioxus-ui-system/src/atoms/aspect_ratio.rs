//! Aspect Ratio atom component
//!
//! A container that maintains a consistent aspect ratio for its content.

use dioxus::prelude::*;
use crate::theme::{use_theme, use_style};
use crate::styles::Style;

/// Aspect ratio properties
#[derive(Props, Clone, PartialEq)]
pub struct AspectRatioProps {
    /// The aspect ratio (width / height), e.g., 16.0/9.0 for 16:9
    #[props(default = 1.0)]
    pub ratio: f64,
    /// Content to display
    pub children: Element,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Aspect ratio container component
///
/// Maintains a consistent aspect ratio by using the padding-bottom trick.
/// The content is absolutely positioned to fill the container.
///
/// # Example
/// ```rust,ignore
/// use dioxus::prelude::*;
/// use dioxus_ui_system::atoms::{AspectRatio, Image};
///
/// rsx! {
///     AspectRatio { ratio: 16.0/9.0,
///         Image { src: "image.jpg", alt: "Video thumbnail" }
///     }
/// }
/// ```
#[component]
pub fn AspectRatio(props: AspectRatioProps) -> Element {
    let _theme = use_theme();
    
    // Calculate percentage padding based on ratio
    // ratio = width / height, so height = width / ratio
    // padding-bottom = height / width * 100 = 1/ratio * 100
    let padding_bottom = if props.ratio > 0.0 {
        (1.0 / props.ratio) * 100.0
    } else {
        100.0
    };
    
    let container_style = use_style(|_t| {
        Style::new()
            .relative()
            .w_full()
            .build()
    });
    
    let spacer_style = format!("padding-bottom: {:.2}%", padding_bottom);
    
    let content_style = use_style(|_t| {
        Style::new()
            .absolute()
            .inset("0")
            .w_full()
            .h_full()
            .build()
    });
    
    rsx! {
        div {
            style: "{container_style} {props.style.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            
            div { style: "{spacer_style}" }
            
            div {
                style: "{content_style}",
                {props.children}
            }
        }
    }
}

/// Predefined aspect ratios for common use cases
pub struct AspectRatios;

impl AspectRatios {
    /// Square (1:1)
    pub const SQUARE: f64 = 1.0;
    /// Standard video (4:3)
    pub const VIDEO: f64 = 4.0 / 3.0;
    /// Widescreen video (16:9)
    pub const WIDESCREEN: f64 = 16.0 / 9.0;
    /// Ultra-wide (21:9)
    pub const ULTRAWIDE: f64 = 21.0 / 9.0;
    /// Portrait (3:4)
    pub const PORTRAIT: f64 = 3.0 / 4.0;
    /// Phone portrait (9:16)
    pub const PHONE: f64 = 9.0 / 16.0;
    /// Classic photo (3:2)
    pub const PHOTO: f64 = 3.0 / 2.0;
    /// Golden ratio
    pub const GOLDEN: f64 = 1.618;
}
