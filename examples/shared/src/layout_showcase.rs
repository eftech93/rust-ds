//! Layout showcase for the Dioxus UI System
//!
//! Demonstrates different layout types with layout switching capability.

use dioxus::prelude::*;
use dioxus_ui_system::atoms::{AlignItems, Box, HStack, SpacingSize, VStack};
use dioxus_ui_system::prelude::*;

/// Layout showcase with layout switcher
#[component]
pub fn LayoutShowcase() -> Element {
    rsx! {
        ThemeProvider {
            LayoutShowcaseInner {}
        }
    }
}

/// Inner layout showcase
#[component]
pub fn LayoutShowcaseInner() -> Element {
    let mut current_layout = use_signal(|| LayoutType::Sidebar);

    // Create navigation items
    let nav_items = vec![
        LayoutNavItem::new("home", "Home", "/")
            .with_icon("home")
            .active(true),
        LayoutNavItem::new("components", "Components", "/components").with_icon("settings"),
        LayoutNavItem::new("docs", "Documentation", "/docs").with_icon("book"),
        LayoutNavItem::new("about", "About", "/about").with_icon("info"),
    ];

    // Brand element
    let brand = rsx! {
        HStack {
            align: AlignItems::Center,
            gap: SpacingSize::Sm,
            Icon {
                name: "star".to_string(),
                size: IconSize::Large,
                color: IconColor::Primary,
            }
            span {
                style: "font-weight: 700; font-size: 18px;",
                "Dioxus UI"
            }
        }
    };

    // Actions (theme selector + layout switcher)
    let actions = rsx! {
        HStack {
            align: AlignItems::Center,
            gap: SpacingSize::Md,

            // Layout Switcher
            LayoutSwitcher {
                current_layout: current_layout(),
                on_change: move |layout| current_layout.set(layout),
            }

            // Theme Selector
            ThemeSelector {}
        }
    };

    // Main content for the current page
    let main_content = rsx! {
        Box {
            style: "max-width: 1200px;",

            Heading {
                level: HeadingLevel::H1,
                "Welcome to Dioxus UI System"
            }

            MutedText {
                "A comprehensive design system with multiple layout options"
            }

            div {
                style: "margin-top: 32px; display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 24px;",

                // Feature cards
                FeatureCard {
                    icon: "layout",
                    title: "4 Layout Types",
                    description: "Sidebar, TopNav, Drawer, and FullWidth layouts to suit any application."
                }

                FeatureCard {
                    icon: "palette",
                    title: "7 Theme Presets",
                    description: "Light, Dark, Rose, Blue, Green, Violet, and Orange themes."
                }

                FeatureCard {
                    icon: "box",
                    title: "100+ Components",
                    description: "Atoms, Molecules, and Organisms following Atomic Design principles."
                }

                FeatureCard {
                    icon: "smartphone",
                    title: "Cross-Platform",
                    description: "Works on Web, Desktop, and Mobile platforms."
                }
            }

            // Current layout indicator
            div {
                style: "margin-top: 48px; padding: 24px; background: #f8fafc; border-radius: 12px; border: 1px solid #e2e8f0;",

                Heading {
                    level: HeadingLevel::H3,
                    "Current Layout"
                }

                p {
                    style: "margin: 8px 0 0 0; color: #64748b;",
                    "You're currently using the "
                    strong {
                        "{layout_name(current_layout())}"
                    }
                    " layout. Use the dropdown in the header to switch between layouts."
                }
            }
        }
    };

    rsx! {
        Layout {
            layout_type: current_layout(),
            nav_items: nav_items,
            brand: Some(brand),
            title: Some("Dashboard".to_string()),
            children: main_content,
            actions: Some(actions),
            collapsible: true,
            sidebar_collapsed: false,
            sidebar_width: 260,
            header_height: 64,
        }
    }
}

/// Layout switcher dropdown component
#[derive(Props, Clone, PartialEq)]
pub struct LayoutSwitcherProps {
    pub current_layout: LayoutType,
    pub on_change: EventHandler<LayoutType>,
}

#[component]
fn LayoutSwitcher(props: LayoutSwitcherProps) -> Element {
    let mut is_open = use_signal(|| false);

    let layout_name = |layout: &LayoutType| match layout {
        LayoutType::Sidebar => "Sidebar",
        LayoutType::TopNav => "Top Navigation",
        LayoutType::Drawer => "Drawer",
        LayoutType::FullWidth => "Full Width",
    };

    let layouts = vec![
        LayoutType::Sidebar,
        LayoutType::TopNav,
        LayoutType::Drawer,
        LayoutType::FullWidth,
    ];

    let current_name = layout_name(&props.current_layout);

    rsx! {
        div {
            style: "position: relative; display: inline-block;",

            // Trigger button
            button {
                style: "display: flex; align-items: center; gap: 8px; padding: 8px 12px; border-radius: 6px; border: 1px solid #e2e8f0; background: white; cursor: pointer; font-size: 14px;",
                onclick: move |_| is_open.toggle(),

                Icon {
                    name: "layout".to_string(),
                    size: IconSize::Small,
                    color: IconColor::Current,
                }
                span { "{current_name}" }
                span { "▼" }
            }

            // Dropdown
            if is_open() {
                // Overlay to close on outside click
                div {
                    style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; z-index: 40;",
                    onclick: move |_| is_open.set(false),
                }

                div {
                    style: "position: absolute; top: calc(100% + 4px); right: 0; min-width: 180px; background: white; border-radius: 8px; border: 1px solid #e2e8f0; box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1); z-index: 50;",
                    onclick: move |e| e.stop_propagation(),

                    for layout in layouts {
                        button {
                            style: "display: flex; align-items: center; gap: 8px; width: 100%; padding: 10px 12px; text-align: left; background: none; border: none; cursor: pointer; border-radius: 6px; margin: 2px; font-size: 14px;",
                            style: if current_name == layout_name(&layout) { "background: #f1f5f9; font-weight: 500;" } else { "" },
                            onclick: move |_| {
                                props.on_change.call(layout.clone());
                                is_open.set(false);
                            },

                            // Layout icon
                            LayoutIcon { layout: layout.clone() }

                            span { "{layout_name(&layout)}" }
                        }
                    }
                }
            }
        }
    }
}

/// Layout icon component
#[derive(Props, Clone, PartialEq)]
struct LayoutIconProps {
    layout: LayoutType,
}

#[component]
fn LayoutIcon(props: LayoutIconProps) -> Element {
    let _icon_name = match props.layout {
        LayoutType::Sidebar => "sidebar",
        LayoutType::TopNav => "layout",
        LayoutType::Drawer => "menu",
        LayoutType::FullWidth => "maximize",
    };

    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            style: "width: 16px; height: 16px;",

            match props.layout {
                LayoutType::Sidebar => rsx! {
                    rect { x: "3", y: "3", width: "18", height: "18", rx: "2" }
                    line { x1: "9", y1: "3", x2: "9", y2: "21" }
                },
                LayoutType::TopNav => rsx! {
                    rect { x: "3", y: "3", width: "18", height: "18", rx: "2" }
                    line { x1: "3", y1: "9", x2: "21", y2: "9" }
                },
                LayoutType::Drawer => rsx! {
                    line { x1: "3", y1: "12", x2: "21", y2: "12" }
                    line { x1: "3", y1: "6", x2: "21", y2: "6" }
                    line { x1: "3", y1: "18", x2: "21", y2: "18" }
                },
                LayoutType::FullWidth => rsx! {
                    path { d: "M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3" }
                },
            }
        }
    }
}

/// Feature card component
#[derive(Props, Clone, PartialEq)]
pub struct FeatureCardProps {
    pub icon: String,
    pub title: String,
    pub description: String,
}

#[component]
fn FeatureCard(props: FeatureCardProps) -> Element {
    rsx! {
        Card {
            CardContent {
                VStack {
                    gap: SpacingSize::Md,

                    div {
                        style: "width: 40px; height: 40px; border-radius: 8px; background: #f1f5f9; display: flex; align-items: center; justify-content: center;",

                        Icon {
                            name: props.icon,
                            size: IconSize::Medium,
                            color: IconColor::Primary,
                        }
                    }

                    Heading {
                        level: HeadingLevel::H4,
                        "{props.title}"
                    }

                    p {
                        style: "margin: 0; color: #64748b; font-size: 14px; line-height: 1.5;",
                        "{props.description}"
                    }
                }
            }
        }
    }
}

/// Helper function to get layout name
fn layout_name(layout: LayoutType) -> &'static str {
    match layout {
        LayoutType::Sidebar => "Sidebar",
        LayoutType::TopNav => "Top Navigation",
        LayoutType::Drawer => "Drawer",
        LayoutType::FullWidth => "Full Width",
    }
}
