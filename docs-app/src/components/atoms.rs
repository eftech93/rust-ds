//! Atom component documentation pages

use crate::docs_ui::{CodeBlock, DocPage, ExampleBox, PropsTable, Section};
use dioxus::prelude::*;
use dioxus_ui_system::atoms::{
    Box, DatePicker, Divider, DividerOrientation, HStack, Heading, HeadingLevel, Progress,
    ProgressSize, ProgressVariant, Rating, Skeleton, SkeletonShape, SkeletonText, Slider,
    SliderMark, Spinner, SpinnerSize, SpinnerVariant, StepIndicator, StepState, Tag, TagData,
    TagGroup, TagVariant, VStack,
};
use dioxus_ui_system::prelude::*;

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
                    li { "Toggle - Two-state button (New!)" }
                    li { "NumberInput - Input with +/- buttons (New!)" }
                    li { "AspectRatio - Maintains consistent ratio (New!)" }
                    li { "PasswordInput - Password with show/hide toggle (New!)" }
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

            Section { title: "Props",
                PropsTable { props: vec![
                    ("children", "Element", "Box content"),
                    ("display", "BoxDisplay", "Display type"),
                    ("flex_direction", "FlexDirection", "Flex direction (when display is flex)"),
                    ("flex_wrap", "FlexWrap", "Flex wrap"),
                    ("justify_content", "JustifyContent", "Justify content"),
                    ("align_items", "AlignItems", "Align items"),
                    ("padding", "SpacingSize", "Padding (all sides)"),
                    ("px", "Option<SpacingSize>", "Padding X (horizontal)"),
                    ("py", "Option<SpacingSize>", "Padding Y (vertical)"),
                    ("pt", "Option<SpacingSize>", "Padding top"),
                    ("pr", "Option<SpacingSize>", "Padding right"),
                    ("pb", "Option<SpacingSize>", "Padding bottom"),
                    ("pl", "Option<SpacingSize>", "Padding left"),
                    ("margin", "SpacingSize", "Margin (all sides)"),
                    ("mx", "Option<SpacingSize>", "Margin X (horizontal)"),
                    ("my", "Option<SpacingSize>", "Margin Y (vertical)"),
                    ("mt", "Option<SpacingSize>", "Margin top"),
                    ("mr", "Option<SpacingSize>", "Margin right"),
                    ("mb", "Option<SpacingSize>", "Margin bottom"),
                    ("ml", "Option<SpacingSize>", "Margin left"),
                    ("gap", "SpacingSize", "Gap between children (for flex/grid)"),
                    ("background", "BackgroundColor", "Background color"),
                    ("border_radius", "RadiusSize", "Border radius"),
                    ("border", "BorderWidth", "Border width"),
                    ("border_color", "Option<BackgroundColor>", "Border color"),
                    ("shadow", "ShadowSize", "Box shadow"),
                    ("width", "Option<String>", "Width (e.g., '100%', '200px')"),
                    ("height", "Option<String>", "Height (e.g., '100%', '200px')"),
                    ("min_width", "Option<String>", "Minimum width"),
                    ("min_height", "Option<String>", "Minimum height"),
                    ("max_width", "Option<String>", "Maximum width"),
                    ("max_height", "Option<String>", "Maximum height"),
                    ("overflow", "Overflow", "Overflow behavior"),
                    ("position", "Position", "Position type"),
                    ("top", "Option<String>", "Top position"),
                    ("right", "Option<String>", "Right position"),
                    ("bottom", "Option<String>", "Bottom position"),
                    ("left", "Option<String>", "Left position"),
                    ("z_index", "Option<i16>", "Z-index"),
                    ("opacity", "Option<f32>", "Opacity (0.0 - 1.0)"),
                    ("cursor", "Option<String>", "Cursor style"),
                    ("onclick", "Option<EventHandler<MouseEvent>>", "Click handler"),
                    ("onmouseenter", "Option<EventHandler<MouseEvent>>", "Mouse enter handler"),
                    ("onmouseleave", "Option<EventHandler<MouseEvent>>", "Mouse leave handler"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("id", "Option<String>", "Element id"),
                ]}
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
                            on_change: move |_| {},
                        }
                    }
                }
                CodeBlock { code: "InputGroup {{\n    label: \"Email\",\n    value: email(),\n    input_type: InputType::Email,\n    onchange: move |v| email.set(v),\n}}".to_string() }
            }

            Section { title: "Input Types",
                p { "Supported input types: Text, Email, Password, Number, Tel, Url, Search" }
            }

            Section { title: "Props",
                PropsTable { props: vec![
                    ("value", "String", "Current value"),
                    ("placeholder", "Option<String>", "Placeholder text"),
                    ("input_type", "InputType", "Input type"),
                    ("disabled", "bool", "Disabled state"),
                    ("readonly", "bool", "Read-only state"),
                    ("required", "bool", "Required field"),
                    ("autofocus", "bool", "Autofocus on mount"),
                    ("on_change", "Option<EventHandler<String>>", "Change handler"),
                    ("onfocus", "Option<EventHandler<FocusEvent>>", "Focus handler"),
                    ("onblur", "Option<EventHandler<FocusEvent>>", "Blur handler"),
                    ("oninput", "Option<EventHandler<FormEvent>>", "Input handler (real-time)"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("name", "Option<String>", "Input name attribute"),
                    ("id", "Option<String>", "Input id attribute"),
                ]}
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
                CodeBlock { code: "Label {{ size: TextSize::Large, \"Large Text\" }}".to_string() }
            }

            Section { title: "Props",
                PropsTable { props: vec![
                    ("children", "Element", "Text content"),
                    ("size", "TextSize", "Typography size"),
                    ("weight", "TextWeight", "Font weight"),
                    ("color", "TextColor", "Text color"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("as_element", "LabelElement", "HTML element to render (defaults to span)"),
                    ("align", "TextAlign", "Text alignment"),
                    ("truncate", "bool", "Truncate with ellipsis"),
                    ("html_for", "Option<String>", "Associates label with form input"),
                    ("line_clamp", "Option<u8>", "Line clamp (number of lines)"),
                    ("id", "Option<String>", "Element id"),
                ]}
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

            Section { title: "Props",
                PropsTable { props: vec![
                    ("name", "String", "SVG path data or icon name"),
                    ("size", "IconSize", "Icon size"),
                    ("color", "IconColor", "Icon color"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("aria_label", "Option<String>", "Accessibility label"),
                    ("flip_h", "bool", "Flip horizontally"),
                    ("flip_v", "bool", "Flip vertically"),
                    ("rotate", "u16", "Rotate degrees (0, 90, 180, 270)"),
                    ("id", "Option<String>", "Element id"),
                ]}
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
                        on_change: move |v| checked.set(v),
                    }
                }
                CodeBlock { code: "Checkbox {{\n    checked: checked(),\n    label: Some(\"Accept terms\".to_string()),\n    onchange: move |v| checked.set(v),\n}}".to_string() }
            }

            Section { title: "Props",
                PropsTable { props: vec![
                    ("checked", "bool", "Whether the checkbox is checked"),
                    ("on_change", "Option<EventHandler<bool>>", "Callback when checked state changes"),
                    ("disabled", "bool", "Whether the checkbox is disabled"),
                    ("label", "Option<String>", "Checkbox label"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("id", "Option<String>", "Element id"),
                ]}
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
                            on_change: move |_| selected.set("option1".to_string()),
                        }
                        Radio {
                            name: "radio".to_string(),
                            value: "option2".to_string(),
                            checked: selected() == "option2",
                            label: Some("Option 2".to_string()),
                            on_change: move |_| selected.set("option2".to_string()),
                        }
                    }
                }
                CodeBlock { code: "Radio {{
    name: \"group\".to_string(),
    value: \"option1\".to_string(),
    checked: selected() == \"option1\",
    label: Some(\"Option 1\".to_string()),
    onchange: move |_| selected.set(\"option1\".to_string()),
}}".to_string() }
            }

            Section { title: "Props",
                PropsTable { props: vec![
                    ("checked", "bool", "Whether the radio is selected"),
                    ("on_change", "Option<EventHandler<()>>", "Callback when selected state changes"),
                    ("disabled", "bool", "Whether the radio is disabled"),
                    ("label", "Option<String>", "Radio label"),
                    ("value", "String", "Radio value"),
                    ("name", "String", "Name for grouping radios"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("id", "Option<String>", "Element id"),
                ]}
            }

            Section { title: "RadioGroup Props",
                PropsTable { props: vec![
                    ("name", "String", "Radio group name"),
                    ("value", "String", "Currently selected value"),
                    ("on_change", "EventHandler<String>", "Callback when selection changes"),
                    ("children", "Element", "Radio group children (Radio components)"),
                    ("direction", "RadioDirection", "Layout direction (Horizontal/Vertical)"),
                ]}
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
                        on_change: move |v| on.set(v),
                    }
                }
                CodeBlock { code: "Switch {{
    checked: enabled(),
    label: Some(\"Enable notifications\".to_string()),
    onchange: move |v| enabled.set(v),
}}".to_string() }
            }

            Section { title: "Props",
                PropsTable { props: vec![
                    ("checked", "bool", "Whether the switch is checked"),
                    ("on_change", "Option<EventHandler<bool>>", "Callback when checked state changes"),
                    ("disabled", "bool", "Whether the switch is disabled"),
                    ("label", "Option<String>", "Switch label"),
                    ("size", "SwitchSize", "Switch size (Sm, Md, Lg)"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("id", "Option<String>", "Element id"),
                ]}
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
                            on_change: move |_| {},
                        }
                    }
                }
                CodeBlock { code: "Select {{
    value: selected(),
    options: vec![
        SelectOption::new(\"option1\", \"Option 1\"),
        SelectOption::new(\"option2\", \"Option 2\"),
    ],
    onchange: move |v| selected.set(v),
}}".to_string() }
            }

            Section { title: "Props",
                PropsTable { props: vec![
                    ("value", "String", "Currently selected value"),
                    ("on_change", "Option<EventHandler<String>>", "Callback when selection changes"),
                    ("options", "Vec<SelectOption>", "Available options"),
                    ("placeholder", "Option<String>", "Placeholder text when no value selected"),
                    ("disabled", "bool", "Whether the select is disabled"),
                    ("error", "bool", "Whether the select has an error"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("id", "Option<String>", "Element id"),
                ]}
            }

            Section { title: "MultiSelect Props",
                PropsTable { props: vec![
                    ("values", "Vec<String>", "Currently selected values"),
                    ("on_change", "Option<EventHandler<Vec<String>>>", "Callback when selection changes"),
                    ("options", "Vec<SelectOption>", "Available options"),
                    ("placeholder", "Option<String>", "Placeholder text"),
                    ("disabled", "bool", "Whether the select is disabled"),
                    ("max_selections", "Option<usize>", "Maximum number of selections"),
                ]}
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
                            on_change: move |_| {},
                        }
                    }
                }
                CodeBlock { code: "TextArea {{
    value: description(),
    rows: 4,
    placeholder: Some(\"Enter description...\".to_string()),
    onchange: move |v| description.set(v),
}}".to_string() }
            }

            Section { title: "Props",
                PropsTable { props: vec![
                    ("value", "String", "Current value"),
                    ("on_change", "Option<EventHandler<String>>", "Callback when value changes"),
                    ("placeholder", "Option<String>", "Placeholder text"),
                    ("rows", "u8", "Number of rows (default: 3)"),
                    ("disabled", "bool", "Whether the textarea is disabled"),
                    ("readonly", "bool", "Whether the textarea is read-only"),
                    ("error", "bool", "Whether the textarea has an error"),
                    ("error_message", "Option<String>", "Error message"),
                    ("required", "bool", "Whether the field is required"),
                    ("max_length", "Option<usize>", "Maximum number of characters"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("id", "Option<String>", "Element id"),
                ]}
            }

            Section { title: "AutoResizeTextArea Props",
                PropsTable { props: vec![
                    ("value", "String", "Current value"),
                    ("on_change", "Option<EventHandler<String>>", "Callback when value changes"),
                    ("placeholder", "Option<String>", "Placeholder text"),
                    ("min_rows", "u8", "Minimum number of rows (default: 2)"),
                    ("max_rows", "u8", "Maximum number of rows (default: 10)"),
                    ("disabled", "bool", "Whether the textarea is disabled"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("id", "Option<String>", "Element id"),
                ]}
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
                CodeBlock { code: "StepIndicator {{
    step: 1,
    state: StepState::Completed,
}}".to_string() }
            }

            Section { title: "Props",
                PropsTable { props: vec![
                    ("step", "u32", "Step number (1-indexed, default: 1)"),
                    ("state", "StepState", "Step state (Pending, Active, Completed, Error)"),
                    ("size", "StepSize", "Step size (Sm, Md, Lg)"),
                    ("icon", "Option<String>", "Optional icon to replace number"),
                    ("bg_color", "Option<String>", "Custom background color (overrides state color)"),
                    ("text_color", "Option<String>", "Custom text color"),
                    ("on_click", "Option<EventHandler<()>>", "Optional click handler"),
                    ("aria_label", "Option<String>", "Optional aria label"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
            }

            Section { title: "StepConnector Props",
                PropsTable { props: vec![
                    ("horizontal", "bool", "Connector orientation (default: true)"),
                    ("completed", "bool", "Whether the connector represents completed progress"),
                    ("color", "Option<String>", "Connector color (overrides default)"),
                    ("thickness", "String", "Connector thickness (default: '2px')"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
            }

            Section { title: "StepLabel Props",
                PropsTable { props: vec![
                    ("label", "String", "Label text"),
                    ("description", "Option<String>", "Optional description/subtitle"),
                    ("state", "StepState", "Step state (affects color)"),
                    ("size", "StepSize", "Text size"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
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

            Section { title: "Props",
                PropsTable { props: vec![
                    ("level", "HeadingLevel", "The heading level (H1-H6, default: H1)"),
                    ("size", "Option<u16>", "Custom font size (overrides default for level)"),
                    ("weight", "u16", "Font weight (default: 600)"),
                    ("color", "Option<String>", "Color override"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("style", "Option<String>", "Additional inline styles"),
                    ("align", "Option<String>", "Text alignment"),
                    ("id", "Option<String>", "Element id"),
                    ("children", "Element", "Children elements"),
                ]}
            }

            Section { title: "Paragraph Props",
                PropsTable { props: vec![
                    ("size", "u16", "Font size (default: 16)"),
                    ("line_height", "f32", "Line height (default: 1.6)"),
                    ("color", "Option<String>", "Color override"),
                    ("align", "Option<String>", "Text alignment"),
                    ("max_chars", "u16", "Maximum width for readability in characters (default: 75)"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("style", "Option<String>", "Additional inline styles"),
                    ("id", "Option<String>", "Element id"),
                    ("children", "Element", "Children elements"),
                ]}
            }

            Section { title: "Caption Props",
                PropsTable { props: vec![
                    ("size", "u8", "Font size (default: 12)"),
                    ("color", "CaptionColor", "Color variant (default: Muted)"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("id", "Option<String>", "Element id"),
                    ("children", "Element", "Children elements"),
                ]}
            }

            Section { title: "Blockquote Props",
                PropsTable { props: vec![
                    ("border_color", "Option<String>", "Border color"),
                    ("background", "Option<String>", "Background color"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("cite", "Option<String>", "Citation/attribution"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("id", "Option<String>", "Element id"),
                    ("children", "Element", "Children elements (quote content)"),
                ]}
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
                CodeBlock { code: "Divider {{
    label: Some(\"OR\".to_string()),
    orientation: DividerOrientation::Horizontal,
    variant: DividerVariant::Solid,
}}".to_string() }
            }

            Section { title: "Props",
                PropsTable { props: vec![
                    ("orientation", "DividerOrientation", "Orientation (horizontal or vertical, default: Horizontal)"),
                    ("variant", "DividerVariant", "Visual style variant (Solid, Dashed, Dotted, default: Solid)"),
                    ("color", "Option<String>", "Color override"),
                    ("thickness", "u8", "Thickness in pixels (default: 1)"),
                    ("spacing", "u16", "Spacing before and after divider (default: 16)"),
                    ("length", "Option<String>", "Width (horizontal) or height (vertical)"),
                    ("label", "Option<String>", "Optional label/text in the middle"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("style", "Option<String>", "Additional inline styles"),
                    ("id", "Option<String>", "Element id"),
                ]}
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
                CodeBlock { code: "Progress {{
    value: Some(75.0),
    max: 100.0,
    show_label: true,
    variant: ProgressVariant::Linear,
    size: ProgressSize::Md,
}}".to_string() }
            }

            Section { title: "Props",
                PropsTable { props: vec![
                    ("value", "Option<f32>", "Current progress value (0-100)"),
                    ("max", "f32", "Maximum value (default: 100)"),
                    ("variant", "ProgressVariant", "Variant (Linear or Circular, default: Linear)"),
                    ("size", "ProgressSize", "Size variant (Sm, Md, Lg, default: Md)"),
                    ("color", "Option<String>", "Color for the progress bar/track"),
                    ("track_color", "Option<String>", "Background track color"),
                    ("show_label", "bool", "Show percentage label (default: false)"),
                    ("label_position", "LabelPosition", "Label position (default: Right)"),
                    ("width", "String", "Width for linear progress (default: '100%')"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("indeterminate", "bool", "Indeterminate state (default: false)"),
                    ("id", "Option<String>", "Element id"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
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
                CodeBlock { code: "Spinner {{
    variant: SpinnerVariant::Circular,
    size: SpinnerSize::Lg,
}}".to_string() }
            }

            Section { title: "Props",
                PropsTable { props: vec![
                    ("variant", "SpinnerVariant", "Spinner variant/style (default: Circular)"),
                    ("size", "SpinnerSize", "Size variant (Xs, Sm, Md, Lg, Xl, default: Md)"),
                    ("color", "Option<String>", "Color override"),
                    ("label", "String", "Accessibility label (default: 'Loading')"),
                    ("speed", "f32", "Speed multiplier (default: 1.0)"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("id", "Option<String>", "Element id"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
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
                CodeBlock { code: "Skeleton {{
    shape: SkeletonShape::Text,
    width: Some(\"200px\".to_string()),
    animated: true,
}}".to_string() }
            }

            Section { title: "Props",
                PropsTable { props: vec![
                    ("shape", "SkeletonShape", "Shape variant (default: Rectangle)"),
                    ("width", "Option<String>", "Width (CSS value or 'auto')"),
                    ("height", "Option<String>", "Height (CSS value or 'auto')"),
                    ("animated", "bool", "Enable animation (default: true)"),
                    ("animation", "SkeletonAnimation", "Animation variant (default: Shimmer)"),
                    ("color", "Option<String>", "Base color"),
                    ("highlight_color", "Option<String>", "Highlight color for animation"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("id", "Option<String>", "Element id"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
            }

            Section { title: "SkeletonCard Props",
                PropsTable { props: vec![
                    ("show_image", "bool", "Show image area (default: true)"),
                    ("image_height", "Option<String>", "Image height"),
                    ("text_lines", "u8", "Number of text lines (default: 3)"),
                    ("show_actions", "bool", "Show action buttons (default: true)"),
                    ("animated", "bool", "Enable animation (default: true)"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("id", "Option<String>", "Element id"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
            }

            Section { title: "SkeletonText Props",
                PropsTable { props: vec![
                    ("lines", "u8", "Number of lines (default: 3)"),
                    ("line_height", "f32", "Line height (default: 1.5)"),
                    ("animated", "bool", "Enable animation (default: true)"),
                    ("last_line_width", "u8", "Last line width as percentage (default: 80)"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("id", "Option<String>", "Element id"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
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
                CodeBlock { code: "Rating {{
    value: rating(),
    interactive: true,
    on_change: Some(EventHandler::new(move |v| rating.set(v))),
    max: 5,
    size: 24,
}}".to_string() }
            }

            Section { title: "Props",
                PropsTable { props: vec![
                    ("value", "f32", "Current rating value"),
                    ("max", "u8", "Maximum rating (default: 5)"),
                    ("allow_half", "bool", "Allow half stars (default: true)"),
                    ("interactive", "bool", "Interactive (clickable) (default: false)"),
                    ("size", "u16", "Size in pixels (default: 20)"),
                    ("fill_color", "Option<String>", "Color for filled stars"),
                    ("empty_color", "Option<String>", "Color for empty stars"),
                    ("on_change", "Option<EventHandler<f32>>", "Change handler (only used when interactive)"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("show_value", "bool", "Show rating value text (default: false)"),
                    ("id", "Option<String>", "Element id"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
            }

            Section { title: "RatingInput Props",
                PropsTable { props: vec![
                    ("value", "f32", "Current rating value (default: 0.0)"),
                    ("max", "u8", "Maximum rating (default: 5)"),
                    ("allow_half", "bool", "Allow half stars (default: false)"),
                    ("size", "u16", "Size in pixels (default: 24)"),
                    ("on_change", "EventHandler<f32>", "Change handler"),
                    ("label", "Option<String>", "Label"),
                    ("clearable", "bool", "Show clear button (default: false)"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("id", "Option<String>", "Element id"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
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
                div {
                    style: "padding: 24px; border: 1px solid #e5e7eb; border-radius: 12px; background: #fff; min-height: 300px;",
                    DatePicker {
                        value: date(),
                        on_change: Some(EventHandler::new(move |d: String| date.set(Some(d)))),
                        label: Some("Select Date".to_string())
                    }
                }
            }

            Section { title: "With Constraints",
                div {
                    style: "padding: 24px; border: 1px solid #e5e7eb; border-radius: 12px; background: #fff; min-height: 300px;",
                    DatePicker {
                        label: Some("Select Date".to_string()),
                        min: Some("2024-01-01".to_string()),
                        max: Some("2024-12-31".to_string())
                    }
                }
                CodeBlock { code: "DatePicker {{
    value: date(),
    on_change: Some(EventHandler::new(move |d| date.set(Some(d)))),
    label: Some(\"Select Date\".to_string()),
    min: Some(\"2024-01-01\".to_string()),
    max: Some(\"2024-12-31\".to_string()),
}}".to_string() }
            }

            Section { title: "Props",
                PropsTable { props: vec![
                    ("value", "Option<String>", "Selected date (YYYY-MM-DD format)"),
                    ("on_change", "Option<EventHandler<String>>", "Change handler"),
                    ("min", "Option<String>", "Minimum date (YYYY-MM-DD)"),
                    ("max", "Option<String>", "Maximum date (YYYY-MM-DD)"),
                    ("placeholder", "Option<String>", "Placeholder text"),
                    ("label", "Option<String>", "Label"),
                    ("required", "bool", "Whether the field is required (default: false)"),
                    ("disabled", "bool", "Whether the field is disabled (default: false)"),
                    ("error", "Option<String>", "Error message"),
                    ("size", "DatePickerSize", "Size variant (default: Md)"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("id", "Option<String>", "Element id"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
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
                CodeBlock { code: "Slider {{
    value: value(),
    on_change: EventHandler::new(move |v| value.set(v)),
    label: Some(\"Volume\".to_string()),
    marks: vec![
        SliderMark {{ value: 0.0, label: Some(\"0%\".to_string()) }},
        SliderMark {{ value: 50.0, label: Some(\"50%\".to_string()) }},
        SliderMark {{ value: 100.0, label: Some(\"100%\".to_string()) }},
    ],
}}".to_string() }
            }

            Section { title: "Props",
                PropsTable { props: vec![
                    ("value", "f64", "Current value (default: 0.0)"),
                    ("min", "f64", "Minimum value (default: 0.0)"),
                    ("max", "f64", "Maximum value (default: 100.0)"),
                    ("step", "f64", "Step increment (default: 1.0)"),
                    ("on_change", "EventHandler<f64>", "Change handler"),
                    ("label", "Option<String>", "Label"),
                    ("disabled", "bool", "Whether the slider is disabled (default: false)"),
                    ("show_value", "bool", "Show current value (default: true)"),
                    ("format_value", "Option<fn(f64) -> String>", "Format function for value display"),
                    ("color", "Option<String>", "Color for the slider track/fill"),
                    ("size", "SliderSize", "Size variant (default: Md)"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("marks", "Vec<SliderMark>", "Marks/ticks on the slider"),
                    ("id", "Option<String>", "Element id"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
            }

            Section { title: "RangeSlider Props",
                PropsTable { props: vec![
                    ("min_value", "f64", "Lower value"),
                    ("max_value", "f64", "Upper value"),
                    ("min", "f64", "Overall minimum (default: 0.0)"),
                    ("max", "f64", "Overall maximum (default: 100.0)"),
                    ("step", "f64", "Step increment (default: 1.0)"),
                    ("on_change", "EventHandler<(f64, f64)>", "Change handler"),
                    ("label", "Option<String>", "Label"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("id", "Option<String>", "Element id"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
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
                    }
                }
                CodeBlock { code: "Tag {{
    variant: TagVariant::Primary,
    removable: true,
    on_remove: Some(EventHandler::new(move |_| {{}})),
    children: rsx! {{ \"Tag Label\" }}
}}".to_string() }
            }

            Section { title: "Tag Group",
                ExampleBox {
                    HStack { gap: SpacingSize::Sm, style: "flex-wrap: wrap;",
                        Tag { removable: true, on_remove: Some(EventHandler::new(move |_| {})), children: rsx! { "Removable" } }
                        Tag { variant: TagVariant::Primary, removable: true, on_remove: Some(EventHandler::new(move |_| {})), children: rsx! { "Tag" } }
                    }
                }
            }

            Section { title: "Props",
                PropsTable { props: vec![
                    ("children", "Element", "Tag content/label"),
                    ("variant", "TagVariant", "Visual variant (default: Default)"),
                    ("size", "TagSize", "Size variant (default: Md)"),
                    ("selected", "bool", "Whether the tag is selected/active (default: false)"),
                    ("selectable", "bool", "Whether the tag is selectable (clickable) (default: false)"),
                    ("on_click", "Option<EventHandler<()>>", "Click handler"),
                    ("removable", "bool", "Removable (shows close button) (default: false)"),
                    ("on_remove", "Option<EventHandler<()>>", "Remove handler"),
                    ("icon", "Option<String>", "Icon (emoji or icon name)"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("color", "Option<String>", "Custom background color"),
                    ("text_color", "Option<String>", "Custom text color"),
                    ("disabled", "bool", "Disabled state (default: false)"),
                    ("id", "Option<String>", "Element id"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
            }

            Section { title: "TagGroup Props",
                PropsTable { props: vec![
                    ("tags", "Vec<TagData>", "Tags to display"),
                    ("multiple", "bool", "Whether multiple tags can be selected (default: false)"),
                    ("selected", "Vec<String>", "Currently selected tag(s)"),
                    ("on_change", "EventHandler<Vec<String>>", "Selection change handler"),
                    ("removable", "bool", "Whether tags are removable (default: false)"),
                    ("on_remove", "Option<EventHandler<String>>", "Remove handler"),
                    ("size", "TagSize", "Size variant (default: Md)"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("id", "Option<String>", "Element id"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
            }

            Section { title: "InputTag Props",
                PropsTable { props: vec![
                    ("tags", "Vec<String>", "Current tags"),
                    ("on_change", "EventHandler<Vec<String>>", "Change handler"),
                    ("placeholder", "Option<String>", "Placeholder text"),
                    ("max_tags", "usize", "Maximum number of tags (default: MAX)"),
                    ("allow_duplicates", "bool", "Whether duplicates are allowed (default: false)"),
                    ("tag_variant", "TagVariant", "Tag variant (default: Default)"),
                    ("size", "TagSize", "Size variant (default: Md)"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("id", "Option<String>", "Element id"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
            }
        }
    }
}
