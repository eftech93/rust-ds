//! Tabs organism component
//!
//! A set of layered sections of content—known as tab panels—that are displayed one at a time.

use crate::styles::Style;
use crate::theme::{use_style, use_theme};
use dioxus::prelude::*;

/// Tab item definition
#[derive(Clone, PartialEq)]
pub struct TabItem {
    /// Tab ID
    pub id: String,
    /// Tab label
    pub label: String,
    /// Tab icon (optional)
    pub icon: Option<String>,
    /// Whether tab is disabled
    pub disabled: bool,
}

impl TabItem {
    /// Create a new tab item
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            disabled: false,
        }
    }

    /// Add an icon
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Tabs properties
#[derive(Props, Clone, PartialEq)]
pub struct TabsProps {
    /// Tab items
    pub tabs: Vec<TabItem>,
    /// Currently active tab ID
    pub active_tab: String,
    /// Callback when tab changes
    pub on_change: EventHandler<String>,
    /// Tab content (rendered based on active tab)
    pub children: Element,
    /// Tabs variant
    #[props(default)]
    pub variant: TabsVariant,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Tabs variant
#[derive(Default, Clone, PartialEq)]
pub enum TabsVariant {
    /// Default underline style
    #[default]
    Default,
    /// Enclosed/pill style
    Enclosed,
    /// Soft background style
    Soft,
}

/// Tabs organism component
#[component]
pub fn Tabs(props: TabsProps) -> Element {
    let _theme = use_theme();
    let variant = props.variant.clone();

    let tabs_container_style = use_style(move |t| match variant {
        TabsVariant::Default => Style::new()
            .w_full()
            .border_bottom(1, &t.colors.border)
            .flex()
            .gap(&t.spacing, "md")
            .build(),
        TabsVariant::Enclosed => Style::new()
            .w_full()
            .bg(&t.colors.muted)
            .rounded(&t.radius, "md")
            .p(&t.spacing, "xs")
            .flex()
            .gap(&t.spacing, "xs")
            .build(),
        TabsVariant::Soft => Style::new().w_full().flex().gap(&t.spacing, "sm").build(),
    });

    rsx! {
        div {
            style: "{tabs_container_style} {props.style.clone().unwrap_or_default()}",

            for tab in props.tabs.clone() {
                TabButton {
                    tab: tab.clone(),
                    is_active: props.active_tab == tab.id,
                    variant: props.variant.clone(),
                    on_click: props.on_change.clone(),
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct TabButtonProps {
    tab: TabItem,
    is_active: bool,
    variant: TabsVariant,
    on_click: EventHandler<String>,
}

#[component]
fn TabButton(props: TabButtonProps) -> Element {
    let _theme = use_theme();
    let mut is_hovered = use_signal(|| false);

    let is_active = props.is_active;
    let is_disabled = props.tab.disabled;
    let variant = props.variant.clone();

    let button_style = use_style(move |t| {
        let base = Style::new()
            .inline_flex()
            .items_center()
            .gap(&t.spacing, "xs")
            .px(&t.spacing, "md")
            .py(&t.spacing, "sm")
            .text(&t.typography, "sm")
            .font_weight(500)
            .cursor(if is_disabled {
                "not-allowed"
            } else {
                "pointer"
            })
            .transition("all 150ms ease")
            .border(0, &t.colors.border)
            .bg_transparent()
            .outline("none");

        match variant {
            TabsVariant::Default => {
                let styled = if is_active {
                    base.text_color(&t.colors.foreground)
                        .border_bottom(2, &t.colors.primary)
                        .mb_px(-1)
                } else {
                    base.text_color(&t.colors.muted_foreground)
                };

                if is_hovered() && !is_disabled && !is_active {
                    styled.text_color(&t.colors.foreground)
                } else {
                    styled
                }
                .build()
            }
            TabsVariant::Enclosed => {
                if is_active {
                    base.bg(&t.colors.background)
                        .text_color(&t.colors.foreground)
                        .shadow(&t.shadows.sm)
                        .rounded(&t.radius, "sm")
                        .build()
                } else if is_hovered() && !is_disabled {
                    base.text_color(&t.colors.foreground).build()
                } else {
                    base.text_color(&t.colors.muted_foreground).build()
                }
            }
            TabsVariant::Soft => {
                if is_active {
                    base.bg(&t.colors.secondary)
                        .text_color(&t.colors.secondary_foreground)
                        .rounded(&t.radius, "md")
                        .build()
                } else if is_hovered() && !is_disabled {
                    base.bg(&t.colors.muted)
                        .text_color(&t.colors.foreground)
                        .rounded(&t.radius, "md")
                        .build()
                } else {
                    base.text_color(&t.colors.muted_foreground).build()
                }
            }
        }
    });

    let handle_click = move |_| {
        if !is_disabled {
            props.on_click.call(props.tab.id.clone());
        }
    };

    let has_icon = props.tab.icon.is_some();

    rsx! {
        button {
            style: "{button_style}",
            disabled: is_disabled,
            onclick: handle_click,
            onmouseenter: move |_| if !is_disabled { is_hovered.set(true) },
            onmouseleave: move |_| is_hovered.set(false),

            if has_icon {
                TabIcon { name: props.tab.icon.clone().unwrap() }
            }

            "{props.tab.label}"
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct TabIconProps {
    name: String,
}

#[component]
fn TabIcon(props: TabIconProps) -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            style: "width: 16px; height: 16px;",

            match props.name.as_str() {
                "settings" => rsx! {
                    path { d: "M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" }
                    circle { cx: "12", cy: "12", r: "3" }
                },
                _ => rsx! {
                    circle { cx: "12", cy: "12", r: "10" }
                },
            }
        }
    }
}

/// Tab panel component for tab content
#[derive(Props, Clone, PartialEq)]
pub struct TabPanelProps {
    /// Panel content
    pub children: Element,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Tab panel component
#[component]
pub fn TabPanel(props: TabPanelProps) -> Element {
    let _theme = use_theme();

    let panel_style = use_style(|t| Style::new().w_full().pt(&t.spacing, "md").build());

    rsx! {
        div {
            role: "tabpanel",
            style: "{panel_style} {props.style.clone().unwrap_or_default()}",
            {props.children}
        }
    }
}

/// Vertical tabs variant
#[derive(Props, Clone, PartialEq)]
pub struct VerticalTabsProps {
    /// Tab items
    pub tabs: Vec<TabItem>,
    /// Currently active tab ID
    pub active_tab: String,
    /// Callback when tab changes
    pub on_change: EventHandler<String>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Vertical tabs component
#[component]
pub fn VerticalTabs(props: VerticalTabsProps) -> Element {
    let _theme = use_theme();

    let container_style = use_style(|t| Style::new().flex().gap(&t.spacing, "lg").build());

    let sidebar_style = use_style(|t| {
        Style::new()
            .w_px(200)
            .flex()
            .flex_col()
            .gap(&t.spacing, "xs")
            .build()
    });

    rsx! {
        div {
            style: "{container_style} {props.style.clone().unwrap_or_default()}",

            div {
                style: "{sidebar_style}",

                for tab in props.tabs.clone() {
                    VerticalTabButton {
                        tab: tab.clone(),
                        is_active: props.active_tab == tab.id,
                        on_click: props.on_change.clone(),
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct VerticalTabButtonProps {
    tab: TabItem,
    is_active: bool,
    on_click: EventHandler<String>,
}

#[component]
fn VerticalTabButton(props: VerticalTabButtonProps) -> Element {
    let _theme = use_theme();
    let mut is_hovered = use_signal(|| false);

    let is_active = props.is_active;
    let is_disabled = props.tab.disabled;

    let button_style = use_style(move |t| {
        let base = Style::new()
            .w_full()
            .flex()
            .items_center()
            .gap(&t.spacing, "sm")
            .px(&t.spacing, "md")
            .py(&t.spacing, "sm")
            .rounded(&t.radius, "md")
            .text(&t.typography, "sm")
            .font_weight(500)
            .cursor(if is_disabled {
                "not-allowed"
            } else {
                "pointer"
            })
            .transition("all 150ms ease")
            .border(0, &t.colors.border)
            .bg_transparent()
            .text_align_left();

        if is_active {
            base.bg(&t.colors.secondary)
                .text_color(&t.colors.secondary_foreground)
        } else if is_hovered() && !is_disabled {
            base.bg(&t.colors.muted).text_color(&t.colors.foreground)
        } else {
            base.text_color(&t.colors.muted_foreground)
        }
        .build()
    });

    let handle_click = move |_| {
        if !is_disabled {
            props.on_click.call(props.tab.id.clone());
        }
    };

    rsx! {
        button {
            style: "{button_style}",
            disabled: is_disabled,
            onclick: handle_click,
            onmouseenter: move |_| if !is_disabled { is_hovered.set(true) },
            onmouseleave: move |_| is_hovered.set(false),
            "{props.tab.label}"
        }
    }
}
