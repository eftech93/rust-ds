//! Documentation section component

use dioxus::prelude::*;
use dioxus_ui_system::prelude::*;

/// Content section with title
#[component]
pub fn Section(title: String, children: Element) -> Element {
    rsx! {
        section {
            h2 { style: "margin: 0 0 16px 0; font-size: 24px; font-weight: 700;", "{title}" }
            VStack { gap: SpacingSize::Md, {children} }
        }
    }
}
