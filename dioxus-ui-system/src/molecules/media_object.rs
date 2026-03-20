//! Media Object molecule component
//!
//! Image + text content with flexible alignment.

use dioxus::prelude::*;
use crate::theme::use_theme;

/// Media alignment
#[derive(Default, Clone, PartialEq, Debug)]
pub enum MediaAlign {
    #[default]
    Start,
    Center,
    End,
}

/// Media object properties
#[derive(Props, Clone, PartialEq)]
pub struct MediaObjectProps {
    /// Media content (image, icon, avatar)
    pub media: Element,
    /// Body content (text)
    pub children: Element,
    /// Media alignment
    #[props(default = MediaAlign::Start)]
    pub align: MediaAlign,
    /// Spacing between media and content
    #[props(default = 16)]
    pub gap: u16,
    /// Whether to reverse the order (content first)
    #[props(default = false)]
    pub reverse: bool,
    /// Stack on mobile
    #[props(default = true)]
    pub responsive: bool,
    /// Media width (when stacked)
    #[props(default)]
    pub media_width: Option<String>,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Media object component (image + text layout)
#[component]
pub fn MediaObject(props: MediaObjectProps) -> Element {
    let class_css = props.class.as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();
    
    let flex_direction = if props.reverse { "row-reverse" } else { "row" };
    let align_items = match props.align {
        MediaAlign::Start => "flex-start",
        MediaAlign::Center => "center",
        MediaAlign::End => "flex-end",
    };
    
    let responsive_css = if props.responsive {
        format!("@media (max-width: 640px) {{ .media-object{class_css} {{ flex-direction: column; }} }}")
    } else {
        String::new()
    };
    
    let media_style = props.media_width.as_ref()
        .map(|w| format!("flex-shrink: 0; width: {};", w))
        .unwrap_or_else(|| "flex-shrink: 0;".to_string());
    
    let gap = props.gap;
    
    rsx! {
        div {
            class: "media-object{class_css}",
            style: "display: flex; flex-direction: {flex_direction}; align-items: {align_items}; gap: {gap}px;",
            
            div {
                class: "media-object-media",
                style: "{media_style}",
                {props.media}
            }
            
            div {
                class: "media-object-content",
                style: "flex: 1; min-width: 0;",
                {props.children}
            }
        }
        
        if !responsive_css.is_empty() {
            style { "{{ {responsive_css} }}" }
        }
    }
}

/// Media content properties
#[derive(Props, Clone, PartialEq)]
pub struct MediaContentProps {
    /// Title/heading
    #[props(default)]
    pub title: Option<String>,
    /// Title element level
    #[props(default = 4)]
    pub title_level: u8,
    /// Description/body text
    #[props(default)]
    pub description: Option<String>,
    /// Additional content
    pub children: Option<Element>,
    /// Metadata (date, author, etc.)
    #[props(default)]
    pub meta: Option<String>,
    /// Actions (buttons, links)
    #[props(default)]
    pub actions: Option<Element>,
}

/// Media content component (structured text content)
#[component]
pub fn MediaContent(props: MediaContentProps) -> Element {
    let theme = use_theme();
    
    let _title_tag = format!("h{}", props.title_level.clamp(1, 6));
    let font_size = match props.title_level {
        1 => 24,
        2 => 20,
        3 => 18,
        _ => 16,
    };
    let title_color = theme.tokens.read().colors.foreground.to_rgba();
    let title_style = format!("margin: 0; font-size: {}px; font-weight: 600; color: {}; line-height: 1.3;", font_size, title_color);
    
    rsx! {
        div {
            class: "media-content",
            style: "display: flex; flex-direction: column; gap: 8px;",
            
            if let Some(title) = props.title.clone() {
                match props.title_level {
                    1 => rsx! { h1 { class: "media-content-title", style: "{title_style}", "{title}" } },
                    2 => rsx! { h2 { class: "media-content-title", style: "{title_style}", "{title}" } },
                    3 => rsx! { h3 { class: "media-content-title", style: "{title_style}", "{title}" } },
                    4 => rsx! { h4 { class: "media-content-title", style: "{title_style}", "{title}" } },
                    5 => rsx! { h5 { class: "media-content-title", style: "{title_style}", "{title}" } },
                    _ => rsx! { h6 { class: "media-content-title", style: "{title_style}", "{title}" } },
                }
            }
            
            if let Some(meta) = props.meta {
                span {
                    class: "media-content-meta",
                    style: "font-size: 12px; color: {theme.tokens.read().colors.muted.to_rgba()};",
                    "{meta}"
                }
            }
            
            if let Some(description) = props.description {
                p {
                    class: "media-content-description",
                    style: "margin: 0; font-size: 14px; color: {theme.tokens.read().colors.foreground.to_rgba()}; line-height: 1.6;",
                    "{description}"
                }
            }
            
            if let Some(children) = props.children {
                div {
                    class: "media-content-body",
                    {children}
                }
            }
            
            if let Some(actions) = props.actions {
                div {
                    class: "media-content-actions",
                    style: "display: flex; gap: 8px; margin-top: 4px;",
                    {actions}
                }
            }
        }
    }
}

/// Comment item properties
#[derive(Props, Clone, PartialEq)]
pub struct CommentProps {
    /// Author name
    pub author: String,
    /// Author avatar
    #[props(default)]
    pub avatar: Option<Element>,
    /// Comment text
    pub content: String,
    /// Timestamp
    pub timestamp: String,
    /// Reply action
    #[props(default)]
    pub on_reply: Option<EventHandler<()>>,
    /// Like action
    #[props(default)]
    pub on_like: Option<EventHandler<()>>,
    /// Whether liked
    #[props(default = false)]
    pub liked: bool,
    /// Like count
    #[props(default = 0)]
    pub like_count: u32,
    /// Nested replies
    #[props(default)]
    pub replies: Option<Element>,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Comment component (nested discussion)
#[component]
pub fn Comment(props: CommentProps) -> Element {
    let theme = use_theme();
    
    let class_css = props.class.as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();
    
    let like_color = if props.liked { "#ef4444".to_string() } else { theme.tokens.read().colors.muted.to_rgba() };
    
    let avatar_element = props.avatar.unwrap_or_else(|| {
        rsx! {
            div {
                style: "width: 40px; height: 40px; border-radius: 50%; background: {theme.tokens.read().colors.muted.to_rgba()}; display: flex; align-items: center; justify-content: center; font-size: 16px; font-weight: 600; color: {theme.tokens.read().colors.foreground.to_rgba()};",
                "{props.author.chars().next().unwrap_or('?').to_uppercase()}"
            }
        }
    });
    
    rsx! {
        div {
            class: "comment{class_css}",
            style: "display: flex; gap: 12px;",
            
            div {
                class: "comment-avatar",
                style: "flex-shrink: 0;",
                {avatar_element}
            }
            
            div {
                class: "comment-body",
                style: "flex: 1;",
                
                div {
                    class: "comment-header",
                    style: "display: flex; align-items: center; gap: 8px; margin-bottom: 4px;",
                    
                    span {
                        class: "comment-author",
                        style: "font-weight: 600; font-size: 14px; color: {theme.tokens.read().colors.foreground.to_rgba()};",
                        "{props.author}"
                    }
                    
                    span {
                        class: "comment-timestamp",
                        style: "font-size: 12px; color: {theme.tokens.read().colors.muted.to_rgba()};",
                        "{props.timestamp}"
                    }
                }
                
                p {
                    class: "comment-content",
                    style: "margin: 0 0 8px 0; font-size: 14px; color: {theme.tokens.read().colors.foreground.to_rgba()}; line-height: 1.5;",
                    "{props.content}"
                }
                
                div {
                    class: "comment-actions",
                    style: "display: flex; gap: 16px;",
                    
                    if let Some(on_reply) = props.on_reply {
                        button {
                            type: "button",
                            class: "comment-reply",
                            style: "font-size: 13px; color: {theme.tokens.read().colors.muted.to_rgba()}; background: none; border: none; cursor: pointer;",
                            onclick: move |_| on_reply.call(()),
                            "Reply"
                        }
                    }
                    
                    if let Some(on_like) = props.on_like {
                        button {
                            type: "button",
                            class: "comment-like",
                            style: "font-size: 13px; color: {like_color}; background: none; border: none; cursor: pointer; display: flex; align-items: center; gap: 4px;",
                            onclick: move |_| on_like.call(()),
                            
                            if props.liked {
                                "❤️"
                            } else {
                                "🤍"
                            }
                            
                            if props.like_count > 0 {
                                "{props.like_count}"
                            }
                        }
                    }
                }
                
                if let Some(replies) = props.replies {
                    div {
                        class: "comment-replies",
                        style: "margin-top: 16px; padding-left: 20px; border-left: 2px solid {theme.tokens.read().colors.border.to_rgba()};",
                        {replies}
                    }
                }
            }
        }
    }
}
