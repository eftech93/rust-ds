//! Molecule component documentation pages

use dioxus::prelude::*;
use dioxus_ui_system::prelude::*;
use dioxus_ui_system::molecules::{DialogFooter, DialogFooterAlign, 
    use_toast, Combobox, ComboboxOption, MediaObject, MediaContent, Comment,
    Pagination, PaginationInfo, ListItem, ListItemVariant, ListGroup, ActionListItem,
    SkeletonMolecule as Skeleton, SkeletonCircle
};
use dioxus_ui_system::atoms::{Box, VStack, HStack};

#[component]
pub fn MoleculesPage() -> Element {
    rsx! {
        DocPage {
            title: "Molecules",
            description: "Groups of atoms bonded together. These components combine multiple atoms to create more complex UI elements.",
            
            Section { title: "Overview",
                p { "Molecules include:" }
                ul {
                    li { "Card - Content containers" }
                    li { "Badge - Status indicators" }
                    li { "Alert - Notification messages" }
                    li { "Avatar - User profile images" }
                    li { "Dialog - Modal windows" }
                    li { "Dropdown - Contextual menus" }
                    li { "Tooltip - Contextual hints" }
                    li { "Separator - Visual dividers" }
                    li { "Skeleton - Loading placeholders" }
                    li { "Stepper - Progress indicators" }
                }
            }
        }
    }
}

#[component]
pub fn CardPage() -> Element {
    rsx! {
        DocPage {
            title: "Card",
            description: "Container component for grouping related content.",
            
            Section { title: "Variants",
                ExampleBox {
                    Box { style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px;",
                        Card { variant: CardVariant::Default, padding: Some("16px".to_string()), "Default Card" }
                        Card { variant: CardVariant::Elevated, padding: Some("16px".to_string()), "Elevated Card" }
                    }
                }
            }
            
            Section { title: "With Header",
                ExampleBox {
                    Card {
                        CardHeader {
                            title: "Card Title",
                            subtitle: Some("Card subtitle description".to_string()),
                        }
                        CardContent { "Card content goes here." }
                    }
                }
            }
        }
    }
}

#[component]
pub fn BadgePage() -> Element {
    rsx! {
        DocPage {
            title: "Badge",
            description: "Small status indicators and labels.",
            
            Section { title: "Variants",
                ExampleBox {
                    HStack { gap: SpacingSize::Md, style: "flex-wrap: wrap;",
                        Badge { "Default" }
                        Badge { variant: BadgeVariant::Secondary, "Secondary" }
                        Badge { variant: BadgeVariant::Success, icon: Some("check".to_string()), "Success" }
                        Badge { variant: BadgeVariant::Warning, "Warning" }
                        Badge { variant: BadgeVariant::Destructive, "Error" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn AlertPage() -> Element {
    rsx! {
        DocPage {
            title: "Alert",
            description: "Status messages and notifications.",
            
            Section { title: "Variants",
                ExampleBox {
                    VStack { gap: SpacingSize::Md,
                        Alert { variant: AlertVariant::Default, title: Some("Note".to_string()), "This is a default alert." }
                        Alert { variant: AlertVariant::Success, title: Some("Success".to_string()), icon: Some("check-circle".to_string()), "Operation completed successfully!" }
                        Alert { variant: AlertVariant::Warning, title: Some("Warning".to_string()), icon: Some("alert-triangle".to_string()), "Please review your settings." }
                    }
                }
            }
        }
    }
}

#[component]
pub fn AvatarPage() -> Element {
    rsx! {
        DocPage {
            title: "Avatar",
            description: "User profile images with fallback initials.",
            
            Section { title: "Sizes",
                ExampleBox {
                    HStack { gap: SpacingSize::Lg, style: "align-items: center;",
                        Avatar { size: AvatarSize::Xs, name: Some("XS".to_string()), src: None, alt: "".to_string(), fallback: None, style: None, class: None }
                        Avatar { size: AvatarSize::Sm, name: Some("SM".to_string()), src: None, alt: "".to_string(), fallback: None, style: None, class: None }
                        Avatar { size: AvatarSize::Md, name: Some("MD".to_string()), src: None, alt: "".to_string(), fallback: None, style: None, class: None }
                        Avatar { size: AvatarSize::Lg, name: Some("LG".to_string()), src: None, alt: "".to_string(), fallback: None, style: None, class: None }
                        Avatar { size: AvatarSize::Xl, name: Some("XL".to_string()), src: None, alt: "".to_string(), fallback: None, style: None, class: None }
                    }
                }
            }
        }
    }
}

#[component]
pub fn DialogPage() -> Element {
    let mut open = use_signal(|| false);
    
    rsx! {
        DocPage {
            title: "Dialog",
            description: "Modal windows for important information or actions.",
            
            Section { title: "Basic Dialog",
                ExampleBox {
                    Button { variant: ButtonVariant::Primary, onclick: move |_| open.set(true), "Open Dialog" }
                    
                    Dialog {
                        open: open(),
                        on_close: move |_| open.set(false),
                        title: Some("Example Dialog".to_string()),
                        description: Some("This is a dialog component.".to_string()),
                        
                        p { "Dialogs are great for displaying important information." }
                        
                        DialogFooter {
                            align: DialogFooterAlign::End,
                            Button { variant: ButtonVariant::Ghost, onclick: move |_| open.set(false), "Cancel" }
                            Button { variant: ButtonVariant::Primary, onclick: move |_| open.set(false), "Confirm" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn DropdownPage() -> Element {
    use dioxus_ui_system::molecules::{DropdownMenu, DropdownMenuItem, DropdownAlign};
    
    let items = vec![
        DropdownMenuItem::new("profile", "Profile").with_icon("user"),
        DropdownMenuItem::new("settings", "Settings").with_icon("settings"),
        DropdownMenuItem::new("logout", "Logout").with_icon("log-out"),
    ];
    
    let items_with_actions = vec![
        DropdownMenuItem::new("copy", "Copy").with_icon("copy").with_shortcut("⌘C"),
        DropdownMenuItem::new("cut", "Cut").with_icon("scissors").with_shortcut("⌘X"),
        DropdownMenuItem::new("paste", "Paste").with_icon("clipboard").with_shortcut("⌘V"),
    ];
    
    rsx! {
        DocPage {
            title: "Dropdown",
            description: "Contextual action menus that appear when triggered by a button click.",
            
            Section { title: "Basic Dropdown",
                ExampleBox {
                    HStack { gap: SpacingSize::Md,
                        DropdownMenu {
                            trigger: rsx! { Button { variant: ButtonVariant::Primary, "Open Menu" } },
                            items: items.clone(),
                            align: DropdownAlign::Start,
                            on_select: move |id| println!("Selected: {}", id),
                        }
                    }
                }
                CodeBlock { code: "DropdownMenu {{
    trigger: rsx! {{ Button {{ \"Open Menu\" }} }},
    items: vec![
        DropdownMenuItem::new(\"profile\", \"Profile\").with_icon(\"user\"),
        DropdownMenuItem::new(\"settings\", \"Settings\").with_icon(\"settings\"),
    ],
    on_select: move |id| println!(\"Selected: {{}}\", id),
}}".to_string() }
            }
            
            Section { title: "With Separators and Shortcuts",
                ExampleBox {
                    DropdownMenu {
                        trigger: rsx! { Button { variant: ButtonVariant::Secondary, "Edit Options" } },
                        items: items_with_actions.clone(),
                        align: DropdownAlign::Start,
                        on_select: move |_| {},
                    }
                }
            }
            
            Section { title: "Alignment",
                p { "Dropdowns can be aligned to start, center, or end of the trigger:" }
                ExampleBox {
                    HStack { gap: SpacingSize::Md,
                        DropdownMenu {
                            trigger: rsx! { Button { variant: ButtonVariant::Ghost, "Align Start" } },
                            items: items.clone(),
                            align: DropdownAlign::Start,
                            on_select: move |_| {},
                        }
                        DropdownMenu {
                            trigger: rsx! { Button { variant: ButtonVariant::Ghost, "Align Center" } },
                            items: items.clone(),
                            align: DropdownAlign::Center,
                            on_select: move |_| {},
                        }
                        DropdownMenu {
                            trigger: rsx! { Button { variant: ButtonVariant::Ghost, "Align End" } },
                            items: items.clone(),
                            align: DropdownAlign::End,
                            on_select: move |_| {},
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn TooltipPage() -> Element {
    use dioxus_ui_system::molecules::{Tooltip, TooltipPlacement, SimpleTooltip};
    
    rsx! {
        DocPage {
            title: "Tooltip",
            description: "Contextual hints that appear when hovering over elements.",
            
            Section { title: "Basic Tooltip",
                ExampleBox {
                    HStack { gap: SpacingSize::Xl, style: "justify-content: center; padding: 32px;",
                        Tooltip {
                            content: "This is a tooltip!".to_string(),
                            placement: TooltipPlacement::Top,
                            span { style: "padding: 8px 16px; background: rgb(226,232,240); border-radius: 6px; cursor: help;", "Hover me (Top)" }
                        }
                        Tooltip {
                            content: "Bottom tooltip".to_string(),
                            placement: TooltipPlacement::Bottom,
                            span { style: "padding: 8px 16px; background: rgb(226,232,240); border-radius: 6px; cursor: help;", "Hover me (Bottom)" }
                        }
                    }
                }
                CodeBlock { code: "Tooltip {{
    content: \"Tooltip text\".to_string(),
    placement: TooltipPlacement::Top,
    Button {{ \"Hover me\" }}
}}".to_string() }
            }
            
            Section { title: "Placements",
                ExampleBox {
                    Box { style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px; padding: 32px; justify-items: center;",
                        Tooltip { content: "Top placement".to_string(), placement: TooltipPlacement::Top,
                            span { style: "padding: 8px 16px; background: rgb(226,232,240); border-radius: 6px;", "Top" }
                        }
                        Tooltip { content: "TopStart placement".to_string(), placement: TooltipPlacement::TopStart,
                            span { style: "padding: 8px 16px; background: rgb(226,232,240); border-radius: 6px;", "TopStart" }
                        }
                        Tooltip { content: "TopEnd placement".to_string(), placement: TooltipPlacement::TopEnd,
                            span { style: "padding: 8px 16px; background: rgb(226,232,240); border-radius: 6px;", "TopEnd" }
                        }
                        Tooltip { content: "Right placement".to_string(), placement: TooltipPlacement::Right,
                            span { style: "padding: 8px 16px; background: rgb(226,232,240); border-radius: 6px;", "Right" }
                        }
                        Tooltip { content: "Center".to_string(), placement: TooltipPlacement::Top,
                            span { style: "padding: 8px 16px; background: rgb(226,232,240); border-radius: 6px; font-weight: bold;", "Center" }
                        }
                        Tooltip { content: "Left placement".to_string(), placement: TooltipPlacement::Left,
                            span { style: "padding: 8px 16px; background: rgb(226,232,240); border-radius: 6px;", "Left" }
                        }
                        Tooltip { content: "Bottom placement".to_string(), placement: TooltipPlacement::Bottom,
                            span { style: "padding: 8px 16px; background: rgb(226,232,240); border-radius: 6px;", "Bottom" }
                        }
                        Tooltip { content: "BottomStart placement".to_string(), placement: TooltipPlacement::BottomStart,
                            span { style: "padding: 8px 16px; background: rgb(226,232,240); border-radius: 6px;", "BottomStart" }
                        }
                        Tooltip { content: "BottomEnd placement".to_string(), placement: TooltipPlacement::BottomEnd,
                            span { style: "padding: 8px 16px; background: rgb(226,232,240); border-radius: 6px;", "BottomEnd" }
                        }
                    }
                }
            }
            
            Section { title: "Simple Tooltip",
                p { "For quick tooltips with just text, use SimpleTooltip:" }
                ExampleBox {
                    HStack { gap: SpacingSize::Xl, style: "justify-content: center; padding: 32px;",
                        SimpleTooltip {
                            text: "Click to save your changes".to_string(),
                            placement: TooltipPlacement::Top,
                            Button { variant: ButtonVariant::Primary, "Save Changes" }
                        }
                        SimpleTooltip {
                            text: "Delete this item permanently".to_string(),
                            placement: TooltipPlacement::Bottom,
                            Button { variant: ButtonVariant::Destructive, "Delete" }
                        }
                    }
                }
                CodeBlock { code: "SimpleTooltip {{
    text: \"Tooltip text\".to_string(),
    placement: TooltipPlacement::Top,
    Button {{ \"Click me\" }}
}}".to_string() }
            }
        }
    }
}

#[component]
pub fn SeparatorPage() -> Element {
    rsx! {
        DocPage {
            title: "Separator",
            description: "Visual dividers between content.",
            
            Section { title: "Horizontal",
                ExampleBox {
                    VStack { gap: SpacingSize::Md,
                        span { "Content above" }
                        Separator {}
                        span { "Content below" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SkeletonMoleculePage() -> Element {
    rsx! {
        DocPage {
            title: "Skeleton",
            description: "Loading placeholders for content.",
            
            Section { title: "Example",
                ExampleBox {
                    VStack { gap: SpacingSize::Md,
                        HStack { gap: SpacingSize::Sm, style: "align-items: center;",
                            SkeletonCircle { size: "48".to_string(), animate: true, style: None }
                            VStack { gap: SpacingSize::Sm, style: "flex: 1;",
                                Skeleton { width: Some("150px".to_string()), height: None, animate: true, rounded: None, style: None, class: None }
                                Skeleton { width: Some("100px".to_string()), height: None, animate: true, rounded: None, style: None, class: None }
                            }
                        }
                        SkeletonText { lines: 3, animate: true, last_line_width: 60, style: None }
                    }
                }
            }
        }
    }
}

#[component]
pub fn StepperPage() -> Element {
    use dioxus_ui_system::molecules::{HorizontalStepper, VerticalStepper, StepItem, StepContent, StepperActions};
    use dioxus_ui_system::atoms::{StepState, StepSize};
    
    let steps = vec![
        StepItem::new("Personal Info").with_description("Enter your details"),
        StepItem::new("Account").with_description("Set up your account"),
        StepItem::new("Review").with_description("Review and confirm"),
        StepItem::new("Complete").with_description("All done!"),
    ];
    
    let vertical_steps = vec![
        StepItem::new("Upload Files").with_icon("📁").with_state(StepState::Completed),
        StepItem::new("Processing").with_icon("⚙️").with_state(StepState::Active),
        StepItem::new("Review").with_icon("👁️").with_state(StepState::Pending),
        StepItem::new("Publish").with_icon("🚀").disabled(),
    ];
    
    rsx! {
        DocPage {
            title: "Stepper",
            description: "Progress indicators for multi-step processes.",
            
            Section { title: "Horizontal Stepper",
                ExampleBox {
                    HorizontalStepper {
                        steps: steps.clone(),
                        active_step: 1,
                    }
                }
            }
            
            Section { title: "Horizontal Stepper (Small)",
                ExampleBox {
                    HorizontalStepper {
                        steps: steps.clone(),
                        active_step: 2,
                        size: StepSize::Sm,
                    }
                }
            }
            
            Section { title: "Vertical Stepper",
                ExampleBox {
                    Box { style: "max-width: 400px;",
                        VerticalStepper {
                            steps: vertical_steps,
                            active_step: 1,
                        }
                    }
                }
            }
            
            Section { title: "With Content & Actions",
                ExampleBox {
                    VStack { gap: SpacingSize::Lg,
                        HorizontalStepper {
                            steps: steps.clone(),
                            active_step: 1,
                        }
                        StepContent {
                            step_index: 1,
                            active_step: 1,
                            Box { style: "padding: 24px; background: rgb(248,250,252); border-radius: 8px;",
                                h3 { style: "margin: 0 0 12px 0;", "Account Information" }
                                p { style: "margin: 0 0 16px 0; color: rgb(100,116,139);", "Please enter your account details to continue." }
                                VStack { gap: SpacingSize::Sm,
                                    Box {
                                        Label { "Username" }
                                        Input { value: "".to_string(), placeholder: "Enter username", onchange: move |_| {} }
                                    }
                                    Box {
                                        Label { "Email" }
                                        Input { value: "".to_string(), placeholder: "Enter email", onchange: move |_| {} }
                                    }
                                }
                            }
                        }
                        StepperActions {
                            current_step: 1,
                            total_steps: 4,
                            on_back: move |_| {},
                            on_next: move |_| {},
                            on_finish: move |_| {},
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn PopoverPage() -> Element {
    rsx! {
        DocPage {
            title: "Popover",
            description: "Floating content panels triggered by user interaction.",
            
            Section { title: "Basic Popover",
                ExampleBox {
                    p { "Popover component for displaying content in a floating panel." }
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

/// Toast documentation page
#[component]
pub fn ToastPage() -> Element {
    let mut toast = use_toast();
    
    rsx! {
        DocPage {
            title: "Toast",
            description: "Transient, non-blocking feedback notifications.",
            
            Section { title: "Toast Variants",
                ExampleBox {
                    HStack { gap: SpacingSize::Md, style: "flex-wrap: wrap;",
                        Button { variant: ButtonVariant::Primary, onclick: move |_| toast.show_success("Success!"), "Success Toast" }
                        Button { variant: ButtonVariant::Secondary, onclick: move |_| toast.show_error("Error occurred"), "Error Toast" }
                        Button { variant: ButtonVariant::Ghost, onclick: move |_| toast.show_warning("Warning message"), "Warning Toast" }
                    }
                }
            }
            
            Section { title: "Usage",
                p { "Use the use_toast hook to show notifications:" }
                CodeBlock { code: "let mut toast = use_toast();\ntoast.show_success(\"Operation completed!\");".to_string() }
            }
        }
    }
}

/// Combobox documentation page
#[component]
pub fn ComboboxPage() -> Element {
    let mut selected = use_signal(|| None::<String>);
    
    let options = vec![
        ComboboxOption::new("rust", "Rust"),
        ComboboxOption::new("go", "Go"),
        ComboboxOption::new("typescript", "TypeScript"),
        ComboboxOption::new("python", "Python"),
    ];
    
    rsx! {
        DocPage {
            title: "Combobox",
            description: "Autocomplete input with dropdown suggestions.",
            
            Section { title: "Basic Combobox",
                ExampleBox {
                    Combobox {
                        options: options.clone(),
                        value: selected(),
                        on_change: EventHandler::new(move |v: String| selected.set(Some(v))),
                        label: Some("Select Language".to_string()),
                        placeholder: Some("Search languages...".to_string())
                    }
                }
            }
            
            Section { title: "Creatable",
                p { "Allow users to create new options:" }
                ExampleBox {
                    Combobox {
                        options: options.clone(),
                        creatable: true,
                        placeholder: Some("Select or create...".to_string()),
                        on_change: EventHandler::new(move |_v: String| {}),
                        value: None
                    }
                }
            }
        }
    }
}

/// Media Object documentation page
#[component]
pub fn MediaObjectPage() -> Element {
    rsx! {
        DocPage {
            title: "Media Object",
            description: "Image + text content with flexible alignment.",
            
            Section { title: "Basic Media Object",
                ExampleBox {
                    MediaObject {
                        media: rsx! {
                            div { style: "width: 64px; height: 64px; background: #3b82f6; border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-size: 24px;", "📷" }
                        },
                        children: rsx! {
                            MediaContent {
                                title: Some("Media Title".to_string()),
                                description: Some("This is a description of the media content. It can contain multiple lines of text.".to_string()),
                                meta: Some("2 hours ago".to_string()),
                                title_level: 4
                            }
                        }
                    }
                }
            }
            
            Section { title: "Comment",
                ExampleBox {
                    Comment {
                        author: "Jane Doe".to_string(),
                        content: "This is a great component library! Love the design.".to_string(),
                        timestamp: "2 hours ago".to_string(),
                        like_count: 5,
                        liked: false
                    }
                }
            }
        }
    }
}

/// Pagination documentation page
#[component]
pub fn PaginationPage() -> Element {
    let mut page = use_signal(|| 1);
    
    rsx! {
        DocPage {
            title: "Pagination",
            description: "Page navigation for lists and tables.",
            
            Section { title: "Basic Pagination",
                ExampleBox {
                    Pagination {
                        total_pages: 10,
                        current_page: page(),
                        on_change: EventHandler::new(move |p: u32| page.set(p)),
                        sibling_count: 1,
                        show_first_last: true,
                        show_prev_next: true,
                        simple: false
                    }
                }
            }
            
            Section { title: "Simple Mode",
                ExampleBox {
                    Pagination {
                        total_pages: 5,
                        current_page: 2,
                        simple: true,
                        sibling_count: 1,
                        show_first_last: true,
                        show_prev_next: true,
                        on_change: EventHandler::new(move |_p: u32| {})
                    }
                }
            }
            
            Section { title: "With Info",
                ExampleBox {
                    VStack { gap: SpacingSize::Md,
                        PaginationInfo { current_page: 1, page_size: 10, total_items: 87 }
                        Pagination { total_pages: 9, current_page: 1, on_change: EventHandler::new(move |_p: u32| {}), sibling_count: 1, show_first_last: true, show_prev_next: true, simple: false }
                    }
                }
            }
        }
    }
}

/// List Item documentation page
#[component]
pub fn ListItemPage() -> Element {
    rsx! {
        DocPage {
            title: "List Item",
            description: "List row items with various configurations.",
            
            Section { title: "Basic List Item",
                ExampleBox {
                    ListGroup { title: Some("Items".to_string()),
                        ListItem { title: "First Item".to_string(), description: Some("Description text".to_string()), variant: ListItemVariant::Default, dense: false, divider: true, hoverable: true }
                        ListItem { title: "Second Item".to_string(), description: Some("Another description".to_string()), variant: ListItemVariant::Selected, dense: false, divider: true, hoverable: true }
                        ListItem { title: "Third Item".to_string(), trailing: Some(rsx! { span { "→" } }), variant: ListItemVariant::Default, dense: false, divider: true, hoverable: true }
                    }
                }
            }
            
            Section { title: "With Leading Icon",
                ExampleBox {
                    ListItem {
                        title: "Settings".to_string(),
                        leading: Some(rsx! { span { "⚙️" } }),
                        trailing: Some(rsx! { span { "→" } }),
                        variant: ListItemVariant::Default, dense: false, divider: true, hoverable: true
                    }
                }
            }
            
            Section { title: "Action List Items",
                ExampleBox {
                    ActionListItem { label: "Edit".to_string(), icon: Some("✏️".to_string()), on_click: EventHandler::new(move |_| {}), description: None, shortcut: None, destructive: false, disabled: false }
                    ActionListItem { label: "Delete".to_string(), icon: Some("🗑️".to_string()), destructive: true, on_click: EventHandler::new(move |_| {}), description: None, shortcut: None, disabled: false }
                }
            }
        }
    }
}
