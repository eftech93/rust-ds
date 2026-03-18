//! Web Server-Side Rendering Example
//!
//! This example demonstrates the Dioxus UI System with server-side rendering.
//! The HTML is rendered on the server and hydrated on the client.
//!
//! ## Running
//!
//! ```bash
//! cd examples/web-ssr
//! cargo run
//! ```
//!
//! The server will try ports 3000, 3001, 3002, 3003, 8080 in order.
//! Check the console output for the actual URL (e.g., http://localhost:3001)

use axum::{response::Html, routing::get, Router};
use dioxus::prelude::*;
use example_shared::{ComponentShowcaseInner, LayoutShowcaseInner};

#[tokio::main]
async fn main() {
    // Initialize logging
    dioxus::logger::init(tracing::Level::INFO).unwrap();
    
    // Try different ports starting from 3000
    let ports = [3000, 3001, 3002, 3003, 8080];
    let mut listener = None;
    
    for port in ports {
        match tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await {
            Ok(l) => {
                listener = Some(l);
                println!("✅ SSR server running on http://localhost:{}", port);
                break;
            }
            Err(_) => {
                println!("⚠️  Port {} is in use, trying next...", port);
            }
        }
    }
    
    let listener = listener.expect("Could not bind to any port");
    
    // Build the router
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/static/*path", get(static_handler));
    
    // Start the server
    axum::serve(listener, app).await.unwrap();
}

/// Handler for the root path - renders the app on the server
async fn root_handler() -> Html<String> {
    // SSR render the component
    let html = dioxus::ssr::render_element(rsx! {
        App {}
    });
    
    // Wrap in a full HTML document
    let full_html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Dioxus UI System - SSR Example</title>
    <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap">
    <style>
        * {{
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }}
        body {{
            font-family: 'Inter', system-ui, -apple-system, sans-serif;
            -webkit-font-smoothing: antialiased;
            -moz-osx-font-smoothing: grayscale;
        }}
    </style>
</head>
<body>
    <div id="main">
        {}
    </div>
    <script>
        // Hydration would happen here in a full SSR setup
        console.log('SSR rendered content loaded');
    </script>
</body>
</html>"#,
        html
    );
    
    Html(full_html)
}

/// Handler for static assets (simplified - in production use a proper static file handler)
async fn static_handler(axum::extract::Path(path): axum::extract::Path<String>) -> Html<String> {
    Html(format!("Static file: {}", path))
}

#[component]
fn App() -> Element {
    rsx! {
        ThemeProvider {
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
            style: "font-family: 'Inter', system-ui, -apple-system, sans-serif;",
            
            // SSR Banner with View Switcher
            div {
                style: "background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 12px 24px; display: flex; align-items: center; justify-content: space-between;",
                
                div {
                    style: "display: flex; align-items: center; gap: 16px;",
                    
                    span {
                        style: "font-weight: 600;",
                        "🔥 SSR"
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
                
                div {
                    style: "display: flex; align-items: center; gap: 12px;",
                    
                    ThemeSelector {}
                    
                    span {
                        style: "font-size: 12px; opacity: 0.8;",
                        "Dioxus UI System"
                    }
                }
            }
            
            // Main content
            if current_view() == "components" {
                ComponentShowcaseInner {}
            } else {
                LayoutShowcaseInner {}
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

// Import needed for SSR
use dioxus_ui_system::prelude::*;
