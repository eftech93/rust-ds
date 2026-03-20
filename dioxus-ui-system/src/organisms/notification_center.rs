//! Notification Center organism component
//!
//! Centralized alert management with categorization and history.

use dioxus::prelude::*;
use crate::theme::use_theme;
use crate::molecules::ToastVariant;

/// Notification item
#[derive(Clone, PartialEq, Debug)]
pub struct Notification {
    pub id: String,
    pub title: String,
    pub message: String,
    pub variant: ToastVariant,
    pub timestamp: String,
    pub read: bool,
    pub action: Option<String>,
}

impl Notification {
    pub fn new(id: impl Into<String>, title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            message: message.into(),
            variant: ToastVariant::Info,
            timestamp: "Just now".to_string(),
            read: false,
            action: None,
        }
    }
    
    pub fn with_variant(mut self, variant: ToastVariant) -> Self {
        self.variant = variant;
        self
    }
    
    pub fn with_timestamp(mut self, timestamp: impl Into<String>) -> Self {
        self.timestamp = timestamp.into();
        self
    }
    
    pub fn with_action(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }
}

/// Notification center properties
#[derive(Props, Clone, PartialEq)]
pub struct NotificationCenterProps {
    /// Notifications list
    #[props(default)]
    pub notifications: Vec<Notification>,
    /// Unread count
    #[props(default = 0)]
    pub unread_count: usize,
    /// On mark as read
    #[props(default)]
    pub on_mark_read: Option<EventHandler<String>>,
    /// On mark all read
    #[props(default)]
    pub on_mark_all_read: Option<EventHandler<()>>,
    /// On dismiss
    #[props(default)]
    pub on_dismiss: Option<EventHandler<String>>,
    /// On action click
    #[props(default)]
    pub on_action: Option<EventHandler<String>>,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Notification center component (dropdown)
#[component]
pub fn NotificationCenter(props: NotificationCenterProps) -> Element {
    let theme = use_theme();
    let mut is_open = use_signal(|| false);
    
    let class_css = props.class.as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();
    
    rsx! {
        div {
            class: "notification-center{class_css}",
            style: "position: relative;",
            
            // Trigger button
            button {
                type: "button",
                class: "notification-center-trigger",
                style: "position: relative; padding: 8px; background: none; border: none; cursor: pointer; font-size: 20px;",
                onclick: move |_| is_open.toggle(),
                
                "🔔"
                
                if props.unread_count > 0 {
                    span {
                        class: "notification-center-badge",
                        style: "position: absolute; top: 0; right: 0; min-width: 18px; height: 18px; padding: 0 5px; background: {theme.tokens.read().colors.destructive.to_rgba()}; color: white; font-size: 11px; font-weight: 600; border-radius: 9999px; display: flex; align-items: center; justify-content: center;",
                        if props.unread_count > 99 {
                            "99+"
                        } else {
                            "{props.unread_count}"
                        }
                    }
                }
            }
            
            // Dropdown
            if is_open() {
                div {
                    class: "notification-center-dropdown",
                    style: "position: absolute; top: calc(100% + 8px); right: 0; width: 380px; max-height: 500px; background: white; border: 1px solid {theme.tokens.read().colors.border.to_rgba()}; border-radius: 12px; box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1); z-index: 50; display: flex; flex-direction: column;",
                    
                    // Header
                    div {
                        class: "notification-center-header",
                        style: "display: flex; align-items: center; justify-content: space-between; padding: 16px; border-bottom: 1px solid {theme.tokens.read().colors.border.to_rgba()};",
                        
                        h3 {
                            style: "margin: 0; font-size: 16px; font-weight: 600;",
                            "Notifications"
                        }
                        
                        if props.unread_count > 0 {
                            if let Some(on_mark_all_read) = props.on_mark_all_read {
                                button {
                                    type: "button",
                                    style: "font-size: 13px; color: {theme.tokens.read().colors.primary.to_rgba()}; background: none; border: none; cursor: pointer;",
                                    onclick: move |_| on_mark_all_read.call(()),
                                    "Mark all read"
                                }
                            }
                        }
                    }
                    
                    // Notification list
                    div {
                        class: "notification-center-list",
                        style: "overflow-y: auto; max-height: 400px;",
                        
                        if props.notifications.is_empty() {
                            div {
                                style: "padding: 48px; text-align: center; color: {theme.tokens.read().colors.muted.to_rgba()};",
                                
                                div {
                                    style: "font-size: 48px; margin-bottom: 16px;",
                                    "🔔"
                                }
                                
                                p {
                                    style: "margin: 0; font-size: 14px;",
                                    "No notifications yet"
                                }
                            }
                        } else {
                            for notification in props.notifications.iter() {
                                {
                                    let (icon, icon_color) = match notification.variant {
                                        ToastVariant::Success => ("✓", "#22c55e"),
                                        ToastVariant::Error => ("✕", "#ef4444"),
                                        ToastVariant::Warning => ("⚠️", "#eab308"),
                                        ToastVariant::Info => ("ℹ️", "#3b82f6"),
                                    };
                                    
                                    let bg_color = if notification.read {
                                        "transparent"
                                    } else {
                                        "#eff6ff"
                                    };
                                    
                                    rsx! {
                                        div {
                                            key: "{notification.id}",
                                            class: "notification-item",
                                            style: "display: flex; gap: 12px; padding: 16px; border-bottom: 1px solid {theme.tokens.read().colors.border.to_rgba()}; background: {bg_color}; cursor: pointer; transition: background 0.15s ease;",
                                            onclick: {
                                                let on_mark_read = props.on_mark_read.clone();
                                                let id = notification.id.clone();
                                                move |_| {
                                                    if let Some(handler) = &on_mark_read {
                                                        handler.call(id.clone());
                                                    }
                                                }
                                            },
                                            
                                            // Icon
                                            div {
                                                class: "notification-item-icon",
                                                style: "flex-shrink: 0; width: 36px; height: 36px; border-radius: 50%; background: {icon_color}15; display: flex; align-items: center; justify-content: center; font-size: 16px; color: {icon_color};",
                                                "{icon}"
                                            }
                                            
                                            // Content
                                            div {
                                                class: "notification-item-content",
                                                style: "flex: 1; min-width: 0;",
                                                
                                                div {
                                                    class: "notification-item-header",
                                                    style: "display: flex; align-items: flex-start; justify-content: space-between; gap: 8px;",
                                                    
                                                    h4 {
                                                        style: "margin: 0; font-size: 14px; font-weight: 600; color: {theme.tokens.read().colors.foreground.to_rgba()};",
                                                        "{notification.title}"
                                                    }
                                                    
                                                    span {
                                                        style: "font-size: 12px; color: {theme.tokens.read().colors.muted.to_rgba()}; white-space: nowrap;",
                                                        "{notification.timestamp}"
                                                    }
                                                }
                                                
                                                p {
                                                    style: "margin: 4px 0 0 0; font-size: 13px; color: {theme.tokens.read().colors.foreground.to_rgba()}; line-height: 1.5;",
                                                    "{notification.message}"
                                                }
                                                
                                                if let Some(action) = notification.action.clone() {
                                                    button {
                                                        type: "button",
                                                        class: "notification-item-action",
                                                        style: "margin-top: 8px; font-size: 13px; font-weight: 500; color: {theme.tokens.read().colors.primary.to_rgba()}; background: none; border: none; cursor: pointer; padding: 0;",
                                                        onclick: {
                                                            let on_action = props.on_action.clone();
                                                            let id = notification.id.clone();
                                                            move |e: Event<dioxus::html::MouseData>| {
                                                                e.stop_propagation();
                                                                if let Some(handler) = &on_action {
                                                                    handler.call(id.clone());
                                                                }
                                                            }
                                                        },
                                                        "{action}"
                                                    }
                                                }
                                            }
                                            
                                            // Unread indicator
                                            if !notification.read {
                                                div {
                                                    style: "flex-shrink: 0; width: 8px; height: 8px; border-radius: 50%; background: {theme.tokens.read().colors.primary.to_rgba()}; margin-top: 4px;",
                                                }
                                            }
                                            
                                            // Dismiss button
                                            if let Some(on_dismiss) = props.on_dismiss.clone() {
                                                button {
                                                    type: "button",
                                                    style: "flex-shrink: 0; background: none; border: none; cursor: pointer; font-size: 14px; color: {theme.tokens.read().colors.muted.to_rgba()}; padding: 4px; margin: -4px; opacity: 0; transition: opacity 0.15s ease;",
                                                    onclick: {
                                                        let on_dismiss = on_dismiss.clone();
                                                        let notif_id = notification.id.clone();
                                                        move |e: Event<dioxus::html::MouseData>| {
                                                            e.stop_propagation();
                                                            on_dismiss.call(notif_id.clone());
                                                        }
                                                    },
                                                    "✕"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    // Footer
                    div {
                        class: "notification-center-footer",
                        style: "padding: 12px; border-top: 1px solid {theme.tokens.read().colors.border.to_rgba()}; text-align: center;",
                        
                        a {
                            href: "/notifications",
                            style: "font-size: 14px; color: {theme.tokens.read().colors.primary.to_rgba()}; text-decoration: none;",
                            "View all notifications"
                        }
                    }
                }
            }
        }
    }
}

/// Banner alert properties
#[derive(Props, Clone, PartialEq)]
pub struct BannerAlertProps {
    /// Alert message
    pub message: String,
    /// Alert variant
    #[props(default = ToastVariant::Info)]
    pub variant: ToastVariant,
    /// Dismissible
    #[props(default = true)]
    pub dismissible: bool,
    /// On dismiss
    #[props(default)]
    pub on_dismiss: Option<EventHandler<()>>,
    /// Action button text
    #[props(default)]
    pub action: Option<String>,
    /// On action
    #[props(default)]
    pub on_action: Option<EventHandler<()>>,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Banner alert component (persistent notification)
#[component]
pub fn BannerAlert(props: BannerAlertProps) -> Element {
    let mut is_dismissed = use_signal(|| false);
    
    let theme = use_theme();
    
    if is_dismissed() {
        return rsx! {};
    }
    
    let class_css = props.class.as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();
    
    let (bg_color, border_color, icon) = match props.variant {
        ToastVariant::Info => ("#eff6ff", "#3b82f6", "ℹ️"),
        ToastVariant::Success => ("#f0fdf4", "#22c55e", "✓"),
        ToastVariant::Warning => ("#fefce8", "#eab308", "⚠️"),
        ToastVariant::Error => ("#fef2f2", "#ef4444", "✕"),
    };
    
    let text_color = theme.tokens.read().colors.foreground.to_rgba();
    
    rsx! {
        div {
            class: "banner-alert{class_css}",
            style: "padding: 12px 16px; background: {bg_color}; border-left: 4px solid {border_color}; display: flex; align-items: center; gap: 12px;",
            
            span {
                class: "banner-alert-icon",
                style: "flex-shrink: 0; font-size: 18px; font-weight: 600; color: {border_color};",
                "{icon}"
            }
            
            p {
                class: "banner-alert-message",
                style: "flex: 1; margin: 0; font-size: 14px; color: {text_color};",
                "{props.message}"
            }
            
            if let Some(action) = props.action {
                if let Some(on_action) = props.on_action {
                    button {
                        type: "button",
                        class: "banner-alert-action",
                        style: "flex-shrink: 0; padding: 6px 12px; font-size: 13px; font-weight: 500; color: {border_color}; background: white; border: 1px solid {border_color}; border-radius: 6px; cursor: pointer;",
                        onclick: move |_| on_action.call(()),
                        "{action}"
                    }
                }
            }
            
            if props.dismissible {
                button {
                    type: "button",
                    class: "banner-alert-dismiss",
                    style: "flex-shrink: 0; background: none; border: none; cursor: pointer; font-size: 18px; color: #9ca3af; padding: 4px;",
                    onclick: move |_| {
                        is_dismissed.set(true);
                        if let Some(handler) = &props.on_dismiss {
                            handler.call(());
                        }
                    },
                    "✕"
                }
            }
        }
    }
}
