//! Sheet molecule component
//!
//! A side panel/drawer that slides in from any edge of the screen.
//! Similar to shadcn/ui Sheet component.

use dioxus::prelude::*;
use crate::theme::{use_theme, use_style};
use crate::styles::Style;
use crate::atoms::{Button, ButtonVariant, Box};
use crate::theme::tokens::Color;

#[cfg(all(feature = "web", target_arch = "wasm32"))]
use wasm_bindgen::prelude::*;
#[cfg(all(feature = "web", target_arch = "wasm32"))]
use wasm_bindgen::JsCast;
#[cfg(all(feature = "web", target_arch = "wasm32"))]
use wasm_bindgen_futures::JsFuture;
#[cfg(all(feature = "web", target_arch = "wasm32"))]
use js_sys::Promise;

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
    pub on_open_change: EventHandler<bool>,
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
#[component]
pub fn Sheet(props: SheetProps) -> Element {
    let _theme = use_theme();
    let side = props.side.clone();
    let side_for_transform = props.side.clone();

    // Handle escape key
    use_effect(move || {
        if !props.open {
            return;
        }
        
        // Register keyboard event listener
        #[cfg(all(feature = "web", target_arch = "wasm32"))]
        {
            let on_close = props.on_open_change.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                if event.key() == "Escape" {
                    on_close.call(false);
                }
            }) as Box<dyn FnMut(_)>);
            if let Some(window) = web_sys::window() {
                window.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).unwrap();
            }
            closure.forget(); // Keep the closure alive
        }
        
        #[cfg(not(all(feature = "web", target_arch = "wasm32")))]
        {
            // Keyboard handling not implemented for non-web targets
            let _ = props.on_open_change;
        }
    });

    if !props.open {
        return rsx! {};
    }

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

    let overlay_opacity = if props.open { "1" } else { "0" };
    let overlay_transition = "transition: opacity 0.3s ease-out;";

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

    // Transform for slide animation
    let transform = if props.open {
        side_for_transform.open_transform()
    } else {
        side_for_transform.closed_transform()
    };
    let sheet_transition = "transition: transform 0.3s cubic-bezier(0.16, 1, 0.3, 1);";

    let handle_overlay_click = move |_| {
        if props.close_on_overlay_click {
            props.on_open_change.call(false);
        }
    };

    rsx! {
        // Overlay - separate element at root level
        div {
            style: "{overlay_style} opacity: {overlay_opacity}; {overlay_transition}",
            onclick: handle_overlay_click,
        }
        
        // Sheet content - sibling to overlay, prevents click propagation issues
        div {
            style: "{sheet_style} transform: {transform}; {sheet_transition}",
            onclick: move |e| e.stop_propagation(),
            
            // Header
            if props.title.is_some() || props.show_close_button {
                SheetHeader {
                    title: props.title.clone(),
                    description: props.description.clone(),
                    show_close_button: props.show_close_button,
                    on_close: props.on_open_change.clone(),
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
    description: Option<String>,
    show_close_button: bool,
    on_close: EventHandler<bool>,
}

#[component]
fn SheetHeader(props: SheetHeaderProps) -> Element {
    let _theme = use_theme();
    
    let header_style = use_style(|t| {
        Style::new()
            .flex()
            .items_center()
            .justify_between
            ()
            .p(&t.spacing, "lg")
            .border_bottom(1, &t.colors.border)
            .build()
    });
    
    let title_section_style = use_style(|_| {
        Style::new()
            .flex()
            .flex_col()
            .gap_px(4)
            .build()
    });
    
    rsx! {
        div {
            style: "{header_style}",
            
            if props.title.is_some() || props.description.is_some() {
                div {
                    style: "{title_section_style}",
                    
                    if let Some(title) = props.title {
                        h2 {
                            style: "margin: 0; font-size: 18px; font-weight: 600;",
                            "{title}"
                        }
                    }
                    
                    if let Some(description) = props.description {
                        p {
                            style: "margin: 0; font-size: 14px; color: #64748b;",
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
                    onclick: move |_| props.on_close.call(false),
                    "✕"
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
            .items_center
            ()
            .gap(&t.spacing, "sm")
            .p(&t.spacing, "lg")
            .border_top(1, &t.colors.border)
            .build()
    });
    
    rsx! {
        div {
            style: "{footer_style} justify-content: {justify};",
            {props.children}
        }
    }
}
