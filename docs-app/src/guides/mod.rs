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
                GuideCard { title: "Accessibility", description: "ARIA roles, focus management and keyboard support", route: "/guides/accessibility" }
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
                CodeBlock { code: "# Cargo.toml\n[dependencies]\ndioxus = \"0.7\"\ndioxus-ui-system = \"0.0.10\"".to_string() }
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

/// Keyboard interactions per component (0.0.10).
const KEYBOARD_REFERENCE: &[(&str, &[(&str, &str)])] = &[
    (
        "Dialog / AlertDialog / ConfirmationDialog / Sheet",
        &[
            (
                "Tab / Shift+Tab",
                "Cycle focus inside the panel (focus is trapped)",
            ),
            (
                "Escape",
                "Close (on_close, or on_open_change(false) for Sheet)",
            ),
        ],
    ),
    (
        "Popover",
        &[("Escape", "Close and return focus to the trigger")],
    ),
    (
        "DropdownMenu",
        &[
            (
                "Enter / Space / ArrowDown",
                "On the trigger: open and focus the first enabled item",
            ),
            (
                "ArrowUp",
                "On the trigger: open and focus the last enabled item",
            ),
            (
                "ArrowUp / ArrowDown",
                "Move between items, skipping disabled ones",
            ),
            ("Home / End", "First / last enabled item"),
            ("Enter / Space", "Select the focused item"),
            ("Escape", "Close and return focus to the trigger"),
            ("Tab", "Close the menu"),
        ],
    ),
    (
        "Tabs / VerticalTabs",
        &[
            (
                "ArrowLeft / ArrowRight",
                "Previous / next tab, activating it (ArrowUp / ArrowDown for VerticalTabs)",
            ),
            ("Home / End", "First / last tab (disabled tabs are skipped)"),
        ],
    ),
    (
        "Accordion",
        &[
            (
                "ArrowUp / ArrowDown",
                "Previous / next enabled header (wraps)",
            ),
            ("Home / End", "First / last header"),
            ("Enter / Space", "Toggle the focused section"),
        ],
    ),
    (
        "Tree",
        &[
            ("ArrowUp / ArrowDown", "Previous / next visible node"),
            ("ArrowRight", "Expand a branch, or move to its first child"),
            ("ArrowLeft", "Collapse a branch, or move to its parent"),
            ("Home / End", "First / last visible node"),
            ("Enter / Space", "Select the focused node"),
        ],
    ),
    (
        "Carousel",
        &[
            (
                "ArrowLeft / ArrowRight",
                "Previous / next slide (ArrowUp / ArrowDown when vertical)",
            ),
            ("Home / End", "First / last slide"),
        ],
    ),
    (
        "MultiSelect (atoms::select)",
        &[
            ("Enter / Space / ArrowUp / ArrowDown", "Open the listbox"),
            (
                "ArrowUp / ArrowDown / Home / End",
                "Move the highlighted option",
            ),
            ("Enter / Space", "Toggle the highlighted option"),
            ("Escape", "Close"),
        ],
    ),
    (
        "MultiSelect (molecules::multi_select)",
        &[
            ("Enter / ArrowDown", "Open the listbox"),
            ("ArrowUp / ArrowDown", "Move the highlighted option"),
            (
                "Enter",
                "Toggle the highlighted option (or create one when creatable)",
            ),
            ("Escape", "Close"),
            (
                "Backspace",
                "With an empty search, remove the last selected tag",
            ),
        ],
    ),
    (
        "RadioGroup",
        &[(
            "Arrow keys",
            "Move to and select the previous / next radio (native radio inputs)",
        )],
    ),
];

/// What each component exposes to assistive technology.
const ROLE_REFERENCE: &[(&str, &str)] = &[
    ("Dialog", "role=\"dialog\" (or \"alertdialog\" with alert: true), aria-modal, aria-labelledby / aria-describedby"),
    ("AlertDialog, ConfirmationDialog family", "role=\"alertdialog\", title and message linked by aria-labelledby / aria-describedby"),
    ("Sheet", "role=\"dialog\", aria-modal, aria-labelledby / aria-describedby"),
    ("Popover", "Trigger: aria-haspopup=\"dialog\", aria-expanded, aria-controls; content: role=\"dialog\""),
    ("DropdownMenu", "Trigger: aria-haspopup=\"menu\", aria-expanded; popup: role=\"menu\"; items: role=\"menuitem\""),
    ("Tabs", "role=\"tablist\" / \"tab\" / tab panel with aria-selected, aria-controls, aria-labelledby"),
    ("Accordion", "Header buttons with aria-expanded, aria-controls; panels: role=\"region\""),
    ("Tree", "role=\"tree\" / \"treeitem\" / \"group\" with aria-level, aria-setsize, aria-posinset"),
    ("Carousel", "Labelled focusable region, slides labelled \"n of m\", aria-live content"),
    ("Progress", "role=\"progressbar\" with aria-valuenow / aria-valuetext, aria-busy when indeterminate"),
    ("RadioGroup", "role=\"radiogroup\" with aria-orientation, named by label or aria_label"),
    ("MultiSelect", "role=\"combobox\" + role=\"listbox\" (aria-multiselectable) + role=\"option\""),
    ("ThemeToggle / ThemeSelector", "Action-describing aria-label; selector popup is a listbox"),
];

#[component]
pub fn AccessibilityPage() -> Element {
    let cell = "padding: 8px; border-bottom: 1px solid rgb(226,232,240);";
    let head = "text-align: left; padding: 8px; border-bottom: 2px solid rgb(226,232,240);";

    rsx! {
        VStack {
            style: "gap: 32px;",

            Box {
                h1 { style: "margin: 0 0 12px 0; font-size: 32px; font-weight: 800;", "Accessibility" }
                p { style: "margin: 0; font-size: 16px; color: rgb(100,116,139);",
                    "Since 0.0.10, interactive components follow the WAI-ARIA authoring practices: they render the right roles and ARIA attributes, manage focus, and can be operated with the keyboard alone." }
            }

            Section { number: 1, title: "What the library provides",
                ul { style: "margin: 0; padding-left: 20px; line-height: 1.8;",
                    li { "Roles and ARIA attributes, with ids for aria-labelledby, aria-describedby and aria-controls derived from your id prop or generated automatically." }
                    li { "Focus trap and restore for Dialog, AlertDialog, ConfirmationDialog and Sheet: focus moves into the panel on open, Tab cycles inside it, and focus returns to the previously focused element on close." }
                    li { "Popover and DropdownMenu move focus into the popup and return it to the trigger on Escape." }
                    li { "Roving tabindex for Tabs and Tree: the widget is a single Tab stop and arrow keys move focus inside it." }
                    li { "Decorative elements (icons, SVGs, hidden HoverCard content) are aria-hidden." }
                }
                table { style: "width: 100%; border-collapse: collapse; font-size: 14px;",
                    thead {
                        tr {
                            th { style: "{head}", "Component" }
                            th { style: "{head}", "Semantics" }
                        }
                    }
                    tbody {
                        for (component, semantics) in ROLE_REFERENCE.iter() {
                            tr {
                                td { style: "{cell} font-weight: 600; white-space: nowrap;", "{component}" }
                                td { style: "{cell} font-family: monospace; font-size: 13px;", "{semantics}" }
                            }
                        }
                    }
                }
            }

            Section { number: 2, title: "Keyboard reference",
                for (component, keys) in KEYBOARD_REFERENCE.iter() {
                    Box {
                        h3 { style: "margin: 8px 0 8px 0; font-size: 16px; font-weight: 600;", "{component}" }
                        table { style: "width: 100%; border-collapse: collapse; font-size: 14px;",
                            thead {
                                tr {
                                    th { style: "{head} width: 35%;", "Key" }
                                    th { style: "{head}", "Action" }
                                }
                            }
                            tbody {
                                for (key, action) in keys.iter() {
                                    tr {
                                        td { style: "{cell} font-family: monospace;", "{key}" }
                                        td { style: "{cell}", "{action}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            Section { number: 3, title: "Tips",
                ul { style: "margin: 0; padding-left: 20px; line-height: 1.8;",
                    li { "Give every Dialog and Sheet a title. If the design has no visible title, pass aria_label instead. Use alert: true for dialogs that interrupt the user and need a response." }
                    li { "Label every RadioGroup with label (visible) or aria_label, and give it an id when you need a stable label id." }
                    li { "Pass aria_label to Tree, Carousel and MultiSelect when the surrounding text does not already name them (MultiSelect falls back to its placeholder)." }
                    li { "Use value_text on Progress when a percentage is not meaningful, e.g. \"Step 2 of 5\"." }
                    li { "Pass a stable id to Dialog, Tabs and Accordion when you need predictable ids for tests or deep links." }
                    li { "Keep Escape working: handle on_close / on_open_change instead of ignoring them." }
                }
                CodeBlock { code: "Dialog {{\n    open: open(),\n    on_close: move |_| open.set(false),\n    aria_label: \"Image preview\",\n    // no visible title\n}}\n\nRadioGroup {{\n    name: \"plan\",\n    value: plan(),\n    label: \"Billing plan\",\n    Radio {{\n        name: \"plan\",\n        value: \"monthly\",\n        label: \"Monthly\",\n        checked: plan() == \"monthly\",\n        on_change: move |v| plan.set(v),\n    }}\n    Radio {{\n        name: \"plan\",\n        value: \"yearly\",\n        label: \"Yearly\",\n        checked: plan() == \"yearly\",\n        on_change: move |v| plan.set(v),\n    }}\n}}\n\nProgress {{\n    value: Some(40.0),\n    aria_label: \"Upload\",\n    value_text: \"2 of 5 files\",\n}}".to_string() }
            }

            Section { number: 4, title: "Building your own widgets",
                p { "The helpers the components use are public in dioxus_ui_system::a11y:" }
                CodeBlock { code: "use dioxus_ui_system::a11y::{{\n    use_unique_id,      // stable ids for aria-labelledby / aria-controls\n    install_focus_trap, // trap Tab inside a container, focus its first element\n    release_focus_trap, // undo the trap and restore previous focus\n    next_index,         // arrow / Home / End index math for roving focus\n}};".to_string() }
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
