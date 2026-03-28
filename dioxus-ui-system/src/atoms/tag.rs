//! Tag/Chip atom component
//!
//! Categorization, filtering, and selection tags.

use crate::theme::use_theme;
use dioxus::prelude::*;

/// Tag variant/style
#[derive(Default, Clone, PartialEq, Debug)]
pub enum TagVariant {
    #[default]
    Default,
    Primary,
    Secondary,
    Success,
    Warning,
    Error,
    Outline,
}

/// Tag size
#[derive(Default, Clone, PartialEq, Debug)]
pub enum TagSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Tag properties
#[derive(Props, Clone, PartialEq)]
pub struct TagProps {
    /// Tag content/label
    pub children: Element,
    /// Visual variant
    #[props(default = TagVariant::Default)]
    pub variant: TagVariant,
    /// Size variant
    #[props(default = TagSize::Md)]
    pub size: TagSize,
    /// Whether the tag is selected/active
    #[props(default = false)]
    pub selected: bool,
    /// Whether the tag is selectable (clickable)
    #[props(default = false)]
    pub selectable: bool,
    /// Click handler
    #[props(default)]
    pub on_click: Option<EventHandler<()>>,
    /// Removable (shows close button)
    #[props(default = false)]
    pub removable: bool,
    /// Remove handler
    #[props(default)]
    pub on_remove: Option<EventHandler<()>>,
    /// Icon (emoji or icon name)
    #[props(default)]
    pub icon: Option<String>,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Custom background color
    pub color: Option<String>,
    /// Custom text color
    pub text_color: Option<String>,
    /// Disabled state
    #[props(default = false)]
    pub disabled: bool,
}

/// Tag component
#[component]
pub fn Tag(props: TagProps) -> Element {
    let theme = use_theme();

    let class_css = props
        .class
        .as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();

    let (bg_color, border_color, text_color) = if let Some(custom_color) = props.color.clone() {
        let text = props
            .text_color
            .clone()
            .unwrap_or_else(|| "white".to_string());
        (custom_color.clone(), custom_color, text)
    } else {
        get_variant_colors(&props.variant, &props.selected, theme.clone())
    };

    let padding = match props.size {
        TagSize::Sm => "2px 8px",
        TagSize::Md => "4px 12px",
        TagSize::Lg => "6px 16px",
    };

    let font_size = match props.size {
        TagSize::Sm => "12px",
        TagSize::Md => "14px",
        TagSize::Lg => "16px",
    };

    let border = if props.variant == TagVariant::Outline || props.selected {
        format!("1px solid {border_color}")
    } else {
        "1px solid transparent".to_string()
    };

    let cursor = if props.disabled {
        "not-allowed"
    } else if props.selectable || props.on_click.is_some() {
        "pointer"
    } else {
        "default"
    };

    let opacity = if props.disabled { "0.5" } else { "1" };

    let variant_name = format!("{:?}", props.variant).to_lowercase();
    let icon_size = match props.size {
        TagSize::Sm => 12,
        _ => 14,
    };
    let remove_size = match props.size {
        TagSize::Sm => 10,
        _ => 12,
    };
    let on_click = props.on_click.clone();

    rsx! {
        span {
            class: "tag tag-{variant_name}{class_css}",
            style: "display: inline-flex; align-items: center; gap: 4px; padding: {padding}; font-size: {font_size}; font-weight: 500; background: {bg_color}; color: {text_color}; border: {border}; border-radius: 9999px; cursor: {cursor}; opacity: {opacity}; transition: all 0.15s ease; user-select: none;",
            onclick: move |_| {
                if let Some(handler) = &on_click {
                    handler.call(());
                }
            },

            if let Some(icon) = props.icon {
                span {
                    class: "tag-icon",
                    style: "font-size: {icon_size}px;",
                    "{icon}"
                }
            }

            span {
                class: "tag-content",
                {props.children}
            }

            if props.removable {
                button {
                    type: "button",
                    class: "tag-remove",
                    style: "display: inline-flex; align-items: center; justify-content: center; margin-left: 2px; padding: 2px; background: none; border: none; cursor: pointer; font-size: {remove_size}px; color: inherit; opacity: 0.7; border-radius: 50%;",
                    onclick: move |e: Event<dioxus::html::MouseData>| {
                        e.stop_propagation();
                        if let Some(handler) = &props.on_remove {
                            handler.call(());
                        }
                    },
                    "✕"
                }
            }
        }
    }
}

fn get_variant_colors(
    variant: &TagVariant,
    selected: &bool,
    theme: crate::theme::ThemeContext,
) -> (String, String, String) {
    let tokens = theme.tokens.read();

    match variant {
        TagVariant::Default => {
            if *selected {
                (
                    tokens.colors.primary.to_rgba(),
                    tokens.colors.primary.to_rgba(),
                    "white".to_string(),
                )
            } else {
                (
                    tokens.colors.muted.to_rgba(),
                    tokens.colors.border.to_rgba(),
                    tokens.colors.foreground.to_rgba(),
                )
            }
        }
        TagVariant::Primary => (
            tokens.colors.primary.to_rgba(),
            tokens.colors.primary.to_rgba(),
            "white".to_string(),
        ),
        TagVariant::Secondary => (
            tokens.colors.secondary.to_rgba(),
            tokens.colors.secondary.to_rgba(),
            "white".to_string(),
        ),
        TagVariant::Success => (
            "#dcfce7".to_string(),
            "#16a34a".to_string(),
            "#166534".to_string(),
        ),
        TagVariant::Warning => (
            "#fef9c3".to_string(),
            "#ca8a04".to_string(),
            "#854d0e".to_string(),
        ),
        TagVariant::Error => (
            tokens.colors.destructive.to_rgba(),
            tokens.colors.destructive.to_rgba(),
            "white".to_string(),
        ),
        TagVariant::Outline => (
            "transparent".to_string(),
            tokens.colors.border.to_rgba(),
            tokens.colors.foreground.to_rgba(),
        ),
    }
}

/// Tag group properties
#[derive(Props, Clone, PartialEq)]
pub struct TagGroupProps {
    /// Tags to display
    pub tags: Vec<TagData>,
    /// Whether multiple tags can be selected
    #[props(default = false)]
    pub multiple: bool,
    /// Currently selected tag(s)
    #[props(default)]
    pub selected: Vec<String>,
    /// Selection change handler
    pub on_change: EventHandler<Vec<String>>,
    /// Whether tags are removable
    #[props(default = false)]
    pub removable: bool,
    /// Remove handler
    #[props(default)]
    pub on_remove: Option<EventHandler<String>>,
    /// Size variant
    #[props(default = TagSize::Md)]
    pub size: TagSize,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Tag data for TagGroup
#[derive(Clone, PartialEq, Debug)]
pub struct TagData {
    pub id: String,
    pub label: String,
    pub icon: Option<String>,
    pub variant: Option<TagVariant>,
}

impl TagData {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            variant: None,
        }
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_variant(mut self, variant: TagVariant) -> Self {
        self.variant = Some(variant);
        self
    }
}

/// Tag group component
#[component]
pub fn TagGroup(props: TagGroupProps) -> Element {
    let class_css = props
        .class
        .as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();

    let selected = props.selected.clone();

    rsx! {
        div {
            class: "tag-group{class_css}",
            style: "display: flex; flex-wrap: wrap; gap: 8px;",

            for tag in props.tags.iter() {
                {
                    let is_selected = selected.contains(&tag.id);
                    let variant = tag.variant.clone().unwrap_or(TagVariant::Default);

                    let on_click = {
                        let on_change = props.on_change.clone();
                        let tag_id = tag.id.clone();
                        let selected = selected.clone();
                        move |_| {
                            let mut new_selected = selected.clone();
                            if props.multiple {
                                if new_selected.contains(&tag_id) {
                                    new_selected.retain(|id| id != &tag_id);
                                } else {
                                    new_selected.push(tag_id.clone());
                                }
                            } else {
                                new_selected = vec![tag_id.clone()];
                            }
                            on_change.call(new_selected);
                        }
                    };

                    let on_remove = props.on_remove.as_ref().map(|handler| {
                        let h = handler.clone();
                        let tag_id = tag.id.clone();
                        Callback::new(move |_| {
                            h.call(tag_id.clone());
                        })
                    });

                    rsx! {
                        Tag {
                            key: "{tag.id}",
                            variant: variant,
                            size: props.size.clone(),
                            selected: is_selected,
                            selectable: true,
                            on_click: on_click,
                            removable: props.removable,
                            on_remove: on_remove,
                            icon: tag.icon.clone(),

                            "{tag.label}"
                        }
                    }
                }
            }
        }
    }
}

/// Input tag properties (tag input with add/remove)
#[derive(Props, Clone, PartialEq)]
pub struct InputTagProps {
    /// Current tags
    #[props(default)]
    pub tags: Vec<String>,
    /// Change handler
    pub on_change: EventHandler<Vec<String>>,
    /// Placeholder text
    #[props(default)]
    pub placeholder: Option<String>,
    /// Maximum number of tags
    #[props(default = usize::MAX)]
    pub max_tags: usize,
    /// Whether duplicates are allowed
    #[props(default = false)]
    pub allow_duplicates: bool,
    /// Tag variant
    #[props(default = TagVariant::Default)]
    pub tag_variant: TagVariant,
    /// Size variant
    #[props(default = TagSize::Md)]
    pub size: TagSize,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Input tag component (tag input)
#[component]
pub fn InputTag(props: InputTagProps) -> Element {
    let mut input_value = use_signal(|| String::new());
    let mut is_focused = use_signal(|| false);

    let class_css = props
        .class
        .as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();

    let can_add = props.tags.len() < props.max_tags;

    let border_color = if is_focused() {
        "#3b82f6".to_string()
    } else {
        "#e2e8f0".to_string()
    };

    rsx! {
        div {
            class: "input-tag{class_css}",
            style: "display: flex; flex-wrap: wrap; gap: 8px; padding: 8px; border: 1px solid {border_color}; border-radius: 8px; background: white; min-height: 42px; align-items: center;",

            for (i, tag) in props.tags.iter().enumerate() {
                {
                    let tag_text = tag.clone();
                    let on_remove = {
                        let on_change = props.on_change.clone();
                        let tags = props.tags.clone();
                        move |_| {
                            let mut new_tags = tags.clone();
                            new_tags.remove(i);
                            on_change.call(new_tags);
                        }
                    };

                    rsx! {
                        Tag {
                            key: "{i}",
                            variant: props.tag_variant.clone(),
                            size: props.size.clone(),
                            removable: true,
                            on_remove: Callback::new(on_remove),

                            "{tag_text}"
                        }
                    }
                }
            }

            if can_add {
                input {
                    type: "text",
                    class: "input-tag-field",
                    placeholder: "{props.placeholder.clone().unwrap_or_else(|| \"Add tag...\".to_string())}",
                    value: "{input_value}",
                    style: "flex: 1; min-width: 80px; border: none; outline: none; font-size: 14px; padding: 4px;",
                    onfocus: move |_| is_focused.set(true),
                    onblur: {
                        let on_change = props.on_change.clone();
                        let tags = props.tags.clone();
                        let allow_duplicates = props.allow_duplicates;
                        move |_| {
                            is_focused.set(false);
                            let value = input_value();
                            if !value.is_empty() {
                                let trimmed = value.trim().to_string();
                                if !trimmed.is_empty() {
                                    if allow_duplicates || !tags.contains(&trimmed) {
                                        let mut new_tags = tags.clone();
                                        new_tags.push(trimmed);
                                        on_change.call(new_tags);
                                    }
                                }
                                input_value.set(String::new());
                            }
                        }
                    },
                    oninput: move |e: Event<FormData>| {
                        input_value.set(e.value());
                    },
                    onkeydown: {
                        let on_change = props.on_change.clone();
                        let tags = props.tags.clone();
                        let allow_duplicates = props.allow_duplicates;
                        move |e: Event<dioxus::html::KeyboardData>| {
                            use dioxus::html::input_data::keyboard_types::Key;
                            if e.key() == Key::Enter {
                                let value = input_value();
                                if !value.is_empty() {
                                    let trimmed = value.trim().to_string();
                                    if !trimmed.is_empty() {
                                        if allow_duplicates || !tags.contains(&trimmed) {
                                            let mut new_tags = tags.clone();
                                            new_tags.push(trimmed);
                                            on_change.call(new_tags);
                                        }
                                    }
                                    input_value.set(String::new());
                                }
                            } else if e.key() == Key::Backspace && input_value().is_empty() && !tags.is_empty() {
                                let mut new_tags = tags.clone();
                                new_tags.pop();
                                on_change.call(new_tags);
                            }
                        }
                    },
                }
            }
        }
    }
}
