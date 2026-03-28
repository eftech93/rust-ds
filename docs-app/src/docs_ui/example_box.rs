//! Example display container

use dioxus::prelude::*;
use dioxus_ui_system::prelude::*;

/// Visual container for component examples
/// Uses overflow_visible to allow dropdowns and popovers to escape the container
#[component]
pub fn ExampleBox(children: Element) -> Element {
    rsx! {
        Card {
            variant: CardVariant::Default,
            padding: Some("24px".to_string()),
            overflow_hidden: false,
            {children}
        }
    }
}
