//! Heading atom component
//!
//! Typography headings for content hierarchy (H1-H6).

use crate::theme::use_theme;
use dioxus::prelude::*;

/// Heading level (H1-H6)
#[derive(Default, Clone, PartialEq, Debug)]
pub enum HeadingLevel {
    #[default]
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl HeadingLevel {
    /// Get the HTML tag name
    #[allow(dead_code)]
    fn as_tag(&self) -> &'static str {
        match self {
            HeadingLevel::H1 => "h1",
            HeadingLevel::H2 => "h2",
            HeadingLevel::H3 => "h3",
            HeadingLevel::H4 => "h4",
            HeadingLevel::H5 => "h5",
            HeadingLevel::H6 => "h6",
        }
    }

    /// Get the default font size in pixels
    fn default_size(&self) -> u16 {
        match self {
            HeadingLevel::H1 => 36,
            HeadingLevel::H2 => 30,
            HeadingLevel::H3 => 24,
            HeadingLevel::H4 => 20,
            HeadingLevel::H5 => 18,
            HeadingLevel::H6 => 16,
        }
    }
}

/// Heading properties
#[derive(Props, Clone, PartialEq)]
pub struct HeadingProps {
    /// The heading level (H1-H6)
    #[props(default = HeadingLevel::H1)]
    pub level: HeadingLevel,
    /// Custom font size (overrides default for level)
    pub size: Option<u16>,
    /// Font weight
    #[props(default = 600)]
    pub weight: u16,
    /// Color override
    pub color: Option<String>,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Additional inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Text alignment
    #[props(default)]
    pub align: Option<String>,
    /// Children elements
    pub children: Element,
}

/// Heading component for content hierarchy
#[component]
pub fn Heading(props: HeadingProps) -> Element {
    let theme = use_theme();

    let font_size = props.size.unwrap_or_else(|| props.level.default_size());
    let color = props.color.unwrap_or_else(|| {
        // Safe fallback for when theme isn't initialized yet (SSR/hydration)
        theme
            .tokens
            .try_read()
            .map(|t| t.colors.foreground.to_rgba())
            .unwrap_or("#111827".to_string())
    });

    let align_css = props
        .align
        .as_ref()
        .map(|a| format!("text-align: {};", a))
        .unwrap_or_default();

    let class_css = props
        .class
        .as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();

    let style_css = props
        .style
        .as_ref()
        .map(|s| format!(" {}", s))
        .unwrap_or_default();

    let weight = props.weight;
    let style = format!(
        "font-size: {}px; font-weight: {}; color: {}; margin: 0; line-height: 1.2;{}{}",
        font_size, weight, color, align_css, style_css
    );

    match props.level {
        HeadingLevel::H1 => rsx! {
            h1 { class: "heading heading-h1{class_css}", style: "{style}", {props.children} }
        },
        HeadingLevel::H2 => rsx! {
            h2 { class: "heading heading-h2{class_css}", style: "{style}", {props.children} }
        },
        HeadingLevel::H3 => rsx! {
            h3 { class: "heading heading-h3{class_css}", style: "{style}", {props.children} }
        },
        HeadingLevel::H4 => rsx! {
            h4 { class: "heading heading-h4{class_css}", style: "{style}", {props.children} }
        },
        HeadingLevel::H5 => rsx! {
            h5 { class: "heading heading-h5{class_css}", style: "{style}", {props.children} }
        },
        HeadingLevel::H6 => rsx! {
            h6 { class: "heading heading-h6{class_css}", style: "{style}", {props.children} }
        },
    }
}

/// Paragraph properties
#[derive(Props, Clone, PartialEq)]
pub struct ParagraphProps {
    /// Font size
    #[props(default = 16)]
    pub size: u16,
    /// Line height
    #[props(default = 1.6)]
    pub line_height: f32,
    /// Color override
    pub color: Option<String>,
    /// Text alignment
    #[props(default)]
    pub align: Option<String>,
    /// Maximum width for readability (characters)
    #[props(default = 75)]
    pub max_chars: u16,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Additional inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Children elements
    pub children: Element,
}

/// Paragraph component for body text
#[component]
pub fn Paragraph(props: ParagraphProps) -> Element {
    let theme = use_theme();

    let color = props.color.unwrap_or_else(|| {
        // Safe fallback for when theme isn't initialized yet (SSR/hydration)
        theme
            .tokens
            .try_read()
            .map(|t| t.colors.foreground.to_rgba())
            .unwrap_or("#111827".to_string())
    });

    let align_css = props
        .align
        .as_ref()
        .map(|a| format!("text-align: {};", a))
        .unwrap_or_default();

    let class_css = props
        .class
        .as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();

    let style_css = props
        .style
        .as_ref()
        .map(|s| format!(" {}", s))
        .unwrap_or_default();

    let max_width = format!("max-width: {}ch;", props.max_chars);

    let line_height = props.line_height;

    rsx! {
        p {
            class: "paragraph{class_css}",
            style: "font-size: {props.size}px; line-height: {line_height}; color: {color}; margin: 0; {max_width} {align_css}{style_css}",
            {props.children}
        }
    }
}

/// Caption/Helper text properties
#[derive(Props, Clone, PartialEq)]
pub struct CaptionProps {
    /// Font size
    #[props(default = 12)]
    pub size: u8,
    /// Color variant
    #[props(default = CaptionColor::Muted)]
    pub color: CaptionColor,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Children elements
    pub children: Element,
}

/// Caption color variants
#[derive(Default, Clone, PartialEq, Debug)]
pub enum CaptionColor {
    #[default]
    Muted,
    Secondary,
    Error,
    Success,
    Custom(String),
}

/// Caption component for supplementary text
#[component]
pub fn Caption(props: CaptionProps) -> Element {
    let theme = use_theme();

    let color = match props.color {
        CaptionColor::Muted => theme
            .tokens
            .try_read()
            .map(|t| t.colors.muted.to_rgba())
            .unwrap_or("#6b7280".to_string()),
        CaptionColor::Secondary => theme
            .tokens
            .try_read()
            .map(|t| t.colors.secondary.to_rgba())
            .unwrap_or("#4b5563".to_string()),
        CaptionColor::Error => theme
            .tokens
            .try_read()
            .map(|t| t.colors.destructive.to_rgba())
            .unwrap_or("#dc2626".to_string()),
        CaptionColor::Success => "#16a34a".to_string(),
        CaptionColor::Custom(c) => c,
    };

    let class_css = props
        .class
        .as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();

    rsx! {
        span {
            class: "caption{class_css}",
            style: "font-size: {props.size}px; color: {color}; line-height: 1.4;",
            {props.children}
        }
    }
}

/// Blockquote properties
#[derive(Props, Clone, PartialEq)]
pub struct BlockquoteProps {
    /// Border color
    pub border_color: Option<String>,
    /// Background color
    pub background: Option<String>,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Citation/attribution
    #[props(default)]
    pub cite: Option<String>,
    /// Children elements (quote content)
    pub children: Element,
}

/// Blockquote component for quoted content
#[component]
pub fn Blockquote(props: BlockquoteProps) -> Element {
    let theme = use_theme();

    let border_color = props.border_color.unwrap_or_else(|| {
        theme
            .tokens
            .try_read()
            .map(|t| t.colors.primary.to_rgba())
            .unwrap_or("#3b82f6".to_string())
    });

    let background = props.background.unwrap_or_else(|| {
        theme
            .tokens
            .try_read()
            .map(|t| t.colors.muted.to_rgba())
            .unwrap_or("#f3f4f6".to_string())
    });

    let class_css = props
        .class
        .as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();

    rsx! {
        blockquote {
            class: "blockquote{class_css}",
            style: "margin: 0; padding: 16px 20px; border-left: 4px solid {border_color}; background: {background}; border-radius: 0 8px 8px 0;",
            {props.children}

            if let Some(cite) = props.cite {
                footer {
                    style: "margin-top: 12px; font-size: 14px; color: {theme.tokens.read().colors.muted.to_rgba()}; font-style: normal;",
                    "— {cite}"
                }
            }
        }
    }
}
