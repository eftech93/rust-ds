//! Guide pages

use dioxus::prelude::*;
use dioxus_ui_system::atoms::{Box, HStack, VStack};
use dioxus_ui_system::prelude::*;

#[component]
pub fn GuidesPage() -> Element {
    rsx! {
        VStack {
            style: "gap: 32px;",

            Box {
                h1 { style: "margin: 0 0 12px 0; font-size: 32px; font-weight: 800;", "Guides" }
                p { style: "margin: 0; font-size: 16px; color: rgb(100,116,139);",
                    "Learn how to build applications with Dioxus UI." }
            }

            Box { style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px;",
                GuideCard { title: "Quick Start", description: "Get up and running in minutes", route: "/guides/quickstart" }
                GuideCard { title: "Styling", description: "Learn different styling approaches", route: "/guides/styling" }
                GuideCard { title: "Forms", description: "Building forms with validation", route: "/guides/forms" }
                GuideCard { title: "Layouts", description: "Creating consistent page layouts", route: "/guides/layouts" }
                GuideCard { title: "Migrating to 0.0.9", description: "Prop and event handler changes in 0.0.9", route: "/guides/migration" }
            }
        }
    }
}

#[component]
fn GuideCard(title: String, description: String, route: String) -> Element {
    rsx! {
        Link {
            to: route,
            style: "padding: 20px; background: white; border: 1px solid rgb(226,232,240); border-radius: 12px; text-decoration: none; color: inherit; display: block;",
            h3 { style: "margin: 0 0 8px 0; font-size: 16px; font-weight: 600;", "{title}" }
            p { style: "margin: 0; font-size: 14px; color: rgb(100,116,139);", "{description}" }
        }
    }
}

#[component]
pub fn QuickStartPage() -> Element {
    rsx! {
        VStack {
            style: "gap: 32px;",

            h1 { style: "margin: 0; font-size: 32px; font-weight: 800;", "Quick Start" }

            Section { number: 1, title: "Add Dependencies",
                CodeBlock { code: "# Cargo.toml\n[dependencies]\ndioxus = \"0.7\"\ndioxus-ui-system = \"0.0.9\"".to_string() }
            }

            Section { number: 2, title: "Wrap with ThemeProvider",
                CodeBlock { code: "#[component]\nfn App() -> Element {{\n    rsx! {{\n        ThemeProvider {{\n            // Your app content\n            Router::<Route> {{}}\n        }}\n    }}\n}}".to_string() }
            }

            Section { number: 3, title: "Use Components",
                CodeBlock { code: "#[component]\nfn MyPage() -> Element {{\n    rsx! {{\n        Card {{\n            CardHeader {{\n                title: \"Hello\",\n                subtitle: Some(\"World\".to_string()),\n            }}\n            CardContent {{\n                Button {{ \"Click me\" }}\n            }}\n        }}\n    }}\n}}".to_string() }
            }
        }
    }
}

#[component]
pub fn StylingPage() -> Element {
    rsx! {
        VStack {
            style: "gap: 32px;",

            h1 { style: "margin: 0; font-size: 32px; font-weight: 800;", "Styling Guide" }

            Section { number: 1, title: "Inline Styles",
                p { "Apply styles directly to elements:" }
                CodeBlock { code: "div {{\n    style: \"padding: 16px; background: rgb(248,250,252);\",\n    \"Content\"\n}}".to_string() }
            }

            Section { number: 2, title: "Using Theme Tokens",
                p { "Access theme values for consistent styling:" }
                CodeBlock { code: "use dioxus_ui_system::theme::use_theme;\n\n#[component]\nfn StyledComponent() -> Element {{\n    let theme = use_theme();\n    let bg = theme.colors.background.to_rgba();\n    \n    rsx! {{\n        div {{ style: \"background: {bg};\", \"Content\" }}\n    }}\n}}".to_string() }
            }
        }
    }
}

#[component]
pub fn FormsPage() -> Element {
    let mut name = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut submitted = use_signal(|| false);

    rsx! {
        VStack {
            style: "gap: 32px;",

            h1 { style: "margin: 0; font-size: 32px; font-weight: 800;", "Building Forms" }

            Section { number: 1, title: "Form Example",
                p { "Complete form with validation:" }

                Card { variant: CardVariant::Default, padding: Some("24px".to_string()),
                    VStack { gap: SpacingSize::Md, style: "max-width: 400px;",
                        if submitted() {
                            Alert { variant: AlertVariant::Success, title: Some("Success".to_string()), "Form submitted!" }
                        }

                        InputGroup {
                            label: "Name",
                            value: name(),
                            required: true,
                            on_change: move |v| name.set(v),
                        }

                        InputGroup {
                            label: "Email",
                            value: email(),
                            input_type: InputType::Email,
                            required: true,
                            on_change: move |v| email.set(v),
                        }

                        Button {
                            variant: ButtonVariant::Primary,
                            disabled: name().is_empty() || email().is_empty(),
                            on_click: move |_| submitted.set(true),
                            "Submit"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn LayoutsPage() -> Element {
    rsx! {
        VStack {
            style: "gap: 32px;",

            h1 { style: "margin: 0; font-size: 32px; font-weight: 800;", "Page Layouts" }

            Section { number: 1, title: "Layout Component",
                p { "Use the Layout organism for consistent page structure:" }
                CodeBlock { code: "Layout {{\n    layout_type: LayoutType::Sidebar,\n    nav_items: vec![\n        LayoutNavItem::new(\"home\", \"Home\", \"/\"),\n    ],\n    brand: Some(rsx! {{ \"MyApp\" }}),\n    \n    // Your content\n    div {{ \"Page content\" }}\n}}".to_string() }
            }
        }
    }
}

/// Breaking changes between 0.0.8 and 0.0.9 (#17, #18).
const MIGRATION_RULES: &[(&str, &str)] = &[
    (
        "onclick / onfocus / onblur / oninput",
        "on_click / on_focus / on_blur / on_input",
    ),
    (
        "onmouseenter / onmouseleave (Box)",
        "on_mouse_enter / on_mouse_leave",
    ),
    (
        "Required EventHandler<T> props",
        "Option<EventHandler<T>>, default None (closures still work)",
    ),
    (
        "Click / hover / focus payload: MouseEvent, FocusEvent",
        "()",
    ),
    (
        "Input.oninput: FormEvent",
        "Input.on_input: String (the current value)",
    ),
    (
        "Radio.on_change: ()",
        "Radio.on_change: String (the radio's value)",
    ),
    (
        "error: bool (Select, TextArea, OtpInput)",
        "error: Option<String> (message shown below the field)",
    ),
    (
        "TextArea.error_message",
        "removed: pass the message as error",
    ),
    (
        "Required label / title / description / placeholder / style: String",
        "Option<String> (label: \"x\" still compiles)",
    ),
];

#[component]
pub fn MigrationPage() -> Element {
    rsx! {
        VStack {
            style: "gap: 32px;",

            Box {
                h1 { style: "margin: 0 0 12px 0; font-size: 32px; font-weight: 800;", "Migrating to 0.0.9" }
                p { style: "margin: 0; font-size: 16px; color: rgb(100,116,139);",
                    "0.0.9 makes component props consistent in one breaking release. Native rsx elements (button {{ onclick: ... }}) are unchanged; only dioxus-ui-system component props were renamed." }
            }

            Section { number: 1, title: "What changed",
                table { style: "width: 100%; border-collapse: collapse; font-size: 14px;",
                    thead {
                        tr {
                            th { style: "text-align: left; padding: 8px; border-bottom: 2px solid rgb(226,232,240);", "0.0.8" }
                            th { style: "text-align: left; padding: 8px; border-bottom: 2px solid rgb(226,232,240);", "0.0.9" }
                        }
                    }
                    tbody {
                        for (before, after) in MIGRATION_RULES.iter() {
                            tr {
                                td { style: "padding: 8px; border-bottom: 1px solid rgb(226,232,240); font-family: monospace;", "{before}" }
                                td { style: "padding: 8px; border-bottom: 1px solid rgb(226,232,240); font-family: monospace;", "{after}" }
                            }
                        }
                    }
                }
            }

            Section { number: 2, title: "Before (0.0.8)",
                CodeBlock { code: "Button {{\n    onclick: move |_evt: MouseEvent| save(),\n    \"Save\"\n}}\nInput {{\n    value: name(),\n    oninput: move |e: FormEvent| name.set(e.value()),\n}}\nTextArea {{\n    error: true,\n    error_message: Some(\"Required\".to_string()),\n}}\nTabs {{ tabs: items, value: tab(), on_change: move |v| tab.set(v) }}".to_string() }
            }

            Section { number: 3, title: "After (0.0.9)",
                CodeBlock { code: "Button {{\n    on_click: move |_| save(),\n    \"Save\"\n}}\nInput {{\n    value: name(),\n    on_input: move |v: String| name.set(v),\n}}\nTextArea {{\n    error: \"Required\",\n}}\n// on_change is now optional; passing a closure works as before\nTabs {{ tabs: items, value: tab(), on_change: move |v| tab.set(v) }}".to_string() }
            }

            Section { number: 4, title: "Full list",
                p { "Every renamed or retyped prop, per component, is listed in the CHANGELOG (section 0.0.9) in the repository." }
            }
        }
    }
}

#[component]
fn Section(number: u32, title: String, children: Element) -> Element {
    rsx! {
        HStack {
            style: "gap: 16px;",

            Box {
                style: "width: 32px; height: 32px; border-radius: 50%; background: rgb(15,23,42); color: white; display: flex; align-items: center; justify-content: center; font-weight: 700; font-size: 14px; flex-shrink: 0;",
                "{number}"
            }

            Box {
                style: "flex: 1;",
                h2 { style: "margin: 0 0 12px 0; font-size: 20px; font-weight: 600;", "{title}" }
                VStack { gap: SpacingSize::Sm, {children} }
            }
        }
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
