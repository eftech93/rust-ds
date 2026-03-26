//! New Molecule component documentation pages (Phase 1-4)

use dioxus::prelude::*;
use dioxus_ui_system::prelude::*;
use dioxus_ui_system::atoms::{Box, VStack, HStack, SpacingSize};
use dioxus_ui_system::molecules::{Command, CommandInput, CommandList, CommandGroup, CommandItem, CommandEmpty, Sheet, SheetSide, MultiSelect, OtpInput, TimePicker, ContextMenu, ContextMenuTrigger, ContextMenuContent, ContextMenuItem, ContextMenuSeparator, HoverCard, HoverCardHeader, HoverCardContent, QrCode, Collapsible, ToggleGroup, ToggleGroupType, ToggleItem, use_sonner, Sonner, ToastPosition};
use crate::docs_ui::{DocPage, Section, ExampleBox, CodeBlock, PropsTable};

/// Command documentation page
#[component]
pub fn CommandPage() -> Element {
    let mut value = use_signal(|| "".to_string());
    let mut message = use_signal(|| "Select a command...".to_string());
    
    rsx! {
        DocPage {
            title: "Command",
            description: "A command palette for quick navigation and actions.",
            
            Section { title: "Basic",
                p { "{message}" }
                ExampleBox {
                    Box { style: "max-width: 400px; border: 1px solid #e5e7eb; border-radius: 8px;",
                        Command {
                            on_select: move |_| message.set("Command selected!".to_string()),
                            CommandInput {
                                placeholder: "Search commands...",
                                value: value(),
                                on_value_change: move |v| value.set(v),
                            }
                            CommandList {
                                CommandEmpty { "No results found." }
                                CommandGroup {
                                    heading: "Suggestions",
                                    CommandItem {
                                        value: "calendar",
                                        on_select: move |_| message.set("Calendar selected".to_string()),
                                        "Calendar"
                                    }
                                    CommandItem {
                                        value: "search",
                                        on_select: move |_| message.set("Search selected".to_string()),
                                        "Search"
                                    }
                                    CommandItem {
                                        value: "settings",
                                        on_select: move |_| message.set("Settings selected".to_string()),
                                        "Settings"
                                    }
                                }
                            }
                        }
                    }
                }
                CodeBlock { code: r#"let mut message = use_signal(|| "Select a command...".to_string());

Command {
    on_select: move |_| message.set("Command selected!".to_string()),
    CommandInput {
        placeholder: "Search...",
        value: value(),
        on_value_change: move |v| value.set(v),
    }
    CommandList {
        CommandGroup {
            heading: "Suggestions",
            CommandItem { 
                value: "calendar", 
                on_select: move |_| message.set("Calendar selected".to_string()),
                "Calendar" 
            }
        }
    }
}"#.to_string() }
            }
        }
    }
}

/// Sheet documentation page
#[component]
pub fn SheetPage() -> Element {
    let mut open = use_signal(|| false);
    let mut open_left = use_signal(|| false);
    
    rsx! {
        DocPage {
            title: "Sheet",
            description: "A side panel that slides in from any edge of the screen.",
            
            Section { title: "Basic (Right Side)",
                ExampleBox {
                    Button { onclick: move |_| open.set(true), "Open Sheet" }
                    
                    Sheet {
                        open: open(),
                        on_open_change: move |o| open.set(o),
                        title: "Edit Profile",
                        description: Some("Make changes to your profile here.".to_string()),
                        VStack { gap: SpacingSize::Md,
                            Box {
                                Label { "Name" }
                                Input { value: "".to_string(), onchange: move |_| {} }
                            }
                            Box {
                                Label { "Email" }
                                Input { value: "".to_string(), onchange: move |_| {} }
                            }
                        }
                    }
                }
                CodeBlock { code: r#"let mut open = use_signal(|| false);

Button { onclick: move |_| open.set(true), "Open Sheet" }

Sheet {
    open: open(),
    on_open_change: move |o| open.set(o),
    title: "Edit Profile",
    description: Some("Description...".to_string()),
    // Content
}"#.to_string() }
            }
            
            Section { title: "Left Side",
                ExampleBox {
                    Button { onclick: move |_| open_left.set(true), "Open Left Sheet" }
                    
                    Sheet {
                        open: open_left(),
                        on_open_change: move |o| open_left.set(o),
                        side: SheetSide::Left,
                        title: "Navigation",
                        VStack { gap: SpacingSize::Md,
                            Button { variant: ButtonVariant::Ghost, "Home" }
                            Button { variant: ButtonVariant::Ghost, "Profile" }
                            Button { variant: ButtonVariant::Ghost, "Settings" }
                        }
                    }
                }
            }
        }
    }
}

/// MultiSelect documentation page
#[component]
pub fn MultiSelectPage() -> Element {
    let mut selected = use_signal(|| vec!["react".to_string()]);
    
    let options = vec![
        SelectOption::new("react", "React"),
        SelectOption::new("vue", "Vue"),
        SelectOption::new("angular", "Angular"),
        SelectOption::new("svelte", "Svelte"),
        SelectOption::new("solid", "Solid"),
    ];
    
    rsx! {
        DocPage {
            title: "MultiSelect",
            description: "A dropdown that allows selecting multiple items with tag display.",
            
            Section { title: "Basic",
                ExampleBox {
                    Box { style: "max-width: 300px;",
                        MultiSelect {
                            options: options.clone(),
                            value: selected(),
                            on_change: move |v| selected.set(v),
                            placeholder: "Select frameworks...",
                        }
                    }
                }
                CodeBlock { code: r#"let options = vec![
    SelectOption::new("react", "React"),
    SelectOption::new("vue", "Vue"),
];

MultiSelect {
    options: options,
    value: selected(),
    on_change: move |v| selected.set(v),
}"#.to_string() }
            }
            
            Section { title: "Creatable",
                ExampleBox {
                    Box { style: "max-width: 300px;",
                        MultiSelect {
                            options: options.clone(),
                            value: vec![],
                            on_change: move |_| {},
                            creatable: true,
                            placeholder: "Add tags...",
                        }
                    }
                }
            }
        }
    }
}

/// OTP Input documentation page
#[component]
pub fn OtpInputPage() -> Element {
    rsx! {
        DocPage {
            title: "OTP Input",
            description: "A one-time password input with individual digit boxes.",
            
            Section { title: "Basic (6 digits)",
                ExampleBox {
                    OtpInput {
                        length: 6,
                        value: "123456".to_string(),
                        on_change: move |_| {},
                    }
                }
                CodeBlock { code: r#"OtpInput {
    length: 6,
    value: otp(),
    on_change: move |v| otp.set(v),
}"#.to_string() }
            }
            
            Section { title: "Masked",
                ExampleBox {
                    OtpInput {
                        length: 6,
                        value: "123456".to_string(),
                        on_change: move |_| {},
                        mask: true,
                    }
                }
            }
            
            Section { title: "Error State",
                ExampleBox {
                    OtpInput {
                        length: 6,
                        value: "123".to_string(),
                        on_change: move |_| {},
                        error: true,
                    }
                }
            }
        }
    }
}

/// Time Picker documentation page
#[component]
pub fn TimePickerPage() -> Element {
    rsx! {
        DocPage {
            title: "Time Picker",
            description: "A time selection component with hours, minutes, and optional seconds.",
            
            Section { title: "24-Hour Format",
                ExampleBox {
                    Box { style: "max-width: 200px;",
                        TimePicker {
                            value: Some("14:30".to_string()),
                            on_change: move |_| {},
                            use_24h: true,
                        }
                    }
                }
                CodeBlock { code: r#"TimePicker {
    value: Some("14:30".to_string()),
    on_change: move |v| time.set(v),
    use_24h: true,
}"#.to_string() }
            }
            
            Section { title: "12-Hour Format (AM/PM)",
                ExampleBox {
                    Box { style: "max-width: 200px;",
                        TimePicker {
                            value: Some("02:30".to_string()),
                            on_change: move |_| {},
                            use_24h: false,
                        }
                    }
                }
            }
            
            Section { title: "With Seconds",
                ExampleBox {
                    Box { style: "max-width: 250px;",
                        TimePicker {
                            value: Some("14:30:45".to_string()),
                            on_change: move |_| {},
                            show_seconds: true,
                        }
                    }
                }
            }
        }
    }
}

/// Context Menu documentation page
#[component]
pub fn ContextMenuPage() -> Element {
    rsx! {
        DocPage {
            title: "Context Menu",
            description: "A right-click context menu for additional actions.",
            
            Section { title: "Basic",
                ExampleBox {
                    ContextMenu {
                        ContextMenuTrigger {
                            Box { 
                                style: "padding: 40px; background: #f3f4f6; border-radius: 8px; text-align: center; cursor: context-menu; user-select: none;",
                                "Right-click here"
                            }
                        }
                        ContextMenuContent {
                            ContextMenuItem { on_click: move |_| {}, "Cut" }
                            ContextMenuItem { on_click: move |_| {}, "Copy" }
                            ContextMenuItem { on_click: move |_| {}, "Paste" }
                            ContextMenuSeparator {}
                            ContextMenuItem { 
                                on_click: move |_| {}, 
                                shortcut: Some("⌘D".to_string()),
                                "Duplicate"
                            }
                        }
                    }
                }
                CodeBlock { code: r#"ContextMenu {
    ContextMenuTrigger {
        div { "Right-click me" }
    }
    ContextMenuContent {
        ContextMenuItem { on_click: move |_| {}, "Cut" }
        ContextMenuItem { on_click: move |_| {}, "Copy" }
        ContextMenuItem { on_click: move |_| {}, "Paste" }
        ContextMenuSeparator {}
        ContextMenuItem { 
            on_click: move |_| {}, 
            shortcut: Some("⌘D".to_string()),
            "Duplicate"
        }
    }
}"#.to_string() }
            }
        }
    }
}

/// Hover Card documentation page
#[component]
pub fn HoverCardPage() -> Element {
    rsx! {
        DocPage {
            title: "Hover Card",
            description: "A card that appears when hovering over a trigger element.",
            
            Section { title: "Basic",
                ExampleBox {
                    Box { style: "padding: 40px;",
                        HoverCard {
                            trigger: rsx! { 
                                span { style: "color: #3b82f6; cursor: pointer; text-decoration: underline; font-weight: 500;", "@username" }
                            },
                            HoverCardHeader {
                                title: "John Doe".to_string(),
                                description: Some("Software Engineer @ Company".to_string()),
                            }
                            HoverCardContent {
                                VStack { gap: SpacingSize::Sm,
                                    p { style: "margin: 0; font-size: 14px; color: #6b7280;", 
                                        "Passionate about building great user experiences with Rust and Dioxus."
                                    }
                                    HStack { gap: SpacingSize::Md,
                                        span { style: "font-size: 12px; color: #6b7280;", "📍 San Francisco" }
                                        span { style: "font-size: 12px; color: #6b7280;", "🔗 github.com/johndoe" }
                                    }
                                }
                            }
                        }
                    }
                }
                CodeBlock { code: r#"HoverCard {
    trigger: rsx! { 
        span { "@username" }
    },
    HoverCardHeader {
        title: "John Doe".to_string(),
        description: Some("Software Engineer".to_string()),
    }
    HoverCardContent {
        "User profile information..."
    }
}"#.to_string() }
            }
        }
    }
}

/// Sonner (Toast) documentation page
#[component]
pub fn SonnerPage() -> Element {
    let mut sonner = use_sonner();
    let toasts_sig = sonner.toasts_signal();
    
    // Read the signal to trigger re-renders
    let toasts_list = toasts_sig();
    
    rsx! {
        DocPage {
            title: "Sonner",
            description: "Modern toast notifications with rich styling and progress bars.",
            
            Section { title: "Basic",
                ExampleBox {
                    HStack { gap: SpacingSize::Md, style: "flex-wrap: wrap;",
                        Button { 
                            onclick: move |_| { sonner.toast("Hello World!"); }, 
                            "Show Toast" 
                        }
                        Button { 
                            variant: ButtonVariant::Secondary, 
                            onclick: move |_| { sonner.success("Operation completed!"); }, 
                            "Success" 
                        }
                        Button { 
                            variant: ButtonVariant::Destructive, 
                            onclick: move |_| { sonner.error("Something went wrong"); }, 
                            "Error" 
                        }
                    }
                }
                CodeBlock { code: r#"let mut sonner = use_sonner();

// Show different variants
sonner.toast("Hello World");
sonner.success("Operation completed!");
sonner.error("Something went wrong");

// Render the Sonner component
Sonner {
    toasts: sonner.toasts(),
}"#.to_string() }
            }
            
            Section { title: "Positions",
                p { "Sonner supports multiple positions: BottomRight, BottomCenter, BottomLeft, TopRight, TopCenter, TopLeft" }
            }
        }
        
        // Sonner is rendered at the page level, outside of any containers
        Sonner {
            toasts: toasts_list.clone(),
            position: ToastPosition::BottomRight,
            on_dismiss: move |id: String| sonner.dismiss(&id),
        }
    }
}

/// QR Code documentation page
#[component]
pub fn QrCodePage() -> Element {
    rsx! {
        DocPage {
            title: "QR Code",
            description: "A QR code generator and display component.",
            
            Section { title: "Basic",
                ExampleBox {
                    HStack { gap: SpacingSize::Lg, style: "justify-content: center;",
                        QrCode {
                            value: "https://example.com".to_string(),
                            size: 150,
                        }
                    }
                }
                CodeBlock { code: r#"QrCode {
    value: "https://example.com".to_string(),
    size: 200,
    level: QrCodeLevel::Medium,
}"#.to_string() }
            }
            
            Section { title: "Error Correction Levels",
                p { "Low (~7%), Medium (~15%), Quartile (~25%), High (~30%)" }
            }
        }
    }
}

/// Collapsible documentation page
#[component]
pub fn CollapsiblePage() -> Element {
    rsx! {
        DocPage {
            title: "Collapsible",
            description: "A component that shows or hides content with smooth animation.",
            
            Section { title: "Basic",
                ExampleBox {
                    Collapsible {
                        trigger: rsx! { "Click to expand/collapse" },
                        "This content is hidden by default and revealed when you click the trigger. You can put any content here including text, images, or other components."
                    }
                }
                CodeBlock { code: r#"Collapsible {
    trigger: rsx! { "Click to expand" },
    "Hidden content revealed!"
}"#.to_string() }
            }
            
            Section { title: "Default Open",
                ExampleBox {
                    Collapsible {
                        default_open: true,
                        trigger: rsx! { "Initially expanded" },
                        "This content is visible by default because default_open is set to true."
                    }
                }
            }
        }
    }
}

/// Toggle Group documentation page
#[component]
pub fn ToggleGroupPage() -> Element {
    let mut single_value = use_signal(|| vec!["bold".to_string()]);
    let mut multi_value = use_signal(|| vec!["bold".to_string(), "italic".to_string()]);
    
    rsx! {
        DocPage {
            title: "Toggle Group",
            description: "A group of toggle buttons for single or multiple selection.",
            
            Section { title: "Single Selection",
                ExampleBox {
                    ToggleGroup {
                        group_type: ToggleGroupType::Single,
                        value: single_value(),
                        on_value_change: move |v| single_value.set(v),
                        ToggleItem { value: "left", "Left" }
                        ToggleItem { value: "center", "Center" }
                        ToggleItem { value: "right", "Right" }
                    }
                }
                CodeBlock { code: r#"ToggleGroup {
    group_type: ToggleGroupType::Single,
    value: selected(),
    on_value_change: move |v| selected.set(v),
    ToggleItem { value: "left", "Left" }
    ToggleItem { value: "center", "Center" }
    ToggleItem { value: "right", "Right" }
}"#.to_string() }
            }
            
            Section { title: "Multiple Selection",
                ExampleBox {
                    ToggleGroup {
                        group_type: ToggleGroupType::Multiple,
                        value: multi_value(),
                        on_value_change: move |v| multi_value.set(v),
                        ToggleItem { value: "bold", "B" }
                        ToggleItem { value: "italic", "I" }
                        ToggleItem { value: "underline", "U" }
                    }
                }
            }
        }
    }
}
