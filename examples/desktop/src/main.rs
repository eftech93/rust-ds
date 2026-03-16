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
use example_shared::{AppHeader, ComponentShowcaseInner};

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
    rsx! {
        ThemeProvider {
            div {
                style: "font-family: system-ui, -apple-system, sans-serif; min-height: 100vh; display: flex; flex-direction: column;",
                
                // Desktop app header with window controls
                AppHeader {}
                
                // Desktop banner
                DesktopBanner {}
                
                // Main content
                div {
                    style: "flex: 1; overflow-y: auto;",
                    ComponentShowcaseInner {}
                }
            }
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
