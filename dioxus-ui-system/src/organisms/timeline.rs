//! Timeline organism component
//!
//! A vertical timeline display for showing chronological events.
//!
//! # Example
//! ```rust,ignore
//! use dioxus_ui_system::organisms::timeline::*;
//!
//! rsx! {
//!     Timeline {
//!         TimelineItem {
//!             dot_color: Some(Color::new(34, 197, 94)),
//!             icon: Some("check".to_string()),
//!             TimelineContent {
//!                 h4 { "Project Started" }
//!                 p { "Initial setup and planning phase" }
//!                 span { "Jan 15, 2024" }
//!             }
//!         }
//!         TimelineItem {
//!             TimelineContent {
//!                 h4 { "Development Phase" }
//!                 p { "Active development in progress" }
//!             }
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::styles::Style;
use crate::theme::{use_theme, Color};

/// Timeline position variant
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum TimelinePosition {
    /// Content on the left side only
    Left,
    /// Content on the right side only
    Right,
    /// Content alternates sides (default)
    #[default]
    Alternate,
}

/// Context to share timeline position with child items
#[derive(Clone, Copy)]
pub struct TimelineContext {
    position: TimelinePosition,
}

impl TimelineContext {
    fn new(position: TimelinePosition) -> Self {
        Self { position }
    }
}

/// Hook to access timeline context
fn use_timeline_context() -> TimelineContext {
    try_use_context::<TimelineContext>().unwrap_or_else(|| TimelineContext {
        position: TimelinePosition::default(),
    })
}

/// Timeline container props
#[derive(Props, Clone, PartialEq)]
pub struct TimelineProps {
    /// Timeline items as children
    pub children: Element,
    /// Position of timeline content
    #[props(default)]
    pub position: TimelinePosition,
}

/// Timeline organism - container for chronological events
///
/// Displays a vertical line with events positioned along it.
/// Supports left, right, or alternating content positioning.
#[component]
pub fn Timeline(props: TimelineProps) -> Element {
    let theme = use_theme();
    let border_color = theme.tokens.read().colors.border.to_rgba();

    // Provide context for child items
    use_context_provider(|| TimelineContext::new(props.position.clone()));

    let timeline_style = Style::new().flex().flex_col().relative().py_px(16).w_full();

    rsx! {
        div {
            style: timeline_style.build(),

            // Central vertical line (for alternate position)
            if props.position == TimelinePosition::Alternate {
                div {
                    style: "
                        position: absolute;
                        left: 50%;
                        top: 0;
                        bottom: 0;
                        width: 2px;
                        background: {border_color};
                        transform: translateX(-50%);
                    ",
                }
            }

            {props.children}
        }
    }
}

/// Timeline item props
#[derive(Props, Clone, PartialEq)]
pub struct TimelineItemProps {
    /// Content for this timeline item
    pub children: Element,
    /// Custom dot element (replaces default dot)
    #[props(default)]
    pub dot: Option<Element>,
    /// Custom dot color
    #[props(default)]
    pub dot_color: Option<Color>,
    /// Icon name to display inside dot
    #[props(default)]
    pub icon: Option<String>,
    /// Whether this is the last item (no connector line after)
    #[props(default = false)]
    pub last: bool,
}

/// Individual timeline item component
///
/// Represents a single event in the timeline with a dot marker
/// and associated content.
#[component]
pub fn TimelineItem(props: TimelineItemProps) -> Element {
    let context = use_timeline_context();
    let position = context.position.clone();

    let theme = use_theme();
    let border_color = theme.tokens.read().colors.border.to_rgba();

    // Determine flex direction based on position
    let (item_style, content_style) = match position {
        TimelinePosition::Left => (
            Style::new()
                .flex()
                .flex_row()
                .relative()
                .w_full()
                .mb_px(if props.last { 0 } else { 24 })
                .build(),
            "flex: 1; padding-right: 24px; text-align: right;",
        ),
        TimelinePosition::Right => (
            Style::new()
                .flex()
                .flex_row()
                .relative()
                .w_full()
                .mb_px(if props.last { 0 } else { 24 })
                .build(),
            "flex: 1; padding-left: 24px;",
        ),
        TimelinePosition::Alternate => (
            Style::new()
                .flex()
                .flex_row()
                .relative()
                .w_full()
                .mb_px(if props.last { 0 } else { 24 })
                .build(),
            "flex: 1; padding: 0 24px;",
        ),
    };

    rsx! {
        div {
            style: item_style,

            // Content area
            div {
                style: content_style,
                {props.children}
            }

            // Separator with dot
            TimelineSeparator {
                dot: props.dot.clone(),
                dot_color: props.dot_color.clone(),
                icon: props.icon.clone(),
                last: props.last,
                line_color: border_color.clone(),
            }

            // Empty space for alternate layout
            if position == TimelinePosition::Alternate {
                div {
                    style: "flex: 1;",
                }
            }
        }
    }
}

/// Timeline separator props
#[derive(Props, Clone, PartialEq)]
pub struct TimelineSeparatorProps {
    /// Custom dot element
    #[props(default)]
    pub dot: Option<Element>,
    /// Custom dot color
    #[props(default)]
    pub dot_color: Option<Color>,
    /// Icon name for dot
    #[props(default)]
    pub icon: Option<String>,
    /// Whether this is the last item
    #[props(default = false)]
    pub last: bool,
    /// Line color
    #[props(default)]
    pub line_color: Option<String>,
}

/// Timeline separator component
///
/// Contains the dot marker and connecting line between items.
#[component]
pub fn TimelineSeparator(props: TimelineSeparatorProps) -> Element {
    let theme = use_theme();
    let default_line_color = theme.tokens.read().colors.border.to_rgba();
    let line_color = props.line_color.clone().unwrap_or(default_line_color);

    let separator_style = Style::new()
        .flex()
        .flex_col()
        .items_center()
        .relative()
        .build();

    rsx! {
        div {
            style: separator_style,

            // Dot
            TimelineDot {
                dot: props.dot.clone(),
                color: props.dot_color.clone(),
                icon: props.icon.clone(),
            }

            // Connector line (if not last item)
            if !props.last {
                div {
                    style: "
                        width: 2px;
                        flex: 1;
                        min-height: 40px;
                        background: {line_color};
                        margin-top: 4px;
                    ",
                }
            }
        }
    }
}

/// Timeline dot props
#[derive(Props, Clone, PartialEq)]
pub struct TimelineDotProps {
    /// Custom dot element (replaces default)
    #[props(default)]
    pub dot: Option<Element>,
    /// Dot background color
    #[props(default)]
    pub color: Option<Color>,
    /// Icon name to display inside dot
    #[props(default)]
    pub icon: Option<String>,
    /// Dot size
    #[props(default = TimelineDotSize::Md)]
    pub size: TimelineDotSize,
}

/// Timeline dot size
#[derive(Clone, PartialEq, Default, Debug)]
pub enum TimelineDotSize {
    /// Small dot (12px)
    Sm,
    /// Medium dot (16px) - default
    #[default]
    Md,
    /// Large dot (24px)
    Lg,
}

/// Timeline dot component
///
/// The circular marker displayed on the timeline.
/// Can display an icon or custom content.
#[component]
pub fn TimelineDot(props: TimelineDotProps) -> Element {
    let theme = use_theme();

    // Use provided color or default to primary
    let bg_color = props
        .color
        .clone()
        .unwrap_or_else(|| theme.tokens.read().colors.primary.clone())
        .to_rgba();

    let fg_color = props
        .color
        .clone()
        .map(|c| {
            // Determine if color is dark to choose appropriate text color
            let luminance = (0.299 * c.r as f32 + 0.587 * c.g as f32 + 0.114 * c.b as f32) / 255.0;
            if luminance < 0.5 {
                "white".to_string()
            } else {
                "black".to_string()
            }
        })
        .unwrap_or_else(|| theme.tokens.read().colors.primary_foreground.to_rgba());

    // Size dimensions
    let (size, font_size) = match props.size {
        TimelineDotSize::Sm => ("12px", "8px"),
        TimelineDotSize::Md => ("16px", "10px"),
        TimelineDotSize::Lg => ("24px", "14px"),
    };

    // Use custom dot if provided
    if let Some(custom_dot) = props.dot {
        return rsx! { {custom_dot} };
    }

    let dot_style = format!(
        "width: {}; height: {}; border-radius: 50%; background: {}; display: flex; align-items: center; justify-content: center; flex-shrink: 0; box-shadow: 0 0 0 2px {};",
        size,
        size,
        bg_color,
        theme.tokens.read().colors.background.to_rgba()
    );

    rsx! {
        div {
            style: dot_style,

            if let Some(icon_name) = props.icon.clone() {
                // Simple icon display using Unicode symbols
                span {
                    style: "color: {fg_color}; font-size: {font_size}; line-height: 1;",
                    match icon_name.as_str() {
                        "check" => "✓",
                        "x" | "close" => "✕",
                        "plus" => "+",
                        "minus" => "−",
                        "star" => "★",
                        "heart" => "♥",
                        "bell" => "🔔",
                        "calendar" => "📅",
                        "user" => "👤",
                        "mail" => "✉",
                        "phone" => "📞",
                        "location" | "pin" => "📍",
                        "flag" => "🚩",
                        "rocket" => "🚀",
                        "fire" => "🔥",
                        "bolt" | "lightning" => "⚡",
                        "info" => "ℹ",
                        "warning" => "⚠",
                        "error" | "alert" => "⚠",
                        _ => "●",
                    }
                }
            } else {
                // Default filled circle (no content needed)
            }
        }
    }
}

/// Timeline content props
#[derive(Props, Clone, PartialEq)]
pub struct TimelineContentProps {
    /// Content children
    pub children: Element,
    /// Optional title
    #[props(default)]
    pub title: Option<String>,
    /// Optional subtitle/timestamp
    #[props(default)]
    pub subtitle: Option<String>,
    /// Content alignment
    #[props(default)]
    pub align: Option<String>,
}

/// Timeline content component
///
/// Wrapper for timeline item content with optional title and subtitle styling.
#[component]
pub fn TimelineContent(props: TimelineContentProps) -> Element {
    let theme = use_theme();
    let text_color = theme.tokens.read().colors.foreground.to_rgba();
    let muted_color = theme.tokens.read().colors.muted_foreground.to_rgba();

    let align = props.align.clone().unwrap_or_else(|| "left".to_string());

    let content_style = Style::new()
        .flex()
        .flex_col()
        .gap_px(4)
        .text_align(&align)
        .build();

    rsx! {
        div {
            style: content_style,

            if let Some(title) = props.title.clone() {
                h4 {
                    style: "margin: 0; color: {text_color}; font-size: 16px; font-weight: 600;",
                    "{title}"
                }
            }

            if let Some(subtitle) = props.subtitle.clone() {
                span {
                    style: "color: {muted_color}; font-size: 12px;",
                    "{subtitle}"
                }
            }

            div {
                style: "color: {text_color}; font-size: 14px;",
                {props.children}
            }
        }
    }
}

/// Timeline opposite content props
///
/// Content displayed on the opposite side of the timeline (for alternate layout)
#[derive(Props, Clone, PartialEq)]
pub struct TimelineOppositeContentProps {
    /// Content children
    pub children: Element,
    /// Text alignment
    #[props(default)]
    pub align: Option<String>,
}

/// Timeline opposite content component
///
/// Displays content on the opposite side of the timeline item.
/// Useful for showing timestamps or supplementary information.
#[component]
pub fn TimelineOppositeContent(props: TimelineOppositeContentProps) -> Element {
    let theme = use_theme();
    let muted_color = theme.tokens.read().colors.muted_foreground.to_rgba();

    let align = props.align.clone().unwrap_or_else(|| "right".to_string());

    let style = Style::new()
        .flex_grow(1)
        .px_px(24)
        .text_align(&align)
        .build();

    rsx! {
        div {
            style: "{style} color: {muted_color}; font-size: 14px;",
            {props.children}
        }
    }
}

/// Convenience builder for creating timeline items
#[derive(Clone, PartialEq)]
pub struct TimelineEvent {
    /// Event title
    pub title: String,
    /// Event description
    pub description: Option<String>,
    /// Event timestamp
    pub timestamp: Option<String>,
    /// Dot color
    pub dot_color: Option<Color>,
    /// Icon name
    pub icon: Option<String>,
}

impl TimelineEvent {
    /// Create a new timeline event
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            timestamp: None,
            dot_color: None,
            icon: None,
        }
    }

    /// Add description
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Add timestamp
    pub fn with_timestamp(mut self, timestamp: impl Into<String>) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }

    /// Set dot color
    pub fn with_color(mut self, color: Color) -> Self {
        self.dot_color = Some(color);
        self
    }

    /// Set icon
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

/// Simple timeline builder component props
#[derive(Props, Clone, PartialEq)]
pub struct SimpleTimelineProps {
    /// List of timeline events
    pub events: Vec<TimelineEvent>,
    /// Timeline position
    #[props(default)]
    pub position: TimelinePosition,
}

/// Simple timeline builder component
///
/// Creates a timeline from a list of TimelineEvent items.
#[component]
pub fn SimpleTimeline(props: SimpleTimelineProps) -> Element {
    let events_len = props.events.len();

    rsx! {
        Timeline {
            position: props.position.clone(),

            for (index, event) in props.events.iter().enumerate() {
                TimelineItem {
                    key: "{index}",
                    dot_color: event.dot_color.clone(),
                    icon: event.icon.clone(),
                    last: index == events_len - 1,

                    TimelineContent {
                        title: Some(event.title.clone()),
                        subtitle: event.timestamp.clone(),

                        if let Some(desc) = event.description.clone() {
                            p {
                                style: "margin: 4px 0 0 0;",
                                "{desc}"
                            }
                        }
                    }
                }
            }
        }
    }
}
