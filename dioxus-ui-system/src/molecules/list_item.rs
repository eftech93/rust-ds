//! List Item molecule component
//!
//! List row items with various configurations.

use dioxus::prelude::*;
use crate::theme::use_theme;

/// List item variant
#[derive(Default, Clone, PartialEq, Debug)]
pub enum ListItemVariant {
    #[default]
    Default,
    Selected,
    Active,
    Disabled,
}

/// List item properties
#[derive(Props, Clone, PartialEq)]
pub struct ListItemProps {
    /// Primary text
    pub title: String,
    /// Secondary text/description
    #[props(default)]
    pub description: Option<String>,
    /// Leading element (icon, avatar, checkbox)
    #[props(default)]
    pub leading: Option<Element>,
    /// Trailing element (icon, button, meta)
    #[props(default)]
    pub trailing: Option<Element>,
    /// Click handler
    #[props(default)]
    pub on_click: Option<EventHandler<()>>,
    /// Variant/state
    #[props(default = ListItemVariant::Default)]
    pub variant: ListItemVariant,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Dense mode (compact)
    #[props(default = false)]
    pub dense: bool,
    /// Divider below
    #[props(default = true)]
    pub divider: bool,
    /// Hover effect
    #[props(default = true)]
    pub hoverable: bool,
}

/// List item component
#[component]
pub fn ListItem(props: ListItemProps) -> Element {
    let theme = use_theme();
    
    let class_css = props.class.as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();
    
    let padding = if props.dense { "8px 12px" } else { "12px 16px" };
    
    let (bg_color, border_left) = match props.variant {
        ListItemVariant::Default => ("transparent".to_string(), "transparent".to_string()),
        ListItemVariant::Selected => {
            let bg = theme.tokens.read().colors.primary.to_rgba();
            let border = bg.clone();
            (format!("{}15", bg.trim_start_matches('#')), border)
        }
        ListItemVariant::Active => {
            (theme.tokens.read().colors.muted.to_rgba(), "transparent".to_string())
        }
        ListItemVariant::Disabled => ("transparent".to_string(), "transparent".to_string()),
    };
    
    let opacity = if props.variant == ListItemVariant::Disabled { "0.5" } else { "1" };
    let cursor = if props.on_click.is_some() && props.variant != ListItemVariant::Disabled { "pointer" } else { "default" };
    let _hover_bg = if props.hoverable && props.variant == ListItemVariant::Default {
        theme.tokens.read().colors.muted.to_rgba()
    } else {
        bg_color.clone()
    };
    
    let on_click = props.on_click.clone();
    
    let title_color = theme.tokens.read().colors.foreground.to_rgba();
    let desc_color = theme.tokens.read().colors.muted.to_rgba();
    
    rsx! {
        div {
            class: "list-item{class_css}",
            style: "display: flex; align-items: center; gap: 12px; padding: {padding}; background: {bg_color}; border-left: 3px solid {border_left}; opacity: {opacity}; cursor: {cursor}; transition: background 0.15s ease;",
            onclick: move |_| {
                if props.variant != ListItemVariant::Disabled {
                    if let Some(handler) = &on_click {
                        handler.call(());
                    }
                }
            },
            
            if let Some(leading) = props.leading {
                div {
                    class: "list-item-leading",
                    style: "flex-shrink: 0; display: flex; align-items: center;",
                    {leading}
                }
            }
            
            div {
                class: "list-item-content",
                style: "flex: 1; min-width: 0;",
                
                div {
                    class: "list-item-title",
                    style: "font-size: 14px; font-weight: 500; color: {title_color}; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                    "{props.title}"
                }
                
                if let Some(description) = props.description {
                    div {
                        class: "list-item-description",
                        style: "font-size: 13px; color: {desc_color}; margin-top: 2px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                        "{description}"
                    }
                }
            }
            
            if let Some(trailing) = props.trailing {
                div {
                    class: "list-item-trailing",
                    style: "flex-shrink: 0; display: flex; align-items: center;",
                    {trailing}
                }
            }
        }
        
        if props.divider {
            div {
                class: "list-item-divider",
                style: "height: 1px; background: {theme.tokens.read().colors.border.to_rgba()}; margin-left: 16px;",
            }
        }
    }
}

/// List group properties
#[derive(Props, Clone, PartialEq)]
pub struct ListGroupProps {
    /// Group title
    #[props(default)]
    pub title: Option<String>,
    /// Group items
    pub children: Element,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Sticky header
    #[props(default = false)]
    pub sticky: bool,
}

/// List group component (sectioned list)
#[component]
pub fn ListGroup(props: ListGroupProps) -> Element {
    let theme = use_theme();
    
    let class_css = props.class.as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();
    
    rsx! {
        div {
            class: "list-group{class_css}",
            style: "border-radius: 8px; overflow: hidden;",
            
            if let Some(title) = props.title {
                div {
                    class: "list-group-header",
                    style: if props.sticky { "position: sticky; top: 0; z-index: 10;" } else { "" },
                    
                    h3 {
                        style: "margin: 0; padding: 12px 16px; font-size: 12px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.5px; color: {theme.tokens.read().colors.muted.to_rgba()}; background: {theme.tokens.read().colors.background.to_rgba()};",
                        "{title}"
                    }
                }
            }
            
            div {
                class: "list-group-items",
                style: "background: white; border: 1px solid {theme.tokens.read().colors.border.to_rgba()}; border-radius: 8px;",
                {props.children}
            }
        }
    }
}

/// Action list item properties
#[derive(Props, Clone, PartialEq)]
pub struct ActionListItemProps {
    /// Action label
    pub label: String,
    /// Icon
    #[props(default)]
    pub icon: Option<String>,
    /// Description/help text
    #[props(default)]
    pub description: Option<String>,
    /// Keyboard shortcut
    #[props(default)]
    pub shortcut: Option<String>,
    /// Click handler
    pub on_click: EventHandler<()>,
    /// Disabled state
    #[props(default = false)]
    pub disabled: bool,
    /// Danger/destructive action
    #[props(default = false)]
    pub destructive: bool,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Action list item (menu item style)
#[component]
pub fn ActionListItem(props: ActionListItemProps) -> Element {
    let theme = use_theme();
    
    let class_css = props.class.as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();
    
    let text_color = if props.destructive {
        theme.tokens.read().colors.destructive.to_rgba()
    } else if props.disabled {
        theme.tokens.read().colors.muted.to_rgba()
    } else {
        theme.tokens.read().colors.foreground.to_rgba()
    };
    
    let _hover_bg = if props.destructive {
        format!("{}15", theme.tokens.read().colors.destructive.to_rgba().trim_start_matches('#'))
    } else {
        theme.tokens.read().colors.muted.to_rgba()
    };
    
    let cursor = if props.disabled { "not-allowed" } else { "pointer" };
    let opacity = if props.disabled { "0.5" } else { "1" };
    
    rsx! {
        button {
            type: "button",
            class: "action-list-item{class_css}",
            style: "display: flex; align-items: center; gap: 12px; width: 100%; padding: 10px 14px; background: transparent; border: none; text-align: left; cursor: {cursor}; opacity: {opacity}; border-radius: 6px; transition: background 0.15s ease;",
            disabled: props.disabled,
            onclick: move |_| props.on_click.call(()),
            
            if let Some(icon) = props.icon {
                span {
                    class: "action-list-item-icon",
                    style: "font-size: 16px; color: {text_color}; flex-shrink: 0;",
                    "{icon}"
                }
            }
            
            div {
                class: "action-list-item-content",
                style: "flex: 1; min-width: 0;",
                
                div {
                    class: "action-list-item-label",
                    style: "font-size: 14px; color: {text_color}; font-weight: 500;",
                    "{props.label}"
                }
                
                if let Some(description) = props.description {
                    div {
                        class: "action-list-item-description",
                        style: "font-size: 12px; color: {theme.tokens.read().colors.muted.to_rgba()}; margin-top: 2px;",
                        "{description}"
                    }
                }
            }
            
            if let Some(shortcut) = props.shortcut {
                kbd {
                    class: "action-list-item-shortcut",
                    style: "padding: 2px 6px; font-size: 11px; background: {theme.tokens.read().colors.muted.to_rgba()}; border-radius: 4px; color: {theme.tokens.read().colors.muted.to_rgba()}; flex-shrink: 0;",
                    "{shortcut}"
                }
            }
        }
    }
}

/// Expandable list item properties
#[derive(Props, Clone, PartialEq)]
pub struct ExpandableListItemProps {
    /// Header content
    pub header: Element,
    /// Expandable content
    pub children: Element,
    /// Initially expanded
    #[props(default = false)]
    pub default_expanded: bool,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Expandable list item component
#[component]
pub fn ExpandableListItem(props: ExpandableListItemProps) -> Element {
    let mut is_expanded = use_signal(|| props.default_expanded);
    
    let class_css = props.class.as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();
    
    rsx! {
        div {
            class: "expandable-list-item{class_css}",
            style: "border-bottom: 1px solid #e2e8f0;",
            
            button {
                type: "button",
                class: "expandable-list-item-header",
                style: "display: flex; align-items: center; justify-content: space-between; width: 100%; padding: 12px 16px; background: transparent; border: none; cursor: pointer;",
                onclick: move |_| is_expanded.toggle(),
                
                div {
                    style: "flex: 1; text-align: left;",
                    {props.header}
                }
                
                span {
                    style: format!("font-size: 12px; transition: transform 0.2s; transform: rotate({}deg);", if is_expanded() { 180 } else { 0 }),
                    "▼"
                }
            }
            
            if is_expanded() {
                div {
                    class: "expandable-list-item-content",
                    style: "padding: 0 16px 16px; animation: expand 0.2s ease;",
                    {props.children}
                }
            }
        }
        

    }
}
