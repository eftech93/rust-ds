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
use example_shared::ComponentShowcase;

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
            ComponentShowcase {}
        }
    }
}
