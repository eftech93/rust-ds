//! Web Client-Side Rendering Example
//!
//! This example demonstrates the Dioxus UI System running in a web browser
//! using WebAssembly and client-side rendering.
//!
//! ## Running
//!
//! ```bash
//! cd examples/web-csr
//! dx serve --platform web
//! ```
//!
//! Or with hot reload:
//!
//! ```bash
//! dx serve --platform web --hot-reload
//! ```

use dioxus::prelude::*;
use example_shared::{ComponentShowcase, LayoutShowcase};

fn main() {
    // Initialize logging
    dioxus::logger::init(tracing::Level::INFO).unwrap();
    
    // Launch the web app
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap"
        }
        
        div {
            style: "font-family: 'Inter', system-ui, -apple-system, sans-serif;",
            AppWithViewSwitcher {}
        }
    }
}

/// App with view switcher for Components and Layouts
#[component]
fn AppWithViewSwitcher() -> Element {
    let mut current_view = use_signal(|| "components".to_string());
    
    rsx! {
        div {
            // View switcher header
            div {
                style: "background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%); color: white; padding: 12px 24px; display: flex; align-items: center; justify-content: space-between;",
                
                div {
                    style: "display: flex; align-items: center; gap: 16px;",
                    
                    h1 {
                        style: "margin: 0; font-size: 20px; font-weight: 700;",
                        "Dioxus UI"
                    }
                    
                    // View buttons
                    div {
                        style: "display: flex; gap: 8px;",
                        
                        ViewButton {
                            label: "Components",
                            is_active: current_view() == "components",
                            onclick: move || current_view.set("components".to_string()),
                        }
                        
                        ViewButton {
                            label: "Layouts",
                            is_active: current_view() == "layouts",
                            onclick: move || current_view.set("layouts".to_string()),
                        }
                    }
                }
                
                span {
                    style: "font-size: 12px; opacity: 0.8;",
                    "Web (CSR)"
                }
            }
            
            // Main content
            if current_view() == "components" {
                ComponentShowcase {}
            } else {
                LayoutShowcase {}
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
    let bg_color = if props.is_active { "rgba(255,255,255,0.2)" } else { "transparent" };
    let border = if props.is_active { "1px solid rgba(255,255,255,0.4)" } else { "1px solid transparent" };
    
    rsx! {
        button {
            style: "padding: 6px 14px; border-radius: 6px; border: {border}; background: {bg_color}; color: white; cursor: pointer; font-weight: 500; font-size: 14px; transition: all 150ms;",
            onclick: move |_| props.onclick.call(()),
            "{props.label}"
        }
    }
}
