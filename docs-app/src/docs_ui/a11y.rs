//! Accessibility documentation helpers (roles list and keyboard table)

use dioxus::prelude::*;

/// Bullet list of the ARIA roles/attributes a component renders
#[component]
pub fn RoleList(items: Vec<&'static str>) -> Element {
    rsx! {
        ul {
            style: "margin: 0; padding-left: 20px; font-size: 14px; line-height: 1.7;",
            for item in items.iter() {
                li { "{item}" }
            }
        }
    }
}

/// Two-column table of keyboard interactions (Key / Action)
#[component]
pub fn KeyboardTable(rows: Vec<(&'static str, &'static str)>) -> Element {
    rsx! {
        table {
            style: "width: 100%; border-collapse: collapse; font-size: 14px;",

            thead {
                tr {
                    style: "background: rgb(248,250,252);",
                    th { style: "text-align: left; padding: 12px; border-bottom: 1px solid rgb(226,232,240); font-weight: 600; width: 30%;", "Key" }
                    th { style: "text-align: left; padding: 12px; border-bottom: 1px solid rgb(226,232,240); font-weight: 600;", "Action" }
                }
            }

            tbody {
                for (key, action) in rows.iter() {
                    tr {
                        td { style: "padding: 12px; border-bottom: 1px solid rgb(241,245,249); font-family: monospace; font-size: 13px;", "{key}" }
                        td { style: "padding: 12px; border-bottom: 1px solid rgb(241,245,249);", "{action}" }
                    }
                }
            }
        }
    }
}
