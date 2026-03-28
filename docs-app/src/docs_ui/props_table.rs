//! Props documentation table

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct PropsTableProps {
    pub props: Vec<(&'static str, &'static str, &'static str)>,
}

/// Table for displaying component props documentation
#[component]
pub fn PropsTable(props: PropsTableProps) -> Element {
    rsx! {
        table {
            style: "width: 100%; border-collapse: collapse; font-size: 14px;",

            thead {
                tr {
                    style: "background: rgb(248,250,252);",
                    th { style: "text-align: left; padding: 12px; border-bottom: 1px solid rgb(226,232,240); font-weight: 600;", "Prop" }
                    th { style: "text-align: left; padding: 12px; border-bottom: 1px solid rgb(226,232,240); font-weight: 600;", "Type" }
                    th { style: "text-align: left; padding: 12px; border-bottom: 1px solid rgb(226,232,240); font-weight: 600;", "Description" }
                }
            }

            tbody {
                for (name, typ, desc) in props.props.iter() {
                    tr {
                        td { style: "padding: 12px; border-bottom: 1px solid rgb(241,245,249); font-family: monospace; font-size: 13px;", "{name}" }
                        td { style: "padding: 12px; border-bottom: 1px solid rgb(241,245,249); font-family: monospace; font-size: 13px; color: rgb(100,116,139);", "{typ}" }
                        td { style: "padding: 12px; border-bottom: 1px solid rgb(241,245,249);", "{desc}" }
                    }
                }
            }
        }
    }
}
