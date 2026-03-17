//! TextArea atom component
//!
//! Displays a form textarea or a component that looks like a textarea.

use dioxus::prelude::*;
use crate::theme::{use_theme, use_style};
use crate::styles::Style;

/// TextArea properties
#[derive(Props, Clone, PartialEq)]
pub struct TextAreaProps {
    /// Current value
    #[props(default)]
    pub value: String,
    /// Callback when value changes
    #[props(default)]
    pub onchange: Option<EventHandler<String>>,
    /// Placeholder text
    #[props(default)]
    pub placeholder: Option<String>,
    /// Number of rows
    #[props(default = 3)]
    pub rows: u8,
    /// Whether the textarea is disabled
    #[props(default)]
    pub disabled: bool,
    /// Whether the textarea is read-only
    #[props(default)]
    pub readonly: bool,
    /// Whether the textarea has an error
    #[props(default)]
    pub error: bool,
    /// Error message
    #[props(default)]
    pub error_message: Option<String>,
    /// Whether the field is required
    #[props(default)]
    pub required: bool,
    /// Maximum number of characters
    #[props(default)]
    pub max_length: Option<usize>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// TextArea atom component
#[component]
pub fn TextArea(props: TextAreaProps) -> Element {
    let _theme = use_theme();
    let mut is_focused = use_signal(|| false);
    
    let disabled = props.disabled;
    let error = props.error;
    
    let textarea_style = use_style(move |t| {
        let base = Style::new()
            .w_full()
            .px(&t.spacing, "md")
            .py(&t.spacing, "md")
            .rounded(&t.radius, "md")
            .border(1, if error { &t.colors.destructive } else { &t.colors.border })
            .bg(&t.colors.background)
            .text_color(&t.colors.foreground)
            .font_size(14)
            .line_height(1.5)
            .outline("none")
            .resize("vertical")
            .transition("all 150ms ease");
        
        let base = if is_focused() {
            base.border_color(&t.colors.ring)
                .shadow(&format!("0 0 0 1px {}", t.colors.ring.to_rgba()))
        } else {
            base
        };
        
        let base = if disabled {
            base.opacity(0.5)
                .bg(&t.colors.muted)
                .cursor("not-allowed")
        } else {
            base
        };
        
        base.build()
    });
    
    let handle_input = move |e: Event<FormData>| {
        let data = e.data();
        let new_value = data.value();
        
        // Enforce max_length
        if let Some(max) = props.max_length {
            if new_value.len() > max {
                return;
            }
        }
        
        if let Some(handler) = &props.onchange {
            handler.call(new_value);
        }
    };
    
    let char_count = props.value.len();
    let show_count = props.max_length.is_some();
    let max_length = props.max_length.unwrap_or(0);
    
    rsx! {
        div {
            style: "width: 100%;",
            
            textarea {
                value: "{props.value}",
                placeholder: props.placeholder.as_deref().unwrap_or(""),
                rows: props.rows,
                disabled: disabled,
                readonly: props.readonly,
                required: props.required,
                style: "{textarea_style} {props.style.clone().unwrap_or_default()}",
                class: "{props.class.clone().unwrap_or_default()}",
                oninput: handle_input,
                onfocus: move |_| is_focused.set(true),
                onblur: move |_| is_focused.set(false),
            }
            
            // Error message or character count
            if error {
                if let Some(err_msg) = props.error_message.clone() {
                    div {
                        style: "margin-top: 4px; color: #ef4444; font-size: 12px;",
                        "{err_msg}"
                    }
                }
            } else if show_count {
                div {
                    style: "margin-top: 4px; color: #64748b; font-size: 12px; text-align: right;",
                    "{char_count} / {max_length}"
                }
            }
        }
    }
}

/// Auto-resizing TextArea that grows with content
#[derive(Props, Clone, PartialEq)]
pub struct AutoResizeTextAreaProps {
    /// Current value
    #[props(default)]
    pub value: String,
    /// Callback when value changes
    #[props(default)]
    pub onchange: Option<EventHandler<String>>,
    /// Placeholder text
    #[props(default)]
    pub placeholder: Option<String>,
    /// Minimum number of rows
    #[props(default = 2)]
    pub min_rows: u8,
    /// Maximum number of rows
    #[props(default = 10)]
    pub max_rows: u8,
    /// Whether the textarea is disabled
    #[props(default)]
    pub disabled: bool,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Auto-resizing TextArea component
#[component]
pub fn AutoResizeTextArea(props: AutoResizeTextAreaProps) -> Element {
    let _theme = use_theme();
    let mut is_focused = use_signal(|| false);
    
    // Calculate rows based on content
    let line_count = props.value.lines().count().max(1);
    let rows = (line_count as u8).clamp(props.min_rows, props.max_rows);
    
    let textarea_style = use_style(move |t| {
        Style::new()
            .w_full()
            .px(&t.spacing, "md")
            .py(&t.spacing, "md")
            .rounded(&t.radius, "md")
            .border(1, &t.colors.border)
            .bg(&t.colors.background)
            .text_color(&t.colors.foreground)
            .font_size(14)
            .line_height(1.5)
            .outline("none")
            .resize("none")
            .overflow_hidden()
            .transition("all 150ms ease")
            .build()
    });
    
    let handle_input = move |e: Event<FormData>| {
        let data = e.data();
        let new_value = data.value();
        if let Some(handler) = &props.onchange {
            handler.call(new_value);
        }
    };
    
    rsx! {
        textarea {
            value: "{props.value}",
            placeholder: props.placeholder.as_deref().unwrap_or(""),
            rows: rows,
            disabled: props.disabled,
            style: "{textarea_style} {props.style.clone().unwrap_or_default()}",
            oninput: handle_input,
            onfocus: move |_| is_focused.set(true),
            onblur: move |_| is_focused.set(false),
        }
    }
}
