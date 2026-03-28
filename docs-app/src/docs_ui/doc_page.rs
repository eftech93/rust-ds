//! Documentation page layout wrapper

use dioxus::prelude::*;
use dioxus_ui_system::prelude::*;

/// Layout wrapper for documentation pages
#[component]
pub fn DocPage(title: String, description: String, children: Element) -> Element {
    rsx! {
        VStack {
            style: "gap: 32px;",

            Box {
                h1 { style: "margin: 0 0 12px 0; font-size: 32px; font-weight: 800;", "{title}" }
                p { style: "margin: 0; font-size: 16px; color: rgb(100,116,139); line-height: 1.6;", "{description}" }
            }

            {children}
        }
    }
}
