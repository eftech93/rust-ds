//! Card molecule component
//!
//! A flexible container for grouping related content.

use dioxus::prelude::*;
use crate::theme::use_style;
use crate::styles::Style;
use crate::atoms::{VStack, HStack, JustifyContent, AlignItems, SpacingSize};

/// Card variants
#[derive(Default, Clone, PartialEq)]
pub enum CardVariant {
    /// Default bordered card
    #[default]
    Default,
    /// Card with subtle background
    Muted,
    /// Elevated card with shadow
    Elevated,
    /// Outlined card
    Outlined,
    /// Ghost card (no border/bg)
    Ghost,
}

/// Card properties
#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    /// Card content
    pub children: Element,
    /// Visual variant
    #[props(default)]
    pub variant: CardVariant,
    /// Custom padding size
    #[props(default)]
    pub padding: Option<String>,
    /// Full width
    #[props(default)]
    pub full_width: bool,
    /// Click handler (makes card interactive)
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
    /// Whether to hide overflow (default: true)
    #[props(default = true)]
    pub overflow_hidden: bool,
}

/// Card molecule component
///
/// # Example
/// ```rust,ignore
/// use dioxus_ui_system::molecules::{Card, CardVariant};
///
/// rsx! {
///     Card {
///         variant: CardVariant::Elevated,
///         CardHeader { title: "Card Title" }
///         CardContent { "Card content goes here" }
///         CardFooter { "Footer content" }
///     }
/// }
/// ```
#[component]
pub fn Card(props: CardProps) -> Element {
    let variant = props.variant.clone();
    let full_width = props.full_width;
    let has_onclick = props.onclick.is_some();
    
    // Interactive states
    let mut is_hovered = use_signal(|| false);
    let mut is_pressed = use_signal(|| false);
    
    let overflow_hidden = props.overflow_hidden;
    let style = use_style(move |t| {
        let base = Style::new()
            .rounded(&t.radius, "lg")
            .bg(&t.colors.card)
            .text_color(&t.colors.card_foreground)
            .transition("all 150ms ease");
        
        let base = if overflow_hidden {
            base.overflow_hidden()
        } else {
            base
        };
            
        // Full width
        let base = if full_width {
            base.w_full()
        } else {
            base
        };
        
        // Apply variant styles
        let styled = match variant {
            CardVariant::Default => base
                .border(1, &t.colors.border),
            CardVariant::Muted => base
                .bg(&t.colors.muted)
                .border(1, &t.colors.border),
            CardVariant::Elevated => base
                .shadow(&t.shadows.md)
                .border(1, &t.colors.border),
            CardVariant::Outlined => base
                .border(2, &t.colors.border),
            CardVariant::Ghost => base,
        };
        
        // Interactive states
        let styled = if has_onclick && !is_pressed() && is_hovered() {
            match variant {
                CardVariant::Elevated => Style {
                    box_shadow: Some(t.shadows.lg.clone()),
                    ..styled
                },
                _ => styled.border_color(&t.colors.foreground.darken(0.2)),
            }
        } else {
            styled
        };
        
        // Apply padding if specified
        let styled = if let Some(ref p) = props.padding {
            Style {
                padding: Some(p.clone()),
                ..styled
            }
        } else {
            styled
        };
        
        // Cursor
        let styled = if has_onclick {
            styled.cursor_pointer()
        } else {
            styled
        };
        
        styled.build()
    });
    
    // Transform for pressed state
    let transform = if is_pressed() && has_onclick {
        "transform: scale(0.99);"
    } else {
        ""
    };
    
    let final_style = if let Some(custom) = &props.style {
        format!("{} {}{}", style(), custom, transform)
    } else {
        format!("{}{}", style(), transform)
    };
    
    let class = props.class.clone().unwrap_or_default();
    
    let onclick_handler = props.onclick.clone();
    
    rsx! {
        div {
            style: "{final_style}",
            class: "{class}",
            onmouseenter: move |_| if has_onclick { is_hovered.set(true) },
            onmouseleave: move |_| { is_hovered.set(false); is_pressed.set(false); },
            onmousedown: move |_| if has_onclick { is_pressed.set(true) },
            onmouseup: move |_| if has_onclick { is_pressed.set(false) },
            onclick: move |e| {
                if let Some(handler) = &onclick_handler {
                    handler.call(e);
                }
            },
            {props.children}
        }
    }
}

/// Card Header properties
#[derive(Props, Clone, PartialEq)]
pub struct CardHeaderProps {
    /// Header content
    pub children: Element,
    /// Optional title (convenience prop)
    #[props(default)]
    pub title: Option<String>,
    /// Optional subtitle
    #[props(default)]
    pub subtitle: Option<String>,
    /// Optional action element
    #[props(default)]
    pub action: Option<Element>,
}

/// Card Header component
#[component]
pub fn CardHeader(props: CardHeaderProps) -> Element {
    let style = use_style(|t| {
        Style::new()
            .p(&t.spacing, "lg")
            .gap(&t.spacing, "xs")
            .build()
    });
    
    let content = if let Some(title) = props.title {
        let subtitle_element = props.subtitle.map(|s| {
            rsx! {
                crate::atoms::Label {
                    size: crate::atoms::TextSize::Small,
                    color: crate::atoms::TextColor::Muted,
                    "{s}"
                }
            }
        });
        
        let action_element = props.action.clone();
        
        rsx! {
            HStack {
                justify: JustifyContent::SpaceBetween,
                align: AlignItems::Start,
                VStack {
                    gap: SpacingSize::Xs,
                    crate::atoms::Heading {
                        level: crate::atoms::HeadingLevel::H4,
                        "{title}"
                    }
                    {subtitle_element}
                }
                {action_element}
            }
        }
    } else {
        props.children
    };
    
    rsx! {
        VStack { style: "{style}", {content} }
    }
}

/// Card Content properties
#[derive(Props, Clone, PartialEq)]
pub struct CardContentProps {
    /// Content
    pub children: Element,
    /// Custom padding
    #[props(default)]
    pub padding: Option<String>,
}

/// Card Content component
#[component]
pub fn CardContent(props: CardContentProps) -> Element {
    let style = use_style(|t| {
        Style::new()
            .p(&t.spacing, "lg")
            .pt_px(0)
            .gap(&t.spacing, "md")
            .build()
    });
    
    let final_style = if let Some(padding) = props.padding {
        format!("padding: {};", padding)
    } else {
        style()
    };
    
    rsx! {
        VStack { style: "{final_style}", {props.children} }
    }
}

/// Card Footer properties
#[derive(Props, Clone, PartialEq)]
pub struct CardFooterProps {
    /// Footer content
    pub children: Element,
    /// Justify content: start, center, end, between
    #[props(default)]
    pub justify: CardFooterJustify,
}

/// Card footer justify options
#[derive(Default, Clone, PartialEq)]
pub enum CardFooterJustify {
    #[default]
    Start,
    Center,
    End,
    Between,
}

/// Card Footer component
#[component]
pub fn CardFooter(props: CardFooterProps) -> Element {
    let justify = match props.justify {
        CardFooterJustify::Start => JustifyContent::Start,
        CardFooterJustify::Center => JustifyContent::Center,
        CardFooterJustify::End => JustifyContent::End,
        CardFooterJustify::Between => JustifyContent::SpaceBetween,
    };
    
    let style = use_style(|t| {
        Style::new()
            .p(&t.spacing, "lg")
            .pt_px(0)
            .gap(&t.spacing, "sm")
            .build()
    });
    
    rsx! {
        HStack {
            style: "{style}",
            justify: justify,
            align: AlignItems::Center,
            {props.children}
        }
    }
}
