//! Layout organism component
//!
//! Provides flexible page layouts including sidebar, drawer, and top navigation variants.

use dioxus::prelude::*;
use crate::theme::use_style;
use crate::styles::Style;
use crate::atoms::{Button, ButtonVariant, Icon, IconSize, IconColor};

/// Layout type variants
#[derive(Clone, PartialEq, Default)]
pub enum LayoutType {
    /// Sidebar layout (default)
    #[default]
    Sidebar,
    /// Top navigation layout
    TopNav,
    /// Drawer layout (mobile-friendly)
    Drawer,
    /// Full-width layout (no navigation)
    FullWidth,
}

/// Main layout properties
#[derive(Props, Clone, PartialEq)]
pub struct LayoutProps {
    /// Layout type
    #[props(default)]
pub layout_type: LayoutType,
    /// Navigation items
    #[props(default)]
    pub nav_items: Vec<LayoutNavItem>,
    /// Brand element (logo/title)
    #[props(default)]
    pub brand: Option<Element>,
    /// Page title
    #[props(default)]
    pub title: Option<String>,
    /// Main content
    pub children: Element,
    /// Right-side actions
    #[props(default)]
    pub actions: Option<Element>,
    /// Whether sidebar is collapsible
    #[props(default = true)]
    pub collapsible: bool,
    /// Initial sidebar collapsed state
    #[props(default)]
    pub sidebar_collapsed: bool,
    /// Sidebar width (default: 260px)
    #[props(default = 260)]
    pub sidebar_width: u16,
    /// Header height (default: 64px)
    #[props(default = 64)]
    pub header_height: u16,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Navigation item for layouts
#[derive(Clone, PartialEq)]
pub struct LayoutNavItem {
    pub id: String,
    pub label: String,
    pub href: String,
    pub icon: Option<String>,
    pub active: bool,
    pub children: Vec<LayoutNavItem>,
}

impl LayoutNavItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            href: href.into(),
            icon: None,
            active: false,
            children: vec![],
        }
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn with_children(mut self, children: Vec<LayoutNavItem>) -> Self {
        self.children = children;
        self
    }
}

/// Main layout component
#[component]
pub fn Layout(props: LayoutProps) -> Element {
    let layout_type = props.layout_type.clone();
    
    match layout_type {
        LayoutType::Sidebar => rsx! {
            SidebarLayoutRenderer {
                nav_items: props.nav_items.clone(),
                brand: props.brand.clone(),
                title: props.title.clone(),
                children: props.children.clone(),
                actions: props.actions.clone(),
                collapsible: props.collapsible,
                sidebar_collapsed: props.sidebar_collapsed,
                sidebar_width: props.sidebar_width,
                header_height: props.header_height,
                class: props.class.clone(),
            }
        },
        LayoutType::TopNav => rsx! {
            TopNavLayoutRenderer {
                nav_items: props.nav_items.clone(),
                brand: props.brand.clone(),
                title: props.title.clone(),
                children: props.children.clone(),
                actions: props.actions.clone(),
                header_height: props.header_height,
                class: props.class.clone(),
            }
        },
        LayoutType::Drawer => rsx! {
            DrawerLayoutRenderer {
                nav_items: props.nav_items.clone(),
                brand: props.brand.clone(),
                title: props.title.clone(),
                children: props.children.clone(),
                actions: props.actions.clone(),
                sidebar_width: props.sidebar_width,
                header_height: props.header_height,
                class: props.class.clone(),
            }
        },
        LayoutType::FullWidth => rsx! {
            FullWidthLayoutRenderer {
                children: props.children.clone(),
                class: props.class.clone(),
            }
        },
    }
}

/// Sidebar layout with collapsible navigation
#[derive(Props, Clone, PartialEq)]
pub struct SidebarLayoutProps {
    nav_items: Vec<LayoutNavItem>,
    brand: Option<Element>,
    title: Option<String>,
    children: Element,
    actions: Option<Element>,
    collapsible: bool,
    sidebar_collapsed: bool,
    sidebar_width: u16,
    header_height: u16,
    class: Option<String>,
}

#[component]
fn SidebarLayoutRenderer(props: SidebarLayoutProps) -> Element {
    let mut is_collapsed = use_signal(|| props.sidebar_collapsed);
    let sidebar_width = if is_collapsed() { 80 } else { props.sidebar_width };

    let layout_style = use_style(|_t| {
        "display: flex; height: 100vh;".to_string()
    });

    let sidebar_style = format!(
        "width: {}px; height: 100vh; display: flex; flex-direction: column; border-right: 1px solid #e2e8f0; transition: width 200ms ease;",
        sidebar_width
    );

    let main_style = "display: flex; flex-direction: column; flex: 1; height: 100vh; overflow: auto;";

    let header_style = format!(
        "height: {}px; display: flex; align-items: center; justify-content: space-between; padding: 0 24px; border-bottom: 1px solid #e2e8f0;",
        props.header_height
    );

    let content_style = "flex: 1; padding: 24px; overflow: auto;";

    rsx! {
        div {
            style: "{layout_style} {props.class.clone().unwrap_or_default()}",
            
            // Sidebar
            aside {
                style: "{sidebar_style}",
                
                // Brand
                div {
                    style: "{header_style}",
                    
                    if let Some(brand) = props.brand.clone() {
                        div {
                            style: "flex: 1; overflow: hidden;",
                            {brand}
                        }
                    }
                    
                    if props.collapsible {
                        button {
                            style: "background: none; border: none; cursor: pointer; padding: 8px;",
                            onclick: move |_| is_collapsed.toggle(),
                            
                            if is_collapsed() {
                                "→"
                            } else {
                                "←"
                            }
                        }
                    }
                }
                
                // Navigation
                nav {
                    style: "flex: 1; overflow-y: auto; padding: 16px 12px;",
                    
                    for item in props.nav_items.clone() {
                        SidebarNavItem {
                            item: item,
                            collapsed: is_collapsed(),
                        }
                    }
                }
            }
            
            // Main content area
            main {
                style: "{main_style}",
                
                // Header
                header {
                    style: "{header_style}",
                    
                    if let Some(title) = props.title.clone() {
                        h1 {
                            style: "margin: 0; font-size: 20px; font-weight: 600;",
                            "{title}"
                        }
                    }
                    
                    if let Some(actions) = props.actions.clone() {
                        div {
                            style: "display: flex; align-items: center; gap: 8px;",
                            {actions}
                        }
                    }
                }
                
                // Content
                div {
                    style: "{content_style}",
                    {props.children}
                }
            }
        }
    }
}

/// Top navigation layout
#[derive(Props, Clone, PartialEq)]
pub struct TopNavLayoutProps {
    nav_items: Vec<LayoutNavItem>,
    brand: Option<Element>,
    title: Option<String>,
    children: Element,
    actions: Option<Element>,
    header_height: u16,
    class: Option<String>,
}

#[component]
fn TopNavLayoutRenderer(props: TopNavLayoutProps) -> Element {
    let header_style = format!(
        "height: {}px; display: flex; align-items: center; justify-content: space-between; padding: 0 24px; border-bottom: 1px solid #e2e8f0;",
        props.header_height
    );

    let content_style = format!("flex: 1; padding: 24px; overflow: auto; min-height: calc(100vh - {}px);", props.header_height);

    rsx! {
        div {
            style: "display: flex; flex-direction: column; min-height: 100vh; {props.class.clone().unwrap_or_default()}",
            
            // Header with navigation
            header {
                style: "{header_style}",
                
                div {
                    style: "display: flex; align-items: center; gap: 24px;",
                    
                    if let Some(brand) = props.brand.clone() {
                        {brand}
                    }
                    
                    nav {
                        style: "display: flex; align-items: center; gap: 24px;",
                        
                        for item in props.nav_items.clone() {
                            TopNavLink { item: item }
                        }
                    }
                }
                
                if let Some(actions) = props.actions.clone() {
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        {actions}
                    }
                }
            }
            
            // Main content
            main {
                style: "{content_style}",
                {props.children}
            }
        }
    }
}

/// Drawer layout (mobile-friendly slide-out navigation)
#[derive(Props, Clone, PartialEq)]
pub struct DrawerLayoutProps {
    nav_items: Vec<LayoutNavItem>,
    brand: Option<Element>,
    title: Option<String>,
    children: Element,
    actions: Option<Element>,
    sidebar_width: u16,
    header_height: u16,
    class: Option<String>,
}

#[component]
fn DrawerLayoutRenderer(props: DrawerLayoutProps) -> Element {
    let mut drawer_open = use_signal(|| false);

    let header_style = format!(
        "height: {}px; display: flex; align-items: center; justify-content: space-between; padding: 0 24px; border-bottom: 1px solid #e2e8f0;",
        props.header_height
    );

    let content_style = format!(
        "flex: 1; padding: 24px; overflow: auto; min-height: calc(100vh - {}px);",
        props.header_height
    );

    let drawer_overlay_style = "position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: rgba(0,0,0,0.5); z-index: 40;";

    let drawer_style = format!(
        "position: fixed; top: 0; left: 0; height: 100vh; width: {}px; background: white; border-right: 1px solid #e2e8f0; box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25); z-index: 50; display: flex; flex-direction: column;",
        props.sidebar_width
    );

    rsx! {
        div {
            style: "display: flex; flex-direction: column; min-height: 100vh; {props.class.clone().unwrap_or_default()}",
            
            // Header
            header {
                style: "{header_style}",
                
                div {
                    style: "display: flex; align-items: center; gap: 16px;",
                    
                    Button {
                        variant: ButtonVariant::Ghost,
                        size: crate::atoms::ButtonSize::Icon,
                        onclick: move |_| drawer_open.set(true),
                        Icon {
                            name: "menu".to_string(),
                            size: IconSize::Medium,
                            color: IconColor::Current,
                        }
                    }
                    
                    if let Some(brand) = props.brand.clone() {
                        {brand}
                    }
                    
                    if let Some(title) = props.title.clone() {
                        h1 {
                            style: "margin: 0; font-size: 20px; font-weight: 600;",
                            "{title}"
                        }
                    }
                }
                
                if let Some(actions) = props.actions.clone() {
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        {actions}
                    }
                }
            }
            
            // Main content
            main {
                style: "{content_style}",
                {props.children}
            }
            
            // Drawer overlay
            if drawer_open() {
                div {
                    style: "{drawer_overlay_style}",
                    onclick: move |_| drawer_open.set(false),
                }
                
                // Drawer
                aside {
                    style: "{drawer_style}",
                    onclick: move |e| e.stop_propagation(),
                    
                    // Drawer header
                    div {
                        style: "{header_style}",
                        
                        if let Some(brand) = props.brand.clone() {
                            div {
                                style: "flex: 1;",
                                {brand}
                            }
                        }
                        
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: crate::atoms::ButtonSize::Icon,
                            onclick: move |_| drawer_open.set(false),
                            "✕"
                        }
                    }
                    
                    // Drawer navigation
                    nav {
                        style: "flex: 1; overflow-y: auto; padding: 16px 12px;",
                        
                        for item in props.nav_items.clone() {
                            SidebarNavItem {
                                item: item,
                                collapsed: false,
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Full-width layout (no navigation)
#[derive(Props, Clone, PartialEq)]
pub struct FullWidthLayoutProps {
    children: Element,
    class: Option<String>,
}

#[component]
fn FullWidthLayoutRenderer(props: FullWidthLayoutProps) -> Element {
    let _theme = use_style(|t| {
        Style::new()
            .w_full()
            .min_h_full()
            .bg(&t.colors.background)
            .build()
    });

    rsx! {
        div {
            style: "width: 100%; min-height: 100vh; {props.class.clone().unwrap_or_default()}",
            {props.children}
        }
    }
}

/// Sidebar navigation item
#[derive(Props, Clone, PartialEq)]
struct SidebarNavItemProps {
    item: LayoutNavItem,
    collapsed: bool,
}

#[component]
fn SidebarNavItem(props: SidebarNavItemProps) -> Element {
    let mut is_hovered = use_signal(|| false);
    let item = props.item.clone();
    let has_icon = item.icon.is_some();

    let link_style = use_style(move |t| {
        let base = Style::new()
            .flex()
            .items_center()
            .gap(&t.spacing, "sm")
            .px(&t.spacing, "sm")
            .py(&t.spacing, "sm")
            .rounded(&t.radius, "md")
            .text(&t.typography, "sm")
            .font_weight(500)
            .transition("all 150ms ease")
            .no_underline();

        if item.active {
            base.bg(&t.colors.secondary)
                .text_color(&t.colors.secondary_foreground)
        } else if is_hovered() {
            base.bg(&t.colors.muted)
                .text_color(&t.colors.foreground)
        } else {
            base.text_color(&t.colors.muted_foreground)
        }.build()
    });

    rsx! {
        a {
            href: "{item.href}",
            style: "{link_style}",
            onmouseenter: move |_| is_hovered.set(true),
            onmouseleave: move |_| is_hovered.set(false),
            
            if has_icon {
                Icon {
                    name: item.icon.clone().unwrap(),
                    size: IconSize::Medium,
                    color: IconColor::Current,
                }
            }
            
            if !props.collapsed {
                span {
                    "{item.label}"
                }
            }
        }
    }
}

/// Top navigation link
#[derive(Props, Clone, PartialEq)]
struct TopNavLinkProps {
    item: LayoutNavItem,
}

#[component]
fn TopNavLink(props: TopNavLinkProps) -> Element {
    let mut is_hovered = use_signal(|| false);
    let item = props.item.clone();
    let has_icon = item.icon.is_some();

    let link_style = use_style(move |t| {
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

        if item.active {
            base.bg(&t.colors.muted)
                .text_color(&t.colors.foreground)
        } else if is_hovered() {
            base.bg(&t.colors.muted)
                .text_color(&t.colors.foreground)
        } else {
            base.text_color(&t.colors.muted_foreground)
        }.build()
    });

    rsx! {
        a {
            href: "{item.href}",
            style: "{link_style}",
            onmouseenter: move |_| is_hovered.set(true),
            onmouseleave: move |_| is_hovered.set(false),
            
            if has_icon {
                Icon {
                    name: item.icon.clone().unwrap(),
                    size: IconSize::Small,
                    color: IconColor::Current,
                }
            }
            
            "{item.label}"
        }
    }
}
