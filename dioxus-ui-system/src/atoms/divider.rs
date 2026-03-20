//! Divider atom component
//!
//! Visual separators for content organization.

use dioxus::prelude::*;
use crate::theme::use_theme;

/// Divider orientation
#[derive(Default, Clone, PartialEq, Debug)]
pub enum DividerOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Divider variant/style
#[derive(Default, Clone, PartialEq, Debug)]
pub enum DividerVariant {
    #[default]
    Solid,
    Dashed,
    Dotted,
}

/// Divider properties
#[derive(Props, Clone, PartialEq)]
pub struct DividerProps {
    /// Orientation (horizontal or vertical)
    #[props(default = DividerOrientation::Horizontal)]
    pub orientation: DividerOrientation,
    /// Visual style variant
    #[props(default = DividerVariant::Solid)]
    pub variant: DividerVariant,
    /// Color override
    pub color: Option<String>,
    /// Thickness in pixels
    #[props(default = 1)]
    pub thickness: u8,
    /// Spacing before and after divider
    #[props(default = 16)]
    pub spacing: u16,
    /// Width (for horizontal) or height (for vertical) - 100% if not specified
    #[props(default)]
    pub length: Option<String>,
    /// Optional label/text in the middle
    #[props(default)]
    pub label: Option<String>,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Additional inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Divider component for visual separation
#[component]
pub fn Divider(props: DividerProps) -> Element {
    let theme = use_theme();
    
    let color = props.color.unwrap_or_else(|| {
        theme.tokens.read().colors.border.to_rgba()
    });
    
    let border_style = match props.variant {
        DividerVariant::Solid => "solid",
        DividerVariant::Dashed => "dashed",
        DividerVariant::Dotted => "dotted",
    };
    
    let class_css = props.class.as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();
    
    let style_css = props.style.as_ref()
        .map(|s| format!("{}", s))
        .unwrap_or_default();
    
    let length_css = props.length.as_ref()
        .map(|l| format!("width: {};", l))
        .unwrap_or_else(|| "width: 100%;".to_string());
    
    // With label - horizontal only
    if let Some(label) = props.label {
        let spacing = props.spacing;
        let thickness = props.thickness;
        return rsx! {
            div {
                class: "divider divider-with-label{class_css}",
                style: "display: flex; align-items: center; margin: {spacing}px 0; {style_css}",
                
                div {
                    style: "flex: 1; height: {thickness}px; background: {color}; border-top: {thickness}px {border_style} {color};",
                }
                
                span {
                    style: "padding: 0 16px; color: {theme.tokens.read().colors.muted.to_rgba()}; font-size: 14px; white-space: nowrap;",
                    "{label}"
                }
                
                div {
                    style: "flex: 1; height: {thickness}px; background: {color}; border-top: {thickness}px {border_style} {color};",
                }
            }
        };
    }
    
    // Without label
    match props.orientation {
        DividerOrientation::Horizontal => {
            let spacing = props.spacing;
            let thickness = props.thickness;
            rsx! {
                hr {
                    class: "divider divider-horizontal{class_css}",
                    style: "border: none; border-top: {thickness}px {border_style} {color}; margin: {spacing}px 0; {length_css} {style_css}",
                }
            }
        }
        DividerOrientation::Vertical => {
            let height = props.length.as_ref()
                .map(|l| format!("height: {};", l))
                .unwrap_or_else(|| "height: 100%;".to_string());
            let thickness = props.thickness;
            let spacing = props.spacing;
            
            rsx! {
                div {
                    class: "divider divider-vertical{class_css}",
                    style: "display: inline-block; width: {thickness}px; border-left: {thickness}px {border_style} {color}; margin: 0 {spacing}px; {height} {style_css}",
                }
            }
        }
    }
}

/// Spacer component for systematic whitespace
#[derive(Props, Clone, PartialEq)]
pub struct SpacerProps {
    /// Size of the spacer (predefined scale)
    #[props(default = SpacerSize::Md)]
    pub size: SpacerSize,
    /// Custom size in pixels (overrides predefined)
    pub custom: Option<u16>,
    /// Direction
    #[props(default = SpacerDirection::Both)]
    pub direction: SpacerDirection,
}

/// Spacer size scale
#[derive(Default, Clone, PartialEq, Debug)]
pub enum SpacerSize {
    Xs,   // 4px
    Sm,   // 8px
    #[default]
    Md,   // 16px
    Lg,   // 24px
    Xl,   // 32px
    Xxl,  // 48px
    Xxxl, // 64px
}

impl SpacerSize {
    fn to_px(&self) -> u16 {
        match self {
            SpacerSize::Xs => 4,
            SpacerSize::Sm => 8,
            SpacerSize::Md => 16,
            SpacerSize::Lg => 24,
            SpacerSize::Xl => 32,
            SpacerSize::Xxl => 48,
            SpacerSize::Xxxl => 64,
        }
    }
}

/// Spacer direction
#[derive(Default, Clone, PartialEq, Debug)]
pub enum SpacerDirection {
    #[default]
    Both,
    Horizontal,
    Vertical,
}

/// Spacer component for systematic whitespace
#[component]
pub fn Spacer(props: SpacerProps) -> Element {
    let size = props.custom.unwrap_or_else(|| props.size.to_px());
    
    let (width, height) = match props.direction {
        SpacerDirection::Both => (size, size),
        SpacerDirection::Horizontal => (size, 0),
        SpacerDirection::Vertical => (0, size),
    };
    
    rsx! {
        div {
            style: "display: block; width: {width}px; height: {height}px; flex-shrink: 0;",
        }
    }
}
