//! Confirmation Dialog organism component
//!
//! Critical decision dialogs with clear action labeling.

use crate::a11y::use_unique_id;
use crate::molecules::dialog::resolve_base_id;
use crate::molecules::Dialog;
use crate::theme::use_theme;
use dioxus::prelude::*;

/// Confirmation dialog properties
#[derive(Props, Clone, PartialEq)]
pub struct ConfirmationDialogProps {
    /// Whether the dialog is open
    pub open: bool,
    /// On close handler
    #[props(default)]
    pub on_close: Option<EventHandler<()>>,
    /// Dialog title
    #[props(default)]
    pub title: Option<String>,
    /// Confirmation message
    pub message: String,
    /// Confirm button text
    #[props(default)]
    pub confirm_text: Option<String>,
    /// Cancel button text
    #[props(default)]
    pub cancel_text: Option<String>,
    /// Confirm button variant
    #[props(default = ConfirmVariant::Danger)]
    pub variant: ConfirmVariant,
    /// Icon
    #[props(default)]
    pub icon: Option<String>,
    /// On confirm handler
    #[props(default)]
    pub on_confirm: Option<EventHandler<()>>,
    /// Loading state
    #[props(default = false)]
    pub loading: bool,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Element id
    #[props(default)]
    pub id: Option<String>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Confirmation variant
#[derive(Default, Clone, PartialEq, Debug)]
pub enum ConfirmVariant {
    #[default]
    Danger,
    Warning,
    Info,
    Success,
}

impl ConfirmVariant {
    fn colors(&self) -> (&'static str, &'static str) {
        match self {
            ConfirmVariant::Danger => ("#ef4444", "#dc2626"),
            ConfirmVariant::Warning => ("#f59e0b", "#d97706"),
            ConfirmVariant::Info => ("#3b82f6", "#2563eb"),
            ConfirmVariant::Success => ("#22c55e", "#16a34a"),
        }
    }

    fn default_icon(&self) -> &'static str {
        match self {
            ConfirmVariant::Danger => "🗑️",
            ConfirmVariant::Warning => "⚠️",
            ConfirmVariant::Info => "ℹ️",
            ConfirmVariant::Success => "✓",
        }
    }
}

/// Confirmation dialog component
///
/// # Accessibility
///
/// Built on [`Dialog`] with `role="alertdialog"` and `aria-modal="true"`:
/// `aria-labelledby` points at the title and `aria-describedby` at the
/// message. Escape (like the cancel button) calls `on_close`; focus is
/// trapped while open and restored on close.
#[component]
pub fn ConfirmationDialog(props: ConfirmationDialogProps) -> Element {
    let theme = use_theme();
    let auto_id = use_unique_id("confirmation-dialog");
    let base_id = resolve_base_id(props.id.as_deref(), &auto_id);
    let message_id = format!("{base_id}-message");

    let class_css = props
        .class
        .as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();

    let (bg_color, _hover_color) = props.variant.colors();
    let icon = props
        .icon
        .unwrap_or_else(|| props.variant.default_icon().to_string());

    rsx! {
        Dialog {
            id: props.id.clone(),
            class: props.class.clone(),
            style: props.style.clone(),
            open: props.open,
            on_close: props.on_close,
            title: props.title.clone(),
            alert: true,
            aria_describedby: message_id.clone(),

            div {
                class: "confirmation-dialog{class_css}",
                style: "text-align: center; padding: 24px 0;",

                // Icon
                div {
                    class: "confirmation-dialog-icon",
                    style: "width: 64px; height: 64px; margin: 0 auto 24px; border-radius: 50%; background: {bg_color}20; display: flex; align-items: center; justify-content: center; font-size: 32px;",
                    "{icon}"
                }

                // Message
                p {
                    id: "{message_id}",
                    class: "confirmation-dialog-message",
                    style: "margin: 0 0 32px 0; font-size: 16px; line-height: 1.6; color: {theme.tokens.read().colors.foreground.to_rgba()};",
                    "{props.message}"
                }

                // Actions
                div {
                    class: "confirmation-dialog-actions",
                    style: "display: flex; gap: 12px; justify-content: center;",

                    button {
                        type: "button",
                        class: "confirmation-dialog-cancel",
                        style: "padding: 12px 24px; font-size: 14px; font-weight: 500; color: {theme.tokens.read().colors.foreground.to_rgba()}; background: white; border: 1px solid {theme.tokens.read().colors.border.to_rgba()}; border-radius: 8px; cursor: pointer; transition: background 0.15s ease;",
                        onclick: move |_| {
                            if let Some(h) = &props.on_close {
                                h.call(());
                            }
                        },
                        "{props.cancel_text.clone().unwrap_or_else(|| \"Cancel\".to_string())}"
                    }

                    button {
                        type: "button",
                        class: "confirmation-dialog-confirm",
                        style: format!("padding: 12px 24px; font-size: 14px; font-weight: 500; color: white; background: {bg_color}; border: none; border-radius: 8px; cursor: pointer; transition: background 0.15s ease; opacity: {};", if props.loading { "0.7" } else { "1" }),
                        disabled: props.loading,
                        onclick: move |_| {
                            if let Some(h) = &props.on_confirm {
                                h.call(());
                            }
                        },

                        if props.loading {
                            span {
                                style: "display: inline-block; margin-right: 8px; animation: spin 1s linear infinite;",
                                "⟳"
                            }
                        }

                        "{props.confirm_text.clone().unwrap_or_else(|| \"Confirm\".to_string())}"
                    }
                }
            }
        }


    }
}

/// Delete confirmation dialog (convenience wrapper)
#[derive(Props, Clone, PartialEq)]
pub struct DeleteConfirmDialogProps {
    pub open: bool,
    #[props(default)]
    pub on_close: Option<EventHandler<()>>,
    #[props(default)]
    pub title: Option<String>,
    pub item_name: String,
    #[props(default)]
    pub on_confirm: Option<EventHandler<()>>,
    #[props(default = false)]
    pub loading: bool,
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

/// Delete confirmation dialog
#[component]
pub fn DeleteConfirmDialog(props: DeleteConfirmDialogProps) -> Element {
    rsx! {
        ConfirmationDialog {
            id: props.id.clone(),
            class: props.class.clone(),
            style: props.style.clone(),
            open: props.open,
            on_close: props.on_close,
            title: props.title.unwrap_or_else(|| "Delete item".to_string()),
            message: format!("Are you sure you want to delete \"{}\"? This action cannot be undone.", props.item_name),
            confirm_text: Some("Delete".to_string()),
            cancel_text: Some("Keep".to_string()),
            variant: ConfirmVariant::Danger,
            icon: Some("🗑️".to_string()),
            on_confirm: props.on_confirm,
            loading: props.loading,
        }
    }
}

/// Unsaved changes dialog
#[derive(Props, Clone, PartialEq)]
pub struct UnsavedChangesDialogProps {
    pub open: bool,
    #[props(default)]
    pub on_close: Option<EventHandler<()>>,
    #[props(default)]
    pub on_save: Option<EventHandler<()>>,
    #[props(default)]
    pub on_discard: Option<EventHandler<()>>,
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

/// Unsaved changes confirmation dialog
///
/// # Accessibility
///
/// `role="alertdialog"` labelled by its title and described by its message;
/// Escape calls `on_close` ("Keep editing").
#[component]
pub fn UnsavedChangesDialog(props: UnsavedChangesDialogProps) -> Element {
    let theme = use_theme();
    let auto_id = use_unique_id("unsaved-changes-dialog");
    let base_id = resolve_base_id(props.id.as_deref(), &auto_id);
    let message_id = format!("{base_id}-message");

    rsx! {
        Dialog {
            id: props.id.clone(),
            class: props.class.clone(),
            style: props.style.clone(),
            open: props.open,
            on_close: props.on_close,
            title: "Unsaved changes",
            alert: true,
            aria_describedby: message_id.clone(),

            div {
                style: "text-align: center; padding: 24px 0;",

                div {
                    style: "width: 64px; height: 64px; margin: 0 auto 24px; border-radius: 50%; background: #f59e0b20; display: flex; align-items: center; justify-content: center; font-size: 32px;",
                    "⚠️"
                }

                p {
                    id: "{message_id}",
                    style: "margin: 0 0 32px 0; font-size: 16px; line-height: 1.6; color: {theme.tokens.read().colors.foreground.to_rgba()};",
                    "You have unsaved changes. What would you like to do?"
                }

                div {
                    style: "display: flex; gap: 12px; justify-content: center; flex-wrap: wrap;",

                    button {
                        type: "button",
                        style: "padding: 12px 24px; font-size: 14px; font-weight: 500; color: {theme.tokens.read().colors.foreground.to_rgba()}; background: white; border: 1px solid {theme.tokens.read().colors.border.to_rgba()}; border-radius: 8px; cursor: pointer;",
                        onclick: move |_| {
                            if let Some(h) = &props.on_close {
                                h.call(());
                            }
                        },
                        "Keep editing"
                    }

                    button {
                        type: "button",
                        style: "padding: 12px 24px; font-size: 14px; font-weight: 500; color: white; background: #ef4444; border: none; border-radius: 8px; cursor: pointer;",
                        onclick: move |_| {
                            if let Some(h) = &props.on_discard {
                                h.call(());
                            }
                        },
                        "Discard changes"
                    }

                    button {
                        type: "button",
                        style: "padding: 12px 24px; font-size: 14px; font-weight: 500; color: white; background: {theme.tokens.read().colors.primary.to_rgba()}; border: none; border-radius: 8px; cursor: pointer;",
                        onclick: move |_| {
                            if let Some(h) = &props.on_save {
                                h.call(());
                            }
                        },
                        "Save changes"
                    }
                }
            }
        }
    }
}

/// Sign out confirmation dialog
#[derive(Props, Clone, PartialEq)]
pub struct SignOutDialogProps {
    pub open: bool,
    #[props(default)]
    pub on_close: Option<EventHandler<()>>,
    #[props(default)]
    pub on_confirm: Option<EventHandler<()>>,
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

/// Sign out confirmation dialog
#[component]
pub fn SignOutDialog(props: SignOutDialogProps) -> Element {
    rsx! {
        ConfirmationDialog {
            id: props.id.clone(),
            class: props.class.clone(),
            style: props.style.clone(),
            open: props.open,
            on_close: props.on_close,
            title: "Sign out",
            message: "Are you sure you want to sign out?",
            confirm_text: "Sign out",
            cancel_text: "Stay signed in",
            variant: ConfirmVariant::Info,
            icon: "👋",
            on_confirm: props.on_confirm,
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
    fn confirmation_dialog_is_alertdialog_labelled_and_described() {
        let html = render(|| {
            rsx! {
                ConfirmationDialog {
                    open: true,
                    id: "confirm",
                    title: "Delete project",
                    message: "This cannot be undone.",
                }
            }
        });
        assert!(html.contains("role=\"alertdialog\""), "{html}");
        assert!(html.contains("aria-modal=\"true\""), "{html}");
        assert!(html.contains("aria-labelledby=\"confirm-title\""), "{html}");
        assert!(html.contains("id=\"confirm-title\""), "{html}");
        assert!(
            html.contains("aria-describedby=\"confirm-message\""),
            "{html}"
        );
        assert!(html.contains("id=\"confirm-message\""), "{html}");
    }

    #[test]
    fn wrappers_inherit_dialog_semantics() {
        let html = render(|| {
            rsx! {
                DeleteConfirmDialog { open: true, id: "del", item_name: "report.pdf" }
                UnsavedChangesDialog { open: true, id: "unsaved" }
            }
        });
        assert_eq!(html.matches("role=\"alertdialog\"").count(), 2, "{html}");
        assert!(html.contains("aria-describedby=\"del-message\""), "{html}");
        assert!(html.contains("id=\"del-message\""), "{html}");
        assert!(
            html.contains("aria-describedby=\"unsaved-message\""),
            "{html}"
        );
        assert!(html.contains("id=\"unsaved-message\""), "{html}");
    }
}
