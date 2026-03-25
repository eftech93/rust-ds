//! Example display container

use dioxus::prelude::*;
use dioxus_ui_system::prelude::*;

/// Visual container for component examples
#[component]
pub fn ExampleBox(children: Element) -> Element {
    rsx! {
        Card { variant: CardVariant::Default, padding: Some("24px".to_string()), {children} }
    }
}
