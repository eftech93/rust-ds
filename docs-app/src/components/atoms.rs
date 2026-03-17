//! Atom component documentation pages

use dioxus::prelude::*;
use dioxus_ui_system::prelude::*;
use dioxus_ui_system::atoms::{StepIndicator, StepState};

/// Atoms overview page
#[component]
pub fn AtomsPage() -> Element {
    rsx! {
        DocPage {
            title: "Atoms",
            description: "Basic building blocks of the design system. Atoms are the smallest UI components that cannot be broken down further.",
            
            Section { title: "Overview",
                p { "Atoms are the fundamental building blocks of the design system. They include:" }
                ul {
                    li { "Button - Interactive action elements" }
                    li { "Input - Text input fields" }
                    li { "Label - Text labels and typography" }
                    li { "Icon - Visual iconography" }
                    li { "Checkbox - Binary selection" }
                    li { "Radio - Single selection from multiple options" }
                    li { "Switch - Toggle controls" }
                    li { "Select - Dropdown selection" }
                    li { "TextArea - Multi-line text input" }
                    li { "Step - Step indicator for steppers" }
                }
            }
            
            Section { title: "Usage",
                p { "Import atoms from the prelude:" }
                CodeBlock { code: "use dioxus_ui_system::prelude::*;".to_string() }
            }
        }
    }
}

/// Button documentation page
#[component]
pub fn ButtonPage() -> Element {
    rsx! {
        DocPage {
            title: "Button",
            description: "Interactive button component with multiple variants and sizes.",
            
            Section { title: "Variants",
                ExampleBox {
                    div { style: "display: flex; flex-wrap: wrap; gap: 12px;",
                        Button { variant: ButtonVariant::Primary, "Primary" }
                        Button { variant: ButtonVariant::Secondary, "Secondary" }
                        Button { variant: ButtonVariant::Ghost, "Ghost" }
                        Button { variant: ButtonVariant::Destructive, "Destructive" }
                        Button { variant: ButtonVariant::Link, "Link" }
                    }
                }
                CodeBlock { code: "Button {{ variant: ButtonVariant::Primary, \"Click me\" }}".to_string() }
            }
            
            Section { title: "Sizes",
                ExampleBox {
                    div { style: "display: flex; flex-wrap: wrap; gap: 12px; align-items: center;",
                        Button { size: ButtonSize::Sm, "Small" }
                        Button { size: ButtonSize::Md, "Medium" }
                        Button { size: ButtonSize::Lg, "Large" }
                    }
                }
                CodeBlock { code: "Button {{ size: ButtonSize::Md, \"Click me\" }}".to_string() }
            }
            
            Section { title: "Props",
                PropsTable { props: vec![
                    ("variant", "ButtonVariant", "Visual style variant"),
                    ("size", "ButtonSize", "Button size"),
                    ("disabled", "bool", "Disabled state"),
                    ("full_width", "bool", "Full width button"),
                    ("onclick", "EventHandler", "Click handler"),
                ]}
            }
        }
    }
}

/// Input documentation page
#[component]
pub fn InputPage() -> Element {
    rsx! {
        DocPage {
            title: "Input",
            description: "Text input field with support for various types and states.",
            
            Section { title: "Basic Usage",
                ExampleBox {
                    div { style: "max-width: 400px;",
                        InputGroup {
                            label: "Email",
                            value: "user@example.com".to_string(),
                            input_type: InputType::Email,
                            onchange: move |_| {},
                        }
                    }
                }
                CodeBlock { code: "InputGroup {{\n    label: \"Email\",\n    value: email(),\n    input_type: InputType::Email,\n    onchange: move |v| email.set(v),\n}}".to_string() }
            }
            
            Section { title: "Input Types",
                p { "Supported input types: Text, Email, Password, Number, Tel, Url, Search" }
            }
        }
    }
}

/// Label documentation page
#[component]
pub fn LabelPage() -> Element {
    rsx! {
        DocPage {
            title: "Typography",
            description: "Text components for headings, labels, and body text.",
            
            Section { title: "Headings",
                ExampleBox {
                    div { style: "display: flex; flex-direction: column; gap: 16px;",
                        Heading { level: HeadingLevel::H1, "Heading 1" }
                        Heading { level: HeadingLevel::H2, "Heading 2" }
                        Heading { level: HeadingLevel::H3, "Heading 3" }
                        Heading { level: HeadingLevel::H4, "Heading 4" }
                    }
                }
            }
            
            Section { title: "Text Sizes",
                ExampleBox {
                    div { style: "display: flex; flex-direction: column; gap: 12px;",
                        Label { size: TextSize::ExtraSmall, "Extra Small" }
                        Label { size: TextSize::Small, "Small" }
                        Label { size: TextSize::Base, "Base" }
                        Label { size: TextSize::Large, "Large" }
                        Label { size: TextSize::ExtraLarge, "Extra Large" }
                    }
                }
            }
        }
    }
}

/// Icon documentation page
#[component]
pub fn IconPage() -> Element {
    rsx! {
        DocPage {
            title: "Icon",
            description: "Built-in icon library with 30+ icons.",
            
            Section { title: "Available Icons",
                div { style: "display: flex; flex-wrap: wrap; gap: 16px;",
                    IconItem { name: "home".to_string() }
                    IconItem { name: "user".to_string() }
                    IconItem { name: "settings".to_string() }
                    IconItem { name: "search".to_string() }
                    IconItem { name: "bell".to_string() }
                    IconItem { name: "heart".to_string() }
                    IconItem { name: "star".to_string() }
                    IconItem { name: "check".to_string() }
                }
            }
            
            Section { title: "Usage",
                CodeBlock { code: "Icon {{\n    name: \"check\".to_string(),\n    size: IconSize::Medium,\n    color: IconColor::Success,\n}}".to_string() }
            }
        }
    }
}

#[component]
fn IconItem(name: String) -> Element {
    rsx! {
        div { style: "display: flex; flex-direction: column; align-items: center; gap: 4px; padding: 12px; background: rgb(248,250,252); border-radius: 8px; min-width: 80px;",
            Icon { name: name.clone(), size: IconSize::Large, color: IconColor::Primary }
            span { style: "font-size: 12px; color: rgb(100,116,139);", "{name}" }
        }
    }
}

/// Checkbox documentation page
#[component]
pub fn CheckboxPage() -> Element {
    let mut checked = use_signal(|| false);
    
    rsx! {
        DocPage {
            title: "Checkbox",
            description: "Binary selection control.",
            
            Section { title: "Basic",
                ExampleBox {
                    Checkbox {
                        checked: checked(),
                        label: Some("Accept terms and conditions".to_string()),
                        onchange: move |v| checked.set(v),
                    }
                }
                CodeBlock { code: "Checkbox {{\n    checked: checked(),\n    label: Some(\"Accept terms\".to_string()),\n    onchange: move |v| checked.set(v),\n}}".to_string() }
            }
        }
    }
}

/// Radio documentation page
#[component]
pub fn RadioPage() -> Element {
    let mut selected = use_signal(|| "option1".to_string());
    
    rsx! {
        DocPage {
            title: "Radio",
            description: "Single selection from multiple options.",
            
            Section { title: "Basic",
                ExampleBox {
                    div { style: "display: flex; flex-direction: column; gap: 8px;",
                        Radio {
                            name: "radio".to_string(),
                            value: "option1".to_string(),
                            checked: selected() == "option1",
                            label: Some("Option 1".to_string()),
                            onchange: move |_| selected.set("option1".to_string()),
                        }
                        Radio {
                            name: "radio".to_string(),
                            value: "option2".to_string(),
                            checked: selected() == "option2",
                            label: Some("Option 2".to_string()),
                            onchange: move |_| selected.set("option2".to_string()),
                        }
                    }
                }
            }
        }
    }
}

/// Switch documentation page
#[component]
pub fn SwitchPage() -> Element {
    let mut on = use_signal(|| true);
    
    rsx! {
        DocPage {
            title: "Switch",
            description: "Toggle control for on/off states.",
            
            Section { title: "Basic",
                ExampleBox {
                    Switch {
                        checked: on(),
                        label: Some(if on() { "Enabled".to_string() } else { "Disabled".to_string() }),
                        onchange: move |v| on.set(v),
                    }
                }
            }
        }
    }
}

/// Select documentation page
#[component]
pub fn SelectPage() -> Element {
    rsx! {
        DocPage {
            title: "Select",
            description: "Dropdown selection component.",
            
            Section { title: "Basic",
                ExampleBox {
                    div { style: "max-width: 300px;",
                        Select {
                            value: "option1".to_string(),
                            options: vec![
                                SelectOption::new("", "Select an option"),
                                SelectOption::new("option1", "Option 1"),
                                SelectOption::new("option2", "Option 2"),
                            ],
                            onchange: move |_| {},
                        }
                    }
                }
            }
        }
    }
}

/// TextArea documentation page
#[component]
pub fn TextAreaPage() -> Element {
    rsx! {
        DocPage {
            title: "TextArea",
            description: "Multi-line text input.",
            
            Section { title: "Basic",
                ExampleBox {
                    div { style: "max-width: 400px;",
                        Label { "Description" }
                        TextArea {
                            value: "Enter your description here...".to_string(),
                            rows: 4,
                            onchange: move |_| {},
                        }
                    }
                }
            }
        }
    }
}

/// Step documentation page
#[component]
pub fn StepPage() -> Element {
    rsx! {
        DocPage {
            title: "Step",
            description: "Step indicator for stepper components.",
            
            Section { title: "States",
                ExampleBox {
                    div { style: "display: flex; gap: 32px;",
                        div { style: "display: flex; flex-direction: column; align-items: center; gap: 8px;",
                            StepIndicator { step: 1, state: StepState::Completed }
                            span { style: "font-size: 12px;", "Completed" }
                        }
                        div { style: "display: flex; flex-direction: column; align-items: center; gap: 8px;",
                            StepIndicator { step: 2, state: StepState::Active }
                            span { style: "font-size: 12px;", "Active" }
                        }
                        div { style: "display: flex; flex-direction: column; align-items: center; gap: 8px;",
                            StepIndicator { step: 3, state: StepState::Pending }
                            span { style: "font-size: 12px;", "Pending" }
                        }
                    }
                }
            }
        }
    }
}

// Shared Components

#[component]
fn DocPage(title: String, description: String, children: Element) -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 32px;",
            
            div {
                h1 { style: "margin: 0 0 12px 0; font-size: 32px; font-weight: 800;", "{title}" }
                p { style: "margin: 0; font-size: 16px; color: rgb(100,116,139); line-height: 1.6;", "{description}" }
            }
            
            {children}
        }
    }
}

#[component]
fn Section(title: String, children: Element) -> Element {
    rsx! {
        section {
            h2 { style: "margin: 0 0 16px 0; font-size: 24px; font-weight: 700;", "{title}" }
            div { style: "display: flex; flex-direction: column; gap: 16px;", {children} }
        }
    }
}

#[component]
fn ExampleBox(children: Element) -> Element {
    rsx! {
        Card { variant: CardVariant::Default, padding: Some("24px".to_string()), {children} }
    }
}

#[component]
fn CodeBlock(code: String) -> Element {
    rsx! {
        pre {
            style: "background: rgb(15,23,42); color: rgb(226,232,240); padding: 16px; border-radius: 8px; font-size: 14px; overflow-x: auto;",
            code { "{code}" }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct PropsTableProps {
    props: Vec<(&'static str, &'static str, &'static str)>,
}

#[component]
fn PropsTable(props: PropsTableProps) -> Element {
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
