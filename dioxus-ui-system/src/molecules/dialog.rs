//! Dialog molecule component
//!
//! A window overlaid on either the primary window or another dialog window.
//!
//! Also hosts the small accessibility building blocks shared by the overlay
//! family (Sheet, Popover, ConfirmationDialog): `FocusTrapScope`,
//! `is_dismiss_key`, `resolve_base_id` and `SR_ONLY_STYLE` (crate-private).

use crate::a11y::{install_focus_trap, release_focus_trap, use_unique_id};
use crate::atoms::{Box, Button, ButtonVariant};
use crate::styles::Style;
use crate::theme::{use_style, use_theme};
use dioxus::prelude::*;

/// Inline style that hides content visually but keeps it available to
/// assistive technology (e.g. the "Close" label of an icon-only button).
pub(crate) const SR_ONLY_STYLE: &str = concat!(
    "position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px; ",
    "overflow: hidden; clip: rect(0, 0, 0, 0); white-space: nowrap; border: 0;",
);

/// Whether `key` should dismiss an open overlay (Escape).
pub(crate) fn is_dismiss_key(key: &Key) -> bool {
    *key == Key::Escape
}

/// ARIA role for a dialog surface.
pub(crate) fn dialog_role(alert: bool) -> &'static str {
    if alert {
        "alertdialog"
    } else {
        "dialog"
    }
}

/// Base for the ids of an overlay's parts: the caller's `id` prop when it is
/// set and non-empty, otherwise the generated `fallback`.
pub(crate) fn resolve_base_id(id: Option<&str>, fallback: &str) -> String {
    match id {
        Some(id) if !id.is_empty() => id.to_string(),
        _ => fallback.to_string(),
    }
}

#[derive(Props, Clone, PartialEq)]
pub(crate) struct FocusTrapScopeProps {
    /// Id of the element to trap focus in
    pub container_id: String,
}

/// Renders nothing. While mounted, keeps Tab focus inside `container_id`;
/// when unmounted (the overlay closed) it restores focus to the element that
/// had it before the overlay opened.
#[component]
pub(crate) fn FocusTrapScope(props: FocusTrapScopeProps) -> Element {
    let install_id = props.container_id.clone();
    let release_id = props.container_id.clone();
    use_effect(move || install_focus_trap(&install_id));
    use_drop(move || release_focus_trap(&release_id));
    rsx! {}
}

/// Dialog properties
#[derive(Props, Clone, PartialEq)]
pub struct DialogProps {
    /// Whether the dialog is open
    pub open: bool,
    /// Callback when dialog should close
    #[props(default)]
    pub on_close: Option<EventHandler<()>>,
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
    /// Element id
    #[props(default)]
    pub id: Option<String>,
    /// Custom CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Use `role="alertdialog"` instead of `role="dialog"` (for dialogs that
    /// interrupt the user and require a response)
    #[props(default)]
    pub alert: bool,
    /// Accessible name, for dialogs without a `title`
    #[props(default)]
    pub aria_label: Option<String>,
    /// Id of an element (usually inside `children`) that describes the
    /// dialog; takes precedence over the `description` prop
    #[props(default)]
    pub aria_describedby: Option<String>,
}

/// Dialog molecule component
///
/// # Accessibility
///
/// - The panel has `role="dialog"` (`role="alertdialog"` when `alert` is set)
///   and `aria-modal="true"`.
/// - `aria-labelledby` points at the rendered `title` (or use `aria_label`);
///   `aria-describedby` points at the rendered `description` (or the element
///   given in `aria_describedby`). Ids derive from the `id` prop when set.
/// - Escape calls `on_close`.
/// - While open, Tab / Shift+Tab cycle inside the panel; when it closes, focus
///   returns to the element that was focused before it opened.
#[component]
pub fn Dialog(props: DialogProps) -> Element {
    let _theme = use_theme();
    let auto_id = use_unique_id("dialog");

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

    // Hooks above run unconditionally; only rendering depends on `open`.
    if !props.open {
        return rsx! {};
    }

    let base_id = resolve_base_id(props.id.as_deref(), &auto_id);
    let content_id = format!("{base_id}-content");
    let title_id = format!("{base_id}-title");
    let description_id = format!("{base_id}-description");
    let labelledby = props.title.as_ref().map(|_| title_id.clone());
    let describedby = props
        .aria_describedby
        .clone()
        .or_else(|| props.description.as_ref().map(|_| description_id.clone()));
    let role = dialog_role(props.alert);

    let handle_overlay_click = move |_| {
        if props.close_on_overlay_click {
            if let Some(handler) = &props.on_close {
                handler.call(());
            }
        }
    };

    let on_close = props.on_close;
    let handle_keydown = move |e: KeyboardEvent| {
        if is_dismiss_key(&e.key()) {
            e.stop_propagation();
            if let Some(handler) = &on_close {
                handler.call(());
            }
        }
    };

    let custom_content_style = props.content_style.clone().unwrap_or_default();

    rsx! {
        div {
            id: "{props.id.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            style: "{overlay_style}",
            onclick: handle_overlay_click,

            div {
                id: "{content_id}",
                role: "{role}",
                "aria-modal": "true",
                "aria-labelledby": labelledby,
                "aria-label": props.aria_label.clone(),
                "aria-describedby": describedby,
                tabindex: "-1",
                style: "{content_style} {custom_content_style}",
                onclick: move |e| e.stop_propagation(),
                onkeydown: handle_keydown,

                FocusTrapScope { container_id: content_id.clone() }

                // Header
                if props.title.is_some() || props.show_close_button {
                    DialogHeader {
                        title: props.title.clone(),
                        title_id: title_id.clone(),
                        show_close_button: props.show_close_button,
                        on_close: props.on_close,
                    }
                }

                // Description
                if let Some(description) = props.description.clone() {
                    DialogDescription { id: description_id.clone(), description: description }
                }

                // Content
                Box {
                    style: "padding: 0 24px 24px 24px; overflow-y: auto; {props.style.clone().unwrap_or_default()}",
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
    #[props(default)]
    title_id: Option<String>,
    show_close_button: bool,
    #[props(default)]
    on_close: Option<EventHandler<()>>,
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
            id: "{props.id.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            style: "{header_style}",

            if let Some(title) = props.title {
                h2 {
                    id: props.title_id.clone(),
                    style: "margin: 0; font-size: 18px; font-weight: 600; {props.style.clone().unwrap_or_default()}",
                    "{title}"
                }
            } else {
                div {}
            }

            if props.show_close_button {
                Button {
                    variant: ButtonVariant::Ghost,
                    on_click: props.on_close,
                    span { "aria-hidden": "true", "✕" }
                    span { style: SR_ONLY_STYLE, "Close" }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct DialogDescriptionProps {
    #[props(default)]
    description: Option<String>,
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
fn DialogDescription(props: DialogDescriptionProps) -> Element {
    rsx! {
        p {
            id: "{props.id.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            style: "margin: 0; padding: 16px 24px 0 24px; font-size: 14px; color: #64748b; {props.style.clone().unwrap_or_default()}",
            if let Some(description) = props.description {
                "{description}"
            }
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
            id: "{props.id.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            style: "{footer_style} justify-content: {justify}; {props.style.clone().unwrap_or_default()}",
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
    #[props(default)]
    pub on_close: Option<EventHandler<()>>,
    /// Dialog title
    #[props(default)]
    pub title: Option<String>,
    /// Dialog description
    #[props(default)]
    pub description: Option<String>,
    /// Cancel button text
    #[props(default = "Cancel".to_string())]
    pub cancel_text: String,
    /// Confirm button text
    #[props(default = "Confirm".to_string())]
    pub confirm_text: String,
    /// Callback when confirmed
    #[props(default)]
    pub on_confirm: Option<EventHandler<()>>,
    /// Whether the action is destructive
    #[props(default)]
    pub destructive: bool,
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

/// Alert dialog component
///
/// # Accessibility
///
/// Built on [`Dialog`] with `role="alertdialog"` and `aria-modal="true"`;
/// `aria-labelledby` points at the title and `aria-describedby` at the
/// description. Escape (like the cancel button) calls `on_close`; focus is
/// trapped while open and restored on close.
#[component]
pub fn AlertDialog(props: AlertDialogProps) -> Element {
    let auto_id = use_unique_id("alert-dialog");
    let base_id = resolve_base_id(props.id.as_deref(), &auto_id);
    let description_id = format!("{base_id}-description");
    let describedby = props.description.as_ref().map(|_| description_id.clone());
    let confirm_variant = if props.destructive {
        ButtonVariant::Destructive
    } else {
        ButtonVariant::Primary
    };

    rsx! {
        Dialog {
            id: props.id.clone(),
            class: props.class.clone(),
            open: props.open,
            on_close: props.on_close,
            title: props.title.clone(),
            show_close_button: false,
            close_on_overlay_click: false,
            alert: true,
            aria_describedby: describedby,

            if let Some(description) = props.description.clone() {
                p {
                    id: "{description_id}",
                    style: "margin: 0 0 24px 0; font-size: 14px; color: #64748b; line-height: 1.5; {props.style.clone().unwrap_or_default()}",
                    "{description}"
                }
            }

            DialogFooter {
                align: DialogFooterAlign::End,

                Button {
                    variant: ButtonVariant::Ghost,
                    on_click: props.on_close,
                    "{props.cancel_text}"
                }

                Button {
                    variant: confirm_variant,
                    on_click: props.on_confirm,
                    "{props.confirm_text}"
                }
            }
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

    /// Value of the first `attr="..."` in `html`.
    fn attr_value<'a>(html: &'a str, attr: &str) -> Option<&'a str> {
        let needle = format!("{attr}=\"");
        let start = html.find(&needle)? + needle.len();
        let len = html[start..].find('"')?;
        Some(&html[start..start + len])
    }

    #[test]
    fn helpers() {
        assert!(is_dismiss_key(&Key::Escape));
        assert!(!is_dismiss_key(&Key::Enter));
        assert_eq!(dialog_role(false), "dialog");
        assert_eq!(dialog_role(true), "alertdialog");
        assert_eq!(resolve_base_id(Some("mine"), "dialog-1"), "mine");
        assert_eq!(resolve_base_id(Some(""), "dialog-1"), "dialog-1");
        assert_eq!(resolve_base_id(None, "dialog-1"), "dialog-1");
    }

    #[test]
    fn open_dialog_has_role_modal_and_relationships() {
        let html = render(|| {
            rsx! {
                Dialog {
                    open: true,
                    id: "dlg",
                    title: "Edit profile",
                    description: "Make changes here",
                    "Body"
                }
            }
        });
        assert!(html.contains("role=\"dialog\""), "{html}");
        assert!(html.contains("aria-modal=\"true\""), "{html}");
        assert!(html.contains("id=\"dlg-content\""), "{html}");
        assert!(html.contains("aria-labelledby=\"dlg-title\""), "{html}");
        assert!(html.contains("id=\"dlg-title\""), "{html}");
        assert!(
            html.contains("aria-describedby=\"dlg-description\""),
            "{html}"
        );
        assert!(html.contains("id=\"dlg-description\""), "{html}");
        // Icon-only close button has an accessible label.
        assert!(html.contains(">Close<"), "{html}");
    }

    #[test]
    fn generated_ids_link_to_existing_elements() {
        let html = render(|| {
            rsx! { Dialog { open: true, title: "Hi", description: "There", "Body" } }
        });
        let labelledby = attr_value(&html, "aria-labelledby").expect("aria-labelledby");
        let describedby = attr_value(&html, "aria-describedby").expect("aria-describedby");
        assert!(html.contains(&format!("id=\"{labelledby}\"")), "{html}");
        assert!(html.contains(&format!("id=\"{describedby}\"")), "{html}");
    }

    #[test]
    fn dialog_without_title_uses_aria_label() {
        let html = render(|| {
            rsx! { Dialog { open: true, show_close_button: false, aria_label: "Settings", "Body" } }
        });
        assert!(html.contains("aria-label=\"Settings\""), "{html}");
        assert!(!html.contains("aria-labelledby"), "{html}");
        assert!(!html.contains("aria-describedby"), "{html}");
    }

    #[test]
    fn closed_dialog_renders_nothing() {
        let html = render(|| rsx! { Dialog { open: false, title: "Hidden", "Body" } });
        assert!(!html.contains("role="), "{html}");
        assert!(!html.contains("Hidden"), "{html}");
    }

    #[test]
    fn alert_dialog_uses_alertdialog_role_and_description() {
        let html = render(|| {
            rsx! {
                AlertDialog {
                    open: true,
                    id: "del",
                    title: "Delete?",
                    description: "This cannot be undone.",
                }
            }
        });
        assert!(html.contains("role=\"alertdialog\""), "{html}");
        assert!(html.contains("aria-modal=\"true\""), "{html}");
        assert!(html.contains("aria-labelledby=\"del-title\""), "{html}");
        assert!(
            html.contains("aria-describedby=\"del-description\""),
            "{html}"
        );
        assert!(html.contains("id=\"del-description\""), "{html}");
    }
}
