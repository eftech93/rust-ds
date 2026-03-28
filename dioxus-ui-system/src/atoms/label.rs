//! Label atom component
//!
//! Text label with typography and theming support.

use crate::styles::Style;
use crate::theme::tokens::Color;
use crate::theme::{use_style, use_theme};
use dioxus::prelude::*;

/// Text sizes (typography presets)
#[derive(Default, Clone, PartialEq)]
pub enum TextSize {
    ExtraSmall,
    Small,
    #[default]
    Base,
    Large,
    ExtraLarge,
    H1,
    H2,
    H3,
    H4,
}

/// Text weights
#[derive(Default, Clone, PartialEq)]
pub enum TextWeight {
    #[default]
    Normal,
    Medium,
    Semibold,
    Bold,
}

/// Text colors
#[derive(Default, Clone, PartialEq)]
pub enum TextColor {
    #[default]
    Default,
    Muted,
    Primary,
    Secondary,
    Destructive,
    Success,
    Warning,
    Inverse,
    Custom(Color),
}

/// Label/Text properties
#[derive(Props, Clone, PartialEq)]
pub struct LabelProps {
    /// Text content
    pub children: Element,
    /// Typography size
    #[props(default)]
    pub size: TextSize,
    /// Font weight
    #[props(default)]
    pub weight: TextWeight,
    /// Text color
    #[props(default)]
    pub color: TextColor,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
    /// HTML element to render (defaults to span)
    #[props(default)]
    pub as_element: LabelElement,
    /// Text alignment
    #[props(default)]
    pub align: TextAlign,
    /// Truncate with ellipsis
    #[props(default)]
    pub truncate: bool,
    /// For element (associates label with form input)
    #[props(default)]
    pub html_for: Option<String>,
    /// Line clamp (number of lines)
    #[props(default)]
    pub line_clamp: Option<u8>,
}

/// HTML element options for Label
#[derive(Default, Clone, PartialEq)]
pub enum LabelElement {
    #[default]
    Span,
    P,
    Div,
    Label,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Strong,
    Em,
    Small,
}

/// Text alignment
#[derive(Default, Clone, PartialEq)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
    Justify,
}

impl TextAlign {
    fn as_str(&self) -> &'static str {
        match self {
            TextAlign::Left => "left",
            TextAlign::Center => "center",
            TextAlign::Right => "right",
            TextAlign::Justify => "justify",
        }
    }
}

/// Label/Text atom component
///
/// # Example
/// ```rust,ignore
/// use dioxus_ui_system::atoms::{Label, TextSize, TextWeight};
///
/// rsx! {
///     Label {
///         size: TextSize::H1,
///         weight: TextWeight::Bold,
///         "Hello, World!"
///     }
/// }
/// ```
#[component]
pub fn Label(props: LabelProps) -> Element {
    let _theme = use_theme();

    let size = props.size.clone();
    let weight = props.weight.clone();
    let color = props.color.clone();
    let align = props.align.clone();
    let truncate = props.truncate;
    let line_clamp = props.line_clamp;

    // Memoized styles
    let style = use_style(move |t| {
        let typography_size = match size {
            TextSize::ExtraSmall => "xs",
            TextSize::Small => "sm",
            TextSize::Base => "base",
            TextSize::Large => "lg",
            TextSize::ExtraLarge => "xl",
            TextSize::H1 => "h1",
            TextSize::H2 => "h2",
            TextSize::H3 => "h3",
            TextSize::H4 => "h4",
        };

        let base = Style::new()
            .text(&t.typography, typography_size)
            .text_align(align.as_str());

        // Apply weight override if different from typography default
        let base = match weight {
            TextWeight::Normal => base.font_weight(400),
            TextWeight::Medium => base.font_weight(500),
            TextWeight::Semibold => base.font_weight(600),
            TextWeight::Bold => base.font_weight(700),
        };

        // Apply color
        let base = match &color {
            TextColor::Default => base.text_color(&t.colors.foreground),
            TextColor::Muted => base.text_color(&t.colors.muted_foreground),
            TextColor::Primary => base.text_color(&t.colors.primary),
            TextColor::Secondary => base.text_color(&t.colors.secondary_foreground),
            TextColor::Destructive => base.text_color(&t.colors.destructive),
            TextColor::Success => base.text_color(&t.colors.success),
            TextColor::Warning => base.text_color(&t.colors.warning),
            TextColor::Inverse => base.text_color(&t.colors.background),
            TextColor::Custom(c) => base.text_color(c),
        };

        // Truncate
        let base = if truncate {
            Style {
                overflow: Some("hidden".into()),
                white_space: Some("nowrap".into()),
                text_decoration: Some("ellipsis".into()),
                ..base
            }
        } else {
            base
        };

        // Line clamp
        let base = if let Some(_clamp) = line_clamp {
            Style {
                overflow: Some("hidden".into()),
                ..base
            }
        } else {
            base
        };

        base.build()
    });

    // Combine with custom styles
    let final_style = if let Some(custom) = &props.style {
        format!("{} {}", style(), custom)
    } else {
        style()
    };

    let class = props.class.clone().unwrap_or_default();
    let html_for = props.html_for.clone();

    // Render the appropriate element
    match props.as_element {
        LabelElement::Span => rsx! {
            span { style: "{final_style}", class: "{class}", {props.children} }
        },
        LabelElement::P => rsx! {
            p { style: "{final_style}", class: "{class}", {props.children} }
        },
        LabelElement::Div => rsx! {
            div { style: "{final_style}", class: "{class}", {props.children} }
        },
        LabelElement::Label => rsx! {
            label { style: "{final_style}", class: "{class}", r#for: html_for, {props.children} }
        },
        LabelElement::H1 => rsx! {
            h1 { style: "{final_style}", class: "{class}", {props.children} }
        },
        LabelElement::H2 => rsx! {
            h2 { style: "{final_style}", class: "{class}", {props.children} }
        },
        LabelElement::H3 => rsx! {
            h3 { style: "{final_style}", class: "{class}", {props.children} }
        },
        LabelElement::H4 => rsx! {
            h4 { style: "{final_style}", class: "{class}", {props.children} }
        },
        LabelElement::H5 => rsx! {
            h5 { style: "{final_style}", class: "{class}", {props.children} }
        },
        LabelElement::H6 => rsx! {
            h6 { style: "{final_style}", class: "{class}", {props.children} }
        },
        LabelElement::Strong => rsx! {
            strong { style: "{final_style}", class: "{class}", {props.children} }
        },
        LabelElement::Em => rsx! {
            em { style: "{final_style}", class: "{class}", {props.children} }
        },
        LabelElement::Small => rsx! {
            small { style: "{final_style}", class: "{class}", {props.children} }
        },
    }
}

/// Convenience component for heading text
#[component]
pub fn Heading(
    children: Element,
    #[props(default)] level: HeadingLevel,
    #[props(default)] weight: TextWeight,
    #[props(default)] color: TextColor,
) -> Element {
    let size = match level {
        HeadingLevel::H1 => TextSize::H1,
        HeadingLevel::H2 => TextSize::H2,
        HeadingLevel::H3 => TextSize::H3,
        HeadingLevel::H4 => TextSize::H4,
    };

    let element = match level {
        HeadingLevel::H1 => LabelElement::H1,
        HeadingLevel::H2 => LabelElement::H2,
        HeadingLevel::H3 => LabelElement::H3,
        HeadingLevel::H4 => LabelElement::H4,
    };

    rsx! {
        Label {
            size: size,
            weight: weight,
            color: color,
            as_element: element,
            {children}
        }
    }
}

/// Heading levels
#[derive(Default, Clone, PartialEq)]
pub enum HeadingLevel {
    H1,
    H2,
    H3,
    #[default]
    H4,
}

/// Convenience component for muted/secondary text
#[component]
pub fn MutedText(children: Element, #[props(default)] size: TextSize) -> Element {
    rsx! {
        Label {
            size: size,
            color: TextColor::Muted,
            {children}
        }
    }
}
