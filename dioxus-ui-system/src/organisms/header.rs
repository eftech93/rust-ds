//! Header organism component
//!
//! Application header with navigation, branding, and actions.

use dioxus::prelude::*;
use crate::theme::{use_theme, use_style};
use crate::styles::Style;
use crate::atoms::{Button, ButtonVariant, Icon, IconSize, IconColor, Heading, HeadingLevel};

/// Navigation item
#[derive(Clone, PartialEq)]
pub struct NavItem {
    pub label: String,
    pub href: String,
    pub icon: Option<String>,
    pub active: bool,
}

/// Header properties
#[derive(Props, Clone, PartialEq)]
pub struct HeaderProps {
    /// Brand/logo element
    #[props(default)]
    pub brand: Option<Element>,
    /// Brand title text (alternative to brand element)
    #[props(default)]
    pub brand_title: Option<String>,
    /// Navigation items
    #[props(default)]
    pub nav_items: Vec<NavItem>,
    /// Right-side actions
    #[props(default)]
    pub actions: Option<Element>,
    /// Whether header is sticky
    #[props(default = true)]
    pub sticky: bool,
    /// Whether to show border
    #[props(default = true)]
    pub bordered: bool,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Header organism component
///
/// # Example
/// ```rust,ignore
/// use dioxus_ui_system::organisms::{Header, NavItem};
///
/// let nav_items = vec![
///     NavItem {
///         label: "Home".to_string(),
///         href: "/".to_string(),
///         icon: Some("home".to_string()),
///         active: true,
///     },
///     NavItem {
///         label: "About".to_string(),
///         href: "/about".to_string(),
///         icon: None,
///         active: false,
///     },
/// ];
///
/// rsx! {
///     Header {
///         brand_title: "My App",
///         nav_items: nav_items,
///     }
/// }
/// ```
#[component]
pub fn Header(props: HeaderProps) -> Element {
    let _theme = use_theme();
    let sticky = props.sticky;
    let bordered = props.bordered;
    
    let style = use_style(move |t| {
        let base = Style::new()
            .w_full()
            .h_px(64)
            .flex()
            .items_center()
            .justify_between()
            .px(&t.spacing, "lg")
            .bg(&t.colors.background)
            .z_index(50);
            
        // Sticky positioning
        let base = if sticky {
            base.position("sticky").top("0")
        } else {
            base
        };
        
        // Border
        if bordered {
            base.border_bottom(1, &t.colors.border)
        } else {
            base
        }.build()
    });
    
    let final_style = if let Some(custom) = &props.style {
        format!("{} {}", style(), custom)
    } else {
        style()
    };
    
    // Brand element
    let brand = if let Some(brand_el) = props.brand {
        brand_el
    } else if let Some(title) = &props.brand_title {
        rsx! {
            div {
                style: "display: flex; align-items: center; gap: 8px;",
                Heading {
                    level: HeadingLevel::H4,
                    "{title}"
                }
            }
        }
    } else {
        rsx! {}
    };
    
    // Navigation
    let nav_items = props.nav_items.clone();
    let has_nav = !nav_items.is_empty();
    
    let nav_style = use_style(|t| {
        Style::new()
            .flex()
            .items_center()
            .gap(&t.spacing, "md")
            .build()
    });
    
    let nav = if has_nav {
        rsx! {
            nav {
                style: "{nav_style}",
                for item in nav_items {
                    HeaderNavLink { item: item }
                }
            }
        }
    } else {
        rsx! {}
    };
    
    rsx! {
        header {
            style: "{final_style}",
            
            // Left section: Brand + Nav
            div {
                style: "display: flex; align-items: center; gap: 32px;",
                {brand}
                {nav}
            }
            
            // Right section: Actions
            if props.actions.is_some() {
                div {
                    style: "display: flex; align-items: center; gap: 8px;",
                    {props.actions.unwrap()}
                }
            }
        }
    }
}

/// Header navigation link component
#[derive(Props, Clone, PartialEq)]
pub struct HeaderNavLinkProps {
    pub item: NavItem,
}

#[component]
pub fn HeaderNavLink(props: HeaderNavLinkProps) -> Element {
    let item = props.item.clone();
    let is_active = item.active;
    
    let style = use_style(move |t| {
        let base = Style::new()
            .inline_flex()
            .items_center()
            .gap(&t.spacing, "xs")
            .px(&t.spacing, "sm")
            .py(&t.spacing, "xs")
            .rounded(&t.radius, "md")
            .text(&t.typography, "sm")
            .font_weight(500)
            .transition("all 150ms ease")
            .no_underline();
            
        if is_active {
            base
                .bg(&t.colors.muted)
                .text_color(&t.colors.foreground)
        } else {
            base
                .text_color(&t.colors.muted_foreground)
        }.build()
    });
    
    let href = item.href.clone();
    let has_icon = item.icon.is_some();
    
    rsx! {
        a {
            style: "{style}",
            href: "{href}",
            
            if has_icon {
                Icon {
                    name: item.icon.unwrap(),
                    size: IconSize::Small,
                    color: IconColor::Current,
                }
            }
            
            "{item.label}"
        }
    }
}

/// Mobile menu toggle button
#[component]
pub fn MobileMenuToggle(
    #[props(default)]
    is_open: bool,
    #[props(default)]
    onclick: Option<EventHandler<()>>,
) -> Element {
    let icon_name = if is_open { "x".to_string() } else { "menu".to_string() };
    
    rsx! {
        Button {
            variant: ButtonVariant::Ghost,
            size: crate::atoms::ButtonSize::Icon,
            onclick: move |_| {
                if let Some(handler) = &onclick {
                    handler.call(());
                }
            },
            Icon {
                name: icon_name,
                size: IconSize::Medium,
                color: IconColor::Current,
            }
        }
    }
}

/// User menu component for header
#[derive(Props, Clone, PartialEq)]
pub struct UserMenuProps {
    /// User name
    pub name: String,
    /// User email
    #[props(default)]
    pub email: Option<String>,
    /// User avatar URL
    #[props(default)]
    pub avatar: Option<String>,
    /// Menu items
    #[props(default)]
    pub menu_items: Vec<UserMenuItem>,
}

/// User menu item
#[derive(Clone, PartialEq)]
pub struct UserMenuItem {
    pub label: String,
    pub icon: Option<String>,
    pub onclick: Option<EventHandler<()>>,
}

/// User Menu component
#[component]
pub fn UserMenu(props: UserMenuProps) -> Element {
    let mut is_open = use_signal(|| false);
    
    let avatar = if let Some(url) = props.avatar {
        rsx! {
            img {
                src: "{url}",
                style: "width: 32px; height: 32px; border-radius: 50%; object-fit: cover;",
                alt: "{props.name}",
            }
        }
    } else {
        // Default avatar with initials
        let initials: String = props.name
            .split_whitespace()
            .filter_map(|s| s.chars().next())
            .collect::<String>()
            .to_uppercase()
            .chars()
            .take(2)
            .collect();
        
        let style = use_style(|t| {
            Style::new()
                .w_px(32)
                .h_px(32)
                .rounded_full()
                .flex()
                .items_center()
                .justify_center()
                .bg(&t.colors.primary)
                .text_color(&t.colors.primary_foreground)
                .font_size(12)
                .font_weight(600)
                .build()
        });
        
        rsx! {
            div { style: "{style}", "{initials}" }
        }
    };
    
    rsx! {
        div {
            style: "position: relative;",
            
            Button {
                variant: ButtonVariant::Ghost,
                onclick: move |_| is_open.toggle(),
                
                div {
                    style: "display: flex; align-items: center; gap: 8px;",
                    {avatar}
                    Icon {
                        name: "chevron-down".to_string(),
                        size: IconSize::Small,
                        color: IconColor::Current,
                    }
                }
            }
            
            if is_open() {
                UserMenuDropdown {
                    items: props.menu_items.clone(),
                    on_close: move || is_open.set(false),
                }
            }
        }
    }
}

/// User menu dropdown
#[derive(Props, Clone, PartialEq)]
pub struct UserMenuDropdownProps {
    pub items: Vec<UserMenuItem>,
    pub on_close: EventHandler<()>,
}

#[component]
pub fn UserMenuDropdown(props: UserMenuDropdownProps) -> Element {
    let style = use_style(|t| {
        Style::new()
            .absolute()
            .right("0")
            .top("calc(100% + 8px)")
            .w_px(200)
            .rounded(&t.radius, "md")
            .border(1, &t.colors.border)
            .bg(&t.colors.popover)
            .shadow(&t.shadows.lg)
            .flex()
            .flex_col()
            .p(&t.spacing, "xs")
            .build()
    });
    
    rsx! {
        div {
            style: "{style}",
            
            for item in props.items {
                UserMenuItemView {
                    item: item,
                    on_close: props.on_close.clone(),
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct UserMenuItemViewProps {
    pub item: UserMenuItem,
    pub on_close: EventHandler<()>,
}

#[component]
pub fn UserMenuItemView(props: UserMenuItemViewProps) -> Element {
    let item = props.item.clone();
    
    let style = use_style(|t| {
        Style::new()
            .flex()
            .items_center()
            .gap(&t.spacing, "sm")
            .w_full()
            .p(&t.spacing, "sm")
            .rounded(&t.radius, "sm")
            .text(&t.typography, "sm")
            .cursor_pointer()
            .transition("all 150ms ease")
            .build()
    });
    
    let has_icon = item.icon.is_some();
    
    rsx! {
        button {
            style: "{style} background: transparent; border: none; text-align: left; color: inherit;",
            onclick: move |_| {
                if let Some(handler) = &item.onclick {
                    handler.call(());
                }
                props.on_close.call(());
            },
            
            if has_icon {
                Icon {
                    name: item.icon.unwrap(),
                    size: IconSize::Small,
                    color: IconColor::Current,
                }
            }
            
            "{item.label}"
        }
    }
}
