//! Collapsible molecule component
//!
//! A component that shows/hides content with smooth animation.
//! Supports both controlled and uncontrolled modes.

use crate::styles::Style;
use crate::theme::{use_style, use_theme};
use dioxus::prelude::*;

/// Collapsible properties
#[derive(Props, Clone, PartialEq)]
pub struct CollapsibleProps {
    /// Whether the collapsible is open (controlled mode)
    #[props(default)]
    pub open: Option<bool>,
    /// Callback when open state changes
    #[props(default)]
    pub on_open_change: Option<EventHandler<bool>>,
    /// The clickable header/trigger element
    pub trigger: Element,
    /// The collapsible content
    pub children: Element,
    /// Default open state for uncontrolled mode
    #[props(default)]
    pub default_open: bool,
    /// Whether to show the default chevron icon
    #[props(default = true)]
    pub show_chevron: bool,
    /// Custom chevron element (replaces default if provided)
    #[props(default)]
    pub chevron: Option<Element>,
    /// Custom inline styles for the container
    #[props(default)]
    pub style: Option<String>,
    /// Custom inline styles for the trigger
    #[props(default)]
    pub trigger_style: Option<String>,
    /// Custom inline styles for the content
    #[props(default)]
    pub content_style: Option<String>,
    /// Whether the collapsible is disabled
    #[props(default)]
    pub disabled: bool,
    /// Transition duration in milliseconds
    #[props(default = 200)]
    pub transition_duration: u16,
}

/// Collapsible molecule component
///
/// # Example
/// ```rust,ignore
/// use dioxus_ui_system::molecules::{Collapsible, CollapsibleProps};
///
/// fn MyComponent() -> Element {
///     rsx! {
///         Collapsible {
///             trigger: rsx! { "Click to expand" },
///             children: rsx! { "This content is collapsible" },
///         }
///     }
/// }
/// ```
#[component]
pub fn Collapsible(props: CollapsibleProps) -> Element {
    let _theme = use_theme();
    let mut internal_open = use_signal(|| props.default_open);

    // Determine if we're in controlled mode and get current open state
    let is_controlled = props.open.is_some();
    let is_open = if is_controlled {
        props.open.unwrap_or(false)
    } else {
        internal_open()
    };
    let is_disabled = props.disabled;

    // Generate unique IDs for accessibility
    let collapsible_id = use_memo(|| format!("collapsible-{}", generate_id()));
    let content_id = use_memo(move || format!("{}-content", collapsible_id()));

    // Handle toggle
    let handle_toggle = move |_| {
        if is_disabled {
            return;
        }

        let new_state = !is_open;

        // Update internal state if uncontrolled
        if props.open.is_none() {
            internal_open.set(new_state);
        }

        // Call callback if provided
        if let Some(on_change) = &props.on_open_change {
            on_change.call(new_state);
        }
    };

    // Container style
    let container_style = use_style(|t| {
        Style::new()
            .w_full()
            .border(1, &t.colors.border)
            .rounded(&t.radius, "md")
            .bg(&t.colors.background)
            .overflow_hidden()
            .build()
    });

    // Trigger/header style
    let trigger_base_style = use_style(move |t| {
        Style::new()
            .w_full()
            .flex()
            .items_center()
            .justify_between()
            .px(&t.spacing, "lg")
            .py(&t.spacing, "md")
            .bg_transparent()
            .border(0, &t.colors.border)
            .outline("none")
            .cursor(if is_disabled {
                "not-allowed"
            } else {
                "pointer"
            })
            .text_color(&t.colors.foreground)
            .text(&t.typography, "base")
            .font_weight(500)
            .transition("all 150ms ease")
            .opacity(if is_disabled { 0.5 } else { 1.0 })
            .build()
    });

    // Content wrapper style with animation
    let duration = props.transition_duration;
    let content_wrapper_style = use_style(move |_t| {
        let transition_str = format!("height {}ms ease, opacity {}ms ease", duration, duration);
        let base = Style::new()
            .w_full()
            .overflow_hidden()
            .transition(&transition_str);

        if is_open {
            base.opacity(1.0)
        } else {
            base.opacity(0.0)
        }
        .build()
    });

    // Content inner style
    let content_inner_style = use_style(|t| {
        Style::new()
            .px(&t.spacing, "lg")
            .pb(&t.spacing, "md")
            .text(&t.typography, "sm")
            .text_color(&t.colors.muted_foreground)
            .line_height(1.6)
            .build()
    });

    // Chevron rotation
    let chevron_rotation = if is_open {
        "rotate(180deg)"
    } else {
        "rotate(0deg)"
    };

    rsx! {
        div {
            style: "{container_style} {props.style.clone().unwrap_or_default()}",
            id: "{collapsible_id}",

            // Trigger button
            button {
                style: "{trigger_base_style} {props.trigger_style.clone().unwrap_or_default()}",
                type: "button",
                aria_expanded: "{is_open}",
                aria_controls: "{content_id}",
                disabled: is_disabled,
                onclick: handle_toggle,

                // Trigger content wrapper
                div {
                    style: "flex: 1; display: flex; align-items: center;",
                    {props.trigger}
                }

                // Chevron indicator
                if props.show_chevron {
                    if let Some(custom_chevron) = props.chevron {
                        span {
                            style: "transform: {chevron_rotation}; transition: transform {props.transition_duration}ms ease; flex-shrink: 0; margin-left: 8px;",
                            {custom_chevron}
                        }
                    } else {
                        span {
                            style: "transform: {chevron_rotation}; transition: transform {props.transition_duration}ms ease; flex-shrink: 0; margin-left: 8px;",
                            CollapsibleChevron {}
                        }
                    }
                }
            }

            // Collapsible content
            div {
                style: "{content_wrapper_style}",
                id: "{content_id}",
                aria_hidden: "{!is_open}",

                // Use a conditional render with animation support
                if is_open {
                    div {
                        style: "{content_inner_style} {props.content_style.clone().unwrap_or_default()}",
                        {props.children}
                    }
                }
            }
        }
    }
}

/// Default chevron icon component
#[component]
fn CollapsibleChevron() -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            style: "width: 16px; height: 16px; display: block;",
            polyline { points: "6 9 12 15 18 9" }
        }
    }
}

/// Simple counter for generating unique IDs
static ID_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Generate a unique ID
fn generate_id() -> u64 {
    ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

/// Simple collapsible variant with text trigger
#[derive(Props, Clone, PartialEq)]
pub struct SimpleCollapsibleProps {
    /// Trigger text
    pub trigger_text: String,
    /// The collapsible content
    pub children: Element,
    /// Whether the collapsible is open (controlled mode)
    #[props(default)]
    pub open: Option<bool>,
    /// Callback when open state changes
    #[props(default)]
    pub on_open_change: Option<EventHandler<bool>>,
    /// Default open state for uncontrolled mode
    #[props(default)]
    pub default_open: bool,
    /// Whether the collapsible is disabled
    #[props(default)]
    pub disabled: bool,
}

/// Simple collapsible with text trigger
#[component]
pub fn SimpleCollapsible(props: SimpleCollapsibleProps) -> Element {
    rsx! {
        Collapsible {
            open: props.open,
            on_open_change: props.on_open_change,
            default_open: props.default_open,
            disabled: props.disabled,
            trigger: rsx! { "{props.trigger_text}" },
            {props.children}
        }
    }
}

/// Collapsible group for managing multiple collapsibles
#[derive(Props, Clone, PartialEq)]
pub struct CollapsibleGroupProps {
    /// Child collapsibles or content
    pub children: Element,
    /// Allow multiple items to be open simultaneously
    #[props(default)]
    pub allow_multiple: bool,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Gap between collapsible items
    #[props(default)]
    pub gap: Option<String>,
}

/// Group container for multiple collapsibles
#[component]
pub fn CollapsibleGroup(props: CollapsibleGroupProps) -> Element {
    let _theme = use_theme();

    let gap_style = props.gap.clone().unwrap_or_else(|| "8px".to_string());

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: {gap_style}; {props.style.clone().unwrap_or_default()}",
            {props.children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collapsible_props_creation() {
        // Test that props can be created with defaults
        let _props = CollapsibleProps {
            open: Some(true),
            on_open_change: None,
            trigger: rsx! { "Test" },
            children: rsx! { "Content" },
            default_open: false,
            show_chevron: true,
            chevron: None,
            style: None,
            trigger_style: None,
            content_style: None,
            disabled: false,
            transition_duration: 200,
        };
    }

    #[test]
    fn test_simple_collapsible_props() {
        let _props = SimpleCollapsibleProps {
            trigger_text: "Click me".to_string(),
            children: rsx! { "Content" },
            open: None,
            on_open_change: None,
            default_open: false,
            disabled: false,
        };
    }

    #[test]
    fn test_unique_id_generation() {
        let id1 = generate_id();
        let id2 = generate_id();
        assert_ne!(id1, id2);
    }
}
