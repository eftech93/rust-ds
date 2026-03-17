//! Organism component documentation pages

use dioxus::prelude::*;
use dioxus_ui_system::prelude::*;
use dioxus_ui_system::organisms::*;

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
                    div { style: "border: 1px solid rgb(226,232,240); border-radius: 8px; overflow: hidden;",
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
                    div { style: "display: flex; gap: 8px; margin-bottom: 16px;",
                        Button { variant: if layout_type() == LayoutType::Sidebar { ButtonVariant::Primary } else { ButtonVariant::Secondary }, onclick: move |_| layout_type.set(LayoutType::Sidebar), "Sidebar" }
                        Button { variant: if layout_type() == LayoutType::TopNav { ButtonVariant::Primary } else { ButtonVariant::Secondary }, onclick: move |_| layout_type.set(LayoutType::TopNav), "TopNav" }
                    }
                    
                    div { style: "height: 300px; border: 1px solid rgb(226,232,240); border-radius: 8px; overflow: hidden;",
                        div { style: "transform: scale(0.5); transform-origin: top left; width: 200%; height: 200%;",
                            Layout {
                                layout_type: layout_type(),
                                nav_items: vec![
                                    LayoutNavItem::new("home", "Home", "#").with_icon("home").active(true),
                                    LayoutNavItem::new("settings", "Settings", "#").with_icon("settings"),
                                ],
                                brand: Some(rsx! { "MyApp" }),
                                title: Some("Dashboard".to_string()),
                                collapsible: true,
                                div { style: "padding: 24px;", Heading { level: HeadingLevel::H2, "Content Area" } }
                            }
                        }
                    }
                }
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
                    div { style: "max-width: 300px;",
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
                    div { style: "max-width: 300px;",
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
            }
        }
    }
}

#[component]
pub fn DataTablePage() -> Element {
    use dioxus_ui_system::organisms::{DataTable, TableColumn, ColumnAlign, Pagination};
    
    #[derive(Clone, PartialEq)]
    struct User {
        id: String,
        name: String,
        email: String,
        role: String,
        status: String,
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
    
    let users = vec![
        User { id: "1".to_string(), name: "Alice Johnson".to_string(), email: "alice@example.com".to_string(), role: "Admin".to_string(), status: "Active".to_string() },
        User { id: "2".to_string(), name: "Bob Smith".to_string(), email: "bob@example.com".to_string(), role: "User".to_string(), status: "Active".to_string() },
        User { id: "3".to_string(), name: "Carol White".to_string(), email: "carol@example.com".to_string(), role: "Editor".to_string(), status: "Inactive".to_string() },
        User { id: "4".to_string(), name: "David Brown".to_string(), email: "david@example.com".to_string(), role: "User".to_string(), status: "Active".to_string() },
        User { id: "5".to_string(), name: "Emma Davis".to_string(), email: "emma@example.com".to_string(), role: "Admin".to_string(), status: "Active".to_string() },
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
            key: "status".to_string(),
            header: "Status".to_string(),
            width: Some("100px".to_string()),
            align: ColumnAlign::Center,
            sortable: false,
            render: Some(render_status),
        },
    ];
    
    rsx! {
        DocPage {
            title: "DataTable",
            description: "Data display component with sorting and pagination.",
            
            Section { title: "Basic Table",
                ExampleBox {
                    DataTable {
                        data: users.clone(),
                        columns: columns.clone(),
                        key_extractor: |u: &User| u.id.clone(),
                        empty_message: "No users found",
                        loading: false,
                    }
                }
            }
            
            Section { title: "With Pagination",
                ExampleBox {
                    div { style: "display: flex; flex-direction: column; gap: 0; border: 1px solid rgb(226,232,240); border-radius: 8px; overflow: hidden;",
                        DataTable {
                            data: users.clone(),
                            columns: columns.clone(),
                            key_extractor: |u: &User| u.id.clone(),
                            empty_message: "No users found",
                            loading: false,
                        }
                        Pagination {
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
                        data: users.clone(),
                        columns: columns.clone(),
                        key_extractor: |u: &User| u.id.clone(),
                        selectable: true,
                        selected_keys: vec![],
                        on_selection_change: move |_| {},
                        empty_message: "No users found",
                        loading: false,
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
                    div { style: "border: 1px solid rgb(226,232,240); border-radius: 12px; overflow: hidden;",
                        Wizard {
                            steps: wizard_steps.clone(),
                            active_step: 0,
                            on_step_change: move |_| {},
                            on_finish: move |_| {},
                            
                            // Step 1: Personal Info
                            div { style: "padding: 24px;",
                                h3 { style: "margin: 0 0 16px 0;", "Personal Information" }
                                div { style: "display: flex; flex-direction: column; gap: 16px;",
                                    div { style: "display: flex; gap: 12px;",
                                        div { style: "flex: 1;",
                                            Label { "First Name" }
                                            Input { value: "Alice".to_string(), onchange: move |_| {} }
                                        }
                                        div { style: "flex: 1;",
                                            Label { "Last Name" }
                                            Input { value: "Johnson".to_string(), onchange: move |_| {} }
                                        }
                                    }
                                    div {
                                        Label { "Email" }
                                        Input { value: "alice@example.com".to_string(), onchange: move |_| {} }
                                    }
                                }
                            }
                            
                            // Step 2: Account
                            div { style: "padding: 24px;",
                                h3 { style: "margin: 0 0 16px 0;", "Account Setup" }
                                div { style: "display: flex; flex-direction: column; gap: 16px;",
                                    div {
                                        Label { "Username" }
                                        Input { value: "alice_j".to_string(), onchange: move |_| {} }
                                    }
                                    div {
                                        Label { "Password" }
                                        Input { input_type: InputType::Password, value: "".to_string(), onchange: move |_| {} }
                                    }
                                }
                            }
                            
                            // Step 3: Preferences
                            div { style: "padding: 24px;",
                                h3 { style: "margin: 0 0 16px 0;", "Preferences" }
                                div { style: "display: flex; flex-direction: column; gap: 12px;",
                                    div { style: "display: flex; align-items: center; gap: 8px;",
                                        input { r#type: "checkbox", checked: "true" }
                                        Label { "Receive email notifications" }
                                    }
                                    div { style: "display: flex; align-items: center; gap: 8px;",
                                        input { r#type: "checkbox" }
                                        Label { "Enable two-factor authentication" }
                                    }
                                    div { style: "display: flex; align-items: center; gap: 8px;",
                                        input { r#type: "checkbox", checked: "true" }
                                        Label { "Subscribe to newsletter" }
                                    }
                                }
                            }
                            
                            // Step 4: Review
                            div { style: "padding: 24px;",
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
                    div { style: "padding: 16px;",
                        CompactStepper {
                            steps: compact_steps,
                            active_step: 1,
                            on_step_click: move |_| {},
                        }
                    }
                }
            }
            
            Section { title: "Validation States",
                ExampleBox {
                    div { style: "display: flex; flex-direction: column; gap: 8px;",
                        p { "Steps can have validation states:" }
                        ul {
                            li { "Valid - Step is complete and valid" }
                            li { "Invalid - Step has errors that need to be fixed" }
                            li { "Pending - Step is not yet started" }
                        }
                        div { style: "margin-top: 16px; padding: 16px; background: rgb(241,245,249); border-radius: 8px;",
                            code { "WizardStep::new(\"Name\").valid(true)" }
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
        div {
            style: "display: flex; flex-direction: column; gap: 32px;",
            
            div {
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
            div { style: "display: flex; flex-direction: column; gap: 16px;", {children} }
        }
    }
}

#[component]
fn ExampleBox(children: Element) -> Element {
    rsx! {
        Card { variant: CardVariant::Default, padding: Some("24px".to_string()), {children} }
    }
}
