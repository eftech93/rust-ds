//! Footer organism component
//!
//! Page footer with links, branding, and legal information.

use dioxus::prelude::*;
use crate::theme::use_theme;

/// Footer link group
#[derive(Clone, PartialEq, Debug)]
pub struct FooterLinkGroup {
    pub title: String,
    pub links: Vec<FooterLink>,
}

/// Footer link
#[derive(Clone, PartialEq, Debug)]
pub struct FooterLink {
    pub label: String,
    pub href: String,
    pub external: bool,
}

impl FooterLink {
    pub fn new(label: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: href.into(),
            external: false,
        }
    }
    
    pub fn external(mut self) -> Self {
        self.external = true;
        self
    }
}

impl FooterLinkGroup {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            links: Vec::new(),
        }
    }
    
    pub fn with_links(mut self, links: Vec<FooterLink>) -> Self {
        self.links = links;
        self
    }
}

/// Footer properties
#[derive(Props, Clone, PartialEq)]
pub struct FooterProps {
    /// Brand/logo element
    #[props(default)]
    pub brand: Option<Element>,
    /// Brand description
    #[props(default)]
    pub description: Option<String>,
    /// Link groups
    #[props(default)]
    pub link_groups: Vec<FooterLinkGroup>,
    /// Social links
    #[props(default)]
    pub social_links: Vec<(String, String)>, // (icon, url)
    /// Bottom bar content
    #[props(default)]
    pub bottom_content: Option<Element>,
    /// Copyright text
    #[props(default)]
    pub copyright: Option<String>,
    /// Variant
    #[props(default = FooterVariant::Default)]
    pub variant: FooterVariant,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Footer variant
#[derive(Default, Clone, PartialEq, Debug)]
pub enum FooterVariant {
    #[default]
    Default,
    Minimal,
    Dark,
}

/// Footer component
#[component]
pub fn Footer(props: FooterProps) -> Element {
    let theme = use_theme();
    
    let class_css = props.class.as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();
    
    let (bg_color, text_color, border_color) = match props.variant {
        FooterVariant::Default => (
            theme.tokens.read().colors.background.to_rgba(),
            theme.tokens.read().colors.foreground.to_rgba(),
            theme.tokens.read().colors.border.to_rgba(),
        ),
        FooterVariant::Minimal => (
            "transparent".to_string(),
            theme.tokens.read().colors.foreground.to_rgba(),
            theme.tokens.read().colors.border.to_rgba(),
        ),
        FooterVariant::Dark => (
            "#0f172a".to_string(),
            "#f8fafc".to_string(),
            "#334155".to_string(),
        ),
    };
    
    let muted_color = if props.variant == FooterVariant::Dark {
        "#94a3b8".to_string()
    } else {
        theme.tokens.read().colors.muted.to_rgba()
    };
    
    let variant_name = format!("{:?}", props.variant).to_lowercase();
    let grid_template = if props.link_groups.len() <= 3 { 
        "grid-template-columns: 2fr repeat(auto-fit, minmax(150px, 1fr));" 
    } else { 
        "grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));" 
    };
    let grid_col = if props.link_groups.is_empty() { "1 / -1" } else { "auto" };
    
    rsx! {
        footer {
            class: "footer footer-{variant_name}{class_css}",
            style: "background: {bg_color}; color: {text_color}; border-top: 1px solid {border_color};",
            
            // Main footer content
            if props.brand.is_some() || !props.link_groups.is_empty() {
                div {
                    class: "footer-main",
                    style: "max-width: 1200px; margin: 0 auto; padding: 48px 24px; display: grid; gap: 48px; {grid_template}",
                    
                    // Brand section
                    if props.brand.is_some() || props.description.is_some() {
                        div {
                            class: "footer-brand",
                            style: "grid-column: {grid_col};",
                            
                            if let Some(brand) = props.brand {
                                div {
                                    style: "margin-bottom: 16px;",
                                    {brand}
                                }
                            }
                            
                            if let Some(description) = props.description {
                                p {
                                    style: "font-size: 14px; line-height: 1.6; color: {muted_color}; margin: 0; max-width: 300px;",
                                    "{description}"
                                }
                            }
                            
                            // Social links
                            if !props.social_links.is_empty() {
                                div {
                                    class: "footer-social",
                                    style: "display: flex; gap: 12px; margin-top: 24px;",
                                    
                                    for (icon, url) in props.social_links.iter() {
                                        a {
                                            key: "{url}",
                                            href: "{url}",
                                            target: "_blank",
                                            rel: "noopener noreferrer",
                                            style: "width: 36px; height: 36px; border-radius: 50%; background: {muted_color}20; display: flex; align-items: center; justify-content: center; font-size: 16px; text-decoration: none; transition: background 0.15s ease;",
                                            "{icon}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    // Link groups
                    for group in props.link_groups.iter() {
                        div {
                            key: "{group.title}",
                            class: "footer-links",
                            
                            h3 {
                                style: "font-size: 14px; font-weight: 600; margin: 0 0 16px 0; color: {text_color};",
                                "{group.title}"
                            }
                            
                            ul {
                                style: "list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column; gap: 12px;",
                                
                                for link in group.links.iter() {
                                    li {
                                        key: "{link.label}",
                                        
                                        a {
                                            href: "{link.href}",
                                            target: if link.external { "_blank" } else { "" },
                                            rel: if link.external { "noopener noreferrer" } else { "" },
                                            style: "font-size: 14px; color: {muted_color}; text-decoration: none; transition: color 0.15s ease;",
                                            "{link.label}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            // Bottom bar
            if props.bottom_content.is_some() || props.copyright.is_some() {
                div {
                    class: "footer-bottom",
                    style: "border-top: 1px solid {border_color}; padding: 24px;",
                    
                    div {
                        style: "max-width: 1200px; margin: 0 auto; display: flex; flex-wrap: wrap; align-items: center; justify-content: space-between; gap: 16px;",
                        
                        if let Some(copyright) = props.copyright {
                            p {
                                style: "font-size: 14px; color: {muted_color}; margin: 0;",
                                "{copyright}"
                            }
                        }
                        
                        if let Some(content) = props.bottom_content {
                            div {
                                class: "footer-bottom-content",
                                {content}
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Simple footer properties
#[derive(Props, Clone, PartialEq)]
pub struct SimpleFooterProps {
    /// Logo/brand element
    #[props(default)]
    pub brand: Option<Element>,
    /// Links
    #[props(default)]
    pub links: Vec<FooterLink>,
    /// Copyright text
    pub copyright: String,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Simple footer (minimal version)
#[component]
pub fn SimpleFooter(props: SimpleFooterProps) -> Element {
    let theme = use_theme();
    
    let class_css = props.class.as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();
    
    rsx! {
        footer {
            class: "simple-footer{class_css}",
            style: "padding: 24px; border-top: 1px solid {theme.tokens.read().colors.border.to_rgba()};",
            
            div {
                style: "max-width: 1200px; margin: 0 auto; display: flex; flex-wrap: wrap; align-items: center; justify-content: space-between; gap: 16px;",
                
                if let Some(brand) = props.brand {
                    div {
                        {brand}
                    }
                }
                
                if !props.links.is_empty() {
                    nav {
                        style: "display: flex; gap: 24px;",
                        
                        for link in props.links.iter() {
                            a {
                                key: "{link.label}",
                                href: "{link.href}",
                                target: if link.external { "_blank" } else { "" },
                                rel: if link.external { "noopener noreferrer" } else { "" },
                                style: "font-size: 14px; color: {theme.tokens.read().colors.foreground.to_rgba()}; text-decoration: none;",
                                "{link.label}"
                            }
                        }
                    }
                }
                
                p {
                    style: "font-size: 14px; color: {theme.tokens.read().colors.muted.to_rgba()}; margin: 0;",
                    "{props.copyright}"
                }
            }
        }
    }
}
