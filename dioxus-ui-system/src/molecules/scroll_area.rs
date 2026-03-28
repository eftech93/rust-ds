//! ScrollArea molecule component
//!
//! A custom scrollable container with styled scrollbars that match the theme.
//! Supports vertical, horizontal, and bidirectional scrolling with
//! customizable scrollbar appearance.

use crate::styles::Style;
use crate::theme::use_style;
use dioxus::prelude::*;

/// Scroll orientation options
#[derive(Default, Clone, PartialEq)]
pub enum ScrollOrientation {
    /// Vertical scrolling only (default)
    #[default]
    Vertical,
    /// Horizontal scrolling only
    Horizontal,
    /// Both directions
    Both,
}

/// ScrollArea properties
#[derive(Props, Clone, PartialEq)]
pub struct ScrollAreaProps {
    /// Content to be scrolled
    pub children: Element,
    /// Scroll orientation - Vertical, Horizontal, or Both
    #[props(default)]
    pub orientation: ScrollOrientation,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Custom scrollbar size (width for vertical, height for horizontal)
    #[props(default)]
    pub scrollbar_size: Option<String>,
    /// Maximum height for the scroll area
    #[props(default)]
    pub max_height: Option<String>,
    /// Maximum width for the scroll area
    #[props(default)]
    pub max_width: Option<String>,
    /// Whether to auto-hide scrollbar when not scrolling
    #[props(default)]
    pub auto_hide: bool,
}

/// ScrollArea molecule component
///
/// A custom scrollable container with theme-matched scrollbars.
/// Uses CSS custom properties for styling and WebKit scrollbar
/// pseudo-elements for Chrome/Safari, with Firefox fallback.
///
/// # Example
/// ```rust,ignore
/// use dioxus_ui_system::molecules::{ScrollArea, ScrollOrientation};
///
/// rsx! {
///     ScrollArea {
///         max_height: "300px".to_string(),
///         orientation: ScrollOrientation::Vertical,
///         // Content that exceeds max_height will be scrollable
///         for i in 0..50 {
///             p { "Item {i}" }
///         }
///     }
/// }
/// ```
#[component]
pub fn ScrollArea(props: ScrollAreaProps) -> Element {
    let orientation = props.orientation.clone();
    let auto_hide = props.auto_hide;
    let scrollbar_size_val = props
        .scrollbar_size
        .clone()
        .unwrap_or_else(|| "8px".to_string());
    let max_height = props.max_height.clone();
    let max_width = props.max_width.clone();

    // Generate scrollbar colors from theme
    let scrollbar_colors = use_style(|t| {
        let thumb_color = t.colors.border.darken(0.2);
        let thumb_hover_color = t.colors.muted_foreground.clone();
        let track_color = t.colors.muted.lighten(0.5);
        let corner_color = t.colors.background.clone();

        (thumb_color, thumb_hover_color, track_color, corner_color)
    });

    // Build the base container style
    let container_style = use_style(move |t| {
        let (thumb_color, _thumb_hover, track_color, _corner_color) = scrollbar_colors().clone();

        // Set overflow based on orientation
        let (overflow_x, overflow_y) = match orientation {
            ScrollOrientation::Vertical => ("hidden", "auto"),
            ScrollOrientation::Horizontal => ("auto", "hidden"),
            ScrollOrientation::Both => ("auto", "auto"),
        };

        let mut style = Style::new()
            .w_full()
            .h_full()
            .overflow_hidden() // Will be overridden by inline styles for specific axes
            .rounded(&t.radius, "md")
            .bg(&t.colors.background);

        // Add max dimensions if specified
        if let Some(ref max_h) = max_height {
            style = Style {
                max_height: Some(max_h.clone()),
                ..style
            };
        }

        if let Some(ref max_w) = max_width {
            style = Style {
                max_width: Some(max_w.clone()),
                ..style
            };
        }

        // Build base style string
        let base_style = style.build();

        // Add CSS custom properties for scrollbar theming
        format!(
            "{} --scrollbar-thumb: {}; --scrollbar-track: {}; --scrollbar-size: {}; overflow-x: {}; overflow-y: {}; scrollbar-width: thin; scrollbar-color: {} {};",
            base_style,
            thumb_color.to_rgba(),
            track_color.to_rgba(),
            scrollbar_size_val,
            overflow_x,
            overflow_y,
            thumb_color.to_rgba(),
            track_color.to_rgba()
        )
    });

    // Build WebKit scrollbar styles
    let webkit_styles = use_style(move |t| {
        let (thumb_color, thumb_hover_color, track_color, corner_color) =
            scrollbar_colors().clone();
        let size = props
            .scrollbar_size
            .clone()
            .unwrap_or_else(|| "8px".to_string());

        let hover_opacity = if auto_hide { "0" } else { "1" };
        let hover_transition = if auto_hide {
            "transition: opacity 0.2s ease;"
        } else {
            ""
        };

        format!(
            r#"
            .scroll-area::-webkit-scrollbar {{
                width: {size};
                height: {size};
                {hover_transition}
                opacity: {hover_opacity};
            }}
            .scroll-area::-webkit-scrollbar-track {{
                background: {track};
                border-radius: {radius}px;
            }}
            .scroll-area::-webkit-scrollbar-thumb {{
                background: {thumb};
                border-radius: {radius}px;
                border: 2px solid transparent;
                background-clip: content-box;
                transition: background-color 0.2s ease;
            }}
            .scroll-area::-webkit-scrollbar-thumb:hover {{
                background: {thumb_hover};
                border: 2px solid transparent;
                background-clip: content-box;
            }}
            .scroll-area::-webkit-scrollbar-corner {{
                background: {corner};
            }}
            .scroll-area:hover::-webkit-scrollbar {{
                opacity: 1;
            }}
            "#,
            size = size,
            track = track_color.to_rgba(),
            thumb = thumb_color.to_rgba(),
            thumb_hover = thumb_hover_color.to_rgba(),
            corner = corner_color.to_rgba(),
            radius = t.radius.md,
            hover_transition = hover_transition,
            hover_opacity = hover_opacity,
        )
    });

    let custom_class = props.class.clone().unwrap_or_default();
    let custom_style = props.style.clone().unwrap_or_default();

    rsx! {
        // Inject WebKit scrollbar styles
        style { "{webkit_styles}" }

        div {
            class: "scroll-area {custom_class}",
            style: "{container_style} {custom_style}",
            {props.children}
        }
    }
}

/// ScrollArea viewport component for more complex layouts
///
/// Use this when you need a separate viewport with
/// different styling from the scroll container.
#[derive(Props, Clone, PartialEq)]
pub struct ScrollViewportProps {
    /// Content inside the viewport
    pub children: Element,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Scroll viewport component
///
/// A viewport container that fills the available scroll area space.
/// Useful for creating sticky headers/footers within scroll areas.
#[component]
pub fn ScrollViewport(props: ScrollViewportProps) -> Element {
    let viewport_style = use_style(|t| {
        Style::new()
            .w_full()
            .min_h_full()
            .p(&t.spacing, "md")
            .build()
    });

    let custom_class = props.class.clone().unwrap_or_default();
    let custom_style = props.style.clone().unwrap_or_default();

    rsx! {
        div {
            class: "scroll-viewport {custom_class}",
            style: "{viewport_style} {custom_style}",
            {props.children}
        }
    }
}

/// Horizontal scroll area component
///
/// Convenience component for horizontal scrolling with
/// common defaults for horizontal scroll layouts.
#[derive(Props, Clone, PartialEq)]
pub struct HorizontalScrollProps {
    /// Content to be scrolled horizontally
    pub children: Element,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Height of the scroll area
    #[props(default)]
    pub height: Option<String>,
    /// Whether to show scrollbar
    #[props(default = true)]
    pub show_scrollbar: bool,
}

/// Horizontal scroll component
///
/// Optimized for horizontal scrolling content like
/// image galleries, tab lists, etc.
#[component]
pub fn HorizontalScroll(props: HorizontalScrollProps) -> Element {
    rsx! {
        ScrollArea {
            orientation: ScrollOrientation::Horizontal,
            class: props.class.clone(),
            style: props.style.clone(),
            scrollbar_size: if props.show_scrollbar { None } else { Some("0px".to_string()) },
            max_height: props.height.clone(),
            {props.children}
        }
    }
}

/// Vertical scroll area component
///
/// Convenience component for vertical scrolling with
/// common defaults for vertical scroll layouts.
#[derive(Props, Clone, PartialEq)]
pub struct VerticalScrollProps {
    /// Content to be scrolled vertically
    pub children: Element,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Maximum height of the scroll area
    #[props(default)]
    pub max_height: Option<String>,
    /// Whether to auto-hide scrollbar
    #[props(default)]
    pub auto_hide: bool,
}

/// Vertical scroll component
///
/// Optimized for vertical scrolling content like
/// lists, content panels, etc.
#[component]
pub fn VerticalScroll(props: VerticalScrollProps) -> Element {
    rsx! {
        ScrollArea {
            orientation: ScrollOrientation::Vertical,
            class: props.class.clone(),
            style: props.style.clone(),
            max_height: props.max_height.clone(),
            auto_hide: props.auto_hide,
            {props.children}
        }
    }
}
