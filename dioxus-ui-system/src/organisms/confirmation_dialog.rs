//! Confirmation Dialog organism component
//!
//! Critical decision dialogs with clear action labeling.

use crate::molecules::Dialog;
use crate::theme::use_theme;
use dioxus::prelude::*;

/// Confirmation dialog properties
#[derive(Props, Clone, PartialEq)]
pub struct ConfirmationDialogProps {
    /// Whether the dialog is open
    pub open: bool,
    /// On close handler
    pub on_close: EventHandler<()>,
    /// Dialog title
    pub title: String,
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
    pub on_confirm: EventHandler<()>,
    /// Loading state
    #[props(default = false)]
    pub loading: bool,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
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
#[component]
pub fn ConfirmationDialog(props: ConfirmationDialogProps) -> Element {
    let theme = use_theme();

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
            open: props.open,
            on_close: props.on_close.clone(),
            title: props.title.clone(),

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
                        onclick: move |_| props.on_close.call(()),
                        "{props.cancel_text.clone().unwrap_or_else(|| \"Cancel\".to_string())}"
                    }

                    button {
                        type: "button",
                        class: "confirmation-dialog-confirm",
                        style: format!("padding: 12px 24px; font-size: 14px; font-weight: 500; color: white; background: {bg_color}; border: none; border-radius: 8px; cursor: pointer; transition: background 0.15s ease; opacity: {};", if props.loading { "0.7" } else { "1" }),
                        disabled: props.loading,
                        onclick: move |_| props.on_confirm.call(()),

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
    pub on_close: EventHandler<()>,
    #[props(default)]
    pub title: Option<String>,
    pub item_name: String,
    pub on_confirm: EventHandler<()>,
    #[props(default = false)]
    pub loading: bool,
}

/// Delete confirmation dialog
#[component]
pub fn DeleteConfirmDialog(props: DeleteConfirmDialogProps) -> Element {
    rsx! {
        ConfirmationDialog {
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
    pub on_close: EventHandler<()>,
    pub on_save: EventHandler<()>,
    pub on_discard: EventHandler<()>,
}

/// Unsaved changes confirmation dialog
#[component]
pub fn UnsavedChangesDialog(props: UnsavedChangesDialogProps) -> Element {
    let theme = use_theme();

    rsx! {
        Dialog {
            open: props.open,
            on_close: props.on_close,
            title: "Unsaved changes",

            div {
                style: "text-align: center; padding: 24px 0;",

                div {
                    style: "width: 64px; height: 64px; margin: 0 auto 24px; border-radius: 50%; background: #f59e0b20; display: flex; align-items: center; justify-content: center; font-size: 32px;",
                    "⚠️"
                }

                p {
                    style: "margin: 0 0 32px 0; font-size: 16px; line-height: 1.6; color: {theme.tokens.read().colors.foreground.to_rgba()};",
                    "You have unsaved changes. What would you like to do?"
                }

                div {
                    style: "display: flex; gap: 12px; justify-content: center; flex-wrap: wrap;",

                    button {
                        type: "button",
                        style: "padding: 12px 24px; font-size: 14px; font-weight: 500; color: {theme.tokens.read().colors.foreground.to_rgba()}; background: white; border: 1px solid {theme.tokens.read().colors.border.to_rgba()}; border-radius: 8px; cursor: pointer;",
                        onclick: move |_| props.on_close.call(()),
                        "Keep editing"
                    }

                    button {
                        type: "button",
                        style: "padding: 12px 24px; font-size: 14px; font-weight: 500; color: white; background: #ef4444; border: none; border-radius: 8px; cursor: pointer;",
                        onclick: move |_| props.on_discard.call(()),
                        "Discard changes"
                    }

                    button {
                        type: "button",
                        style: "padding: 12px 24px; font-size: 14px; font-weight: 500; color: white; background: {theme.tokens.read().colors.primary.to_rgba()}; border: none; border-radius: 8px; cursor: pointer;",
                        onclick: move |_| props.on_save.call(()),
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
    pub on_close: EventHandler<()>,
    pub on_confirm: EventHandler<()>,
}

/// Sign out confirmation dialog
#[component]
pub fn SignOutDialog(props: SignOutDialogProps) -> Element {
    rsx! {
        ConfirmationDialog {
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
