//! Organism component documentation pages

use dioxus::prelude::*;
use dioxus_ui_system::prelude::*;
use dioxus_ui_system::organisms::*;
use dioxus_ui_system::organisms::FooterVariant;
use dioxus_ui_system::molecules::ToastVariant;
use dioxus_ui_system::atoms::{Box, VStack, HStack};
use crate::docs_ui::{DocPage, Section, ExampleBox, CodeBlock};

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
        }
    }
}

#[component]
pub fn AccordionPage() -> Element {
    let mut expanded = use_signal(|| vec!["item1".to_string()]);
    
    let items = vec![
        AccordionItem::new("item1", "Is it accessible?", "Yes. It adheres to the WAI-ARIA design pattern."),
        AccordionItem::new("item2", "Is it styled?", "Yes. It comes with default styles."),
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
        }
    }
}

#[component]
pub fn DataTablePage() -> Element {
    use dioxus_ui_system::organisms::{DataTable, TableColumn, ColumnAlign, TablePagination, TableFilter, FilterOption};
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
        let bg = if user.status == "Active" { "rgb(34,197,94)" } else { "rgb(148,163,184)" };
        rsx! {
            span {
                style: "padding: 4px 12px; background: {bg}; color: white; border-radius: 9999px; font-size: 12px;",
                "{user.status}"
            }
        }
    }
    
    let all_users = vec![
        User { id: "1".to_string(), name: "Alice Johnson".to_string(), email: "alice@example.com".to_string(), role: "Admin".to_string(), status: "Active".to_string(), department: "Engineering".to_string() },
        User { id: "2".to_string(), name: "Bob Smith".to_string(), email: "bob@example.com".to_string(), role: "User".to_string(), status: "Active".to_string(), department: "Marketing".to_string() },
        User { id: "3".to_string(), name: "Carol White".to_string(), email: "carol@example.com".to_string(), role: "Editor".to_string(), status: "Inactive".to_string(), department: "Design".to_string() },
        User { id: "4".to_string(), name: "David Brown".to_string(), email: "david@example.com".to_string(), role: "User".to_string(), status: "Active".to_string(), department: "Engineering".to_string() },
        User { id: "5".to_string(), name: "Emma Davis".to_string(), email: "emma@example.com".to_string(), role: "Admin".to_string(), status: "Active".to_string(), department: "Sales".to_string() },
        User { id: "6".to_string(), name: "Frank Wilson".to_string(), email: "frank@example.com".to_string(), role: "User".to_string(), status: "Inactive".to_string(), department: "Marketing".to_string() },
        User { id: "7".to_string(), name: "Grace Lee".to_string(), email: "grace@example.com".to_string(), role: "Editor".to_string(), status: "Active".to_string(), department: "Design".to_string() },
        User { id: "8".to_string(), name: "Henry Taylor".to_string(), email: "henry@example.com".to_string(), role: "User".to_string(), status: "Active".to_string(), department: "Engineering".to_string() },
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
                FilterOption { label: "Admin".to_string(), value: "Admin".to_string() },
                FilterOption { label: "User".to_string(), value: "User".to_string() },
                FilterOption { label: "Editor".to_string(), value: "Editor".to_string() },
            ],
        },
        TableFilter {
            key: "status".to_string(),
            label: "All Status".to_string(),
            options: vec![
                FilterOption { label: "Active".to_string(), value: "Active".to_string() },
                FilterOption { label: "Inactive".to_string(), value: "Inactive".to_string() },
            ],
        },
        TableFilter {
            key: "department".to_string(),
            label: "All Departments".to_string(),
            options: vec![
                FilterOption { label: "Engineering".to_string(), value: "Engineering".to_string() },
                FilterOption { label: "Marketing".to_string(), value: "Marketing".to_string() },
                FilterOption { label: "Design".to_string(), value: "Design".to_string() },
                FilterOption { label: "Sales".to_string(), value: "Sales".to_string() },
            ],
        },
    ];
    
    // State for search and filters
    let mut search_query = use_signal(|| "".to_string());
    let mut active_filters = use_signal(|| HashMap::<String, String>::new());
    
    // Filter the data based on search query and active filters
    let filtered_users: Vec<User> = all_users.clone().into_iter().filter(|user| {
        // Text search
        let search_lower = search_query().to_lowercase();
        let matches_search = search_lower.is_empty() || 
            user.name.to_lowercase().contains(&search_lower) ||
            user.email.to_lowercase().contains(&search_lower) ||
            user.role.to_lowercase().contains(&search_lower) ||
            user.department.to_lowercase().contains(&search_lower);
        
        // Custom filters
        let matches_role = active_filters().get("role").map_or(true, |v| v == &user.role);
        let matches_status = active_filters().get("status").map_or(true, |v| v == &user.status);
        let matches_dept = active_filters().get("department").map_or(true, |v| v == &user.department);
        
        matches_search && matches_role && matches_status && matches_dept
    }).collect();
    
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
        }
    }
}

#[component]
pub fn StepperWizardPage() -> Element {
    use dioxus_ui_system::organisms::{Wizard, WizardStep, StepSummary, StepSummaryItem, CompactStepper};
    use dioxus_ui_system::molecules::StepItem;
    
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
                                            Input { value: "Alice".to_string(), onchange: move |_| {} }
                                        }
                                        Box { style: "flex: 1;",
                                            Label { "Last Name" }
                                            Input { value: "Johnson".to_string(), onchange: move |_| {} }
                                        }
                                    }
                                    Box {
                                        Label { "Email" }
                                        Input { value: "alice@example.com".to_string(), onchange: move |_| {} }
                                    }
                                }
                            }
                            
                            // Step 2: Account
                            Box { style: "padding: 24px;",
                                h3 { style: "margin: 0 0 16px 0;", "Account Setup" }
                                VStack { gap: SpacingSize::Md,
                                    Box {
                                        Label { "Username" }
                                        Input { value: "alice_j".to_string(), onchange: move |_| {} }
                                    }
                                    Box {
                                        Label { "Password" }
                                        Input { input_type: InputType::Password, value: "".to_string(), onchange: move |_| {} }
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
        }
    }
}

/// File Upload documentation page
#[component]
pub fn FileUploadPage() -> Element {
    rsx! {
        DocPage {
            title: "File Upload",
            description: "Drag-and-drop file upload with progress and preview.",
            
            Section { title: "Basic File Upload",
                ExampleBox {
                    FileUpload {
                        on_upload: EventHandler::new(move |_files: Vec<UploadedFile>| {}),
                        label: Some("Upload Documents".to_string()),
                        helper_text: Some("PDF, PNG, JPG up to 10MB".to_string()),
                        accept: Some(".pdf,.png,.jpg".to_string()),
                        multiple: false, max_files: 1, loading: false, disabled: false
                    }
                }
            }
            
            Section { title: "Multiple Files",
                ExampleBox {
                    FileUpload {
                        on_upload: EventHandler::new(move |_files: Vec<UploadedFile>| {}),
                        multiple: true,
                        max_files: 5,
                        label: Some("Upload Images".to_string()),
                        loading: false, disabled: false
                    }
                }
                CodeBlock { code: "FileUpload {{
    on_upload: EventHandler::new(move |files| {{
        // Handle uploaded files
    }}),
    label: Some(\"Upload Documents\".to_string()),
    helper_text: Some(\"PDF, PNG, JPG up to 10MB\".to_string()),
    accept: Some(\".pdf,.png,.jpg\".to_string()),
    multiple: true,
    max_files: 5,
}}".to_string() }
            }
        }
    }
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
        }
    }
}
