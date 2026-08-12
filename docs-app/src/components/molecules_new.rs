//! New Molecule component documentation pages (Phase 1-4)

use crate::docs_ui::{CodeBlock, DocPage, ExampleBox, PropsTable, Section};
use dioxus::prelude::*;
use dioxus_ui_system::atoms::{Box, HStack, SpacingSize, VStack};
use dioxus_ui_system::molecules::{
    use_sonner, Collapsible, Command, CommandEmpty, CommandGroup, CommandInput, CommandItem,
    CommandList, ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuSeparator,
    ContextMenuTrigger, HoverCard, HoverCardContent, HoverCardHeader, MultiSelect, OtpInput,
    QrCode, Sheet, SheetSide, Sonner, TimePicker, ToastPosition, ToggleGroup, ToggleGroupType,
    ToggleItem,
};
use dioxus_ui_system::prelude::*;

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

            Section { title: "Props",
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "CommandProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Child elements"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("on_select", "Option<EventHandler<()>>", "Callback when an item is selected"),
                    ("id", "Option<String>", "Element id"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "CommandInputProps" }
                PropsTable { props: vec![
                    ("placeholder", "Option<String>", "Placeholder text"),
                    ("value", "String", "Current value"),
                    ("on_value_change", "EventHandler<String>", "Callback when value changes"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "CommandListProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Child elements"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "CommandGroupProps" }
                PropsTable { props: vec![
                    ("heading", "Option<String>", "Group heading text"),
                    ("children", "Element", "Child command items"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "CommandItemProps" }
                PropsTable { props: vec![
                    ("value", "String", "Search value for filtering"),
                    ("on_select", "EventHandler<()>", "Callback when item is selected"),
                    ("children", "Element", "Child elements"),
                    ("disabled", "bool", "Whether the item is disabled"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "CommandEmptyProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Content to display when no results"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "CommandShortcutProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Keyboard shortcut text"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
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
                                Input { value: "".to_string(), on_change: move |_| {} }
                            }
                            Box {
                                Label { "Email" }
                                Input { value: "".to_string(), on_change: move |_| {} }
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

            Section { title: "Props",
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "SheetProps" }
                PropsTable { props: vec![
                    ("open", "bool", "Whether the sheet is open"),
                    ("on_open_change", "EventHandler<bool>", "Callback when open state changes"),
                    ("side", "SheetSide", "Which edge to slide in from (Top, Right, Bottom, Left)"),
                    ("children", "Element", "Sheet content"),
                    ("title", "Option<String>", "Sheet title"),
                    ("description", "Option<String>", "Sheet description"),
                    ("show_close_button", "bool", "Whether to show the close button (default: true)"),
                    ("close_on_overlay_click", "bool", "Whether clicking the overlay closes the sheet (default: true)"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "SheetFooterProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Footer content (usually buttons)"),
                    ("align", "SheetFooterAlign", "Align content (Start, Center, End, Between)"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
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

            Section { title: "Props",
                PropsTable { props: vec![
                    ("options", "Vec<SelectOption>", "Available options"),
                    ("value", "Vec<String>", "Currently selected values"),
                    ("on_change", "EventHandler<Vec<String>>", "Callback when selection changes"),
                    ("placeholder", "Option<String>", "Placeholder text"),
                    ("disabled", "bool", "Whether the select is disabled"),
                    ("max_selected", "Option<usize>", "Maximum number of selections allowed"),
                    ("creatable", "bool", "Allow creating new options"),
                    ("searchable", "bool", "Enable search/filter functionality (default: true)"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("id", "Option<String>", "Element id"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
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

            Section { title: "Props",
                PropsTable { props: vec![
                    ("length", "usize", "Number of input boxes (default: 6)"),
                    ("value", "String", "Current value"),
                    ("on_change", "EventHandler<String>", "Change handler"),
                    ("disabled", "bool", "Disabled state"),
                    ("error", "bool", "Error state styling"),
                    ("mask", "bool", "Mask input (show • instead of numbers)"),
                    ("auto_focus", "bool", "Auto-focus first input on mount"),
                    ("style", "Option<String>", "Custom inline styles for the container"),
                    ("class", "Option<String>", "Custom class name"),
                    ("on_complete", "Option<EventHandler<String>>", "Callback when input is completed"),
                    ("id", "Option<String>", "Element id"),
                ]}
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

            Section { title: "Props",
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "TimePickerProps" }
                PropsTable { props: vec![
                    ("value", "Option<String>", "Current time value (HH:MM or HH:MM:SS)"),
                    ("on_change", "EventHandler<Option<String>>", "Change handler"),
                    ("use_24h", "bool", "Use 24-hour format (default: true)"),
                    ("show_seconds", "bool", "Show seconds column"),
                    ("disabled", "bool", "Disabled state"),
                    ("minute_step", "u32", "Minute step interval (default: 1)"),
                    ("placeholder", "Option<String>", "Placeholder text"),
                    ("label", "Option<String>", "Label text"),
                    ("error", "Option<String>", "Error message"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("id", "Option<String>", "Element id"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "TimeInputProps" }
                PropsTable { props: vec![
                    ("value", "Option<String>", "Current time value"),
                    ("on_change", "EventHandler<Option<String>>", "Change handler"),
                    ("use_24h", "bool", "Use 24-hour format (default: true)"),
                    ("show_seconds", "bool", "Show seconds"),
                    ("disabled", "bool", "Disabled state"),
                    ("placeholder", "Option<String>", "Placeholder text"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("id", "Option<String>", "Element id"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
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

            Section { title: "Props",
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "ContextMenuProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Child elements (trigger and content)"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "ContextMenuTriggerProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Child element that triggers the menu on right-click"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "ContextMenuContentProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Menu items and content"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "ContextMenuItemProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Item content"),
                    ("on_click", "Option<EventHandler<()>>", "Callback when item is clicked"),
                    ("disabled", "bool", "Whether the item is disabled"),
                    ("shortcut", "Option<String>", "Optional keyboard shortcut display"),
                    ("icon", "Option<Element>", "Optional icon element"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "ContextMenuCheckboxItemProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Item label"),
                    ("checked", "bool", "Whether the checkbox is checked"),
                    ("on_checked_change", "Option<EventHandler<bool>>", "Callback when checked state changes"),
                    ("disabled", "bool", "Whether the item is disabled"),
                    ("shortcut", "Option<String>", "Optional keyboard shortcut display"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "ContextMenuSubProps" }
                PropsTable { props: vec![
                    ("trigger", "Element", "Trigger element that opens the submenu"),
                    ("children", "Element", "Submenu content"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "ContextMenuSubTriggerProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Trigger content"),
                    ("disabled", "bool", "Whether the item is disabled"),
                    ("shortcut", "Option<String>", "Optional keyboard shortcut display"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "ContextMenuLabelProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Label text content"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
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

            Section { title: "Props",
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "HoverCardProps" }
                PropsTable { props: vec![
                    ("trigger", "Element", "Trigger element"),
                    ("children", "Element", "Card content"),
                    ("open_delay", "u64", "Delay before showing the card in ms (default: 200)"),
                    ("close_delay", "u64", "Delay before hiding the card in ms (default: 100)"),
                    ("side", "Side", "Card placement side (Top, Right, Bottom, Left)"),
                    ("align", "Align", "Card alignment (Start, Center, End)"),
                    ("style", "Option<String>", "Custom inline styles for the card"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "HoverCardHeaderProps" }
                PropsTable { props: vec![
                    ("title", "String", "Header title"),
                    ("description", "Option<String>", "Optional description"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "HoverCardContentProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Content to display"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "HoverCardFooterProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Footer content"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "HoverCardAvatarProps" }
                PropsTable { props: vec![
                    ("src", "String", "Avatar image URL"),
                    ("alt", "Option<String>", "Avatar alt text"),
                    ("size", "u16", "Avatar size in pixels (default: 40)"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "HoverCardUserInfoProps" }
                PropsTable { props: vec![
                    ("name", "String", "User name"),
                    ("handle", "String", "User handle/username"),
                    ("avatar_url", "Option<String>", "Optional avatar URL"),
                    ("bio", "Option<String>", "Optional bio/description"),
                    ("stats", "Option<String>", "Optional stats"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
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

            Section { title: "Props",
                PropsTable { props: vec![
                    ("toasts", "Vec<Toast>", "Array of active toasts"),
                    ("position", "ToastPosition", "Position on screen"),
                    ("rich_colors", "bool", "Rich colors for variants (default: true)"),
                    ("close_button", "bool", "Show close button (default: true)"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("on_dismiss", "Option<EventHandler<String>>", "Callback when toast is dismissed"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                ]}
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

            Section { title: "Props",
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "QrCodeProps" }
                PropsTable { props: vec![
                    ("value", "String", "Value to encode"),
                    ("size", "u32", "Size in pixels (default: 200)"),
                    ("level", "QrCodeLevel", "Error correction level"),
                    ("fg_color", "Option<String>", "Foreground color (default black)"),
                    ("bg_color", "Option<String>", "Background color (default white)"),
                    ("include_margin", "bool", "Include margin/quiet zone (default: true)"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("class", "Option<String>", "Custom class name"),
                    ("title", "Option<String>", "Title/alt text for accessibility"),
                    ("id", "Option<String>", "Element id"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "QrCodeSvgProps" }
                PropsTable { props: vec![
                    ("value", "String", "Value to encode"),
                    ("size", "u32", "Size in pixels (default: 200)"),
                    ("level", "QrCodeLevel", "Error correction level"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("class", "Option<String>", "Custom class name"),
                    ("id", "Option<String>", "Element id"),
                ]}
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

            Section { title: "Props",
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "CollapsibleProps" }
                PropsTable { props: vec![
                    ("open", "Option<bool>", "Whether the collapsible is open (controlled mode)"),
                    ("on_open_change", "Option<EventHandler<bool>>", "Callback when open state changes"),
                    ("trigger", "Element", "The clickable header/trigger element"),
                    ("children", "Element", "The collapsible content"),
                    ("default_open", "bool", "Default open state for uncontrolled mode"),
                    ("show_chevron", "bool", "Whether to show the default chevron icon (default: true)"),
                    ("chevron", "Option<Element>", "Custom chevron element"),
                    ("style", "Option<String>", "Custom inline styles for the container"),
                    ("trigger_style", "Option<String>", "Custom inline styles for the trigger"),
                    ("content_style", "Option<String>", "Custom inline styles for the content"),
                    ("disabled", "bool", "Whether the collapsible is disabled"),
                    ("transition_duration", "u16", "Transition duration in ms (default: 200)"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "CollapsibleGroupProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Child collapsibles or content"),
                    ("allow_multiple", "bool", "Allow multiple items to be open simultaneously"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("gap", "Option<String>", "Gap between collapsible items"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "SimpleCollapsibleProps" }
                PropsTable { props: vec![
                    ("trigger_text", "String", "Trigger text"),
                    ("children", "Element", "The collapsible content"),
                    ("open", "Option<bool>", "Whether the collapsible is open (controlled mode)"),
                    ("on_open_change", "Option<EventHandler<bool>>", "Callback when open state changes"),
                    ("default_open", "bool", "Default open state for uncontrolled mode"),
                    ("disabled", "bool", "Whether the collapsible is disabled"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
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

            Section { title: "Props",
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "ToggleGroupProps" }
                PropsTable { props: vec![
                    ("group_type", "ToggleGroupType", "Selection type - Single or Multiple"),
                    ("value", "Vec<String>", "Currently selected values"),
                    ("on_value_change", "Option<EventHandler<Vec<String>>>", "Callback when selection changes"),
                    ("children", "Element", "Toggle group children"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("class", "Option<String>", "Custom class name"),
                    ("id", "Option<String>", "Element id"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "ToggleItemProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Toggle item content"),
                    ("value", "String", "Unique value for this toggle item"),
                    ("disabled", "bool", "Whether this item is disabled"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("class", "Option<String>", "Custom class name"),
                    ("id", "Option<String>", "Element id"),
                ]}
            }
        }
    }
}

/// Breadcrumb documentation page
#[component]
pub fn BreadcrumbPage() -> Element {
    let items = vec![
        BreadcrumbItem::new("Home", "/"),
        BreadcrumbItem::new("Products", "/products"),
        BreadcrumbItem::new("Electronics", "/products/electronics"),
        BreadcrumbItem::current("Laptops"),
    ];

    rsx! {
        DocPage {
            title: "Breadcrumb",
            description: "Displays the path to the current resource using a hierarchy of links.",

            Section { title: "Basic",
                ExampleBox {
                    Breadcrumb {
                        items: items.clone(),
                    }
                }
                CodeBlock { code: r#"let items = vec![
    BreadcrumbItem::new("Home", "/"),
    BreadcrumbItem::new("Products", "/products"),
    BreadcrumbItem::current("Laptops"),
];

Breadcrumb {
    items: items,
}"#.to_string() }
            }

            Section { title: "Custom Separator",
                ExampleBox {
                    Breadcrumb {
                        items: items.clone(),
                        separator: Some(" / ".to_string()),
                    }
                }
            }

            Section { title: "With Icons",
                ExampleBox {
                    Breadcrumb {
                        items: vec![
                            BreadcrumbItem::new("Home", "/").with_icon("home"),
                            BreadcrumbItem::new("Documents", "/docs").with_icon("folder"),
                            BreadcrumbItem::current("Report.pdf"),
                        ],
                    }
                }
            }

            Section { title: "Props",
                PropsTable { props: vec![
                    ("items", "Vec<BreadcrumbItem>", "Breadcrumb items"),
                    ("separator", "Option<String>", "Custom separator (default: ›)"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("class", "Option<String>", "Custom class name"),
                    ("id", "Option<String>", "Element id"),
                ]}
            }
        }
    }
}

/// Input Group documentation page
#[component]
pub fn InputGroupPage() -> Element {
    let mut email = use_signal(|| String::new());
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());

    rsx! {
        DocPage {
            title: "Input Group",
            description: "Combines Label, Input, and helper text into a cohesive form field.",

            Section { title: "Basic",
                ExampleBox {
                    Box { style: "max-width: 300px;",
                        InputGroup {
                            label: "Email",
                            value: email(),
                            placeholder: Some("you@example.com".to_string()),
                            hint: Some("We'll never share your email.".to_string()),
                            on_change: Some(EventHandler::new(move |v: String| email.set(v))),
                        }
                    }
                }
                CodeBlock { code: r#"InputGroup {
    label: "Email",
    value: email(),
    placeholder: Some("you@example.com".to_string()),
    hint: Some("We'll never share your email.".to_string()),
    on_change: Some(EventHandler::new(move |v| email.set(v))),
}"#.to_string() }
            }

            Section { title: "With Error",
                ExampleBox {
                    Box { style: "max-width: 300px;",
                        InputGroup {
                            label: "Username",
                            value: username(),
                            placeholder: Some("johndoe".to_string()),
                            error: Some("Username is already taken.".to_string()),
                            on_change: Some(EventHandler::new(move |v: String| username.set(v))),
                        }
                    }
                }
            }

            Section { title: "Required",
                ExampleBox {
                    Box { style: "max-width: 300px;",
                        InputGroup {
                            label: "Password",
                            value: password(),
                            placeholder: Some("••••••••".to_string()),
                            input_type: InputType::Password,
                            required: true,
                            on_change: Some(EventHandler::new(move |v: String| password.set(v))),
                        }
                    }
                }
            }

            Section { title: "Props",
                PropsTable { props: vec![
                    ("label", "String", "Field label"),
                    ("value", "String", "Current input value"),
                    ("placeholder", "Option<String>", "Placeholder text"),
                    ("input_type", "InputType", "Input type"),
                    ("error", "Option<String>", "Error message (shows error state when Some)"),
                    ("hint", "Option<String>", "Help/hint text"),
                    ("required", "bool", "Required field indicator"),
                    ("disabled", "bool", "Disabled state"),
                    ("on_change", "Option<EventHandler<String>>", "Change handler"),
                    ("leading_icon", "Option<Element>", "Optional icon before input"),
                    ("trailing_icon", "Option<Element>", "Optional icon after input"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                ]}
            }
        }
    }
}

/// ScrollArea documentation page
#[component]
pub fn ScrollAreaPage() -> Element {
    rsx! {
        DocPage {
            title: "ScrollArea",
            description: "A custom scrollable container with styled scrollbars that match the theme.",

            Section { title: "Vertical Scroll",
                ExampleBox {
                    ScrollArea {
                        max_height: Some("200px".to_string()),
                        orientation: ScrollOrientation::Vertical,
                        VStack { gap: SpacingSize::Sm,
                            for i in 1..=20 {
                                Box {
                                    key: "{i}",
                                    style: "padding: 8px 12px; background: #f1f5f9; border-radius: 6px;",
                                    "Item {i}"
                                }
                            }
                        }
                    }
                }
                CodeBlock { code: r#"ScrollArea {
    max_height: Some("200px".to_string()),
    orientation: ScrollOrientation::Vertical,
    // Content that exceeds max_height will be scrollable
    for i in 1..=50 {
        p { "Item {i}" }
    }
}"#.to_string() }
            }

            Section { title: "Horizontal Scroll",
                ExampleBox {
                    ScrollArea {
                        max_height: Some("120px".to_string()),
                        orientation: ScrollOrientation::Horizontal,
                        HStack { gap: SpacingSize::Md, style: "white-space: nowrap;",
                            for i in 1..=15 {
                                Box {
                                    key: "{i}",
                                    style: "padding: 16px 24px; background: #f1f5f9; border-radius: 6px; min-width: 100px; text-align: center;",
                                    "Card {i}"
                                }
                            }
                        }
                    }
                }
            }

            Section { title: "Auto-hide Scrollbar",
                ExampleBox {
                    ScrollArea {
                        max_height: Some("150px".to_string()),
                        auto_hide: true,
                        VStack { gap: SpacingSize::Sm,
                            for i in 1..=15 {
                                Box {
                                    key: "{i}",
                                    style: "padding: 8px 12px; background: #f1f5f9; border-radius: 6px;",
                                    "Row {i}"
                                }
                            }
                        }
                    }
                }
            }

            Section { title: "Props",
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "ScrollAreaProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Content to be scrolled"),
                    ("orientation", "ScrollOrientation", "Scroll orientation (Vertical, Horizontal, Both)"),
                    ("class", "Option<String>", "Custom class name"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("scrollbar_size", "Option<String>", "Custom scrollbar size"),
                    ("max_height", "Option<String>", "Maximum height for the scroll area"),
                    ("max_width", "Option<String>", "Maximum width for the scroll area"),
                    ("auto_hide", "bool", "Whether to auto-hide scrollbar when not scrolling"),
                    ("id", "Option<String>", "Element id"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "ScrollViewportProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Content inside the viewport"),
                    ("class", "Option<String>", "Custom class name"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("id", "Option<String>", "Element id"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "HorizontalScrollProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Content to be scrolled horizontally"),
                    ("class", "Option<String>", "Custom class name"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("height", "Option<String>", "Height of the scroll area"),
                    ("show_scrollbar", "bool", "Whether to show scrollbar (default: true)"),
                    ("id", "Option<String>", "Element id"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "VerticalScrollProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Content to be scrolled vertically"),
                    ("class", "Option<String>", "Custom class name"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("max_height", "Option<String>", "Maximum height of the scroll area"),
                    ("auto_hide", "bool", "Whether to auto-hide scrollbar"),
                    ("id", "Option<String>", "Element id"),
                ]}
            }
        }
    }
}
