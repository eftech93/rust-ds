//! Rich Text Editor organism component
//!
//! A WYSIWYG text editor with formatting toolbar supporting bold, italic,
//! underline, headings, lists, links, code blocks, and text alignment.

use crate::styles::Style;
use crate::theme::{use_style, use_theme};
use dioxus::prelude::*;

/// Rich text editor features configuration
#[derive(Clone, PartialEq, Debug)]
pub struct RichTextFeatures {
    /// Enable bold formatting
    pub bold: bool,
    /// Enable italic formatting
    pub italic: bool,
    /// Enable underline formatting
    pub underline: bool,
    /// Enable strikethrough formatting
    pub strikethrough: bool,
    /// Enable heading formatting
    pub heading: bool,
    /// Enable bullet list
    pub bullet_list: bool,
    /// Enable numbered list
    pub numbered_list: bool,
    /// Enable link insertion
    pub link: bool,
    /// Enable code blocks
    pub code_block: bool,
    /// Enable blockquotes
    pub quote: bool,
    /// Enable left alignment
    pub align_left: bool,
    /// Enable center alignment
    pub align_center: bool,
    /// Enable right alignment
    pub align_right: bool,
}

impl Default for RichTextFeatures {
    fn default() -> Self {
        Self {
            bold: true,
            italic: true,
            underline: true,
            strikethrough: true,
            heading: true,
            bullet_list: true,
            numbered_list: true,
            link: true,
            code_block: true,
            quote: true,
            align_left: true,
            align_center: true,
            align_right: true,
        }
    }
}

/// Rich text editor properties
#[derive(Props, Clone, PartialEq)]
pub struct RichTextEditorProps {
    /// Current HTML content
    #[props(default)]
    pub value: String,
    /// Change handler - called when content changes
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,
    /// Placeholder text when editor is empty
    #[props(default)]
    pub placeholder: Option<String>,
    /// Disabled state
    #[props(default = false)]
    pub disabled: bool,
    /// Minimum height of editor (default: "200px")
    #[props(default = "200px")]
    pub min_height: &'static str,
    /// Maximum height of editor
    #[props(default)]
    pub max_height: Option<String>,
    /// Feature flags to enable/disable formatting options
    #[props(default)]
    pub features: RichTextFeatures,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Rich text editor component
#[component]
pub fn RichTextEditor(props: RichTextEditorProps) -> Element {
    let _theme = use_theme();
    let content = use_signal(|| props.value.clone());
    let is_focused = use_signal(|| false);

    let class_css = props
        .class
        .as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();

    let container_style = use_style(|t| {
        Style::new()
            .w_full()
            .border(1, &t.colors.border)
            .rounded(&t.radius, "md")
            .bg(&t.colors.background)
            .build()
    });

    let _on_input = {
        let on_change = props.on_change.clone();
        move |_e: Event<dioxus::events::KeyboardData>| {
            if let Some(handler) = &on_change {
                // In a real implementation, we would get the innerHTML from the contenteditable
                // For now, we'll just trigger the handler
                handler.call(content());
            }
        }
    };

    rsx! {
        div {
            class: "rich-text-editor{class_css}",
            style: "{container_style}",

            // Toolbar
            if !props.disabled {
                RichTextToolbar {
                    features: props.features.clone(),
                    disabled: props.disabled,
                }
            }

            // Content editable area
            RichTextContent {
                value: props.value.clone(),
                placeholder: props.placeholder.clone(),
                disabled: props.disabled,
                min_height: props.min_height,
                max_height: props.max_height.clone(),
                is_focused: is_focused(),
                on_input: props.on_change.clone(),
            }
        }
    }
}

/// Rich text toolbar properties
#[derive(Props, Clone, PartialEq)]
pub struct RichTextToolbarProps {
    /// Feature flags
    pub features: RichTextFeatures,
    /// Disabled state
    #[props(default = false)]
    pub disabled: bool,
}

/// Rich text toolbar component with formatting buttons
#[component]
pub fn RichTextToolbar(props: RichTextToolbarProps) -> Element {
    let theme = use_theme();

    let toolbar_style = use_style(|t| {
        Style::new()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap_px(4)
            .p(&t.spacing, "sm")
            .border_bottom(1, &t.colors.border)
            .build()
    });

    let button_style = use_style(|t| {
        Style::new()
            .inline_flex()
            .items_center()
            .justify_center()
            .w_px(32)
            .h_px(32)
            .rounded(&t.radius, "sm")
            .bg_transparent()
            .cursor_pointer()
            .text_color(&t.colors.foreground)
            .build()
    });

    let button_hover_style = format!(
        "background: {};",
        theme.tokens.read().colors.muted.to_rgba()
    );

    let disabled_style = if props.disabled {
        "opacity: 0.5; pointer-events: none;"
    } else {
        ""
    };

    rsx! {
        div {
            class: "rich-text-toolbar",
            style: "{toolbar_style} {disabled_style}",

            // Text formatting group
            if props.features.bold {
                ToolbarButton {
                    title: "Bold",
                    icon: "B",
                    command: "bold",
                    style: "{button_style}",
                    hover_style: &button_hover_style,
                }
            }
            if props.features.italic {
                ToolbarButton {
                    title: "Italic",
                    icon: "I",
                    command: "italic",
                    style: "{button_style}",
                    hover_style: &button_hover_style,
                }
            }
            if props.features.underline {
                ToolbarButton {
                    title: "Underline",
                    icon: "U",
                    command: "underline",
                    style: "{button_style}",
                    hover_style: &button_hover_style,
                }
            }
            if props.features.strikethrough {
                ToolbarButton {
                    title: "Strikethrough",
                    icon: "S",
                    command: "strikeThrough",
                    style: "{button_style}",
                    hover_style: &button_hover_style,
                }
            }

            // Separator
            if props.features.bold || props.features.italic || props.features.underline || props.features.strikethrough {
                ToolbarSeparator {}
            }

            // Headings
            if props.features.heading {
                ToolbarButton {
                    title: "Heading 1",
                    icon: "H1",
                    command: "formatBlock",
                    value: Some("H1"),
                    style: "{button_style}",
                    hover_style: &button_hover_style,
                }
                ToolbarButton {
                    title: "Heading 2",
                    icon: "H2",
                    command: "formatBlock",
                    value: Some("H2"),
                    style: "{button_style}",
                    hover_style: &button_hover_style,
                }
                ToolbarButton {
                    title: "Heading 3",
                    icon: "H3",
                    command: "formatBlock",
                    value: Some("H3"),
                    style: "{button_style}",
                    hover_style: &button_hover_style,
                }
                ToolbarSeparator {}
            }

            // Lists
            if props.features.bullet_list {
                ToolbarButton {
                    title: "Bullet List",
                    icon: "•",
                    command: "insertUnorderedList",
                    style: "{button_style}",
                    hover_style: &button_hover_style,
                }
            }
            if props.features.numbered_list {
                ToolbarButton {
                    title: "Numbered List",
                    icon: "1.",
                    command: "insertOrderedList",
                    style: "{button_style}",
                    hover_style: &button_hover_style,
                }
            }

            // Separator
            if props.features.bullet_list || props.features.numbered_list {
                ToolbarSeparator {}
            }

            // Alignment
            if props.features.align_left {
                ToolbarButton {
                    title: "Align Left",
                    icon: "←",
                    command: "justifyLeft",
                    style: "{button_style}",
                    hover_style: &button_hover_style,
                }
            }
            if props.features.align_center {
                ToolbarButton {
                    title: "Align Center",
                    icon: "↔",
                    command: "justifyCenter",
                    style: "{button_style}",
                    hover_style: &button_hover_style,
                }
            }
            if props.features.align_right {
                ToolbarButton {
                    title: "Align Right",
                    icon: "→",
                    command: "justifyRight",
                    style: "{button_style}",
                    hover_style: &button_hover_style,
                }
            }

            // Separator
            if props.features.align_left || props.features.align_center || props.features.align_right {
                ToolbarSeparator {}
            }

            // Special blocks
            if props.features.quote {
                ToolbarButton {
                    title: "Quote",
                    icon: "\"",
                    command: "formatBlock",
                    value: Some("BLOCKQUOTE"),
                    style: "{button_style}",
                    hover_style: &button_hover_style,
                }
            }
            if props.features.code_block {
                ToolbarButton {
                    title: "Code Block",
                    icon: "</>",
                    command: "formatBlock",
                    value: Some("PRE"),
                    style: "{button_style}",
                    hover_style: &button_hover_style,
                }
            }
            if props.features.link {
                ToolbarButton {
                    title: "Insert Link",
                    icon: "L",
                    command: "createLink",
                    value: Some("https://"),
                    style: "{button_style}",
                    hover_style: &button_hover_style,
                }
            }
        }
    }
}

/// Toolbar separator component
#[component]
fn ToolbarSeparator() -> Element {
    let theme = use_theme();

    rsx! {
        div {
            class: "rich-text-separator",
            style: "width: 1px; height: 24px; background: {theme.tokens.read().colors.border.to_rgba()}; margin: 0 4px;",
        }
    }
}

/// Toolbar button properties
#[derive(Props, Clone, PartialEq)]
pub struct ToolbarButtonProps {
    /// Button title (tooltip)
    pub title: &'static str,
    /// Button icon/text
    pub icon: &'static str,
    /// execCommand command name
    pub command: &'static str,
    /// Optional command value
    #[props(default)]
    pub value: Option<&'static str>,
    /// Base style
    pub style: String,
    /// Hover style
    pub hover_style: String,
}

/// Toolbar button component
#[component]
fn ToolbarButton(props: ToolbarButtonProps) -> Element {
    rsx! {
        button {
            type: "button",
            class: "rich-text-toolbar-button",
            title: "{props.title}",
            style: "{props.style} {props.hover_style}",
            onclick: move |_| {
                // Execute the formatting command
                execute_command(props.command, props.value);
            },
            "{props.icon}"
        }
    }
}

/// Execute a document.execCommand
fn execute_command(command: &str, value: Option<&str>) {
    // In a browser environment, this would call:
    // document.execCommand(command, false, value.unwrap_or(""));
    // For Dioxus, we use wasm-bindgen or inline JS
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace = document)]
            fn execCommand(command: &str, show_ui: bool, value: Option<&str>) -> bool;
        }

        let _ = execCommand(command, false, value);
    }

    // For non-WASM targets, commands are no-ops
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = command;
        let _ = value;
    }
}

/// Rich text content area properties
#[derive(Props, Clone, PartialEq)]
pub struct RichTextContentProps {
    /// Current value
    pub value: String,
    /// Placeholder text
    #[props(default)]
    pub placeholder: Option<String>,
    /// Disabled state
    #[props(default = false)]
    pub disabled: bool,
    /// Minimum height
    pub min_height: &'static str,
    /// Maximum height
    #[props(default)]
    pub max_height: Option<String>,
    /// Whether the editor is focused
    pub is_focused: bool,
    /// Input handler
    #[props(default)]
    pub on_input: Option<EventHandler<String>>,
}

/// Rich text content editable area
#[component]
pub fn RichTextContent(props: RichTextContentProps) -> Element {
    let theme = use_theme();
    let inner_html = use_signal(|| props.value.clone());

    let max_height_style = props
        .max_height
        .as_ref()
        .map(|h| format!("max-height: {};", h))
        .unwrap_or_default();

    let disabled_style = if props.disabled {
        "background: #f5f5f5; cursor: not-allowed; opacity: 0.7;"
    } else {
        ""
    };

    let focus_style = if props.is_focused {
        format!(
            "outline: 2px solid {}; outline-offset: -2px;",
            theme.tokens.read().colors.primary.to_rgba()
        )
    } else {
        String::new()
    };

    let content_style = use_style(|t| {
        Style::new()
            .w_full()
            .min_h(props.min_height)
            .p(&t.spacing, "md")
            .font_size(14)
            .line_height(1.6)
            .text_color(&t.colors.foreground)
            .build()
    });

    let on_input = {
        let on_input_handler = props.on_input.clone();
        move |_e: Event<dioxus::events::FormData>| {
            // In a real implementation, we would get innerHTML from the event target
            // For now, we trigger the handler with current content
            if let Some(handler) = &on_input_handler {
                handler.call(inner_html());
            }
        }
    };

    rsx! {
        div {
            class: "rich-text-content",
            style: "{content_style} {max_height_style} {disabled_style} {focus_style} overflow: auto;",

            // Placeholder (shown when empty)
            if inner_html().is_empty() && props.placeholder.is_some() {
                div {
                    style: "position: absolute; color: {theme.tokens.read().colors.muted.to_rgba()}; pointer-events: none;",
                    "{props.placeholder.clone().unwrap()}"
                }
            }

            // Content editable div
            div {
                contenteditable: !props.disabled,
                style: "min-height: 100%; outline: none;",
                oninput: on_input,

                // Render the initial content
                dangerous_inner_html: "{inner_html}",
            }
        }
    }
}

/// Simple rich text editor variant with fewer options
#[derive(Props, Clone, PartialEq)]
pub struct SimpleRichTextProps {
    /// Current HTML content
    #[props(default)]
    pub value: String,
    /// Change handler
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,
    /// Placeholder text
    #[props(default)]
    pub placeholder: Option<String>,
    /// Disabled state
    #[props(default = false)]
    pub disabled: bool,
}

/// Simple rich text editor with basic formatting only
#[component]
pub fn SimpleRichText(props: SimpleRichTextProps) -> Element {
    let features = RichTextFeatures {
        bold: true,
        italic: true,
        underline: true,
        strikethrough: false,
        heading: false,
        bullet_list: true,
        numbered_list: true,
        link: true,
        code_block: false,
        quote: false,
        align_left: false,
        align_center: false,
        align_right: false,
    };

    rsx! {
        RichTextEditor {
            value: props.value,
            on_change: props.on_change,
            placeholder: props.placeholder,
            disabled: props.disabled,
            features: features,
        }
    }
}

/// Minimal rich text editor with only text formatting
#[derive(Props, Clone, PartialEq)]
pub struct MinimalRichTextProps {
    /// Current HTML content
    #[props(default)]
    pub value: String,
    /// Change handler
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,
    /// Placeholder text
    #[props(default)]
    pub placeholder: Option<String>,
    /// Disabled state
    #[props(default = false)]
    pub disabled: bool,
}

/// Minimal rich text editor with text formatting only
#[component]
pub fn MinimalRichText(props: MinimalRichTextProps) -> Element {
    let features = RichTextFeatures {
        bold: true,
        italic: true,
        underline: true,
        strikethrough: false,
        heading: false,
        bullet_list: false,
        numbered_list: false,
        link: false,
        code_block: false,
        quote: false,
        align_left: false,
        align_center: false,
        align_right: false,
    };

    rsx! {
        RichTextEditor {
            value: props.value,
            on_change: props.on_change,
            placeholder: props.placeholder,
            disabled: props.disabled,
            min_height: "100px",
            features: features,
        }
    }
}

/// Full-featured rich text editor with all formatting options
#[derive(Props, Clone, PartialEq)]
pub struct FullRichTextProps {
    /// Current HTML content
    #[props(default)]
    pub value: String,
    /// Change handler
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,
    /// Placeholder text
    #[props(default)]
    pub placeholder: Option<String>,
    /// Disabled state
    #[props(default = false)]
    pub disabled: bool,
    /// Minimum height (default: "300px")
    #[props(default = "300px")]
    pub min_height: &'static str,
    /// Maximum height
    #[props(default)]
    pub max_height: Option<String>,
}

/// Full-featured rich text editor with all formatting options
#[component]
pub fn FullRichText(props: FullRichTextProps) -> Element {
    rsx! {
        RichTextEditor {
            value: props.value,
            on_change: props.on_change,
            placeholder: props.placeholder,
            disabled: props.disabled,
            min_height: props.min_height,
            max_height: props.max_height,
            features: RichTextFeatures::default(),
        }
    }
}
