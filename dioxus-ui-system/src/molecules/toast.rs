//! Toast notification molecule component
//!
//! Transient, non-blocking feedback notifications.

use dioxus::prelude::*;
use crate::theme::use_theme;

/// Toast variant/type
#[derive(Default, Clone, PartialEq, Debug)]
pub enum ToastVariant {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

impl ToastVariant {
    fn icon(&self) -> &'static str {
        match self {
            ToastVariant::Info => "ℹ️",
            ToastVariant::Success => "✓",
            ToastVariant::Warning => "⚠️",
            ToastVariant::Error => "✕",
        }
    }
    
    fn colors(&self) -> (&'static str, &'static str) {
        match self {
            // (background, border)
            ToastVariant::Info => ("#eff6ff", "#3b82f6"),
            ToastVariant::Success => ("#f0fdf4", "#22c55e"),
            ToastVariant::Warning => ("#fefce8", "#eab308"),
            ToastVariant::Error => ("#fef2f2", "#ef4444"),
        }
    }
}

/// Toast properties
#[derive(Props, Clone, PartialEq)]
pub struct ToastProps {
    /// Toast message
    pub message: String,
    /// Toast variant
    #[props(default = ToastVariant::Info)]
    pub variant: ToastVariant,
    /// Duration before auto-dismiss
    #[props(default = 5000)]
    pub duration_ms: u64,
    /// Whether to show close button
    #[props(default = true)]
    pub closable: bool,
    /// Close handler
    pub on_close: EventHandler<()>,
    /// Action button text
    #[props(default)]
    pub action: Option<String>,
    /// Action handler
    #[props(default)]
    pub on_action: Option<EventHandler<()>>,
    /// Position in stack
    #[props(default = 0)]
    pub index: usize,
}

/// Toast notification component
#[component]
pub fn Toast(props: ToastProps) -> Element {
    let theme = use_theme();
    let (bg_color, border_color) = props.variant.colors();
    let icon = props.variant.icon();
    
    let text_color = theme.tokens.read().colors.foreground.to_rgba();
    
    let handle_close = {
        let on_close = props.on_close.clone();
        move |_| {
            on_close.call(());
        }
    };
    
    let top = 16 + props.index * 80;
    
    let variant_name = format!("{:?}", props.variant).to_lowercase();
    
    rsx! {
        div {
            class: "toast toast-{variant_name}",
            style: "position: fixed; top: {top}px; right: 16px; z-index: 9999; min-width: 300px; max-width: 400px; padding: 16px; background: {bg_color}; border-left: 4px solid {border_color}; border-radius: 8px; box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1); display: flex; align-items: flex-start; gap: 12px;",
            
            // Icon
            span {
                class: "toast-icon",
                style: "font-size: 20px; flex-shrink: 0; color: {border_color}; font-weight: 600;",
                "{icon}"
            }
            
            // Content
            div {
                class: "toast-content",
                style: "flex: 1;",
                
                p {
                    style: "margin: 0; font-size: 14px; color: {text_color}; line-height: 1.5;",
                    "{props.message}"
                }
                
                if let Some(action) = props.action {
                    button {
                        type: "button",
                        class: "toast-action",
                        style: "margin-top: 8px; font-size: 13px; font-weight: 500; color: {border_color}; background: none; border: none; cursor: pointer; padding: 0;",
                        onclick: move |_| {
                            if let Some(handler) = &props.on_action {
                                handler.call(());
                            }
                        },
                        "{action}"
                    }
                }
            }
            
            // Close button
            if props.closable {
                button {
                    type: "button",
                    class: "toast-close",
                    style: "flex-shrink: 0; background: none; border: none; cursor: pointer; font-size: 18px; color: #9ca3af; padding: 0; width: 24px; height: 24px; display: flex; align-items: center; justify-content: center; border-radius: 4px;",
                    onclick: handle_close,
                    "✕"
                }
            }
        }
    }
}

/// Toast manager for showing toasts
#[derive(Clone, Copy)]
pub struct ToastManager {
    toasts: Signal<Vec<ToastItem>>,
}

#[derive(Clone, PartialEq)]
struct ToastItem {
    id: usize,
    message: String,
    variant: ToastVariant,
    duration_ms: u64,
    action: Option<String>,
}

static NEXT_TOAST_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);

impl ToastManager {
    pub fn new() -> Self {
        Self {
            toasts: use_signal(|| Vec::new()),
        }
    }
    
    pub fn show(&mut self, message: impl Into<String>) {
        self.show_with_variant(message, ToastVariant::Info);
    }
    
    pub fn show_success(&mut self, message: impl Into<String>) {
        self.show_with_variant(message, ToastVariant::Success);
    }
    
    pub fn show_error(&mut self, message: impl Into<String>) {
        self.show_with_variant(message, ToastVariant::Error);
    }
    
    pub fn show_warning(&mut self, message: impl Into<String>) {
        self.show_with_variant(message, ToastVariant::Warning);
    }
    
    fn show_with_variant(&mut self, message: impl Into<String>, variant: ToastVariant) {
        let id = NEXT_TOAST_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let item = ToastItem {
            id,
            message: message.into(),
            variant,
            duration_ms: 5000,
            action: None,
        };
        
        let mut toasts = self.toasts.write();
        toasts.push(item);
    }
    
    pub fn show_custom(&mut self, message: impl Into<String>, variant: ToastVariant, duration_ms: u64) {
        let id = NEXT_TOAST_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let item = ToastItem {
            id,
            message: message.into(),
            variant,
            duration_ms,
            action: None,
        };
        
        let mut toasts = self.toasts.write();
        toasts.push(item);
    }
    
    pub fn dismiss(&mut self, id: usize) {
        let mut toasts = self.toasts.write();
        toasts.retain(|toast| toast.id != id);
    }
    
    pub fn dismiss_all(&mut self) {
        self.toasts.set(Vec::new());
    }
    
    pub fn render(&self) -> Element {
        let toasts_read = self.toasts.read();
        let toasts: Vec<_> = toasts_read.iter().cloned().collect();
        
        rsx! {
            div {
                class: "toast-container",
                
                for (index, toast) in toasts.into_iter().enumerate() {
                    {
                        let toast_id = toast.id;
                        let message = toast.message.clone();
                        let variant = toast.variant.clone();
                        rsx! {
                            Toast {
                                key: "{toast.id}",
                                message: message,
                                variant: variant,
                                duration_ms: toast.duration_ms,
                                index: index,
                                on_close: {
                                    let mut manager = *self;
                                    move |_| manager.dismiss(toast_id)
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Hook to access toast manager
pub fn use_toast() -> ToastManager {
    ToastManager::new()
}

/// Toast provider component (wraps app and provides toast container)
#[derive(Props, Clone, PartialEq)]
pub struct ToastProviderProps {
    pub children: Element,
}

#[component]
pub fn ToastProvider(props: ToastProviderProps) -> Element {
    let toast_manager = use_toast();
    
    rsx! {
        {props.children}
        {toast_manager.render()}
    }
}
