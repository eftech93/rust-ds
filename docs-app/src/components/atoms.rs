//! Atom component documentation pages

use dioxus::prelude::*;
use dioxus_ui_system::prelude::*;
use dioxus_ui_system::atoms::{StepIndicator, StepState, Box, VStack, HStack, 
    Heading, HeadingLevel, Divider, DividerOrientation, Skeleton, SkeletonShape,
    SkeletonText,
    Progress, ProgressVariant, ProgressSize, Spinner, SpinnerVariant, SpinnerSize, Rating, Slider, SliderMark,
    DatePicker, Tag, TagVariant, TagGroup, TagData
};


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
                    li { "Box - Foundational layout primitive" }
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

/// Box documentation page
#[component]
pub fn BoxPage() -> Element {
    rsx! {
        DocPage {
            title: "Box",
            description: "A foundational layout primitive that provides consistent spacing, borders, backgrounds, and flexbox utilities.",
            
            Section { title: "Basic Usage",
                ExampleBox {
                    VStack { gap: SpacingSize::Md,
                        Box { padding: SpacingSize::Md, background: BackgroundColor::Card, border_radius: RadiusSize::Md, border: BorderWidth::Thin,
                            "This is a Box with card background, medium padding, and rounded corners."
                        }
                        Box { padding: SpacingSize::Lg, background: BackgroundColor::Primary, border_radius: RadiusSize::Lg,
                            span { style: "color: white;", "Primary background with large padding" }
                        }
                    }
                }
                CodeBlock { code: "Box {{
    padding: SpacingSize::Md,
    background: BackgroundColor::Card,
    border_radius: RadiusSize::Md,
    border: BorderWidth::Thin,
    \"Your content here\"
}}".to_string() }
            }
            
            Section { title: "Flexbox Layout",
                ExampleBox {
                    Box { display: BoxDisplay::Flex, gap: SpacingSize::Md, padding: SpacingSize::Md, background: BackgroundColor::Muted, border_radius: RadiusSize::Md,
                        Box { padding: SpacingSize::Sm, background: BackgroundColor::Primary, "Flex Item 1" }
                        Box { padding: SpacingSize::Sm, background: BackgroundColor::Secondary, "Flex Item 2" }
                        Box { padding: SpacingSize::Sm, background: BackgroundColor::Accent, "Flex Item 3" }
                    }
                }
                CodeBlock { code: "Box {{
    display: BoxDisplay::Flex,
    gap: SpacingSize::Md,
    padding: SpacingSize::Md,
    // ... children
}}".to_string() }
            }
            
            Section { title: "Convenience Components",
                p { "Use VStack, HStack, and Center for common layouts:" }
                ExampleBox {
                    VStack { gap: SpacingSize::Md,
                        VStack { gap: SpacingSize::Sm, padding: SpacingSize::Md, background: BackgroundColor::Card,
                            span { "VStack Item 1" }
                            span { "VStack Item 2" }
                            span { "VStack Item 3" }
                        }
                        HStack { gap: SpacingSize::Sm, padding: SpacingSize::Md, background: BackgroundColor::Card,
                            span { "HStack 1" }
                            span { "HStack 2" }
                            span { "HStack 3" }
                        }
                        Center { padding: SpacingSize::Xl, background: BackgroundColor::Muted,
                            "Centered Content"
                        }
                    }
                }
            }
            
            Section { title: "Spacing Options",
                ExampleBox {
                    VStack { gap: SpacingSize::Sm,
                        Box { padding: SpacingSize::None, background: BackgroundColor::Card, border: BorderWidth::Thin, "No padding" }
                        Box { padding: SpacingSize::Xs, background: BackgroundColor::Card, border: BorderWidth::Thin, "Extra small padding" }
                        Box { padding: SpacingSize::Sm, background: BackgroundColor::Card, border: BorderWidth::Thin, "Small padding" }
                        Box { padding: SpacingSize::Md, background: BackgroundColor::Card, border: BorderWidth::Thin, "Medium padding" }
                        Box { padding: SpacingSize::Lg, background: BackgroundColor::Card, border: BorderWidth::Thin, "Large padding" }
                        Box { padding: SpacingSize::Xl, background: BackgroundColor::Card, border: BorderWidth::Thin, "Extra large padding" }
                    }
                }
            }
            
            Section { title: "Background Colors",
                ExampleBox {
                    div { style: "display: flex; flex-wrap: wrap; gap: 8px;",
                        Box { padding: SpacingSize::Md, background: BackgroundColor::Primary, span { style: "color: white;", "Primary" } }
                        Box { padding: SpacingSize::Md, background: BackgroundColor::Secondary, "Secondary" }
                        Box { padding: SpacingSize::Md, background: BackgroundColor::Accent, "Accent" }
                        Box { padding: SpacingSize::Md, background: BackgroundColor::Muted, "Muted" }
                        Box { padding: SpacingSize::Md, background: BackgroundColor::Card, border: BorderWidth::Thin, "Card" }
                        Box { padding: SpacingSize::Md, background: BackgroundColor::Destructive, span { style: "color: white;", "Destructive" } }
                        Box { padding: SpacingSize::Md, background: BackgroundColor::Success, span { style: "color: white;", "Success" } }
                        Box { padding: SpacingSize::Md, background: BackgroundColor::Warning, "Warning" }
                    }
                }
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
                    HStack { gap: SpacingSize::Md, style: "flex-wrap: wrap;",
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
                    HStack { gap: SpacingSize::Md, style: "flex-wrap: wrap; align-items: center;",
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
                    Box { style: "max-width: 400px;",
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
                    VStack { gap: SpacingSize::Md,
                        Heading { level: HeadingLevel::H1, "Heading 1" }
                        Heading { level: HeadingLevel::H2, "Heading 2" }
                        Heading { level: HeadingLevel::H3, "Heading 3" }
                        Heading { level: HeadingLevel::H4, "Heading 4" }
                    }
                }
            }
            
            Section { title: "Text Sizes",
                ExampleBox {
                    VStack { gap: SpacingSize::Sm,
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
                HStack { gap: SpacingSize::Md, style: "flex-wrap: wrap;",
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
        VStack { gap: SpacingSize::Xs, style: "align-items: center; padding: 12px; background: rgb(248,250,252); border-radius: 8px; min-width: 80px;",
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
                    VStack { gap: SpacingSize::Sm,
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
                    Box { style: "max-width: 300px;",
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
                    Box { style: "max-width: 400px;",
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
                    HStack { gap: SpacingSize::Xl,
                        VStack { gap: SpacingSize::Sm, style: "align-items: center;",
                            StepIndicator { step: 1, state: StepState::Completed }
                            span { style: "font-size: 12px;", "Completed" }
                        }
                        VStack { gap: SpacingSize::Sm, style: "align-items: center;",
                            StepIndicator { step: 2, state: StepState::Active }
                            span { style: "font-size: 12px;", "Active" }
                        }
                        VStack { gap: SpacingSize::Sm, style: "align-items: center;",
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

#[component]
fn Section(title: String, children: Element) -> Element {
    rsx! {
        section {
            h2 { style: "margin: 0 0 16px 0; font-size: 24px; font-weight: 700;", "{title}" }
            VStack { gap: SpacingSize::Md, {children} }
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

/// Heading documentation page
#[component]
pub fn HeadingPage() -> Element {
    rsx! {
        DocPage {
            title: "Heading",
            description: "Typography headings for content hierarchy (H1-H6).",
            
            Section { title: "Heading Levels",
                ExampleBox {
                    VStack { gap: SpacingSize::Md,
                        Heading { level: HeadingLevel::H1, "Heading 1" }
                        Heading { level: HeadingLevel::H2, "Heading 2" }
                        Heading { level: HeadingLevel::H3, "Heading 3" }
                        Heading { level: HeadingLevel::H4, "Heading 4" }
                        Heading { level: HeadingLevel::H5, "Heading 5" }
                        Heading { level: HeadingLevel::H6, "Heading 6" }
                    }
                }
            }
            
            Section { title: "Usage",
                CodeBlock { code: "Heading {{\n    level: HeadingLevel::H1,\n    \"Page Title\"\n}}".to_string() }
            }
        }
    }
}

/// Divider documentation page
#[component]
pub fn DividerPage() -> Element {
    rsx! {
        DocPage {
            title: "Divider",
            description: "Visual separators for content organization.",
            
            Section { title: "Horizontal Divider",
                ExampleBox {
                    VStack { gap: SpacingSize::Md,
                        span { "Content above" }
                        Divider {}
                        span { "Content below" }
                    }
                }
            }
            
            Section { title: "With Label",
                ExampleBox {
                    Divider { label: Some("OR".to_string()), orientation: DividerOrientation::Horizontal, variant: DividerVariant::Solid }
                }
            }
            
            Section { title: "Vertical",
                ExampleBox {
                    HStack { gap: SpacingSize::Md, style: "height: 40px;",
                        span { "Left" }
                        Divider { orientation: DividerOrientation::Vertical, variant: DividerVariant::Solid }
                        span { "Right" }
                    }
                }
            }
        }
    }
}

/// Progress documentation page
#[component]
pub fn ProgressPage() -> Element {
    let mut value = use_signal(|| 65.0);
    
    rsx! {
        DocPage {
            title: "Progress",
            description: "Linear and circular progress indicators.",
            
            Section { title: "Linear Progress",
                ExampleBox {
                    VStack { gap: SpacingSize::Md,
                        Progress { value: Some(value()), max: 100.0, show_label: true, variant: ProgressVariant::Linear, size: ProgressSize::Md }
                        Button { onclick: move |_| value.set((value() + 10.0) % 100.0), "Increase" }
                    }
                }
            }
            
            Section { title: "Circular Progress",
                ExampleBox {
                    HStack { gap: SpacingSize::Lg,
                        Progress { value: Some(75.0), variant: ProgressVariant::Circular, show_label: true, size: ProgressSize::Md }
                        Progress { value: Some(30.0), variant: ProgressVariant::Circular, size: ProgressSize::Lg, show_label: true }
                    }
                }
            }
            
            Section { title: "Indeterminate",
                ExampleBox {
                    Progress { value: None, indeterminate: true, variant: ProgressVariant::Linear, size: ProgressSize::Md }
                }
            }
        }
    }
}

/// Spinner documentation page
#[component]
pub fn SpinnerPage() -> Element {
    rsx! {
        DocPage {
            title: "Spinner",
            description: "Loading spinners and indicators.",
            
            Section { title: "Variants",
                ExampleBox {
                    HStack { gap: SpacingSize::Lg, style: "align-items: center;",
                        Spinner { variant: SpinnerVariant::Circular, size: SpinnerSize::Lg }
                        Spinner { variant: SpinnerVariant::Dots, size: SpinnerSize::Lg }
                        Spinner { variant: SpinnerVariant::Pulse, size: SpinnerSize::Lg }
                        Spinner { variant: SpinnerVariant::Bars, size: SpinnerSize::Lg }
                    }
                }
            }
            
            Section { title: "Sizes",
                ExampleBox {
                    HStack { gap: SpacingSize::Md, style: "align-items: center;",
                        Spinner { variant: SpinnerVariant::Circular, size: SpinnerSize::Xs }
                        Spinner { variant: SpinnerVariant::Circular, size: SpinnerSize::Sm }
                        Spinner { variant: SpinnerVariant::Circular, size: SpinnerSize::Md }
                        Spinner { variant: SpinnerVariant::Circular, size: SpinnerSize::Lg }
                        Spinner { variant: SpinnerVariant::Circular, size: SpinnerSize::Xl }
                    }
                }
            }
        }
    }
}

/// Skeleton Atom documentation page
#[component]
pub fn SkeletonAtomPage() -> Element {
    rsx! {
        DocPage {
            title: "Skeleton",
            description: "Loading placeholders that mimic content structure.",
            
            Section { title: "Basic Skeleton",
                ExampleBox {
                    VStack { gap: SpacingSize::Md,
                        Skeleton { shape: SkeletonShape::Text, width: Some("200px".to_string()), height: None, animated: true }
                        Skeleton { shape: SkeletonShape::Text, width: Some("150px".to_string()), height: None, animated: true }
                        Skeleton { shape: SkeletonShape::Text, width: Some("100px".to_string()), height: None, animated: true }
                    }
                }
            }
            
            Section { title: "Shapes",
                ExampleBox {
                    HStack { gap: SpacingSize::Md,
                        Skeleton { shape: SkeletonShape::Rectangle, width: Some("100px".to_string()), height: Some("60px".to_string()), animated: true }
                        Skeleton { shape: SkeletonShape::Circle, width: Some("60px".to_string()), height: Some("60px".to_string()), animated: true }
                        Skeleton { shape: SkeletonShape::Rounded, width: Some("100px".to_string()), height: Some("60px".to_string()), animated: true }
                    }
                }
            }
            
            Section { title: "Text Lines",
                ExampleBox {
                    SkeletonText { lines: 4, line_height: 1.5, last_line_width: 80, animated: true }
                }
            }
        }
    }
}

/// Rating documentation page
#[component]
pub fn RatingPage() -> Element {
    let mut rating = use_signal(|| 3.5);
    
    rsx! {
        DocPage {
            title: "Rating",
            description: "Star rating display and input.",
            
            Section { title: "Display Rating",
                ExampleBox {
                    VStack { gap: SpacingSize::Md,
                        Rating { value: 5.0, show_value: true, max: 5, size: 20, interactive: false }
                        Rating { value: 3.5, show_value: true, max: 5, size: 20, interactive: false }
                        Rating { value: 2.0, show_value: true, max: 5, size: 20, interactive: false }
                    }
                }
            }
            
            Section { title: "Interactive",
                ExampleBox {
                    VStack { gap: SpacingSize::Md,
                        Rating { value: rating(), interactive: true, on_change: Some(EventHandler::new(move |v: f32| rating.set(v))), max: 5, size: 24 }
                        span { "Current: {rating}" }
                    }
                }
            }
        }
    }
}

/// DatePicker documentation page
#[component]
pub fn DatePickerPage() -> Element {
    let mut date = use_signal(|| None::<String>);
    
    rsx! {
        DocPage {
            title: "DatePicker",
            description: "Date and date range selection input.",
            
            Section { title: "Basic Date Picker",
                ExampleBox {
                    DatePicker {
                        value: date(),
                        on_change: Some(EventHandler::new(move |d: String| date.set(Some(d)))),
                        label: Some("Select Date".to_string())
                    }
                }
            }
            
            Section { title: "With Constraints",
                ExampleBox {
                    DatePicker {
                        label: Some("Select Date".to_string()),
                        min: Some("2024-01-01".to_string()),
                        max: Some("2024-12-31".to_string())
                    }
                }
            }
        }
    }
}

/// Slider documentation page
#[component]
pub fn SliderPage() -> Element {
    let mut value = use_signal(|| 50.0);
    
    rsx! {
        DocPage {
            title: "Slider",
            description: "Range slider input for selecting numeric values.",
            
            Section { title: "Basic Slider",
                ExampleBox {
                    Slider {
                        value: value(),
                        on_change: EventHandler::new(move |v: f64| value.set(v)),
                        label: Some("Volume".to_string())
                    }
                }
            }
            
            Section { title: "With Marks",
                ExampleBox {
                    Slider {
                        value: 50.0,
                        on_change: EventHandler::new(move |_v: f64| {}),
                        marks: vec![
                            SliderMark { value: 0.0, label: Some("0%".to_string()) },
                            SliderMark { value: 50.0, label: Some("50%".to_string()) },
                            SliderMark { value: 100.0, label: Some("100%".to_string()) },
                        ]
                    }
                }
            }
        }
    }
}

/// Tag documentation page
#[component]
pub fn TagPage() -> Element {
    let mut selected_tags = use_signal(|| vec!["rust".to_string()]);
    
    rsx! {
        DocPage {
            title: "Tag",
            description: "Categorization, filtering, and selection tags.",
            
            Section { title: "Tag Variants",
                ExampleBox {
                    HStack { gap: SpacingSize::Sm, style: "flex-wrap: wrap;",
                        Tag { variant: TagVariant::Default, children: rsx! { "Default" } }
                        Tag { variant: TagVariant::Primary, children: rsx! { "Primary" } }
                        Tag { variant: TagVariant::Success, children: rsx! { "Success" } }
                        Tag { variant: TagVariant::Warning, children: rsx! { "Warning" } }
                        Tag { variant: TagVariant::Error, children: rsx! { "Error" } }
                    }
                }
            }
            
            Section { title: "Selectable Tags",
                ExampleBox {
                    TagGroup {
                        tags: vec![
                            TagData::new("rust", "Rust"),
                            TagData::new("go", "Go"),
                            TagData::new("typescript", "TypeScript"),
                        ],
                        selected: selected_tags(),
                        on_change: EventHandler::new(move |v: Vec<String>| selected_tags.set(v))
                    }
                }
            }
            
            Section { title: "Removable Tags",
                ExampleBox {
                    HStack { gap: SpacingSize::Sm, style: "flex-wrap: wrap;",
                        Tag { removable: true, on_remove: Some(EventHandler::new(move |_| {})), children: rsx! { "Removable" } }
                        Tag { variant: TagVariant::Primary, removable: true, on_remove: Some(EventHandler::new(move |_| {})), children: rsx! { "Tag" } }
                    }
                }
            }
        }
    }
}
