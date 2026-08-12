//! Molecule component documentation pages

use crate::docs_ui::{CodeBlock, DocPage, ExampleBox, PropsTable, Section};
use dioxus::prelude::*;
use dioxus_ui_system::atoms::{Box, HStack, VStack};
use dioxus_ui_system::molecules::{
    use_toast, ActionListItem, Combobox, ComboboxOption, Comment, DialogFooter, DialogFooterAlign,
    ListGroup, ListItem, ListItemVariant, MediaContent, MediaObject, Pagination, PaginationInfo,
    SkeletonCircle, SkeletonMolecule as Skeleton,
};
use dioxus_ui_system::prelude::*;

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
                    li { "Command - Command palette (New!)" }
                    li { "Sheet - Side panels/drawers (New!)" }
                    li { "MultiSelect - Multiple selection (New!)" }
                    li { "OtpInput - OTP code input (New!)" }
                    li { "TimePicker - Time selection (New!)" }
                    li { "ContextMenu - Right-click menu (New!)" }
                    li { "HoverCard - Hover preview cards (New!)" }
                    li { "Sonner - Modern toast notifications (New!)" }
                    li { "QRCode - QR code display (New!)" }
                    li { "Collapsible - Expand/collapse content (New!)" }
                    li { "ToggleGroup - Group of toggles (New!)" }
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

            Section { title: "Usage",
                CodeBlock { code: "Card {{
    variant: CardVariant::Default,
    padding: Some(\"16px\".to_string()),
    CardHeader {{
        title: \"Card Title\",
        subtitle: Some(\"Subtitle\".to_string()),
    }}
    CardContent {{ \"Content here\" }}
}}".to_string() }
            }

            Section { title: "Props",
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "CardProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Card content"),
                    ("variant", "CardVariant", "Visual variant (Default, Muted, Elevated, Outlined, Ghost)"),
                    ("padding", "Option<String>", "Custom padding size"),
                    ("full_width", "bool", "Full width"),
                    ("onclick", "Option<EventHandler<MouseEvent>>", "Click handler (makes card interactive)"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("overflow_hidden", "bool", "Whether to hide overflow (default: true)"),
                    ("id", "Option<String>", "Element id"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "CardHeaderProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Header content"),
                    ("title", "Option<String>", "Optional title (convenience prop)"),
                    ("subtitle", "Option<String>", "Optional subtitle"),
                    ("action", "Option<Element>", "Optional action element"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "CardContentProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Content"),
                    ("padding", "Option<String>", "Custom padding"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "CardFooterProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Footer content"),
                    ("justify", "CardFooterJustify", "Justify content: Start, Center, End, Between"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
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
                CodeBlock { code: "Badge {{
    variant: BadgeVariant::Success,
    icon: Some(\"check\".to_string()),
    \"Success\"
}}".to_string() }
            }

            Section { title: "Props",
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "BadgeProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Badge content"),
                    ("variant", "BadgeVariant", "Visual variant"),
                    ("size", "BadgeSize", "Badge size (Sm, Md, Lg)"),
                    ("icon", "Option<String>", "Optional leading icon"),
                    ("onclick", "Option<EventHandler<MouseEvent>>", "Click handler (makes badge interactive)"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("class", "Option<String>", "Custom class name"),
                    ("id", "Option<String>", "Element id"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "StatusBadgeProps" }
                PropsTable { props: vec![
                    ("status", "String", "Status text"),
                    ("status_type", "StatusType", "Status type (Default, Info, Success, Warning, Error)"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
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
                CodeBlock { code: "Alert {{
    variant: AlertVariant::Success,
    title: Some(\"Success\".to_string()),
    icon: Some(\"check-circle\".to_string()),
    \"Operation completed!\"
}}".to_string() }
            }

            Section { title: "Props",
                PropsTable { props: vec![
                    ("children", "Element", "Alert content"),
                    ("variant", "AlertVariant", "Alert variant (Default, Destructive, Success, Warning, Info)"),
                    ("title", "Option<String>", "Optional title"),
                    ("icon", "Option<String>", "Optional icon name"),
                    ("dismissible", "bool", "Whether alert is dismissible"),
                    ("on_dismiss", "Option<EventHandler<()>>", "Callback when dismissed"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("class", "Option<String>", "Custom class name"),
                    ("id", "Option<String>", "Element id"),
                ]}
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
                CodeBlock { code: "Avatar {{
    size: AvatarSize::Md,
    name: Some(\"JD\".to_string()),
    src: Some(\"avatar.png\".to_string()),
    alt: \"User avatar\".to_string(),
}}".to_string() }
            }

            Section { title: "Props",
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "AvatarProps" }
                PropsTable { props: vec![
                    ("src", "Option<String>", "Image source URL"),
                    ("alt", "String", "Alt text for the image"),
                    ("name", "Option<String>", "User name (used for fallback initials)"),
                    ("size", "AvatarSize", "Avatar size (Xs, Sm, Md, Lg, Xl)"),
                    ("fallback", "Option<Element>", "Custom fallback content"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("class", "Option<String>", "Custom class name"),
                    ("id", "Option<String>", "Element id"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "AvatarGroupProps" }
                PropsTable { props: vec![
                    ("max", "usize", "Maximum number of avatars to show (default: 4)"),
                    ("size", "AvatarSize", "Size of all avatars in the group"),
                    ("children", "Element", "Avatar children"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
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
                CodeBlock { code: "let mut open = use_signal(|| false);

Dialog {{
    open: open(),
    on_close: move |_| open.set(false),
    title: Some(\"Dialog Title\".to_string()),
    description: Some(\"Description\".to_string()),
    // Content here
    DialogFooter {{
        align: DialogFooterAlign::End,
        Button {{ onclick: move |_| open.set(false), \"Cancel\" }}
        Button {{ variant: ButtonVariant::Primary, onclick: move |_| open.set(false), \"Confirm\" }}
    }}
}}".to_string() }
            }

            Section { title: "Props",
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "DialogProps" }
                PropsTable { props: vec![
                    ("open", "bool", "Whether the dialog is open"),
                    ("on_close", "EventHandler<()>", "Callback when dialog should close"),
                    ("children", "Element", "Dialog content"),
                    ("title", "Option<String>", "Dialog title"),
                    ("description", "Option<String>", "Dialog description"),
                    ("show_close_button", "bool", "Whether to show close button (default: true)"),
                    ("close_on_overlay_click", "bool", "Whether clicking overlay closes the dialog (default: true)"),
                    ("content_style", "Option<String>", "Custom inline styles for the content"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "DialogFooterProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Footer content (usually buttons)"),
                    ("align", "DialogFooterAlign", "Align content (Start, Center, End, Between)"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "AlertDialogProps" }
                PropsTable { props: vec![
                    ("open", "bool", "Whether the dialog is open"),
                    ("on_close", "EventHandler<()>", "Callback when dialog should close"),
                    ("title", "String", "Dialog title"),
                    ("description", "String", "Dialog description"),
                    ("cancel_text", "String", "Cancel button text (default: Cancel)"),
                    ("confirm_text", "String", "Confirm button text (default: Confirm)"),
                    ("on_confirm", "EventHandler<()>", "Callback when confirmed"),
                    ("destructive", "bool", "Whether the action is destructive"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
            }
        }
    }
}

#[component]
pub fn DropdownPage() -> Element {
    use dioxus_ui_system::molecules::{DropdownAlign, DropdownMenu, DropdownMenuItem};

    let items = vec![
        DropdownMenuItem::new("profile", "Profile").with_icon("user"),
        DropdownMenuItem::new("settings", "Settings").with_icon("settings"),
        DropdownMenuItem::new("logout", "Logout").with_icon("log-out"),
    ];

    let items_with_actions = vec![
        DropdownMenuItem::new("copy", "Copy")
            .with_icon("copy")
            .with_shortcut("⌘C"),
        DropdownMenuItem::new("cut", "Cut")
            .with_icon("scissors")
            .with_shortcut("⌘X"),
        DropdownMenuItem::new("paste", "Paste")
            .with_icon("clipboard")
            .with_shortcut("⌘V"),
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

            Section { title: "Props",
                PropsTable { props: vec![
                    ("trigger", "Element", "Trigger element"),
                    ("items", "Vec<DropdownMenuItem>", "Menu items"),
                    ("on_select", "EventHandler<String>", "Callback when an item is selected"),
                    ("align", "DropdownAlign", "Menu alignment (Start, End, Center)"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                ]}
            }
        }
    }
}

#[component]
pub fn TooltipPage() -> Element {
    use dioxus_ui_system::molecules::{SimpleTooltip, Tooltip, TooltipPlacement};

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

            Section { title: "Props",
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "TooltipProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Trigger element"),
                    ("content", "String", "Tooltip content"),
                    ("placement", "TooltipPlacement", "Tooltip placement"),
                    ("delay", "u64", "Delay before showing tooltip in ms (default: 200)"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "SimpleTooltipProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Element to wrap"),
                    ("text", "String", "Tooltip text"),
                    ("placement", "TooltipPlacement", "Tooltip placement"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
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
                CodeBlock { code: "Separator {{}}".to_string() }
            }

            Section { title: "Props",
                PropsTable { props: vec![
                    ("orientation", "SeparatorOrientation", "Separator orientation (Horizontal, Vertical)"),
                    ("decorative", "bool", "Whether the separator is decorative only (default: true)"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("class", "Option<String>", "Custom class name"),
                    ("id", "Option<String>", "Element id"),
                ]}
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
                CodeBlock { code: "Skeleton {{
    width: Some(\"150px\".to_string()),
    height: None,
    animate: true,
}}".to_string() }
            }

            Section { title: "Props",
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "SkeletonProps" }
                PropsTable { props: vec![
                    ("width", "Option<String>", "Width of the skeleton"),
                    ("height", "Option<String>", "Height of the skeleton"),
                    ("animate", "bool", "Whether to show animation (default: true)"),
                    ("rounded", "Option<String>", "Border radius"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("class", "Option<String>", "Custom class name"),
                    ("id", "Option<String>", "Element id"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "SkeletonCircleProps" }
                PropsTable { props: vec![
                    ("size", "String", "Size of the circle in px (default: 40)"),
                    ("animate", "bool", "Whether to show animation (default: true)"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "SkeletonTextProps" }
                PropsTable { props: vec![
                    ("lines", "usize", "Number of lines (default: 3)"),
                    ("animate", "bool", "Whether to show animation (default: true)"),
                    ("last_line_width", "u8", "Last line width as percentage (default: 60)"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "SkeletonCardProps" }
                PropsTable { props: vec![
                    ("animate", "bool", "Whether to show animation (default: true)"),
                    ("show_avatar", "bool", "Show avatar placeholder (default: true)"),
                    ("text_lines", "usize", "Number of text lines (default: 2)"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                ]}
            }
        }
    }
}

#[component]
pub fn StepperPage() -> Element {
    use dioxus_ui_system::atoms::{StepSize, StepState};
    use dioxus_ui_system::molecules::{
        HorizontalStepper, StepContent, StepItem, StepperActions, VerticalStepper,
    };

    let steps = vec![
        StepItem::new("Personal Info").with_description("Enter your details"),
        StepItem::new("Account").with_description("Set up your account"),
        StepItem::new("Review").with_description("Review and confirm"),
        StepItem::new("Complete").with_description("All done!"),
    ];

    let vertical_steps = vec![
        StepItem::new("Upload Files")
            .with_icon("📁")
            .with_state(StepState::Completed),
        StepItem::new("Processing")
            .with_icon("⚙️")
            .with_state(StepState::Active),
        StepItem::new("Review")
            .with_icon("👁️")
            .with_state(StepState::Pending),
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
                                        Input { value: "".to_string(), placeholder: "Enter username", on_change: move |_| {} }
                                    }
                                    Box {
                                        Label { "Email" }
                                        Input { value: "".to_string(), placeholder: "Enter email", on_change: move |_| {} }
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
                CodeBlock { code: "let steps = vec![
    StepItem::new(\"Step 1\").with_description(\"Description\"),
    StepItem::new(\"Step 2\").with_icon(\"⚙️\"),
];

HorizontalStepper {{
    steps: steps,
    active_step: 1,
}}".to_string() }
            }

            Section { title: "Props",
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "HorizontalStepperProps" }
                PropsTable { props: vec![
                    ("steps", "Vec<StepItem>", "List of steps"),
                    ("active_step", "usize", "Current active step index (0-based)"),
                    ("size", "StepSize", "Step size"),
                    ("on_step_click", "Option<EventHandler<usize>>", "Optional click handler for steps"),
                    ("allow_click_completed", "bool", "Allow clicking completed steps (default: true)"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "VerticalStepperProps" }
                PropsTable { props: vec![
                    ("steps", "Vec<StepItem>", "List of steps"),
                    ("active_step", "usize", "Current active step index (0-based)"),
                    ("size", "StepSize", "Step size"),
                    ("on_step_click", "Option<EventHandler<usize>>", "Optional click handler for steps"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "StepItemProps" }
                PropsTable { props: vec![
                    ("index", "usize", "Step index (0-based)"),
                    ("step", "StepItem", "Step data"),
                    ("size", "StepSize", "Step size"),
                    ("show_connector", "bool", "Whether to show connector (default: true)"),
                    ("connector_completed", "bool", "Connector is completed"),
                    ("horizontal", "bool", "Horizontal layout (default: true)"),
                    ("on_click", "Option<EventHandler<usize>>", "Click handler"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "StepContentProps" }
                PropsTable { props: vec![
                    ("active_step", "usize", "Current active step"),
                    ("step_index", "usize", "Step index this content belongs to"),
                    ("children", "Element", "Content to display"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "StepperActionsProps" }
                PropsTable { props: vec![
                    ("current_step", "usize", "Current step"),
                    ("total_steps", "usize", "Total steps"),
                    ("on_back", "Option<EventHandler<()>>", "Back button handler"),
                    ("on_next", "Option<EventHandler<()>>", "Next button handler"),
                    ("on_finish", "Option<EventHandler<()>>", "Finish button handler"),
                    ("on_skip", "Option<EventHandler<()>>", "Skip button handler"),
                    ("finish_label", "String", "Custom finish button label (default: Finish)"),
                    ("next_label", "String", "Custom next button label (default: Next)"),
                    ("back_label", "String", "Custom back button label (default: Back)"),
                    ("disable_back", "bool", "Disable back button"),
                    ("disable_next", "bool", "Disable next button"),
                    ("show_skip", "bool", "Show skip button"),
                    ("align", "StepperActionsAlign", "Alignment (End, Center, SpaceBetween)"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
            }
        }
    }
}

#[component]
pub fn PopoverPage() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        DocPage {
            title: "Popover",
            description: "Floating content panels triggered by user interaction.",

            Section { title: "Basic Popover",
                ExampleBox {
                    Box { style: "padding: 40px;",
                        Popover {
                            open: open(),
                            on_open_change: move |o| open.set(o),
                            trigger: rsx! {
                                Button { variant: ButtonVariant::Primary, "Open Popover" }
                            },
                            PopoverHeader {
                                title: "Popover Title".to_string(),
                                description: Some("This is a description of the popover content.".to_string()),
                            }
                            p { "Here is some content inside the popover panel." }
                            PopoverFooter {
                                Button { variant: ButtonVariant::Ghost, onclick: move |_| open.set(false), "Cancel" }
                                Button { variant: ButtonVariant::Primary, onclick: move |_| open.set(false), "Confirm" }
                            }
                        }
                    }
                }
                CodeBlock { code: r#"let mut open = use_signal(|| false);

Popover {
    open: open(),
    on_open_change: move |o| open.set(o),
    trigger: rsx! {
        Button { variant: ButtonVariant::Primary, "Open Popover" }
    },
    PopoverHeader {
        title: "Popover Title".to_string(),
        description: Some("Description...".to_string()),
    }
    "Content here"
    PopoverFooter {
        Button { variant: ButtonVariant::Ghost, onclick: move |_| open.set(false), "Cancel" }
        Button { variant: ButtonVariant::Primary, onclick: move |_| open.set(false), "Confirm" }
    }
}"#.to_string() }
            }

            Section { title: "Props",
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "PopoverProps" }
                PropsTable { props: vec![
                    ("trigger", "Element", "Trigger element"),
                    ("children", "Element", "Popover content"),
                    ("open", "bool", "Whether the popover is open"),
                    ("on_open_change", "Option<EventHandler<bool>>", "Callback when open state changes"),
                    ("placement", "PopoverPlacement", "Popover placement"),
                    ("style", "Option<String>", "Custom inline styles"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "PopoverHeaderProps" }
                PropsTable { props: vec![
                    ("title", "String", "Header title"),
                    ("description", "Option<String>", "Optional description"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "PopoverFooterProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Footer content"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
            }
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

            Section { title: "Props",
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "ToastProps" }
                PropsTable { props: vec![
                    ("message", "String", "Toast message"),
                    ("variant", "ToastVariant", "Toast variant (Info, Success, Warning, Error)"),
                    ("duration_ms", "u64", "Duration before auto-dismiss (default: 5000)"),
                    ("closable", "bool", "Whether to show close button (default: true)"),
                    ("on_close", "EventHandler<()>", "Close handler"),
                    ("action", "Option<String>", "Action button text"),
                    ("on_action", "Option<EventHandler<()>>", "Action handler"),
                    ("index", "usize", "Position in stack"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "ToastProviderProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Child elements"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
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
                CodeBlock { code: "let options = vec![
    ComboboxOption::new(\"rust\", \"Rust\"),
    ComboboxOption::new(\"go\", \"Go\"),
];

Combobox {{
    options: options,
    value: selected(),
    on_change: EventHandler::new(move |v| selected.set(Some(v))),
    label: Some(\"Select Language\".to_string()),
    placeholder: Some(\"Search...\".to_string()),
}}".to_string() }
            }

            Section { title: "Props",
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "ComboboxProps" }
                PropsTable { props: vec![
                    ("options", "Vec<ComboboxOption>", "Available options"),
                    ("value", "Option<String>", "Currently selected value"),
                    ("on_change", "EventHandler<String>", "Change handler"),
                    ("placeholder", "Option<String>", "Placeholder text"),
                    ("label", "Option<String>", "Label"),
                    ("disabled", "bool", "Whether the field is disabled"),
                    ("error", "Option<String>", "Error message"),
                    ("creatable", "bool", "Allow creating new options"),
                    ("on_create", "Option<EventHandler<String>>", "Created option handler"),
                    ("clearable", "bool", "Clearable (default: true)"),
                    ("loading", "bool", "Loading state"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("id", "Option<String>", "Element id"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "MultiComboboxProps" }
                PropsTable { props: vec![
                    ("options", "Vec<ComboboxOption>", "Available options"),
                    ("values", "Vec<String>", "Selected values"),
                    ("on_change", "EventHandler<Vec<String>>", "Change handler"),
                    ("placeholder", "Option<String>", "Placeholder text"),
                    ("label", "Option<String>", "Label"),
                    ("max", "usize", "Maximum selections (default: usize::MAX)"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("id", "Option<String>", "Element id"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
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
                CodeBlock { code: "Comment {{
    author: \"Jane Doe\".to_string(),
    content: \"Comment text here\".to_string(),
    timestamp: \"2 hours ago\".to_string(),
    like_count: 5,
    liked: false,
}}".to_string() }
            }

            Section { title: "Props",
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "MediaObjectProps" }
                PropsTable { props: vec![
                    ("media", "Element", "Media content (image, icon, avatar)"),
                    ("children", "Element", "Body content (text)"),
                    ("align", "MediaAlign", "Media alignment (Start, Center, End)"),
                    ("gap", "u16", "Spacing between media and content (default: 16)"),
                    ("reverse", "bool", "Whether to reverse the order"),
                    ("responsive", "bool", "Stack on mobile (default: true)"),
                    ("media_width", "Option<String>", "Media width when stacked"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("id", "Option<String>", "Element id"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "MediaContentProps" }
                PropsTable { props: vec![
                    ("title", "Option<String>", "Title/heading"),
                    ("title_level", "u8", "Title element level (default: 4)"),
                    ("description", "Option<String>", "Description/body text"),
                    ("children", "Option<Element>", "Additional content"),
                    ("meta", "Option<String>", "Metadata (date, author, etc.)"),
                    ("actions", "Option<Element>", "Actions (buttons, links)"),
                    ("id", "Option<String>", "Element id"),
                    ("class", "Option<String>", "Custom CSS classes"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "CommentProps" }
                PropsTable { props: vec![
                    ("author", "String", "Author name"),
                    ("avatar", "Option<Element>", "Author avatar"),
                    ("content", "String", "Comment text"),
                    ("timestamp", "String", "Timestamp"),
                    ("on_reply", "Option<EventHandler<()>>", "Reply action"),
                    ("on_like", "Option<EventHandler<()>>", "Like action"),
                    ("liked", "bool", "Whether liked"),
                    ("like_count", "u32", "Like count"),
                    ("replies", "Option<Element>", "Nested replies"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("id", "Option<String>", "Element id"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
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
                CodeBlock { code: "let mut page = use_signal(|| 1);

Pagination {{
    total_pages: 10,
    current_page: page(),
    on_change: EventHandler::new(move |p| page.set(p)),
    sibling_count: 1,
    show_first_last: true,
    show_prev_next: true,
}}".to_string() }
            }

            Section { title: "Props",
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "PaginationProps" }
                PropsTable { props: vec![
                    ("total_pages", "u32", "Total number of pages"),
                    ("current_page", "u32", "Current page (1-indexed, default: 1)"),
                    ("on_change", "EventHandler<u32>", "Page change handler"),
                    ("sibling_count", "u32", "Number of sibling pages to show (default: 1)"),
                    ("show_first_last", "bool", "Show first/last buttons (default: true)"),
                    ("show_prev_next", "bool", "Show previous/next buttons (default: true)"),
                    ("simple", "bool", "Simple mode (prev/next only, default: false)"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("id", "Option<String>", "Element id"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "PageSizeSelectorProps" }
                PropsTable { props: vec![
                    ("options", "Vec<u32>", "Available page sizes (default: [10,25,50,100])"),
                    ("value", "u32", "Current page size"),
                    ("on_change", "EventHandler<u32>", "Change handler"),
                    ("label", "Option<String>", "Label"),
                    ("suffix", "Option<String>", "Per page suffix"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("id", "Option<String>", "Element id"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "PaginationInfoProps" }
                PropsTable { props: vec![
                    ("current_page", "u32", "Current page"),
                    ("page_size", "u32", "Page size"),
                    ("total_items", "u32", "Total items"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("id", "Option<String>", "Element id"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
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
                CodeBlock { code: "ListItem {{
    title: \"Item Title\".to_string(),
    description: Some(\"Description text\".to_string()),
    leading: Some(rsx! {{ span {{ \"📁\" }} }}),
    trailing: Some(rsx! {{ span {{ \"→\" }} }}),
    variant: ListItemVariant::Default,
    hoverable: true,
}}".to_string() }
            }

            Section { title: "Props",
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "ListItemProps" }
                PropsTable { props: vec![
                    ("title", "String", "Primary text"),
                    ("description", "Option<String>", "Secondary text/description"),
                    ("leading", "Option<Element>", "Leading element (icon, avatar, checkbox)"),
                    ("trailing", "Option<Element>", "Trailing element (icon, button, meta)"),
                    ("on_click", "Option<EventHandler<()>>", "Click handler"),
                    ("variant", "ListItemVariant", "Variant/state (Default, Selected, Active, Disabled)"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("dense", "bool", "Dense mode (compact)"),
                    ("divider", "bool", "Divider below"),
                    ("hoverable", "bool", "Hover effect"),
                    ("id", "Option<String>", "Element id"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "ListGroupProps" }
                PropsTable { props: vec![
                    ("title", "Option<String>", "Group title"),
                    ("children", "Element", "Group items"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("sticky", "bool", "Sticky header"),
                    ("id", "Option<String>", "Element id"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "ActionListItemProps" }
                PropsTable { props: vec![
                    ("label", "String", "Action label"),
                    ("icon", "Option<String>", "Icon"),
                    ("description", "Option<String>", "Description/help text"),
                    ("shortcut", "Option<String>", "Keyboard shortcut"),
                    ("on_click", "EventHandler<()>", "Click handler"),
                    ("disabled", "bool", "Disabled state"),
                    ("destructive", "bool", "Danger/destructive action"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("id", "Option<String>", "Element id"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
                h3 { style: "font-size: 16px; margin: 16px 0 8px 0;", "ExpandableListItemProps" }
                PropsTable { props: vec![
                    ("header", "Element", "Header content"),
                    ("children", "Element", "Expandable content"),
                    ("default_expanded", "bool", "Initially expanded"),
                    ("class", "Option<String>", "Additional CSS classes"),
                    ("id", "Option<String>", "Element id"),
                    ("style", "Option<String>", "Custom inline styles"),
                ]}
            }
        }
    }
}
