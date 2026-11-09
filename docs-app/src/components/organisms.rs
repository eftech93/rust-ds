//! Organism component documentation pages

use crate::docs_ui::{CodeBlock, DocPage, ExampleBox, PropsTable, Section};
use dioxus::prelude::*;
use dioxus_ui_system::atoms::{Box, HStack, VStack};
use dioxus_ui_system::molecules::ToastVariant;
use dioxus_ui_system::organisms::file_upload::{FileType, UploadedFile};
use dioxus_ui_system::organisms::FooterVariant;
use dioxus_ui_system::organisms::*;
use dioxus_ui_system::prelude::*;

#[component]
pub fn OrganismsPage() -> Element {
    rsx! {
        DocPage {
            title: "Organisms",
            description: "Complex UI components composed of molecules and atoms.",

            Section { title: "Overview",
                p { "Organisms include:" }
                ul {
                    li { "Header - Application navigation" }
                    li { "Layout - Page layout system" }
                    li { "Tabs - Tabbed content" }
                    li { "Accordion - Collapsible sections" }
                    li { "Cards - Advanced card patterns" }
                    li { "DataTable - Data display" }
                    li { "Stepper Wizard - Multi-step forms" }
                    li { "Charts - Data visualization (Bar, Line, Pie, Sparkline)" }
                    li { "ThreeView - 3D viewer with Three.js (dioxus-three v0.0.2) - Now with Web support!" }
                    li { "Calendar - Full calendar view (New!)" }
                    li { "DateRangePicker - Date range selection (New!)" }
                    li { "Carousel - Image/content slider (New!)" }
                    li { "Tree - Hierarchical tree view (New!)" }
                    li { "Timeline - Vertical timeline (New!)" }
                    li { "Menubar - Application menu bar (New!)" }
                    li { "ResizablePanels - Split-pane layout (New!)" }
                    li { "Kanban - Drag-and-drop task board (New!)" }
                    li { "ImageUploader - Image upload with preview (New!)" }
                }
            }
        }
    }
}

#[component]
pub fn HeaderPage() -> Element {
    rsx! {
        DocPage {
            title: "Header",
            description: "Application header with navigation and branding.",

            Section { title: "Example",
                ExampleBox {
                    Box { style: "border: 1px solid rgb(226,232,240); border-radius: 8px; overflow: hidden;",
                        Header {
                            brand_title: "MyApp",
                            nav_items: vec![
                                NavItem { label: "Home".to_string(), href: "#".to_string(), icon: Some("home".to_string()), active: true },
                                NavItem { label: "About".to_string(), href: "#".to_string(), icon: None, active: false },
                            ],
                            actions: rsx! { Button { variant: ButtonVariant::Primary, size: ButtonSize::Sm, "Sign In" } },
                        }
                    }
                }
                CodeBlock { code: "Header {{
    brand_title: \"MyApp\",
    nav_items: vec![
        NavItem {{
            label: \"Home\".to_string(),
            href: \"/\".to_string(),
            icon: Some(\"home\".to_string()),
            active: true,
        }},
    ],
    actions: rsx! {{ Button {{ \"Sign In\" }} }},
}}".to_string() }
            }

            Section { title: "Props",
                h4 { "HeaderProps" }
                PropsTable { props: vec![
                    ("brand", "Option", "Brand/logo element"),
                    ("brand_title", "Option", "Brand title text (alternative to brand element)"),
                    ("nav_items", "Vec", "Navigation items"),
                    ("actions", "Option", "Right-side actions"),
                    ("sticky", "bool", "Whether header is sticky"),
                    ("bordered", "bool", "Whether to show border"),
                    ("style", "Option", "Custom inline styles"),
                    ("id", "Option", "Element id"),
                    ("class", "Option", "Custom CSS classes")
                ] }
                h4 { "HeaderNavLinkProps" }
                PropsTable { props: vec![
                    ("item", "NavItem", ""),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "UserMenuProps" }
                PropsTable { props: vec![
                    ("name", "String", "User name"),
                    ("email", "Option", "User email"),
                    ("avatar", "Option", "User avatar URL"),
                    ("menu_items", "Vec", "Menu items"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "UserMenuDropdownProps" }
                PropsTable { props: vec![
                    ("items", "Vec", ""),
                    ("on_close", "EventHandler", ""),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "UserMenuItemViewProps" }
                PropsTable { props: vec![
                    ("item", "UserMenuItem", ""),
                    ("on_close", "EventHandler", ""),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
            }
        }
    }
}

#[component]
pub fn LayoutPage() -> Element {
    let mut layout_type = use_signal(|| LayoutType::Sidebar);

    rsx! {
        DocPage {
            title: "Layout",
            description: "Page layout system with multiple variants.",

            Section { title: "Layout Types",
                ExampleBox {
                    HStack { gap: SpacingSize::Sm, style: "margin-bottom: 16px;",
                        Button { variant: if layout_type() == LayoutType::Sidebar { ButtonVariant::Primary } else { ButtonVariant::Secondary }, onclick: move |_| layout_type.set(LayoutType::Sidebar), "Sidebar" }
                        Button { variant: if layout_type() == LayoutType::TopNav { ButtonVariant::Primary } else { ButtonVariant::Secondary }, onclick: move |_| layout_type.set(LayoutType::TopNav), "TopNav" }
                    }

                    Box { style: "height: 300px; border: 1px solid rgb(226,232,240); border-radius: 8px; overflow: hidden;",
                        Box { style: "transform: scale(0.5); transform-origin: top left; width: 200%; height: 200%;",
                            Layout {
                                layout_type: layout_type(),
                                nav_items: vec![
                                    LayoutNavItem::new("home", "Home", "#").with_icon("home").active(true),
                                    LayoutNavItem::new("settings", "Settings", "#").with_icon("settings"),
                                ],
                                brand: Some(rsx! { "MyApp" }),
                                title: Some("Dashboard".to_string()),
                                collapsible: true,
                                Box { style: "padding: 24px;", Heading { level: HeadingLevel::H2, "Content Area" } }
                            }
                        }
                    }
                }
                CodeBlock { code: "Layout {{
    layout_type: LayoutType::Sidebar,
    nav_items: vec![
        LayoutNavItem::new(\"home\", \"Home\", \"/\").with_icon(\"home\"),
    ],
    brand: Some(rsx! {{ \"MyApp\" }}),
    title: Some(\"Dashboard\".to_string()),
    collapsible: true,
    // Content goes here
}}".to_string() }
            }

            Section { title: "Props",
                h4 { "LayoutProps" }
                PropsTable { props: vec![
                    ("layout_type", "LayoutType", "Layout type"),
                    ("nav_items", "Vec", "Navigation items"),
                    ("brand", "Option", "Brand element (logo/title)"),
                    ("title", "Option", "Page title"),
                    ("children", "Element", "Main content"),
                    ("actions", "Option", "Right-side actions"),
                    ("collapsible", "bool", "Whether sidebar is collapsible"),
                    ("sidebar_collapsed", "bool", "Initial sidebar collapsed state"),
                    ("sidebar_width", "u16", "Sidebar width (default: 260px)"),
                    ("header_height", "u16", "Header height (default: 64px)"),
                    ("class", "Option", "Custom class name"),
                    ("id", "Option", "Element id"),
                    ("style", "Option", "Custom inline styles")
                ] }
            }
        }
    }
}

#[component]
pub fn TabsPage() -> Element {
    let mut active = use_signal(|| "account".to_string());

    let tabs = vec![
        TabItem::new("account", "Account").with_icon("user"),
        TabItem::new("password", "Password"),
        TabItem::new("notifications", "Notifications"),
    ];

    rsx! {
        DocPage {
            title: "Tabs",
            description: "Tabbed content navigation.",

            Section { title: "Example",
                ExampleBox {
                    Tabs {
                        tabs: tabs,
                        active_tab: active(),
                        on_change: move |id| active.set(id),
                        TabPanel {
                            if active() == "account" {
                                "Manage your account settings."
                            } else if active() == "password" {
                                "Change your password."
                            } else {
                                "Configure notifications."
                            }
                        }
                    }
                }
                CodeBlock { code: "let tabs = vec![
    TabItem::new(\"account\", \"Account\").with_icon(\"user\"),
    TabItem::new(\"password\", \"Password\"),
];

let mut active = use_signal(|| \"account\".to_string());

Tabs {{
    tabs: tabs,
    active_tab: active(),
    on_change: move |id| active.set(id),
    TabPanel {{
        if active() == \"account\" {{
            \"Account settings content\"
        }} else {{
            \"Other content\"
        }}
    }}
}}".to_string() }
            }

            Section { title: "Props",
                h4 { "TabsProps" }
                PropsTable { props: vec![
                    ("tabs", "Vec", "Tab items"),
                    ("active_tab", "String", "Currently active tab ID"),
                    ("on_change", "EventHandler", "Callback when tab changes"),
                    ("children", "Element", "Tab content (rendered based on active tab)"),
                    ("variant", "TabsVariant", "Tabs variant"),
                    ("style", "Option", "Custom inline styles"),
                    ("id", "Option", ""),
                    ("class", "Option", "")
                ] }
                h4 { "TabPanelProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Panel content"),
                    ("style", "Option", "Custom inline styles"),
                    ("id", "Option", ""),
                    ("class", "Option", "")
                ] }
                h4 { "VerticalTabsProps" }
                PropsTable { props: vec![
                    ("tabs", "Vec", "Tab items"),
                    ("active_tab", "String", "Currently active tab ID"),
                    ("on_change", "EventHandler", "Callback when tab changes"),
                    ("style", "Option", "Custom inline styles"),
                    ("id", "Option", ""),
                    ("class", "Option", "")
                ] }
            }
        }
    }
}

#[component]
pub fn AccordionPage() -> Element {
    let mut expanded = use_signal(|| vec!["item1".to_string()]);

    let items = vec![
        AccordionItem::new(
            "item1",
            "Is it accessible?",
            "Yes. It adheres to the WAI-ARIA design pattern.",
        ),
        AccordionItem::new(
            "item2",
            "Is it styled?",
            "Yes. It comes with default styles.",
        ),
        AccordionItem::new("item3", "Is it animated?", "Yes. It's animated by default."),
    ];

    rsx! {
        DocPage {
            title: "Accordion",
            description: "Collapsible content sections.",

            Section { title: "Example",
                ExampleBox {
                    Accordion {
                        items: items,
                        expanded: expanded(),
                        on_change: move |ids| expanded.set(ids),
                        multiple: true,
                    }
                }
                CodeBlock { code: "let items = vec![
    AccordionItem::new(
        \"item1\",
        \"Question 1?\",
        \"Answer to question 1.\"
    ),
];

let mut expanded = use_signal(|| vec![\"item1\".to_string()]);

Accordion {{
    items: items,
    expanded: expanded(),
    on_change: move |ids| expanded.set(ids),
    multiple: true,
}}".to_string() }
            }

            Section { title: "Props",
                h4 { "AccordionProps" }
                PropsTable { props: vec![
                    ("items", "Vec", "Accordion items"),
                    ("expanded", "Vec", "Currently expanded item IDs (for single: one item, for multiple: many)"),
                    ("on_change", "EventHandler<Vec>", "Callback when expanded items change"),
                    ("multiple", "bool", "Whether multiple items can be expanded"),
                    ("collapsible", "bool", "Whether items can be collapsed"),
                    ("style", "Option", "Custom inline styles"),
                    ("id", "Option", "Element id"),
                    ("class", "Option", "Custom CSS classes")
                ] }
                h4 { "AccordionItem2Props" }
                PropsTable { props: vec![
                    ("title", "String", "Item title"),
                    ("children", "Element", "Item content"),
                    ("default_expanded", "bool", "Whether item is expanded"),
                    ("style", "Option", "Custom inline styles"),
                    ("id", "Option", "Element id"),
                    ("class", "Option", "Custom CSS classes")
                ] }
            }
        }
    }
}

#[component]
pub fn CardsPage() -> Element {
    rsx! {
        DocPage {
            title: "Card Organisms",
            description: "Pre-built card patterns for common use cases.",

            Section { title: "Action Card",
                ExampleBox {
                    ActionCard {
                        title: "Deploy Project",
                        description: "Your project is ready to deploy.",
                        action_label: "Deploy Now",
                        on_action: move |_| {},
                        icon: Some("rocket".to_string()),
                    }
                }
            }

            Section { title: "Profile Card",
                ExampleBox {
                    ProfileCard {
                        name: "Sarah Chen".to_string(),
                        role: Some("Senior Engineer".to_string()),
                        avatar_url: None,
                        description: Some("Full-stack developer".to_string()),
                        action_label: "Connect".to_string(),
                        on_action: Some(EventHandler::new(move |_| {})),
                        stats: vec![("Projects".to_string(), "24".to_string())],
                    }
                }
            }

            Section { title: "Stat Card",
                ExampleBox {
                    Box { style: "max-width: 300px;",
                        StatCard {
                            label: "Revenue",
                            value: "$48.2K",
                            change: Some("+12%".to_string()),
                            change_positive: Some(true),
                            icon: Some("trending-up".to_string()),
                            icon_bg: "rgb(219,234,254)".to_string(),
                        }
                    }
                }
            }

            Section { title: "Pricing Card",
                ExampleBox {
                    Box { style: "max-width: 300px;",
                        PricingCard {
                            plan: "Pro".to_string(),
                            price: "$29".to_string(),
                            period: "/month".to_string(),
                            description: Some("For growing teams".to_string()),
                            features: vec!["Unlimited".to_string(), "Priority".to_string()],
                            cta_label: "Upgrade".to_string(),
                            on_cta: EventHandler::new(move |_| {}),
                            recommended: true,
                        }
                    }
                }
                CodeBlock { code: "ActionCard {{
    title: \"Deploy Project\",
    description: \"Your project is ready to deploy.\",
    action_label: \"Deploy Now\",
    on_action: move |_| {{}},
    icon: Some(\"rocket\".to_string()),
}}".to_string() }
            }

            Section { title: "Props",
                h4 { "ActionCardProps" }
                PropsTable { props: vec![
                    ("title", "String", "Card title"),
                    ("description", "String", "Card description"),
                    ("action_label", "String", "Action button text"),
                    ("on_action", "EventHandler", "Action button callback"),
                    ("icon", "Option", "Optional icon name"),
                    ("variant", "CardVariant", "Card variant"),
                    ("badge", "Option", "Optional badge"),
                    ("id", "Option", "Element id"),
                    ("class", "Option", "Custom CSS classes"),
                    ("style", "Option", "Custom inline styles")
                ] }
                h4 { "DualActionCardProps" }
                PropsTable { props: vec![
                    ("title", "String", "Card title"),
                    ("description", "String", "Card description"),
                    ("primary_label", "String", "Primary action label"),
                    ("secondary_label", "String", "Secondary action label"),
                    ("on_primary", "EventHandler", "Primary action callback"),
                    ("on_secondary", "EventHandler", "Secondary action callback"),
                    ("icon", "Option", "Optional icon"),
                    ("id", "Option", "Element id"),
                    ("class", "Option", "Custom CSS classes"),
                    ("style", "Option", "Custom inline styles")
                ] }
                h4 { "ImageCardProps" }
                PropsTable { props: vec![
                    ("image_url", "String", "Image URL"),
                    ("image_alt", "String", "Image alt text"),
                    ("title", "String", "Card title"),
                    ("description", "String", "Card description"),
                    ("action_label", "Option", "Optional action"),
                    ("on_action", "Option<EventHandler>", "Action callback"),
                    ("aspect_ratio", "String", "Image aspect ratio (default: 16/9)"),
                    ("id", "Option", "Element id"),
                    ("class", "Option", "Custom CSS classes"),
                    ("style", "Option", "Custom inline styles")
                ] }
                h4 { "ImageActionCardProps" }
                PropsTable { props: vec![
                    ("image_url", "String", "Image URL"),
                    ("title", "String", "Card title"),
                    ("description", "String", "Card description"),
                    ("primary_label", "String", "Primary action"),
                    ("secondary_label", "String", "Secondary action"),
                    ("on_primary", "EventHandler", "Primary callback"),
                    ("on_secondary", "EventHandler", "Secondary callback"),
                    ("badge", "Option", "Optional badge on image"),
                    ("id", "Option", "Element id"),
                    ("class", "Option", "Custom CSS classes"),
                    ("style", "Option", "Custom inline styles")
                ] }
                h4 { "ProfileCardProps" }
                PropsTable { props: vec![
                    ("avatar_url", "Option", "Avatar URL"),
                    ("name", "String", "User name"),
                    ("role", "Option", "User role/title"),
                    ("description", "Option", "User description"),
                    ("action_label", "String", "Follow/Connect button label"),
                    ("on_action", "Option<EventHandler>", "Action callback"),
                    ("stats", "Vec", "Show social stats"),
                    ("id", "Option", "Element id"),
                    ("class", "Option", "Custom CSS classes"),
                    ("style", "Option", "Custom inline styles")
                ] }
                h4 { "PricingCardProps" }
                PropsTable { props: vec![
                    ("plan", "String", "Plan name"),
                    ("price", "String", "Price (e.g., \"$29\")"),
                    ("period", "String", "Billing period (e.g., \"/month\")"),
                    ("description", "Option", "Plan description"),
                    ("features", "Vec", "List of features"),
                    ("cta_label", "String", "CTA button label"),
                    ("on_cta", "EventHandler", "CTA callback"),
                    ("recommended", "bool", "Is this the recommended plan"),
                    ("id", "Option", "Element id"),
                    ("class", "Option", "Custom CSS classes"),
                    ("style", "Option", "Custom inline styles")
                ] }
                h4 { "HorizontalCardProps" }
                PropsTable { props: vec![
                    ("image_url", "String", "Image URL"),
                    ("title", "String", "Card title"),
                    ("description", "String", "Card description"),
                    ("action_label", "Option", "Optional action"),
                    ("on_action", "Option<EventHandler>", "Action callback"),
                    ("id", "Option", "Element id"),
                    ("class", "Option", "Custom CSS classes"),
                    ("style", "Option", "Custom inline styles")
                ] }
                h4 { "NotificationCardProps" }
                PropsTable { props: vec![
                    ("title", "String", "Notification title"),
                    ("message", "String", "Notification message"),
                    ("notification_type", "NotificationType", "Notification type"),
                    ("timestamp", "Option", "Timestamp"),
                    ("on_dismiss", "Option<EventHandler>", "Dismiss callback"),
                    ("icon", "Option", "Icon name (auto-selected based on type if not provided)"),
                    ("id", "Option", "Element id"),
                    ("class", "Option", "Custom CSS classes"),
                    ("style", "Option", "Custom inline styles")
                ] }
                h4 { "StatCardProps" }
                PropsTable { props: vec![
                    ("label", "String", "Stat label"),
                    ("value", "String", "Stat value"),
                    ("change", "Option", "Change indicator (e.g., \"+12%\")"),
                    ("change_positive", "Option", "Is the change positive"),
                    ("icon", "Option", "Icon name"),
                    ("icon_bg", "String", "Icon background color"),
                    ("id", "Option", "Element id"),
                    ("class", "Option", "Custom CSS classes"),
                    ("style", "Option", "Custom inline styles")
                ] }
                h4 { "ExpandableCardProps" }
                PropsTable { props: vec![
                    ("title", "String", "Card title"),
                    ("preview", "Element", "Preview content (always visible)"),
                    ("expanded_content", "Element", "Expanded content (shown when expanded)"),
                    ("default_expanded", "bool", "Initial expanded state"),
                    ("id", "Option", "Element id"),
                    ("class", "Option", "Custom CSS classes"),
                    ("style", "Option", "Custom inline styles")
                ] }
                h4 { "MediaCardProps" }
                PropsTable { props: vec![
                    ("media_url", "String", "Media URL (image or video thumbnail)"),
                    ("media_type", "MediaType", "Media type indicator"),
                    ("title", "String", "Card title"),
                    ("creator", "Option", "Creator/Author"),
                    ("duration", "Option", "Duration (for video/audio)"),
                    ("on_like", "Option<EventHandler>", "Like callback"),
                    ("on_share", "Option<EventHandler>", "Share callback"),
                    ("on_play", "Option<EventHandler>", "Play callback"),
                    ("id", "Option", "Element id"),
                    ("class", "Option", "Custom CSS classes"),
                    ("style", "Option", "Custom inline styles")
                ] }
            }
        }
    }
}

#[component]
pub fn DataTablePage() -> Element {
    use dioxus_ui_system::organisms::{
        ColumnAlign, DataTable, FilterOption, TableColumn, TableFilter, TablePagination,
    };
    use std::collections::HashMap;

    #[derive(Clone, PartialEq)]
    struct User {
        id: String,
        name: String,
        email: String,
        role: String,
        status: String,
        department: String,
    }

    fn render_status(user: &User) -> Element {
        let bg = if user.status == "Active" {
            "rgb(34,197,94)"
        } else {
            "rgb(148,163,184)"
        };
        rsx! {
            span {
                style: "padding: 4px 12px; background: {bg}; color: white; border-radius: 9999px; font-size: 12px;",
                "{user.status}"
            }
        }
    }

    let all_users = vec![
        User {
            id: "1".to_string(),
            name: "Alice Johnson".to_string(),
            email: "alice@example.com".to_string(),
            role: "Admin".to_string(),
            status: "Active".to_string(),
            department: "Engineering".to_string(),
        },
        User {
            id: "2".to_string(),
            name: "Bob Smith".to_string(),
            email: "bob@example.com".to_string(),
            role: "User".to_string(),
            status: "Active".to_string(),
            department: "Marketing".to_string(),
        },
        User {
            id: "3".to_string(),
            name: "Carol White".to_string(),
            email: "carol@example.com".to_string(),
            role: "Editor".to_string(),
            status: "Inactive".to_string(),
            department: "Design".to_string(),
        },
        User {
            id: "4".to_string(),
            name: "David Brown".to_string(),
            email: "david@example.com".to_string(),
            role: "User".to_string(),
            status: "Active".to_string(),
            department: "Engineering".to_string(),
        },
        User {
            id: "5".to_string(),
            name: "Emma Davis".to_string(),
            email: "emma@example.com".to_string(),
            role: "Admin".to_string(),
            status: "Active".to_string(),
            department: "Sales".to_string(),
        },
        User {
            id: "6".to_string(),
            name: "Frank Wilson".to_string(),
            email: "frank@example.com".to_string(),
            role: "User".to_string(),
            status: "Inactive".to_string(),
            department: "Marketing".to_string(),
        },
        User {
            id: "7".to_string(),
            name: "Grace Lee".to_string(),
            email: "grace@example.com".to_string(),
            role: "Editor".to_string(),
            status: "Active".to_string(),
            department: "Design".to_string(),
        },
        User {
            id: "8".to_string(),
            name: "Henry Taylor".to_string(),
            email: "henry@example.com".to_string(),
            role: "User".to_string(),
            status: "Active".to_string(),
            department: "Engineering".to_string(),
        },
    ];

    let columns = vec![
        TableColumn {
            key: "name".to_string(),
            header: "Name".to_string(),
            width: Some("150px".to_string()),
            align: ColumnAlign::Left,
            sortable: true,
            render: None,
        },
        TableColumn {
            key: "email".to_string(),
            header: "Email".to_string(),
            width: Some("200px".to_string()),
            align: ColumnAlign::Left,
            sortable: true,
            render: None,
        },
        TableColumn {
            key: "role".to_string(),
            header: "Role".to_string(),
            width: None,
            align: ColumnAlign::Left,
            sortable: true,
            render: None,
        },
        TableColumn {
            key: "department".to_string(),
            header: "Department".to_string(),
            width: None,
            align: ColumnAlign::Left,
            sortable: true,
            render: None,
        },
        TableColumn {
            key: "status".to_string(),
            header: "Status".to_string(),
            width: Some("100px".to_string()),
            align: ColumnAlign::Center,
            sortable: false,
            render: Some(render_status),
        },
    ];

    // Filter definitions
    let filters = vec![
        TableFilter {
            key: "role".to_string(),
            label: "All Roles".to_string(),
            options: vec![
                FilterOption {
                    label: "Admin".to_string(),
                    value: "Admin".to_string(),
                },
                FilterOption {
                    label: "User".to_string(),
                    value: "User".to_string(),
                },
                FilterOption {
                    label: "Editor".to_string(),
                    value: "Editor".to_string(),
                },
            ],
        },
        TableFilter {
            key: "status".to_string(),
            label: "All Status".to_string(),
            options: vec![
                FilterOption {
                    label: "Active".to_string(),
                    value: "Active".to_string(),
                },
                FilterOption {
                    label: "Inactive".to_string(),
                    value: "Inactive".to_string(),
                },
            ],
        },
        TableFilter {
            key: "department".to_string(),
            label: "All Departments".to_string(),
            options: vec![
                FilterOption {
                    label: "Engineering".to_string(),
                    value: "Engineering".to_string(),
                },
                FilterOption {
                    label: "Marketing".to_string(),
                    value: "Marketing".to_string(),
                },
                FilterOption {
                    label: "Design".to_string(),
                    value: "Design".to_string(),
                },
                FilterOption {
                    label: "Sales".to_string(),
                    value: "Sales".to_string(),
                },
            ],
        },
    ];

    // State for search and filters
    let mut search_query = use_signal(|| "".to_string());
    let mut active_filters = use_signal(|| HashMap::<String, String>::new());

    // Filter the data based on search query and active filters
    let filtered_users: Vec<User> = all_users
        .clone()
        .into_iter()
        .filter(|user| {
            // Text search
            let search_lower = search_query().to_lowercase();
            let matches_search = search_lower.is_empty()
                || user.name.to_lowercase().contains(&search_lower)
                || user.email.to_lowercase().contains(&search_lower)
                || user.role.to_lowercase().contains(&search_lower)
                || user.department.to_lowercase().contains(&search_lower);

            // Custom filters
            let matches_role = active_filters()
                .get("role")
                .map_or(true, |v| v == &user.role);
            let matches_status = active_filters()
                .get("status")
                .map_or(true, |v| v == &user.status);
            let matches_dept = active_filters()
                .get("department")
                .map_or(true, |v| v == &user.department);

            matches_search && matches_role && matches_status && matches_dept
        })
        .collect();

    rsx! {
        DocPage {
            title: "DataTable",
            description: "Data display component with sorting, pagination, search, and custom filters.",

            Section { title: "Basic Table",
                ExampleBox {
                    DataTable {
                        data: all_users.clone(),
                        columns: columns.clone(),
                        key_extractor: |u: &User| u.id.clone(),
                        empty_message: "No users found",
                        loading: false,
                        show_search: false,
                        show_filters: false,
                    }
                }
            }

            Section { title: "With Search & Filters",
                p { "Search across all fields and filter by role, status, and department:" }
                ExampleBox {
                    DataTable {
                        data: filtered_users.clone(),
                        columns: columns.clone(),
                        key_extractor: |u: &User| u.id.clone(),
                        empty_message: "No users match your criteria",
                        loading: false,
                        search_query: Some(search_query()),
                        on_search_change: Some(EventHandler::new(move |q: String| search_query.set(q))),
                        search_placeholder: "Search users...",
                        filters: filters.clone(),
                        active_filters: active_filters(),
                        on_filter_change: Some(EventHandler::new(move |(key, value): (String, String)| {
                            let mut new_filters = active_filters();
                            if value.is_empty() {
                                new_filters.remove(&key);
                            } else {
                                new_filters.insert(key, value);
                            }
                            active_filters.set(new_filters);
                        })),
                    }
                }
                CodeBlock { code: "// Search and filter state
let mut search_query = use_signal(|| \"\".to_string());
let mut active_filters = use_signal(|| HashMap::<String, String>::new());

// Filter definitions
let filters = vec![
    TableFilter {
        key: \"role\".to_string(),
        label: \"All Roles\".to_string(),
        options: vec![
            FilterOption { label: \"Admin\".to_string(), value: \"Admin\".to_string() },
            FilterOption { label: \"User\".to_string(), value: \"User\".to_string() },
        ],
    },
];

// Filtered data
let filtered_users: Vec<User> = all_users.into_iter().filter(|user| {
    let search_lower = search_query().to_lowercase();
    let matches_search = search_lower.is_empty() || 
        user.name.to_lowercase().contains(&search_lower);
    let matches_role = active_filters().get(\"role\").map_or(true, |v| v == &user.role);
    matches_search && matches_role
}).collect();

// Render
DataTable {{
    data: filtered_users,
    columns: columns,
    key_extractor: |u: &User| u.id.clone(),
    search_query: Some(search_query()),
    on_search_change: Some(EventHandler::new(move |q| search_query.set(q))),
    filters: filters,
    active_filters: active_filters(),
    on_filter_change: Some(EventHandler::new(move |(k, v)| {{
        // Handle filter change
    }})),
}}".to_string() }
            }

            Section { title: "Search Only",
                p { "Table with text search only:" }
                ExampleBox {
                    DataTable {
                        data: filtered_users.clone(),
                        columns: columns.clone(),
                        key_extractor: |u: &User| u.id.clone(),
                        empty_message: "No users found",
                        loading: false,
                        search_query: Some(search_query()),
                        on_search_change: Some(EventHandler::new(move |q: String| search_query.set(q))),
                        search_placeholder: "Search by name, email...",
                        show_filters: false,
                    }
                }
            }

            Section { title: "With Pagination",
                ExampleBox {
                    VStack { style: "border: 1px solid rgb(226,232,240); border-radius: 8px; overflow: hidden;",
                        DataTable {
                            data: all_users.clone(),
                            columns: columns.clone(),
                            key_extractor: |u: &User| u.id.clone(),
                            empty_message: "No users found",
                            loading: false,
                            show_search: false,
                            show_filters: false,
                        }
                        TablePagination {
                            current_page: 0,
                            total_pages: 5,
                            on_page_change: move |_| {},
                            show_first_last: true,
                        }
                    }
                }
            }

            Section { title: "Selectable Rows",
                ExampleBox {
                    DataTable {
                        data: all_users.clone(),
                        columns: columns.clone(),
                        key_extractor: |u: &User| u.id.clone(),
                        selectable: true,
                        selected_keys: vec![],
                        on_selection_change: move |_| {},
                        empty_message: "No users found",
                        loading: false,
                        show_search: false,
                        show_filters: false,
                    }
                }
            }

            Section { title: "Loading State",
                ExampleBox {
                    DataTable {
                        data: vec![] as Vec<User>,
                        columns: columns.clone(),
                        key_extractor: |u: &User| u.id.clone(),
                        empty_message: "Loading data...",
                        loading: true,
                        show_search: false,
                        show_filters: false,
                    }
                }
            }

            Section { title: "Props",
                h4 { "DataTableProps" }
                PropsTable { props: vec![
                    ("columns", "Vec", "Column definitions"),
                    ("data", "Vec", "Table data"),
                    ("key_extractor", "fn", "Unique key extractor for rows"),
                    ("selectable", "bool", "Row selection enabled"),
                    ("selected_keys", "Vec", "Selected row keys"),
                    ("on_selection_change", "Option<EventHandler>", "Selection change handler"),
                    ("on_row_click", "Option<EventHandler>", "Row click handler"),
                    ("empty_message", "&'static str", "Empty state message"),
                    ("loading", "bool", "Loading state"),
                    ("style", "Option", "Custom inline styles"),
                    ("id", "Option", "Element id"),
                    ("class", "Option", "CSS class"),
                    ("search_placeholder", "&'static str", "Search placeholder text"),
                    ("search_query", "Option", "Search query for filtering"),
                    ("on_search_change", "Option<EventHandler>", "Search change handler"),
                    ("filters", "Vec", "Custom filter definitions"),
                    ("active_filters", "HashMap", "Active filter values"),
                    ("on_filter_change", "Option<EventHandler>", "Filter change handler"),
                    ("show_search", "bool", "Show search input"),
                    ("show_filters", "bool", "Show filter controls")
                ] }
                h4 { "DataTableHeaderProps" }
                PropsTable { props: vec![
                    ("columns", "Vec", "Column definitions"),
                    ("selectable", "bool", ""),
                    ("selected_count", "usize", ""),
                    ("total_count", "usize", ""),
                    ("on_select_all", "Option<EventHandler>", ""),
                    ("all_keys", "Vec", ""),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "DataTableRowProps" }
                PropsTable { props: vec![
                    ("row", "T", ""),
                    ("columns", "Vec", ""),
                    ("key_extractor", "fn", ""),
                    ("selectable", "bool", ""),
                    ("is_selected", "bool", ""),
                    ("on_select", "Option<EventHandler>", ""),
                    ("on_click", "Option<EventHandler>", ""),
                    ("selected_keys", "Vec", ""),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "DataTableCellProps" }
                PropsTable { props: vec![
                    ("row", "T", ""),
                    ("column", "TableColumn", ""),
                    ("base_style", "String", ""),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "DataTableFilterProps" }
                PropsTable { props: vec![
                    ("filter", "TableFilter", ""),
                    ("active_value", "String", ""),
                    ("on_change", "Option<EventHandler>", ""),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "PaginationProps" }
                PropsTable { props: vec![
                    ("current_page", "usize", ""),
                    ("total_pages", "usize", ""),
                    ("on_page_change", "EventHandler", ""),
                    ("show_first_last", "bool", ""),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
            }
        }
    }
}

#[component]
pub fn StepperWizardPage() -> Element {
    use dioxus_ui_system::molecules::StepItem;
    use dioxus_ui_system::organisms::{
        CompactStepper, StepSummary, StepSummaryItem, Wizard, WizardStep,
    };

    let wizard_steps = vec![
        WizardStep::new("Personal Information").with_description("Enter your personal details"),
        WizardStep::new("Account Setup").with_description("Create your account credentials"),
        WizardStep::new("Preferences").with_description("Set your preferences"),
        WizardStep::new("Review").with_description("Review your information"),
    ];

    let compact_steps = vec![
        StepItem::new("Personal Info"),
        StepItem::new("Account"),
        StepItem::new("Preferences"),
        StepItem::new("Review"),
    ];

    let summary_steps = vec![
        StepSummaryItem::new("Full Name", "Alice Johnson"),
        StepSummaryItem::new("Email", "alice@example.com"),
        StepSummaryItem::new("Username", "alice_j"),
        StepSummaryItem::new("Plan", "Pro Plan"),
    ];

    rsx! {
        DocPage {
            title: "Stepper Wizard",
            description: "Full-featured wizard with validation for multi-step forms.",

            Section { title: "Basic Wizard",
                ExampleBox {
                    Box { style: "border: 1px solid rgb(226,232,240); border-radius: 12px; overflow: hidden;",
                        Wizard {
                            steps: wizard_steps.clone(),
                            active_step: 0,
                            on_step_change: move |_| {},
                            on_finish: move |_| {},

                            // Step 1: Personal Info
                            Box { style: "padding: 24px;",
                                h3 { style: "margin: 0 0 16px 0;", "Personal Information" }
                                VStack { gap: SpacingSize::Md,
                                    HStack { gap: SpacingSize::Sm,
                                        Box { style: "flex: 1;",
                                            Label { "First Name" }
                                            Input { value: "Alice".to_string(), on_change: move |_| {} }
                                        }
                                        Box { style: "flex: 1;",
                                            Label { "Last Name" }
                                            Input { value: "Johnson".to_string(), on_change: move |_| {} }
                                        }
                                    }
                                    Box {
                                        Label { "Email" }
                                        Input { value: "alice@example.com".to_string(), on_change: move |_| {} }
                                    }
                                }
                            }

                            // Step 2: Account
                            Box { style: "padding: 24px;",
                                h3 { style: "margin: 0 0 16px 0;", "Account Setup" }
                                VStack { gap: SpacingSize::Md,
                                    Box {
                                        Label { "Username" }
                                        Input { value: "alice_j".to_string(), on_change: move |_| {} }
                                    }
                                    Box {
                                        Label { "Password" }
                                        Input { input_type: InputType::Password, value: "".to_string(), on_change: move |_| {} }
                                    }
                                }
                            }

                            // Step 3: Preferences
                            Box { style: "padding: 24px;",
                                h3 { style: "margin: 0 0 16px 0;", "Preferences" }
                                VStack { gap: SpacingSize::Sm,
                                    HStack { gap: SpacingSize::Sm, style: "align-items: center;",
                                        input { r#type: "checkbox", checked: "true" }
                                        Label { "Receive email notifications" }
                                    }
                                    HStack { gap: SpacingSize::Sm, style: "align-items: center;",
                                        input { r#type: "checkbox" }
                                        Label { "Enable two-factor authentication" }
                                    }
                                    HStack { gap: SpacingSize::Sm, style: "align-items: center;",
                                        input { r#type: "checkbox", checked: "true" }
                                        Label { "Subscribe to newsletter" }
                                    }
                                }
                            }

                            // Step 4: Review
                            Box { style: "padding: 24px;",
                                h3 { style: "margin: 0 0 16px 0;", "Review Your Information" }
                                StepSummary {
                                    steps: summary_steps,
                                    editable: true,
                                    on_edit: move |_idx| {},
                                }
                            }
                        }
                    }
                }
            }

            Section { title: "Compact Stepper",
                ExampleBox {
                    Box { style: "padding: 16px;",
                        CompactStepper {
                            steps: compact_steps,
                            active_step: 1,
                            on_step_click: move |_| {},
                        }
                    }
                }
                CodeBlock { code: "let steps = vec![
    WizardStep::new(\"Step 1\").with_description(\"Description\"),
    WizardStep::new(\"Step 2\").with_description(\"Description\"),
];

Wizard {{
    steps: steps,
    active_step: 0,
    on_step_change: move |step| {{}},
    on_finish: move |_| {{}},
    // Step content goes here
}}".to_string() }
            }

            Section { title: "Validation States",
                ExampleBox {
                    VStack { gap: SpacingSize::Sm,
                        p { "Steps can have validation states:" }
                        ul {
                            li { "Valid - Step is complete and valid" }
                            li { "Invalid - Step has errors that need to be fixed" }
                            li { "Pending - Step is not yet started" }
                        }
                        Box { style: "margin-top: 16px; padding: 16px; background: rgb(241,245,249); border-radius: 8px;",
                            code { "WizardStep::new(\"Name\").valid(true)" }
                        }
                    }
                }
            }

            Section { title: "Props",
                h4 { "StepperProps" }
                PropsTable { props: vec![
                    ("steps", "Vec", "List of steps"),
                    ("active_step", "usize", "Current active step index"),
                    ("variant", "StepperVariant", "Stepper variant"),
                    ("size", "StepSize", "Step size"),
                    ("title", "Option", "Optional title"),
                    ("description", "Option", "Optional description"),
                    ("on_step_change", "Option<EventHandler>", "Step change callback"),
                    ("show_numbers", "bool", "Show step numbers"),
                    ("allow_back_navigation", "bool", "Allow clicking on completed steps"),
                    ("show_content", "bool", "Show content area"),
                    ("children", "Element", "Content elements for each step"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "CompactStepperProps" }
                PropsTable { props: vec![
                    ("steps", "Vec", "List of steps"),
                    ("active_step", "usize", "Current active step"),
                    ("size", "StepSize", "Step size"),
                    ("on_step_click", "Option<EventHandler>", "Click handler"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "WizardProps" }
                PropsTable { props: vec![
                    ("steps", "Vec", "List of steps"),
                    ("active_step", "usize", "Current active step"),
                    ("on_step_change", "EventHandler", "Step change callback"),
                    ("on_finish", "EventHandler", "Finish callback"),
                    ("on_cancel", "Option<EventHandler>", "Cancel callback"),
                    ("title", "Option", "Wizard title"),
                    ("show_progress", "bool", "Show progress summary"),
                    ("validate", "bool", "Enable validation"),
                    ("children", "Element", "Step content"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "StepSummaryProps" }
                PropsTable { props: vec![
                    ("steps", "Vec", "Steps data"),
                    ("editable", "bool", "Show edit buttons"),
                    ("on_edit", "Option<EventHandler>", "Edit callback"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
            }
        }
    }
}

#[component]
pub fn ChartsPage() -> Element {
    use dioxus_ui_system::organisms::charts::*;
    use dioxus_ui_system::theme::tokens::Color;

    // Sample data for charts
    let bar_data = vec![
        ChartDataPoint::new("Jan", 100.0),
        ChartDataPoint::new("Feb", 150.0),
        ChartDataPoint::new("Mar", 200.0),
        ChartDataPoint::new("Apr", 180.0),
        ChartDataPoint::new("May", 250.0),
        ChartDataPoint::new("Jun", 300.0),
    ];

    let line_data = vec![
        ChartDataPoint::new("Mon", 20.0),
        ChartDataPoint::new("Tue", 45.0),
        ChartDataPoint::new("Wed", 30.0),
        ChartDataPoint::new("Thu", 60.0),
        ChartDataPoint::new("Fri", 55.0),
        ChartDataPoint::new("Sat", 80.0),
        ChartDataPoint::new("Sun", 65.0),
    ];

    let pie_data = vec![
        ChartDataPoint::new("Desktop", 45.0),
        ChartDataPoint::new("Mobile", 35.0),
        ChartDataPoint::new("Tablet", 20.0),
    ];

    let sparkline_data = vec![10.0, 15.0, 8.0, 20.0, 25.0, 18.0, 30.0, 22.0, 35.0, 28.0];

    rsx! {
        DocPage {
            title: "Charts",
            description: "Pure Rust data visualization components with no external dependencies.",

            Section { title: "Bar Chart",
                ExampleBox {
                    BarChart {
                        title: Some("Monthly Revenue".to_string()),
                        data: Some(bar_data.clone()),
                        width: "100%".to_string(),
                        height: "300px".to_string(),
                        variant: BarChartVariant::Vertical,
                        show_values: false,
                    }
                }
                CodeBlock { code: "BarChart {{
    title: Some(\"Monthly Revenue\".to_string()),
    data: Some(vec![
        ChartDataPoint::new(\"Jan\", 100.0),
        ChartDataPoint::new(\"Feb\", 150.0),
        // ...
    ]),
    width: \"100%\".to_string(),
    height: \"300px\".to_string(),
}}".to_string() }
            }

            Section { title: "Bar Chart with Tooltips",
                p { "Hover over bars to see detailed values:" }
                ExampleBox {
                    BarChart {
                        title: Some("Sales by Region".to_string()),
                        data: Some(bar_data.clone()),
                        width: "100%".to_string(),
                        height: "300px".to_string(),
                        variant: BarChartVariant::Vertical,
                        tooltip: ChartTooltip::default(),
                    }
                }
                CodeBlock { code: "// Basic tooltip (shows label and value)
BarChart {{
    data: Some(bar_data.clone()),
    tooltip: ChartTooltip::default(),
}}

// Custom tooltip formatter
BarChart {{
    data: Some(bar_data.clone()),
    tooltip: ChartTooltip::with_formatter(|point, series| {{
        format!(\"{{}}: ${{:.2}}\", point.label, point.value)
    }}),
}}

// Disable tooltips
BarChart {{
    data: Some(bar_data.clone()),
    tooltip: ChartTooltip::disabled(),
}}".to_string() }
            }

            Section { title: "Line Chart",
                ExampleBox {
                    LineChart {
                        title: Some("Weekly Traffic".to_string()),
                        data: Some(line_data.clone()),
                        width: "100%".to_string(),
                        height: "250px".to_string(),
                        variant: LineChartVariant::Line,
                        show_points: true,
                        tooltip: ChartTooltip::default(),
                    }
                }
            }

            Section { title: "Multi-Series Line Chart with Tooltips",
                p { "Hover over points to see values for each series:" }
                ExampleBox {
                    LineChart {
                        title: Some("Revenue Comparison".to_string()),
                        series: Some(vec![
                            ChartSeries::new(
                                "This Year",
                                Color::new(59, 130, 246),
                                vec![
                                    ChartDataPoint::new("Q1", 120.0),
                                    ChartDataPoint::new("Q2", 180.0),
                                    ChartDataPoint::new("Q3", 220.0),
                                    ChartDataPoint::new("Q4", 280.0),
                                ]
                            ),
                            ChartSeries::new(
                                "Last Year",
                                Color::new(148, 163, 184),
                                vec![
                                    ChartDataPoint::new("Q1", 100.0),
                                    ChartDataPoint::new("Q2", 140.0),
                                    ChartDataPoint::new("Q3", 180.0),
                                    ChartDataPoint::new("Q4", 220.0),
                                ]
                            ),
                        ]),
                        width: "100%".to_string(),
                        height: "250px".to_string(),
                        show_points: true,
                        tooltip: ChartTooltip::with_formatter(|point, series| {
                            let series_name = series.unwrap_or("");
                            format!("{} - {}: ${:.0}K", series_name, point.label, point.value)
                        }),
                    }
                }
                CodeBlock { code: "// Multi-series with custom tooltips
LineChart {{
    series: Some(vec![
        ChartSeries::new(\"This Year\", color1, data1),
        ChartSeries::new(\"Last Year\", color2, data2),
    ]),
    tooltip: ChartTooltip::with_formatter(|point, series| {{
        format!(\"{{}} - {{}}: ${{:.0}}K\", 
            series.unwrap_or(\"\"), 
            point.label, 
            point.value
        )
    }}),
}}".to_string() }
            }

            Section { title: "Area Chart",
                ExampleBox {
                    LineChart {
                        title: Some("Weekly Traffic (Area)".to_string()),
                        data: Some(line_data.clone()),
                        width: "100%".to_string(),
                        height: "250px".to_string(),
                        variant: LineChartVariant::Area,
                        show_points: true,
                    }
                }
            }

            Section { title: "Pie Chart",
                ExampleBox {
                    Box { style: "display: flex; justify-content: center;",
                        PieChart {
                            title: Some("Traffic Sources".to_string()),
                            data: pie_data.clone(),
                            width: "400px".to_string(),
                            height: "300px".to_string(),
                            show_labels: true,
                            legend_position: LegendPosition::Right,
                            tooltip: ChartTooltip::default(),
                        }
                    }
                }
                CodeBlock { code: "PieChart {{
    title: Some(\"Traffic Sources\".to_string()),
    data: vec![
        ChartDataPoint::new(\"Desktop\", 45.0),
        ChartDataPoint::new(\"Mobile\", 35.0),
        ChartDataPoint::new(\"Tablet\", 20.0),
    ],
    width: \"400px\".to_string(),
    height: \"300px\".to_string(),
    show_labels: true,
    legend_position: LegendPosition::Right,
    tooltip: ChartTooltip::default(),
}}".to_string() }
            }

            Section { title: "Pie Chart with Custom Tooltips",
                p { "Hover over slices to see custom formatted values:" }
                ExampleBox {
                    Box { style: "display: flex; justify-content: center;",
                        PieChart {
                            title: Some("Market Share".to_string()),
                            data: vec![
                                ChartDataPoint::new("Product A", 4500.0),
                                ChartDataPoint::new("Product B", 3200.0),
                                ChartDataPoint::new("Product C", 1800.0),
                                ChartDataPoint::new("Product D", 1200.0),
                            ],
                            width: "350px".to_string(),
                            height: "300px".to_string(),
                            show_labels: false,
                            legend_position: LegendPosition::Right,
                            tooltip: ChartTooltip::with_formatter(|point, _| {
                                format!("{}: {} units ({:.1}%)",
                                    point.label,
                                    point.value as i32,
                                    point.value
                                )
                            }),
                        }
                    }
                }
                CodeBlock { code: "PieChart {{
    data: vec![
        ChartDataPoint::new(\"Product A\", 4500.0),
        ChartDataPoint::new(\"Product B\", 3200.0),
        // ...
    ],
    tooltip: ChartTooltip::with_formatter(|point, _| {{
        format!(\"{{}}: {{}} units\", point.label, point.value as i32)
    }}),
}}".to_string() }
            }

            Section { title: "Donut Chart",
                ExampleBox {
                    Box { style: "display: flex; justify-content: center;",
                        DonutChart {
                            data: pie_data.clone(),
                            width: "300px".to_string(),
                            height: "300px".to_string(),
                            show_center_text: true,
                            legend_position: LegendPosition::Bottom,
                        }
                    }
                }
            }

            Section { title: "Gauge Chart",
                ExampleBox {
                    Box { style: "display: flex; justify-content: center;",
                        GaugeChart {
                            data: vec![
                                ChartDataPoint::new("Completed", 75.0),
                                ChartDataPoint::new("Remaining", 25.0),
                            ],
                            width: "300px".to_string(),
                            height: "200px".to_string(),
                            legend_position: LegendPosition::None,
                        }
                    }
                }
            }

            Section { title: "Sparkline",
                ExampleBox {
                    VStack { gap: SpacingSize::Md,
                        HStack { gap: SpacingSize::Sm, style: "align-items: center;",
                            span { "Revenue:" }
                            Sparkline {
                                data: sparkline_data.clone(),
                                width: "120px".to_string(),
                                height: "30px".to_string(),
                                variant: SparklineVariant::Line,
                                show_last_point: true,
                            }
                        }
                        HStack { gap: SpacingSize::Sm, style: "align-items: center;",
                            span { "Users:" }
                            Sparkline {
                                data: vec![50.0, 45.0, 55.0, 48.0, 60.0, 58.0, 65.0],
                                width: "120px".to_string(),
                                height: "30px".to_string(),
                                variant: SparklineVariant::Area,
                                color: Some(Color::new(34, 197, 94)),
                                show_last_point: true,
                            }
                        }
                        HStack { gap: SpacingSize::Sm, style: "align-items: center;",
                            span { "Sales:" }
                            Sparkline {
                                data: vec![30.0, 35.0, 32.0, 40.0, 38.0, 45.0, 42.0],
                                width: "120px".to_string(),
                                height: "30px".to_string(),
                                variant: SparklineVariant::Bars,
                                color: Some(Color::new(234, 179, 8)),
                            }
                        }
                    }
                }
                CodeBlock { code: "Sparkline {{
    data: vec![10.0, 15.0, 8.0, 20.0, 25.0],
    width: \"120px\".to_string(),
    height: \"30px\".to_string(),
    variant: SparklineVariant::Line,
    show_last_point: true,
}}".to_string() }
            }

            Section { title: "Trend Indicator",
                ExampleBox {
                    VStack { gap: SpacingSize::Md,
                        HStack { gap: SpacingSize::Sm, style: "align-items: center;",
                            span { "Stock Price:" }
                            TrendIndicator {
                                data: vec![100.0, 105.0, 103.0, 110.0, 115.0, 112.0, 120.0],
                                width: "100px".to_string(),
                                show_percentage: true,
                            }
                        }
                        HStack { gap: SpacingSize::Sm, style: "align-items: center;",
                            span { "Active Users:" }
                            TrendIndicator {
                                data: vec![1000.0, 950.0, 900.0, 880.0, 850.0],
                                width: "100px".to_string(),
                                show_percentage: true,
                            }
                        }
                    }
                }
            }

            Section { title: "Chart Types Reference",
                p { "Available chart components:" }
                ul {
                    li { "BarChart - Vertical, horizontal, stacked, and grouped bar charts" }
                    li { "LineChart - Lines with smooth/step variants, area fills" }
                    li { "PieChart - Standard pie charts" }
                    li { "DonutChart - Hollow-center pie charts" }
                    li { "GaugeChart - Semi-circular gauge charts" }
                    li { "Sparkline - Mini inline charts" }
                    li { "TrendIndicator - Sparkline with trend arrow" }
                }
            }

            Section { title: "Tooltip Configuration",
                p { "All chart components support tooltips for better data exploration:" }
                ul {
                    li { "ChartTooltip::default() - Shows label and formatted value" }
                    li { "ChartTooltip::with_formatter(f) - Custom tooltip content" }
                    li { "ChartTooltip::disabled() - Hide tooltips" }
                    li { "show_series_name - Include series name in tooltip (default: true)" }
                    li { "show_value - Include value in tooltip (default: true)" }
                    li { "value_format - Custom value formatter function" }
                }
                CodeBlock { code: "use dioxus_ui_system::organisms::charts::ChartTooltip;

// Default tooltip
ChartTooltip::default()

// Custom formatter
ChartTooltip::with_formatter(|point, series| {{
    format!(\"{{}}: ${{:.2}}\", point.label, point.value)
}})

// Disable tooltips
ChartTooltip::disabled()

// Advanced configuration
ChartTooltip {{
    enabled: true,
    show_series_name: true,
    show_value: true,
    value_format: Some(format_currency),
    ..Default::default()
}}".to_string() }
            }

            Section { title: "Props",
                h4 { "BarChartProps" }
                PropsTable { props: vec![
                    ("title", "Option", "Chart title"),
                    ("data", "Option<Vec>", "Single series data (for simple charts)"),
                    ("series", "Option<Vec>", "Multiple series data (for complex charts)"),
                    ("width", "String", "Chart width (e.g., \"100%\", \"600px\")"),
                    ("height", "String", "Chart height"),
                    ("variant", "BarChartVariant", "Chart variant"),
                    ("margin", "ChartMargin", "Chart margins"),
                    ("x_axis", "ChartAxis", "X-axis configuration"),
                    ("y_axis", "ChartAxis", "Y-axis configuration"),
                    ("bar_color", "Option", "Bar color (for single series)"),
                    ("bar_radius", "u8", "Bar corner radius"),
                    ("bar_gap", "f32", "Gap between bars (0.0 - 1.0, as percentage of bar width)"),
                    ("show_values", "bool", "Show values on bars"),
                    ("value_format", "Option String>", "Value formatter"),
                    ("legend_position", "LegendPosition", "Legend position"),
                    ("tooltip", "ChartTooltip", "Tooltip configuration"),
                    ("animation", "ChartAnimation", "Animation configuration"),
                    ("on_bar_click", "Option<EventHandler>", "Click handler for bars"),
                    ("style", "Option", "Custom styles"),
                    ("id", "Option", ""),
                    ("class", "Option", "")
                ] }
                h4 { "LineChartProps" }
                PropsTable { props: vec![
                    ("title", "Option", "Chart title"),
                    ("data", "Option<Vec>", "Single series data"),
                    ("series", "Option<Vec>", "Multiple series data"),
                    ("width", "String", "Chart width"),
                    ("height", "String", "Chart height"),
                    ("variant", "LineChartVariant", "Chart variant"),
                    ("margin", "ChartMargin", "Chart margins"),
                    ("x_axis", "ChartAxis", "X-axis configuration"),
                    ("y_axis", "ChartAxis", "Y-axis configuration"),
                    ("line_color", "Option", "Line color (for single series)"),
                    ("line_width", "u8", "Line width"),
                    ("show_points", "bool", "Show data points"),
                    ("point_radius", "u8", "Point radius"),
                    ("show_values", "bool", "Show values on points"),
                    ("value_format", "Option String>", "Value formatter"),
                    ("legend_position", "LegendPosition", "Legend position"),
                    ("tooltip", "ChartTooltip", "Tooltip configuration"),
                    ("animation", "ChartAnimation", "Animation configuration"),
                    ("on_point_click", "Option<EventHandler>", "Click handler for points"),
                    ("style", "Option", "Custom styles"),
                    ("id", "Option", ""),
                    ("class", "Option", "")
                ] }
                h4 { "PieChartProps" }
                PropsTable { props: vec![
                    ("title", "Option", "Chart title"),
                    ("data", "Vec", "Data points"),
                    ("width", "String", "Chart width"),
                    ("height", "String", "Chart height"),
                    ("variant", "PieChartVariant", "Chart variant"),
                    ("inner_radius_pct", "f32", "Inner radius (for donut, as percentage of outer radius)"),
                    ("start_angle", "f32", "Start angle (in degrees, 0 = 3 o'clock)"),
                    ("corner_radius", "u8", "Corner radius for slices"),
                    ("slice_gap", "f32", "Gap between slices (in degrees)"),
                    ("show_labels", "bool", "Show labels on slices"),
                    ("show_percentages", "bool", "Show percentage labels"),
                    ("label_format", "Option String>", "Label formatter"),
                    ("legend_position", "LegendPosition", "Legend position"),
                    ("tooltip", "ChartTooltip", "Tooltip configuration"),
                    ("animation", "ChartAnimation", "Animation configuration"),
                    ("on_slice_click", "Option<EventHandler>", "Click handler for slices"),
                    ("style", "Option", "Custom styles"),
                    ("colors", "Option<Vec>", "Colors for slices (uses theme colors if not provided)"),
                    ("id", "Option", ""),
                    ("class", "Option", "")
                ] }
                h4 { "SparklineProps" }
                PropsTable { props: vec![
                    ("data", "Vec", "Data values"),
                    ("width", "String", "Chart width"),
                    ("height", "String", "Chart height"),
                    ("variant", "SparklineVariant", "Sparkline variant"),
                    ("color", "Option", "Line/bar color"),
                    ("fill_color", "Option", "Fill color for area (uses color with opacity if not set)"),
                    ("line_width", "f32", "Line width"),
                    ("show_last_point", "bool", "Show current value indicator"),
                    ("show_min_max", "bool", "Show min/max indicators"),
                    ("animation", "ChartAnimation", "Animation configuration"),
                    ("onclick", "Option<EventHandler>", "Click handler"),
                    ("style", "Option", "Custom styles"),
                    ("id", "Option", ""),
                    ("class", "Option", "")
                ] }
            }
        }
    }
}

/// Footer documentation page
#[component]
pub fn FooterPage() -> Element {
    rsx! {
        DocPage {
            title: "Footer",
            description: "Page footer with links, branding, and legal information.",

            Section { title: "Default Footer",
                ExampleBox {
                    Footer {
                        brand: Some(rsx! { span { style: "font-size: 20px; font-weight: 700;", "Brand" } }),
                        description: Some("A modern design system for Dioxus.".to_string()),
                        link_groups: vec![
                            FooterLinkGroup::new("Product").with_links(vec![
                                FooterLink::new("Features", "#"),
                                FooterLink::new("Pricing", "#"),
                                FooterLink::new("Docs", "#"),
                            ]),
                            FooterLinkGroup::new("Company").with_links(vec![
                                FooterLink::new("About", "#"),
                                FooterLink::new("Blog", "#"),
                                FooterLink::new("Careers", "#"),
                            ]),
                        ],
                        copyright: Some("© 2024 Company Inc.".to_string()),
                        variant: FooterVariant::Default
                    }
                }
            }

            Section { title: "Simple Footer",
                ExampleBox {
                    SimpleFooter {
                        brand: Some(rsx! { span { style: "font-weight: 600;", "Logo" } }),
                        links: vec![
                            FooterLink::new("Privacy", "#"),
                            FooterLink::new("Terms", "#"),
                        ],
                        copyright: "© 2024".to_string()
                    }
                }
                CodeBlock { code: "Footer {{
    brand: Some(rsx! {{ span {{ \"Brand\" }} }}),
    description: Some(\"Description text\".to_string()),
    link_groups: vec![
        FooterLinkGroup::new(\"Product\").with_links(vec![
            FooterLink::new(\"Features\", \"/features\"),
        ]),
    ],
    copyright: Some(\"© 2024\".to_string()),
}}".to_string() }
            }

            Section { title: "Props",
                h4 { "FooterProps" }
                PropsTable { props: vec![
                    ("brand", "Option", "Brand/logo element"),
                    ("description", "Option", "Brand description"),
                    ("link_groups", "Vec", "Link groups"),
                    ("social_links", "Vec, // (icon, url)", "Social links"),
                    ("bottom_content", "Option", "Bottom bar content"),
                    ("copyright", "Option", "Copyright text"),
                    ("variant", "FooterVariant", "Variant"),
                    ("class", "Option", "Additional CSS classes"),
                    ("id", "Option", "Element id"),
                    ("style", "Option", "Custom inline styles")
                ] }
                h4 { "SimpleFooterProps" }
                PropsTable { props: vec![
                    ("brand", "Option", "Logo/brand element"),
                    ("links", "Vec", "Links"),
                    ("copyright", "String", "Copyright text"),
                    ("class", "Option", "Additional CSS classes"),
                    ("id", "Option", "Element id"),
                    ("style", "Option", "Custom inline styles")
                ] }
            }
        }
    }
}

/// Notification Center documentation page
#[component]
pub fn NotificationCenterPage() -> Element {
    rsx! {
        DocPage {
            title: "Notification Center",
            description: "Centralized alert management with categorization.",

            Section { title: "Notification Center",
                ExampleBox {
                    NotificationCenter {
                        notifications: vec![
                            Notification::new("1", "New Message", "You have a new message from John").with_variant(ToastVariant::Info),
                            Notification::new("2", "Success!", "Your changes have been saved.").with_variant(ToastVariant::Success),
                        ],
                        unread_count: 2
                    }
                }
            }

            Section { title: "Banner Alert",
                ExampleBox {
                    VStack { gap: SpacingSize::Md,
                        BannerAlert { message: "This is an info banner".to_string(), variant: ToastVariant::Info, dismissible: true }
                        BannerAlert { message: "Success! Operation completed.".to_string(), variant: ToastVariant::Success, dismissible: true }
                        BannerAlert { message: "Warning: Check your settings.".to_string(), variant: ToastVariant::Warning, dismissible: true }
                    }
                }
                CodeBlock { code: "NotificationCenter {{
    notifications: vec![
        Notification::new(\"1\", \"Title\", \"Message\")
            .with_variant(ToastVariant::Info),
    ],
    unread_count: 1,
}}".to_string() }
            }

            Section { title: "Props",
                h4 { "NotificationCenterProps" }
                PropsTable { props: vec![
                    ("notifications", "Vec", "Notifications list"),
                    ("unread_count", "usize", "Unread count"),
                    ("on_mark_read", "Option<EventHandler>", "On mark as read"),
                    ("on_mark_all_read", "Option<EventHandler>", "On mark all read"),
                    ("on_dismiss", "Option<EventHandler>", "On dismiss"),
                    ("on_action", "Option<EventHandler>", "On action click"),
                    ("class", "Option", "Additional CSS classes"),
                    ("id", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "BannerAlertProps" }
                PropsTable { props: vec![
                    ("message", "String", "Alert message"),
                    ("variant", "ToastVariant", "Alert variant"),
                    ("dismissible", "bool", "Dismissible"),
                    ("on_dismiss", "Option<EventHandler>", "On dismiss"),
                    ("action", "Option", "Action button text"),
                    ("on_action", "Option<EventHandler>", "On action"),
                    ("class", "Option", "Additional CSS classes"),
                    ("id", "Option", ""),
                    ("style", "Option", "")
                ] }
            }
        }
    }
}

/// Hero documentation page
#[component]
pub fn HeroPage() -> Element {
    rsx! {
        DocPage {
            title: "Hero",
            description: "Prominent page header with CTA for landing pages.",

            Section { title: "Basic Hero",
                ExampleBox {
                    Hero {
                        title: "Build faster with Dioxus UI".to_string(),
                        subtitle: Some("A comprehensive design system with 100+ components for building modern web applications.".to_string()),
                        primary_cta: Some(HeroCta::new("Get Started").with_href("#")),
                        secondary_cta: Some(HeroCta::new("Learn More").with_href("#")),
                        align: HeroAlign::Center, size: HeroSize::Lg
                    }
                }
            }

            Section { title: "With Features",
                ExampleBox {
                    Hero {
                        title: "Everything you need".to_string(),
                        subtitle: Some("Build better products with our comprehensive toolkit.".to_string()),
                        features: vec![
                            "100+ Components".to_string(),
                            "Type Safe".to_string(),
                            "Customizable".to_string(),
                        ],
                        primary_cta: Some(HeroCta::new("Get Started")),
                        align: HeroAlign::Center, size: HeroSize::Lg
                    }
                }
            }

            Section { title: "Social Proof",
                ExampleBox {
                    SocialProofBar {
                        text: "Trusted by 10,000+ developers".to_string(),
                        items: vec![
                            rsx! { div { style: "width: 32px; height: 32px; background: #3b82f6; border-radius: 50%;", "" } },
                            rsx! { div { style: "width: 32px; height: 32px; background: #22c55e; border-radius: 50%;", "" } },
                            rsx! { div { style: "width: 32px; height: 32px; background: #f59e0b; border-radius: 50%;", "" } },
                        ]
                    }
                }
                CodeBlock { code: "Hero {{
    title: \"Build faster\".to_string(),
    subtitle: Some(\"Description here\".to_string()),
    primary_cta: Some(HeroCta::new(\"Get Started\").with_href(\"#\")),
    secondary_cta: Some(HeroCta::new(\"Learn More\").with_href(\"#\")),
    align: HeroAlign::Center,
    size: HeroSize::Lg,
}}".to_string() }
            }

            Section { title: "Props",
                h4 { "HeroProps" }
                PropsTable { props: vec![
                    ("title", "String", "Main headline"),
                    ("subtitle", "Option", "Subtitle/description"),
                    ("background", "Option", "Background element (image, gradient, video)"),
                    ("primary_cta", "Option", "Primary CTA button"),
                    ("secondary_cta", "Option", "Secondary CTA button"),
                    ("social_proof", "Option", "Social proof element"),
                    ("features", "Vec", "Feature highlights"),
                    ("align", "HeroAlign", "Alignment"),
                    ("size", "HeroSize", "Size variant"),
                    ("class", "Option", "Additional CSS classes"),
                    ("id", "Option", "Element id"),
                    ("style", "Option", "Custom inline styles")
                ] }
                h4 { "HeroWithImageProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Hero content"),
                    ("image", "Element", "Image element"),
                    ("image_position", "ImagePosition", "Image position"),
                    ("class", "Option", "Additional CSS classes"),
                    ("id", "Option", "Element id"),
                    ("style", "Option", "Custom inline styles")
                ] }
                h4 { "SocialProofBarProps" }
                PropsTable { props: vec![
                    ("text", "String", "Text (e.g., \"Trusted by 10,000+ companies\")"),
                    ("items", "Vec", "Logos or avatars"),
                    ("class", "Option", "Additional CSS classes"),
                    ("id", "Option", "Element id"),
                    ("style", "Option", "Custom inline styles")
                ] }
            }
        }
    }
}

/// File Upload documentation page
#[component]
pub fn FileUploadPage() -> Element {
    // State for tracking selected files in each example
    let mut single_file = use_signal(|| Vec::<UploadedFile>::new());
    let mut multiple_files = use_signal(|| Vec::<UploadedFile>::new());
    let mut image_files = use_signal(|| Vec::<UploadedFile>::new());
    let mut doc_files = use_signal(|| Vec::<UploadedFile>::new());

    rsx! {
        DocPage {
            title: "File Upload",
            description: "Drag-and-drop file upload with file name, type, and size display. Supports single/multiple files, file type filtering, and size limits.",

            Section { title: "Single File Upload",
                p { "Upload a single file with type and size validation:" }
                ExampleBox {
                    VStack { gap: SpacingSize::Md,
                        FileUpload {
                            on_upload: EventHandler::new(move |files: Vec<UploadedFile>| {
                                single_file.set(files);
                            }),
                            on_change: Some(EventHandler::new(move |files: Vec<UploadedFile>| {
                                single_file.set(files);
                            })),
                            single: true,
                            max_size_mb: Some(5.0),
                            accept: Some(".pdf,.doc,.docx".to_string()),
                            label: Some("Upload Document".to_string()),
                            show_file_list: true,
                        }

                        // Display selected file info
                        if !single_file().is_empty() {
                            div {
                                style: "padding: 12px; background: rgb(241,245,249); border-radius: 8px; margin-top: 8px;",
                                h4 { style: "margin: 0 0 8px 0; font-size: 14px;", "Selected File:" }
                                FileInfoDisplay { file: single_file()[0].clone() }
                            }
                        }
                    }
                }
                CodeBlock { code: "// Single file with validation
let mut files = use_signal(|| Vec::<UploadedFile>::new());

FileUpload {{
    on_upload: EventHandler::new(move |f| files.set(f)),
    on_change: Some(EventHandler::new(move |f| files.set(f))),
    single: true,                              // Single file only
    max_size_mb: Some(5.0),                   // 5MB limit
    accept: Some(\".pdf,.doc,.docx\".to_string()), // Document types only
    label: Some(\"Upload Document\".to_string()),
}}".to_string() }
            }

            Section { title: "Multiple Files Upload",
                p { "Upload multiple files with individual file info display:" }
                ExampleBox {
                    VStack { gap: SpacingSize::Md,
                        FileUpload {
                            on_upload: EventHandler::new(move |files: Vec<UploadedFile>| {
                                multiple_files.set(files);
                            }),
                            on_change: Some(EventHandler::new(move |files: Vec<UploadedFile>| {
                                multiple_files.set(files);
                            })),
                            multiple: true,
                            max_files: 5,
                            max_size_mb: Some(10.0),
                            label: Some("Upload Files".to_string()),
                            show_file_list: true,
                        }

                        // Display all selected files
                        if !multiple_files().is_empty() {
                            div {
                                style: "padding: 12px; background: rgb(241,245,249); border-radius: 8px; margin-top: 8px;",
                                h4 { style: "margin: 0 0 12px 0; font-size: 14px;", "Selected Files ({multiple_files().len()}):" }
                                VStack { gap: SpacingSize::Sm,
                                    for file in multiple_files().iter().cloned() {
                                        FileInfoDisplay { file: file.clone() }
                                    }
                                }
                            }
                        }
                    }
                }
                CodeBlock { code: "// Multiple files with limits
FileUpload {{
    on_upload: EventHandler::new(move |f| files.set(f)),
    on_change: Some(EventHandler::new(move |f| files.set(f))),
    multiple: true,                           // Allow multiple
    max_files: 5,                             // Max 5 files
    max_size_mb: Some(10.0),                  // 10MB each
    label: Some(\"Upload Files\".to_string()),
    show_file_list: true,                     // Show file list
}}".to_string() }
            }

            Section { title: "Images Only",
                p { "Restrict to image files with previews:" }
                ExampleBox {
                    VStack { gap: SpacingSize::Md,
                        FileUpload {
                            on_upload: EventHandler::new(move |files: Vec<UploadedFile>| {
                                image_files.set(files);
                            }),
                            on_change: Some(EventHandler::new(move |files: Vec<UploadedFile>| {
                                image_files.set(files);
                            })),
                            multiple: true,
                            max_files: 3,
                            max_size_mb: Some(2.0),
                            accept: Some("image/*".to_string()),
                            label: Some("Upload Images".to_string()),
                            helper_text: Some("JPG, PNG, GIF up to 2MB each".to_string()),
                            show_file_list: true,
                        }

                        if !image_files().is_empty() {
                            div {
                                style: "padding: 12px; background: rgb(241,245,249); border-radius: 8px; margin-top: 8px;",
                                h4 { style: "margin: 0 0 12px 0; font-size: 14px;", "Selected Images:" }
                                div {
                                    style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: 12px;",
                                    for file in image_files().iter().cloned() {
                                        ImageFileCard { file: file.clone() }
                                    }
                                }
                            }
                        }
                    }
                }
                CodeBlock { code: "// Image uploads only
FileUpload {{
    accept: Some(\"image/*\".to_string()),   // Images only
    max_size_mb: Some(2.0),                  // 2MB limit
    max_files: 3,                            // Max 3 images
    label: Some(\"Upload Images\".to_string()),
}}".to_string() }
            }

            Section { title: "Documents with Type Filter",
                p { "Accept only specific document types:" }
                ExampleBox {
                    VStack { gap: SpacingSize::Md,
                        FileUpload {
                            on_upload: EventHandler::new(move |files: Vec<UploadedFile>| {
                                doc_files.set(files);
                            }),
                            on_change: Some(EventHandler::new(move |files: Vec<UploadedFile>| {
                                doc_files.set(files);
                            })),
                            multiple: true,
                            max_files: 4,
                            max_size_mb: Some(20.0),
                            accept: Some(".pdf,.doc,.docx,.xls,.xlsx,.ppt,.pptx".to_string()),
                            label: Some("Upload Documents".to_string()),
                            helper_text: Some("Office documents up to 20MB".to_string()),
                            show_file_list: true,
                        }

                        if !doc_files().is_empty() {
                            div {
                                style: "padding: 12px; background: rgb(241,245,249); border-radius: 8px; margin-top: 8px;",
                                h4 { style: "margin: 0 0 12px 0; font-size: 14px;", "Selected Documents:" }
                                VStack { gap: SpacingSize::Sm,
                                    for file in doc_files().iter().cloned() {
                                        DocumentFileCard { file: file.clone() }
                                    }
                                }
                            }
                        }
                    }
                }
                CodeBlock { code: "// Specific document types
FileUpload {{
    accept: Some(\".pdf,.doc,.docx,.xls,.xlsx\".to_string()),
    max_size_mb: Some(20.0),
    max_files: 4,
    label: Some(\"Upload Documents\".to_string()),
    helper_text: Some(\"Office documents up to 20MB\".to_string()),
}}".to_string() }
            }

            Section { title: "Configuration Options",
                p { "Available properties for the FileUpload component:" }
                ul {
                    li { code { "on_upload" }, " - Callback when files are uploaded" }
                    li { code { "on_change" }, " - Callback when file selection changes" }
                    li { code { "single" }, " - Allow only single file (default: false)" }
                    li { code { "multiple" }, " - Allow multiple files (default: false)" }
                    li { code { "max_files" }, " - Maximum number of files allowed" }
                    li { code { "max_size_mb" }, " - Maximum file size in MB" }
                    li { code { "accept" }, " - Accepted file types (MIME or extensions)" }
                    li { code { "label" }, " - Label text for the upload area" }
                    li { code { "helper_text" }, " - Helper text shown below label" }
                    li { code { "show_file_list" }, " - Show selected files list (default: true)" }
                    li { code { "disabled" }, " - Disable the upload component" }
                    li { code { "loading" }, " - Show loading state" }
                }
            }

            Section { title: "File Type Icons",
                p { "The component automatically detects and displays icons for different file types:" }
                ExampleBox {
                    VStack { gap: SpacingSize::Md,
                        HStack { gap: SpacingSize::Md, style: "flex-wrap: wrap;",
                            FileTypeBadge { icon: "🖼️", label: "Images", types: ".jpg, .png, .gif, .svg" }
                            FileTypeBadge { icon: "📄", label: "Documents", types: ".pdf, .doc, .txt" }
                            FileTypeBadge { icon: "🎬", label: "Videos", types: ".mp4, .mov, .avi" }
                            FileTypeBadge { icon: "🎵", label: "Audio", types: ".mp3, .wav, .aac" }
                            FileTypeBadge { icon: "📦", label: "Archives", types: ".zip, .rar, .7z" }
                            FileTypeBadge { icon: "📎", label: "Other", types: "Any other type" }
                        }
                    }
                }
            }

            Section { title: "Props",
                h4 { "FileUploadProps" }
                PropsTable { props: vec![
                    ("accept", "Option", "Accepted file types (MIME types or extensions, e.g., \"image/*\", \".pdf\")"),
                    ("max_size_mb", "Option", "Maximum file size in MB"),
                    ("max_files", "usize", "Maximum number of files"),
                    ("multiple", "bool", "Multiple files allowed"),
                    ("single", "bool", "Single file mode (alias for !multiple)"),
                    ("on_upload", "EventHandler<Vec>", "Upload handler"),
                    ("on_change", "Option<EventHandler<Vec>>", "Change handler (called when files are selected)"),
                    ("label", "Option", "Label"),
                    ("helper_text", "Option", "Helper text"),
                    ("error", "Option", "Error message"),
                    ("loading", "bool", "Loading state"),
                    ("disabled", "bool", "Disabled state"),
                    ("class", "Option", "Additional CSS classes"),
                    ("id", "Option", "Element id"),
                    ("style", "Option", "Custom inline styles"),
                    ("show_file_list", "bool", "Show file list")
                ] }
                h4 { "FileUploadItemProps" }
                PropsTable { props: vec![
                    ("file", "UploadedFile", ""),
                    ("on_remove", "EventHandler", ""),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
            }
        }
    }
}

/// File info display component
#[component]
fn FileInfoDisplay(file: UploadedFile) -> Element {
    let size_text = format_file_size(file.size);
    let file_type = FileType::from_file_name(&file.name);

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 12px; padding: 8px 12px; background: white; border-radius: 6px; border: 1px solid rgb(226,232,240);",

            // File type icon
            div {
                style: "font-size: 24px;",
                "{file_type.icon()}"
            }

            // File details
            div {
                style: "flex: 1;",
                p {
                    style: "margin: 0; font-size: 14px; font-weight: 500; color: rgb(15,23,42); white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                    "{file.name}"
                }
                p {
                    style: "margin: 4px 0 0 0; font-size: 12px; color: rgb(100,116,139);",
                    "{file.file_type} • {size_text}"
                }
            }
        }
    }
}

/// Image file card with preview
#[component]
fn ImageFileCard(file: UploadedFile) -> Element {
    let size_text = format_file_size(file.size);

    rsx! {
        div {
            style: "padding: 12px; background: white; border-radius: 8px; border: 1px solid rgb(226,232,240);",

            // Image placeholder/icon
            div {
                style: "height: 80px; background: linear-gradient(135deg, #e0e7ff 0%, #c7d2fe 100%); border-radius: 6px; display: flex; align-items: center; justify-content: center; font-size: 32px; margin-bottom: 8px;",
                "🖼️"
            }

            // File info
            p {
                style: "margin: 0; font-size: 13px; font-weight: 500; color: rgb(15,23,42); white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                "{file.name}"
            }
            p {
                style: "margin: 4px 0 0 0; font-size: 11px; color: rgb(100,116,139);",
                "{file.file_type} • {size_text}"
            }
        }
    }
}

/// Document file card
#[component]
fn DocumentFileCard(file: UploadedFile) -> Element {
    let size_text = format_file_size(file.size);
    let file_type = FileType::from_file_name(&file.name);
    let color = file_type.color();

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 12px; padding: 12px; background: white; border-radius: 8px; border: 1px solid rgb(226,232,240);",

            // Icon with color
            div {
                style: "width: 40px; height: 40px; border-radius: 8px; background: {color}20; display: flex; align-items: center; justify-content: center; font-size: 20px;",
                "{file_type.icon()}"
            }

            // File info
            div {
                style: "flex: 1;",
                p {
                    style: "margin: 0; font-size: 14px; font-weight: 500; color: rgb(15,23,42); white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                    "{file.name}"
                }
                p {
                    style: "margin: 4px 0 0 0; font-size: 12px; color: rgb(100,116,139);",
                    "{file.file_type} • {size_text}"
                }
            }
        }
    }
}

/// File type badge for documentation
#[component]
fn FileTypeBadge(icon: String, label: String, types: String) -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 8px; padding: 8px 12px; background: rgb(248,250,252); border-radius: 8px; border: 1px solid rgb(226,232,240);",
            span { style: "font-size: 20px;", "{icon}" }
            div {
                p { style: "margin: 0; font-size: 13px; font-weight: 500; color: rgb(15,23,42);", "{label}" }
                p { style: "margin: 2px 0 0 0; font-size: 11px; color: rgb(100,116,139);", "{types}" }
            }
        }
    }
}

/// Format file size helper
fn format_file_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.1} {}", size, UNITS[unit_index])
}

/// Confirmation Dialog documentation page
#[component]
pub fn ConfirmationDialogPage() -> Element {
    let mut show_delete = use_signal(|| false);
    let mut show_unsaved = use_signal(|| false);
    let mut show_signout = use_signal(|| false);

    rsx! {
        DocPage {
            title: "Confirmation Dialog",
            description: "Critical decision dialogs with clear action labeling.",

            Section { title: "Delete Confirmation",
                ExampleBox {
                    VStack { gap: SpacingSize::Md, style: "align-items: flex-start;",
                        Button { variant: ButtonVariant::Destructive, onclick: move |_| show_delete.set(true), "Delete Item" }

                        DeleteConfirmDialog {
                            open: show_delete(),
                            on_close: move |_| show_delete.set(false),
                            item_name: "My Document".to_string(),
                            on_confirm: EventHandler::new(move |_| show_delete.set(false)),
                            loading: false
                        }
                    }
                }
            }

            Section { title: "Unsaved Changes",
                ExampleBox {
                    VStack { gap: SpacingSize::Md, style: "align-items: flex-start;",
                        Button { onclick: move |_| show_unsaved.set(true), "Show Unsaved Dialog" }

                        UnsavedChangesDialog {
                            open: show_unsaved(),
                            on_close: move |_| show_unsaved.set(false),
                            on_save: EventHandler::new(move |_| show_unsaved.set(false)),
                            on_discard: EventHandler::new(move |_| show_unsaved.set(false)),
                        }
                    }
                }
            }

            Section { title: "Sign Out",
                ExampleBox {
                    VStack { gap: SpacingSize::Md, style: "align-items: flex-start;",
                        Button { onclick: move |_| show_signout.set(true), "Sign Out" }

                        SignOutDialog {
                            open: show_signout(),
                            on_close: move |_| show_signout.set(false),
                            on_confirm: EventHandler::new(move |_| show_signout.set(false)),
                        }
                    }
                }
                CodeBlock { code: "let mut show_delete = use_signal(|| false);

DeleteConfirmDialog {{
    open: show_delete(),
    on_close: move |_| show_delete.set(false),
    item_name: \"My Document\".to_string(),
    on_confirm: EventHandler::new(move |_| {{
        // Handle delete
        show_delete.set(false);
    }}),
    loading: false,
}}".to_string() }
            }

            Section { title: "Props",
                h4 { "ConfirmationDialogProps" }
                PropsTable { props: vec![
                    ("open", "bool", "Whether the dialog is open"),
                    ("on_close", "EventHandler", "On close handler"),
                    ("title", "String", "Dialog title"),
                    ("message", "String", "Confirmation message"),
                    ("confirm_text", "Option", "Confirm button text"),
                    ("cancel_text", "Option", "Cancel button text"),
                    ("variant", "ConfirmVariant", "Confirm button variant"),
                    ("icon", "Option", "Icon"),
                    ("on_confirm", "EventHandler", "On confirm handler"),
                    ("loading", "bool", "Loading state"),
                    ("class", "Option", "Additional CSS classes"),
                    ("id", "Option", "Element id"),
                    ("style", "Option", "Custom inline styles")
                ] }
                h4 { "DeleteConfirmDialogProps" }
                PropsTable { props: vec![
                    ("open", "bool", ""),
                    ("on_close", "EventHandler", ""),
                    ("title", "Option", ""),
                    ("item_name", "String", ""),
                    ("on_confirm", "EventHandler", ""),
                    ("loading", "bool", ""),
                    ("id", "Option", "Element id"),
                    ("class", "Option", "Custom CSS classes"),
                    ("style", "Option", "Custom inline styles")
                ] }
                h4 { "UnsavedChangesDialogProps" }
                PropsTable { props: vec![
                    ("open", "bool", ""),
                    ("on_close", "EventHandler", ""),
                    ("on_save", "EventHandler", ""),
                    ("on_discard", "EventHandler", ""),
                    ("id", "Option", "Element id"),
                    ("class", "Option", "Custom CSS classes"),
                    ("style", "Option", "Custom inline styles")
                ] }
                h4 { "SignOutDialogProps" }
                PropsTable { props: vec![
                    ("open", "bool", ""),
                    ("on_close", "EventHandler", ""),
                    ("on_confirm", "EventHandler", ""),
                    ("id", "Option", "Element id"),
                    ("class", "Option", "Custom CSS classes"),
                    ("style", "Option", "Custom inline styles")
                ] }
            }
        }
    }
}
