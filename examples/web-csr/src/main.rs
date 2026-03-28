//! Web Client-Side Rendering Example with Documentation
//!
//! This example demonstrates the Dioxus UI System with a Storybook-like
//! documentation interface.
//!
//! ## Running
//!
//! ```bash
//! cd examples/web-csr
//! dx serve --platform web
//! ```

use dioxus::prelude::*;
use dioxus_ui_system::atoms::{AlignItems, Box, HStack, JustifyContent, SpacingSize, VStack};
use dioxus_ui_system::prelude::*;
use dioxus_ui_system::theme::ThemeProvider;
use example_shared::{ComponentShowcase, LayoutShowcase};

fn main() {
    dioxus::logger::init(tracing::Level::INFO).unwrap();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap"
        }

        ThemeProvider {
            Box {
                style: "font-family: 'Inter', system-ui, -apple-system, sans-serif;",
                AppWithNav {}
            }
        }
    }
}

/// Main app with navigation between showcase and docs
#[component]
fn AppWithNav() -> Element {
    let mut current_page = use_signal(|| "showcase".to_string());

    rsx! {
        div {
            // Header with navigation
            Header {
                brand_title: "Dioxus UI",
                nav_items: vec![
                    NavItem {
                        label: "Showcase".to_string(),
                        href: "#showcase".to_string(),
                        icon: Some("layout".to_string()),
                        active: current_page() == "showcase",
                    },
                    NavItem {
                        label: "Docs".to_string(),
                        href: "#docs".to_string(),
                        icon: Some("book".to_string()),
                        active: current_page() == "docs",
                    },
                ],
                actions: rsx! {
                    ThemeSelector {}
                    ThemeToggle {}
                },
            }

            // Main content
            VStack {
                HStack {
                    style: "background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%); color: white; padding: 12px 24px;",
                    align: AlignItems::Center,
                    justify: JustifyContent::Center,
                    gap: SpacingSize::Md,

                    NavTab {
                        label: "🎨 Component Showcase",
                        is_active: current_page() == "showcase",
                        onclick: move |_| current_page.set("showcase".to_string()),
                    }

                    NavTab {
                        label: "📚 Documentation",
                        is_active: current_page() == "docs",
                        onclick: move |_| current_page.set("docs".to_string()),
                    }
                }

                // Page content
                div {
                    style: "min-height: calc(100vh - 120px);",

                    if current_page() == "showcase" {
                        ShowcaseView {}
                    } else {
                        // Documentation view placeholder
                        div {
                            style: "padding: 48px; text-align: center;",
                            h2 { "Documentation" }
                            p { "See the docs-app crate for full documentation." }
                        }
                    }
                }
            }
        }
    }
}

/// Showcase view with component demos
#[component]
fn ShowcaseView() -> Element {
    let mut current_tab = use_signal(|| "components".to_string());

    rsx! {
        div {
            // Sub-navigation for showcase
            div {
                style: "background: #f8fafc; border-bottom: 1px solid #e2e8f0; padding: 12px 24px; display: flex; gap: 8px;",

                Button {
                    variant: if current_tab() == "components" { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                    size: ButtonSize::Sm,
                    onclick: move |_| current_tab.set("components".to_string()),
                    "Components"
                }

                Button {
                    variant: if current_tab() == "layouts" { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                    size: ButtonSize::Sm,
                    onclick: move |_| current_tab.set("layouts".to_string()),
                    "Layouts"
                }
            }

            // Content
            if current_tab() == "components" {
                ComponentShowcase {}
            } else {
                LayoutShowcase {}
            }
        }
    }
}

/// Navigation tab component
#[derive(Props, Clone, PartialEq)]
struct NavTabProps {
    label: String,
    is_active: bool,
    onclick: EventHandler<()>,
}

#[component]
fn NavTab(props: NavTabProps) -> Element {
    let bg_color_val = if props.is_active {
        "rgba(255,255,255,0.2)"
    } else {
        "transparent"
    };
    let border_val = if props.is_active {
        "1px solid rgba(255,255,255,0.4)"
    } else {
        "1px solid transparent"
    };

    rsx! {
        button {
            style: "padding: 8px 20px; border-radius: 8px; border: {border_val}; background: {bg_color_val}; color: white; cursor: pointer; font-weight: 500; font-size: 14px; transition: all 150ms;",
            onclick: move |_| props.onclick.call(()),
            "{props.label}"
        }
    }
}
