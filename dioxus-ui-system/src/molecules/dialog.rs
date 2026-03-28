//! Dialog molecule component
//!
//! A window overlaid on either the primary window or another dialog window.

use crate::atoms::{Box, Button, ButtonVariant};
use crate::styles::Style;
use crate::theme::{use_style, use_theme};
use dioxus::prelude::*;

/// Dialog properties
#[derive(Props, Clone, PartialEq)]
pub struct DialogProps {
    /// Whether the dialog is open
    pub open: bool,
    /// Callback when dialog should close
    pub on_close: EventHandler<()>,
    /// Dialog content
    pub children: Element,
    /// Dialog title
    #[props(default)]
    pub title: Option<String>,
    /// Dialog description
    #[props(default)]
    pub description: Option<String>,
    /// Whether to show close button
    #[props(default = true)]
    pub show_close_button: bool,
    /// Whether clicking overlay closes the dialog
    #[props(default = true)]
    pub close_on_overlay_click: bool,
    /// Custom inline styles for the content
    #[props(default)]
    pub content_style: Option<String>,
}

/// Dialog molecule component
#[component]
pub fn Dialog(props: DialogProps) -> Element {
    let _theme = use_theme();

    if !props.open {
        return rsx! {};
    }

    let overlay_style = use_style(|_t| {
        Style::new()
            .fixed()
            .top("0")
            .left("0")
            .w_full()
            .h_full()
            .bg(&Color::new_rgba(0, 0, 0, 0.5))
            .z_index(100)
            .flex()
            .items_center()
            .justify_center()
            .build()
    });

    let content_style = use_style(|t| {
        Style::new()
            .w_full()
            .max_w_px(512)
            .max_h_px(600)
            .rounded(&t.radius, "lg")
            .bg(&t.colors.background)
            .shadow(&t.shadows.xl)
            .overflow_hidden()
            .flex()
            .flex_col()
            .build()
    });

    let handle_overlay_click = move |_| {
        if props.close_on_overlay_click {
            props.on_close.call(());
        }
    };

    let custom_content_style = props.content_style.clone().unwrap_or_default();

    rsx! {
        div {
            style: "{overlay_style}",
            onclick: handle_overlay_click,

            div {
                style: "{content_style} {custom_content_style}",
                onclick: move |e| e.stop_propagation(),

                // Header
                if props.title.is_some() || props.show_close_button {
                    DialogHeader {
                        title: props.title.clone(),
                        show_close_button: props.show_close_button,
                        on_close: props.on_close.clone(),
                    }
                }

                // Description
                if let Some(description) = props.description.clone() {
                    DialogDescription { description: description }
                }

                // Content
                Box {
                    style: "padding: 0 24px 24px 24px; overflow-y: auto;",
                    {props.children}
                }
            }
        }
    }
}

use crate::theme::tokens::Color;

#[derive(Props, Clone, PartialEq)]
struct DialogHeaderProps {
    title: Option<String>,
    show_close_button: bool,
    on_close: EventHandler<()>,
}

#[component]
fn DialogHeader(props: DialogHeaderProps) -> Element {
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

    rsx! {
        div {
            style: "{header_style}",

            if let Some(title) = props.title {
                h2 {
                    style: "margin: 0; font-size: 18px; font-weight: 600;",
                    "{title}"
                }
            } else {
                div {}
            }

            if props.show_close_button {
                Button {
                    variant: ButtonVariant::Ghost,
                    onclick: move |_| props.on_close.call(()),
                    "✕"
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct DialogDescriptionProps {
    description: String,
}

#[component]
fn DialogDescription(props: DialogDescriptionProps) -> Element {
    rsx! {
        p {
            style: "margin: 0; padding: 16px 24px 0 24px; font-size: 14px; color: #64748b;",
            "{props.description}"
        }
    }
}

/// Dialog footer component for action buttons
#[derive(Props, Clone, PartialEq)]
pub struct DialogFooterProps {
    /// Footer content (usually buttons)
    pub children: Element,
    /// Align content
    #[props(default)]
    pub align: DialogFooterAlign,
}

/// Dialog footer alignment
#[derive(Default, Clone, PartialEq)]
pub enum DialogFooterAlign {
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

/// Dialog footer component
#[component]
pub fn DialogFooter(props: DialogFooterProps) -> Element {
    let _theme = use_theme();

    let justify = match props.align {
        DialogFooterAlign::Start => "flex-start",
        DialogFooterAlign::Center => "center",
        DialogFooterAlign::End => "flex-end",
        DialogFooterAlign::Between => "space-between",
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
            style: "{footer_style} justify-content: {justify};",
            {props.children}
        }
    }
}

/// Alert dialog for important confirmations
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogProps {
    /// Whether the dialog is open
    pub open: bool,
    /// Callback when dialog should close
    pub on_close: EventHandler<()>,
    /// Dialog title
    pub title: String,
    /// Dialog description
    pub description: String,
    /// Cancel button text
    #[props(default = "Cancel".to_string())]
    pub cancel_text: String,
    /// Confirm button text
    #[props(default = "Confirm".to_string())]
    pub confirm_text: String,
    /// Callback when confirmed
    pub on_confirm: EventHandler<()>,
    /// Whether the action is destructive
    #[props(default)]
    pub destructive: bool,
}

/// Alert dialog component
#[component]
pub fn AlertDialog(props: AlertDialogProps) -> Element {
    let confirm_variant = if props.destructive {
        ButtonVariant::Destructive
    } else {
        ButtonVariant::Primary
    };

    rsx! {
        Dialog {
            open: props.open,
            on_close: props.on_close.clone(),
            title: props.title.clone(),
            show_close_button: false,
            close_on_overlay_click: false,

            p {
                style: "margin: 0 0 24px 0; font-size: 14px; color: #64748b; line-height: 1.5;",
                "{props.description}"
            }

            DialogFooter {
                align: DialogFooterAlign::End,

                Button {
                    variant: ButtonVariant::Ghost,
                    onclick: move |_| props.on_close.call(()),
                    "{props.cancel_text}"
                }

                Button {
                    variant: confirm_variant,
                    onclick: move |_| props.on_confirm.call(()),
                    "{props.confirm_text}"
                }
            }
        }
    }
}
