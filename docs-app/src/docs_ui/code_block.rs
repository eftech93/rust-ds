//! Syntax-highlighted code display

use dioxus::prelude::*;

/// Syntax-highlighted code display
/// 
/// Note: Currently uses simple styling. Could be enhanced with
/// proper syntax highlighting in the future.
#[component]
pub fn CodeBlock(code: String) -> Element {
    rsx! {
        pre {
            style: "background: rgb(15,23,42); color: rgb(226,232,240); padding: 16px; border-radius: 8px; font-size: 14px; overflow-x: auto;",
            code { "{code}" }
        }
    }
}
