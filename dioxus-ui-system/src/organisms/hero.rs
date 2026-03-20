//! Hero Section organism component
//!
//! Prominent page header with CTA for landing pages.

use dioxus::prelude::*;
use crate::theme::use_theme;

/// Hero section properties
#[derive(Props, Clone, PartialEq)]
pub struct HeroProps {
    /// Main headline
    pub title: String,
    /// Subtitle/description
    #[props(default)]
    pub subtitle: Option<String>,
    /// Background element (image, gradient, video)
    #[props(default)]
    pub background: Option<Element>,
    /// Primary CTA button
    #[props(default)]
    pub primary_cta: Option<HeroCta>,
    /// Secondary CTA button
    #[props(default)]
    pub secondary_cta: Option<HeroCta>,
    /// Social proof element
    #[props(default)]
    pub social_proof: Option<Element>,
    /// Feature highlights
    #[props(default)]
    pub features: Vec<String>,
    /// Alignment
    #[props(default = HeroAlign::Center)]
    pub align: HeroAlign,
    /// Size variant
    #[props(default = HeroSize::Lg)]
    pub size: HeroSize,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Hero CTA configuration
#[derive(Clone, PartialEq, Debug)]
pub struct HeroCta {
    pub label: String,
    pub on_click: Option<EventHandler<()>>,
    pub href: Option<String>,
}

impl HeroCta {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            on_click: None,
            href: None,
        }
    }
    
    pub fn with_on_click(mut self, handler: EventHandler<()>) -> Self {
        self.on_click = Some(handler);
        self
    }
    
    pub fn with_href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }
}

/// Hero alignment
#[derive(Default, Clone, PartialEq, Debug)]
pub enum HeroAlign {
    #[default]
    Center,
    Left,
    Right,
}

/// Hero size
#[derive(Default, Clone, PartialEq, Debug)]
pub enum HeroSize {
    Sm,
    Md,
    #[default]
    Lg,
    Xl,
}

impl HeroSize {
    fn to_padding(&self) -> &'static str {
        match self {
            HeroSize::Sm => "48px 24px",
            HeroSize::Md => "80px 24px",
            HeroSize::Lg => "120px 24px",
            HeroSize::Xl => "160px 24px",
        }
    }
}

/// Hero component
#[component]
pub fn Hero(props: HeroProps) -> Element {
    let theme = use_theme();
    
    let class_css = props.class.as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();
    
    let (text_align, align_items) = match props.align {
        HeroAlign::Center => ("center", "center"),
        HeroAlign::Left => ("left", "flex-start"),
        HeroAlign::Right => ("right", "flex-end"),
    };
    
    let padding = props.size.to_padding();
    
    rsx! {
        section {
            class: "hero{class_css}",
            style: "position: relative; padding: {padding}; display: flex; flex-direction: column; align-items: {align_items}; text-align: {text_align}; overflow: hidden;",
            
            // Background
            if let Some(background) = props.background {
                div {
                    class: "hero-background",
                    style: "position: absolute; inset: 0; z-index: 0;",
                    {background}
                }
            }
            
            // Content
            div {
                class: "hero-content",
                style: "position: relative; z-index: 1; max-width: 900px;",
                
                // Title
                h1 {
                    class: "hero-title",
                    style: "margin: 0 0 24px 0; font-size: clamp(36px, 5vw, 64px); font-weight: 800; line-height: 1.1; color: {theme.tokens.read().colors.foreground.to_rgba()}; letter-spacing: -0.02em;",
                    "{props.title}"
                }
                
                // Subtitle
                if let Some(subtitle) = props.subtitle {
                    p {
                        class: "hero-subtitle",
                        style: "margin: 0 0 32px 0; font-size: clamp(18px, 2vw, 24px); line-height: 1.6; color: {theme.tokens.read().colors.muted.to_rgba()}; max-width: 640px;",
                        "{subtitle}"
                    }
                }
                
                // Features
                if !props.features.is_empty() {
                    ul {
                        class: "hero-features",
                        style: format!("list-style: none; padding: 0; margin: 0 0 32px 0; display: flex; flex-wrap: wrap; justify-content: {}; gap: 16px 32px;", if props.align == HeroAlign::Center { "center" } else { "flex-start" }),
                        
                        for feature in props.features.iter() {
                            li {
                                key: "{feature}",
                                style: "display: flex; align-items: center; gap: 8px; font-size: 16px; color: {theme.tokens.read().colors.foreground.to_rgba()};",
                                
                                span {
                                    style: "color: #22c55e; font-weight: 600;",
                                    "✓"
                                }
                                
                                "{feature}"
                            }
                        }
                    }
                }
                
                // CTAs
                if props.primary_cta.is_some() || props.secondary_cta.is_some() {
                    div {
                        class: "hero-ctas",
                        style: format!("display: flex; flex-wrap: wrap; justify-content: {}; gap: 16px;", if props.align == HeroAlign::Center { "center" } else { "flex-start" }),
                        
                        {{
                            let cta = props.primary_cta.clone();
                            cta.map(|cta| {
                                if let Some(href) = cta.href {
                                    rsx! {
                                        a {
                                            href: "{href}",
                                            class: "hero-cta hero-cta-primary",
                                            style: "display: inline-flex; align-items: center; justify-content: center; padding: 16px 32px; font-size: 16px; font-weight: 600; color: white; background: {theme.tokens.read().colors.primary.to_rgba()}; border-radius: 8px; text-decoration: none; transition: transform 0.15s ease, box-shadow 0.15s ease;",
                                            "{cta.label}"
                                        }
                                    }
                                } else {
                                    rsx! {
                                        button {
                                            type: "button",
                                            class: "hero-cta hero-cta-primary",
                                            style: "padding: 16px 32px; font-size: 16px; font-weight: 600; color: white; background: {theme.tokens.read().colors.primary.to_rgba()}; border: none; border-radius: 8px; cursor: pointer; transition: transform 0.15s ease, box-shadow 0.15s ease;",
                                            onclick: move |_| {
                                                if let Some(handler) = &cta.on_click {
                                                    handler.call(());
                                                }
                                            },
                                            "{cta.label}"
                                        }
                                    }
                                }
                            })
                        }}
                        
                        {{
                            let cta = props.secondary_cta.clone();
                            cta.map(|cta| {
                                if let Some(href) = cta.href {
                                    rsx! {
                                        a {
                                            href: "{href}",
                                            class: "hero-cta hero-cta-secondary",
                                            style: "display: inline-flex; align-items: center; justify-content: center; padding: 16px 32px; font-size: 16px; font-weight: 600; color: {theme.tokens.read().colors.foreground.to_rgba()}; background: transparent; border: 2px solid {theme.tokens.read().colors.border.to_rgba()}; border-radius: 8px; text-decoration: none; transition: border-color 0.15s ease;",
                                            "{cta.label}"
                                        }
                                    }
                                } else {
                                    rsx! {
                                        button {
                                            type: "button",
                                            class: "hero-cta hero-cta-secondary",
                                            style: "padding: 16px 32px; font-size: 16px; font-weight: 600; color: {theme.tokens.read().colors.foreground.to_rgba()}; background: transparent; border: 2px solid {theme.tokens.read().colors.border.to_rgba()}; border-radius: 8px; cursor: pointer; transition: border-color 0.15s ease;",
                                            onclick: move |_| {
                                                if let Some(handler) = &cta.on_click {
                                                    handler.call(());
                                                }
                                            },
                                            "{cta.label}"
                                        }
                                    }
                                }
                            })
                        }}
                    }
                }
                
                // Social proof
                if let Some(social_proof) = props.social_proof {
                    div {
                        class: "hero-social-proof",
                        style: "margin-top: 48px;",
                        {social_proof}
                    }
                }
            }
        }
    }
}

/// Hero with image properties
#[derive(Props, Clone, PartialEq)]
pub struct HeroWithImageProps {
    /// Hero content
    pub children: Element,
    /// Image element
    pub image: Element,
    /// Image position
    #[props(default = ImagePosition::Right)]
    pub image_position: ImagePosition,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Image position
#[derive(Default, Clone, PartialEq, Debug)]
pub enum ImagePosition {
    #[default]
    Right,
    Left,
}

/// Hero with image (split layout)
#[component]
pub fn HeroWithImage(props: HeroWithImageProps) -> Element {
    let class_css = props.class.as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();
    
    let (content_order, image_order) = match props.image_position {
        ImagePosition::Left => (2, 1),
        ImagePosition::Right => (1, 2),
    };
    
    rsx! {
        section {
            class: "hero-split{class_css}",
            style: "padding: 80px 24px; display: grid; grid-template-columns: 1fr 1fr; gap: 64px; align-items: center; max-width: 1200px; margin: 0 auto;",
            
            div {
                class: "hero-split-content",
                style: "order: {content_order};",
                {props.children}
            }
            
            div {
                class: "hero-split-image",
                style: "order: {image_order};",
                {props.image}
            }
        }
    }
}

/// Social proof bar properties
#[derive(Props, Clone, PartialEq)]
pub struct SocialProofBarProps {
    /// Text (e.g., "Trusted by 10,000+ companies")
    pub text: String,
    /// Logos or avatars
    pub items: Vec<Element>,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Social proof bar component
#[component]
pub fn SocialProofBar(props: SocialProofBarProps) -> Element {
    let theme = use_theme();
    
    let class_css = props.class.as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();
    
    rsx! {
        div {
            class: "social-proof-bar{class_css}",
            style: "display: flex; flex-wrap: wrap; align-items: center; justify-content: center; gap: 16px;",
            
            if !props.items.is_empty() {
                div {
                    class: "social-proof-items",
                    style: "display: flex; align-items: center;",
                    
                    for (i, item) in props.items.iter().enumerate() {
                        div {
                            key: "{i}",
                            style: if i > 0 { 
                                format!("margin-left: -12px; z-index: {}; border: 2px solid white; border-radius: 50%;", props.items.len() - i)
                            } else { 
                                format!("margin-left: 0; z-index: {}; border: 2px solid white; border-radius: 50%;", props.items.len() - i)
                            },
                            {item.clone()}
                        }
                    }
                }
            }
            
            p {
                class: "social-proof-text",
                style: "margin: 0; font-size: 14px; color: {theme.tokens.read().colors.muted.to_rgba()};",
                "{props.text}"
            }
        }
    }
}
