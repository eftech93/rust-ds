//! Desktop Example
//!
//! This example demonstrates the Dioxus UI System running as a native desktop app.
//! It uses the system's native WebView for rendering.
//!
//! ## Running
//!
//! ```bash
//! cd examples/desktop
//! cargo run
//! ```
//!
//! ## Building
//!
//! ```bash
//! # macOS
//! cargo bundle --release --target x86_64-apple-darwin
//!
//! # Windows
//! cargo build --release --target x86_64-pc-windows-msvc
//!
//! # Linux
//! cargo build --release --target x86_64-unknown-linux-gnu
//! ```

use dioxus::prelude::*;
use dioxus_ui_system::prelude::*;
use dioxus_ui_system::atoms::{Box, VStack, HStack, SpacingSize};
use example_shared::{AppHeader, ComponentShowcaseInner, LayoutShowcaseInner};

fn main() {
    // Initialize logging
    dioxus::logger::init(tracing::Level::INFO).unwrap();
    
    println!("Starting Dioxus UI Desktop Example");
    
    // Configure the window
    let window = dioxus::desktop::WindowBuilder::new()
        .with_title("Dioxus UI System - Desktop")
        .with_inner_size(dioxus::desktop::LogicalSize::new(1400.0, 900.0))
        .with_min_inner_size(dioxus::desktop::LogicalSize::new(800.0, 600.0));
    
    // Configure the app
    let config = dioxus::desktop::Config::new()
        .with_window(window)
        .with_menu(None);
    
    // Launch the app
    dioxus::LaunchBuilder::desktop().with_cfg(config).launch(App);
}

#[component]
fn App() -> Element {
    let mut current_view = use_signal(|| "components".to_string());
    
    rsx! {
        ThemeProvider {
            VStack {
                style: "font-family: system-ui, -apple-system, sans-serif; min-height: 100vh;",
                height: Some("100vh".to_string()),
                
                // Desktop app header with window controls
                AppHeader {}
                
                // Desktop banner
                DesktopBanner {}
                
                // View switcher
                HStack {
                    style: "background: #f1f5f9; border-bottom: 1px solid #e2e8f0; padding: 12px 24px;",
                    gap: SpacingSize::Sm,
                    
                    ViewButton {
                        label: "Component Showcase",
                        is_active: current_view() == "components",
                        onclick: move || current_view.set("components".to_string()),
                    }
                    
                    ViewButton {
                        label: "Layout Showcase",
                        is_active: current_view() == "layouts",
                        onclick: move || current_view.set("layouts".to_string()),
                    }
                }
                
                // Main content
                Box {
                    style: "flex: 1; overflow-y: auto;",
                    
                    if current_view() == "components" {
                        ComponentShowcaseInner {}
                    } else {
                        LayoutShowcaseInner {}
                    }
                }
            }
        }
    }
}

/// View switcher button
#[derive(Props, Clone, PartialEq)]
struct ViewButtonProps {
    label: String,
    is_active: bool,
    onclick: EventHandler<()>,
}

#[component]
fn ViewButton(props: ViewButtonProps) -> Element {
    let bg_color = if props.is_active { "#0f172a" } else { "transparent" };
    let text_color = if props.is_active { "white" } else { "#64748b" };
    
    rsx! {
        button {
            style: "padding: 8px 16px; border-radius: 6px; border: none; background: {bg_color}; color: {text_color}; cursor: pointer; font-weight: 500; transition: all 150ms;",
            onclick: move |_| props.onclick.call(()),
            "{props.label}"
        }
    }
}

/// Desktop-specific banner
#[component]
fn DesktopBanner() -> Element {
    rsx! {
        div {
            style: "background: linear-gradient(135deg, #11998e 0%, #38ef7d 100%); color: white; padding: 8px; text-align: center;",
            
            Label {
                size: TextSize::Small,
                weight: TextWeight::Medium,
                "🖥️ Native Desktop App (WebView)"
            }
        }
    }
}
