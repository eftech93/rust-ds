//! Sheet molecule component
//!
//! A side panel/drawer that slides in from any edge of the screen.
//! Similar to shadcn/ui Sheet component.

use super::dialog::{is_dismiss_key, resolve_base_id, FocusTrapScope, SR_ONLY_STYLE};
use crate::a11y::use_unique_id;
use crate::atoms::{Box, Button, ButtonVariant};
use crate::styles::Style;
use crate::theme::tokens::Color;
use crate::theme::{use_style, use_theme};
use dioxus::prelude::*;

/// Which edge the sheet slides in from
#[derive(Default, Clone, PartialEq)]
pub enum SheetSide {
    /// Slides in from the top
    Top,
    /// Slides in from the right (default)
    #[default]
    Right,
    /// Slides in from the bottom
    Bottom,
    /// Slides in from the left
    Left,
}

impl SheetSide {
    /// Get the CSS transform value for the closed state
    fn closed_transform(&self) -> &'static str {
        match self {
            SheetSide::Top => "translateY(-100%)",
            SheetSide::Right => "translateX(100%)",
            SheetSide::Bottom => "translateY(100%)",
            SheetSide::Left => "translateX(-100%)",
        }
    }

    /// Get the CSS transform value for the open state
    fn open_transform(&self) -> &'static str {
        "translate(0, 0)"
    }

    /// Check if this is a horizontal side (left or right)
    #[allow(dead_code)]
    fn is_horizontal(&self) -> bool {
        matches!(self, SheetSide::Left | SheetSide::Right)
    }

    /// Check if this is a vertical side (top or bottom)
    #[allow(dead_code)]
    fn is_vertical(&self) -> bool {
        matches!(self, SheetSide::Top | SheetSide::Bottom)
    }
}

/// Sheet component properties
#[derive(Props, Clone, PartialEq)]
pub struct SheetProps {
    /// Whether the sheet is open
    pub open: bool,
    /// Callback when open state changes
    #[props(default)]
    pub on_open_change: Option<EventHandler<bool>>,
    /// Which edge to slide in from
    #[props(default)]
    pub side: SheetSide,
    /// Sheet content
    pub children: Element,
    /// Sheet title (shown in header)
    #[props(default)]
    pub title: Option<String>,
    /// Sheet description (shown below title)
    #[props(default)]
    pub description: Option<String>,
    /// Whether to show the close button
    #[props(default = true)]
    pub show_close_button: bool,
    /// Whether clicking the overlay closes the sheet
    #[props(default = true)]
    pub close_on_overlay_click: bool,
    /// Element id
    #[props(default)]
    pub id: Option<String>,
    /// Custom CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Accessible name, for sheets without a `title`
    #[props(default)]
    pub aria_label: Option<String>,
}

/// Sheet molecule component
///
/// A side panel that slides in from any edge of the screen.
///
/// # Example
/// ```rust,ignore
/// use dioxus::prelude::*;
/// use dioxus_ui_system::molecules::{Sheet, SheetSide};
///
/// fn MyComponent() -> Element {
///     let mut open = use_signal(|| false);
///     
///     rsx! {
///         button {
///             onclick: move |_| open.set(true),
///             "Open Sheet"
///         }
///         
///         Sheet {
///             open: open(),
///             on_open_change: move |is_open| open.set(is_open),
///             side: SheetSide::Right,
///             title: "Sheet Title".to_string(),
///             "Sheet content goes here"
///         }
///     }
/// }
/// ```
///
/// # Accessibility
///
/// - The panel has `role="dialog"` and `aria-modal="true"`; `aria-labelledby`
///   / `aria-describedby` point at the rendered title / description (ids
///   derive from the `id` prop when set). Use `aria_label` when there is no
///   title.
/// - Escape calls `on_open_change(false)`.
/// - While open, Tab / Shift+Tab cycle inside the panel; when it closes, focus
///   returns to the element that was focused before it opened.
#[component]
pub fn Sheet(props: SheetProps) -> Element {
    let _theme = use_theme();
    let side = props.side.clone();
    let side_for_transform = props.side.clone();
    let auto_id = use_unique_id("sheet");

    let overlay_style = use_style(|_| {
        Style::new()
            .fixed()
            .inset("0")
            .w_full()
            .h_full()
            .bg(&Color::new_rgba(0, 0, 0, 0.5))
            .z_index(9999)
            .build()
    });

    // Build sheet content style based on side
    let sheet_style = use_style(move |t| {
        let mut style = Style::new()
            .fixed()
            .z_index(10000)
            .bg(&t.colors.background)
            .shadow(&t.shadows.xl)
            .overflow_hidden()
            .flex()
            .flex_col();

        // Position and sizing based on side
        style = match side {
            SheetSide::Top => style
                .top("0")
                .left("0")
                .right("0")
                .h_px(300)
                .max_h("85vh")
                .rounded(&t.radius, "lg")
                .rounded_px(0), // Remove border radius on bottom
            SheetSide::Right => style
                .top("0")
                .right("0")
                .bottom("0")
                .w_px(400)
                .max_w("100%")
                .rounded(&t.radius, "lg")
                .rounded_px(0), // Remove border radius on left
            SheetSide::Bottom => style
                .left("0")
                .right("0")
                .bottom("0")
                .h_px(300)
                .max_h("85vh")
                .rounded(&t.radius, "lg")
                .rounded_px(0), // Remove border radius on top
            SheetSide::Left => style
                .top("0")
                .left("0")
                .bottom("0")
                .w_px(400)
                .max_w("100%")
                .rounded(&t.radius, "lg")
                .rounded_px(0), // Remove border radius on right
        };

        style.build()
    });

    // Hooks above run unconditionally; only rendering depends on `open`.
    if !props.open {
        return rsx! {};
    }

    let overlay_opacity = if props.open { "1" } else { "0" };
    let overlay_transition = "transition: opacity 0.3s ease-out;";

    let base_id = resolve_base_id(props.id.as_deref(), &auto_id);
    let content_id = format!("{base_id}-content");
    let title_id = format!("{base_id}-title");
    let description_id = format!("{base_id}-description");
    let header_shown = props.title.is_some() || props.show_close_button;
    let labelledby = props.title.as_ref().map(|_| title_id.clone());
    let describedby = props
        .description
        .as_ref()
        .filter(|_| header_shown)
        .map(|_| description_id.clone());

    // Transform for slide animation
    let transform = if props.open {
        side_for_transform.open_transform()
    } else {
        side_for_transform.closed_transform()
    };
    let sheet_transition = "transition: transform 0.3s cubic-bezier(0.16, 1, 0.3, 1);";

    let handle_overlay_click = move |_| {
        if props.close_on_overlay_click {
            if let Some(handler) = &props.on_open_change {
                handler.call(false);
            }
        }
    };

    let on_open_change = props.on_open_change;
    let handle_keydown = move |e: KeyboardEvent| {
        if is_dismiss_key(&e.key()) {
            e.stop_propagation();
            if let Some(handler) = &on_open_change {
                handler.call(false);
            }
        }
    };

    rsx! {
        // Overlay - separate element at root level
        div {
            id: "{props.id.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            style: "{overlay_style} opacity: {overlay_opacity}; {overlay_transition} {props.style.clone().unwrap_or_default()}",
            onclick: handle_overlay_click,
        }

        // Sheet content - sibling to overlay, prevents click propagation issues
        div {
            id: "{content_id}",
            role: "dialog",
            "aria-modal": "true",
            "aria-labelledby": labelledby,
            "aria-label": props.aria_label.clone(),
            "aria-describedby": describedby,
            tabindex: "-1",
            style: "{sheet_style} transform: {transform}; {sheet_transition}",
            onclick: move |e| e.stop_propagation(),
            onkeydown: handle_keydown,

            FocusTrapScope { container_id: content_id.clone() }

            // Header
            if header_shown {
                SheetHeader {
                    title: props.title.clone(),
                    title_id: title_id.clone(),
                    description: props.description.clone(),
                    description_id: description_id.clone(),
                    show_close_button: props.show_close_button,
                    on_close: props.on_open_change,
                }
            }

            // Content
            Box {
                style: "flex: 1; overflow-y: auto; padding: 24px;",
                {props.children}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct SheetHeaderProps {
    title: Option<String>,
    #[props(default)]
    title_id: Option<String>,
    description: Option<String>,
    #[props(default)]
    description_id: Option<String>,
    show_close_button: bool,
    #[props(default)]
    on_close: Option<EventHandler<bool>>,
    /// Element id
    #[props(default)]
    pub id: Option<String>,
    /// Custom CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
}

#[component]
fn SheetHeader(props: SheetHeaderProps) -> Element {
    let _theme = use_theme();

    let header_style = use_style(|t| {
        Style::new()
            .flex()
            .items_center()
            .justify_between()
            .p(&t.spacing, "lg")
            .border_bottom(1, &t.colors.border)
            .build()
    });

    let title_section_style = use_style(|_| Style::new().flex().flex_col().gap_px(4).build());

    rsx! {
        div {
            id: "{props.id.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            style: "{header_style}",

            if props.title.is_some() || props.description.is_some() {
                div {
                    style: "{title_section_style}",

                    if let Some(title) = props.title {
                        h2 {
                            id: props.title_id.clone(),
                            style: "margin: 0; font-size: 18px; font-weight: 600;",
                            "{title}"
                        }
                    }

                    if let Some(description) = props.description {
                        p {
                            id: props.description_id.clone(),
                            style: "margin: 0; font-size: 14px; color: #64748b; {props.style.clone().unwrap_or_default()}",
                            "{description}"
                        }
                    }
                }
            } else {
                div {}
            }

            if props.show_close_button {
                Button {
                    variant: ButtonVariant::Ghost,
                    on_click: move |_| {
                        if let Some(handler) = &props.on_close {
                            handler.call(false);
                        }
                    },
                    span { "aria-hidden": "true", "✕" }
                    span { style: SR_ONLY_STYLE, "Close" }
                }
            }
        }
    }
}

/// Sheet footer component for action buttons
#[derive(Props, Clone, PartialEq)]
pub struct SheetFooterProps {
    /// Footer content (usually buttons)
    pub children: Element,
    /// Align content
    #[props(default)]
    pub align: SheetFooterAlign,
    /// Element id
    #[props(default)]
    pub id: Option<String>,
    /// Custom CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Sheet footer alignment
#[derive(Default, Clone, PartialEq)]
pub enum SheetFooterAlign {
    /// Start alignment
    #[default]
    Start,
    /// Center alignment
    Center,
    /// End alignment
    End,
    /// Space between
    Between,
}

/// Sheet footer component
#[component]
pub fn SheetFooter(props: SheetFooterProps) -> Element {
    let _theme = use_theme();

    let justify = match props.align {
        SheetFooterAlign::Start => "flex-start",
        SheetFooterAlign::Center => "center",
        SheetFooterAlign::End => "flex-end",
        SheetFooterAlign::Between => "space-between",
    };

    let footer_style = use_style(|t| {
        Style::new()
            .flex()
            .items_center()
            .gap(&t.spacing, "sm")
            .p(&t.spacing, "lg")
            .border_top(1, &t.colors.border)
            .build()
    });

    rsx! {
        div {
            id: "{props.id.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            style: "{footer_style} justify-content: {justify}; {props.style.clone().unwrap_or_default()}",
            {props.children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(app: fn() -> Element) -> String {
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
        dioxus_ssr::render(&dom)
    }

    #[test]
    fn open_sheet_is_modal_dialog_with_relationships() {
        let html = render(|| {
            rsx! {
                Sheet {
                    open: true,
                    id: "nav",
                    title: "Navigation",
                    description: "Site sections",
                    "Links"
                }
            }
        });
        assert!(html.contains("role=\"dialog\""), "{html}");
        assert!(html.contains("aria-modal=\"true\""), "{html}");
        assert!(html.contains("id=\"nav-content\""), "{html}");
        assert!(html.contains("aria-labelledby=\"nav-title\""), "{html}");
        assert!(html.contains("id=\"nav-title\""), "{html}");
        assert!(
            html.contains("aria-describedby=\"nav-description\""),
            "{html}"
        );
        assert!(html.contains("id=\"nav-description\""), "{html}");
        assert!(html.contains(">Close<"), "{html}");
    }

    #[test]
    fn sheet_without_header_uses_aria_label_only() {
        let html = render(|| {
            rsx! {
                Sheet {
                    open: true,
                    show_close_button: false,
                    description: "Not rendered without a header",
                    aria_label: "Filters",
                    "Body"
                }
            }
        });
        assert!(html.contains("aria-label=\"Filters\""), "{html}");
        assert!(!html.contains("aria-labelledby"), "{html}");
        assert!(!html.contains("aria-describedby"), "{html}");
    }

    #[test]
    fn closed_sheet_renders_nothing() {
        let html = render(|| rsx! { Sheet { open: false, title: "Hidden", "Body" } });
        assert!(!html.contains("role="), "{html}");
        assert!(!html.contains("Hidden"), "{html}");
    }
}
