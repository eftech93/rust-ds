//! Shared components for all example apps
//!
//! This crate provides a comprehensive showcase of all UI components
//! that can be used across different platform examples.

use dioxus::prelude::*;
use dioxus_ui_system::prelude::*;

/// Main showcase component that displays all UI components (includes ThemeProvider)
#[component]
pub fn ComponentShowcase() -> Element {
    rsx! {
        ThemeProvider {
            ComponentShowcaseInner {}
        }
    }
}

/// Inner showcase content (without ThemeProvider - use when you have your own)
#[component]
pub fn ComponentShowcaseInner() -> Element {
    rsx! {
        div {
            style: "min-height: 100vh; padding: 32px; max-width: 1400px; margin: 0 auto;",
            
            // Header
            ShowcaseHeader {}
            
            // Component sections
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(400px, 1fr)); gap: 24px; margin-top: 32px;",
                
                ButtonShowcase {}
                InputShowcase {}
                BadgeShowcase {}
                CardShowcase {}
                IconShowcase {}
                TypographyShowcase {}
                InteractiveDemo {}
                ThemeShowcase {}
            }
            
            // Footer
            ShowcaseFooter {}
        }
    }
}

/// Header section
#[component]
fn ShowcaseHeader() -> Element {
    rsx! {
        div {
            style: "text-align: center; margin-bottom: 48px;",
            
            Heading {
                level: HeadingLevel::H1,
                "Dioxus UI System"
            }
            
            MutedText {
                size: TextSize::Large,
                "A pure Rust design system for Dioxus"
            }
            
            div {
                style: "margin-top: 16px;",
                Badge {
                    variant: BadgeVariant::Success,
                    icon: Some("check".to_string()),
                    "v0.1.0"
                }
            }
        }
    }
}

/// Button showcase section
#[component]
fn ButtonShowcase() -> Element {
    rsx! {
        Card {
            CardHeader {
                title: "Buttons",
                subtitle: Some("Interactive button variants and sizes".to_string()),
            }
            CardContent {
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    
                    // Variants
                    div {
                        style: "display: flex; flex-wrap: wrap; gap: 8px;",
                        Button { variant: ButtonVariant::Primary, "Primary" }
                        Button { variant: ButtonVariant::Secondary, "Secondary" }
                        Button { variant: ButtonVariant::Ghost, "Ghost" }
                        Button { variant: ButtonVariant::Destructive, "Destructive" }
                        Button { variant: ButtonVariant::Link, "Link" }
                    }
                    
                    // Sizes
                    div {
                        style: "display: flex; flex-wrap: wrap; gap: 8px; align-items: center;",
                        Button { size: ButtonSize::Sm, "Small" }
                        Button { size: ButtonSize::Md, "Medium" }
                        Button { size: ButtonSize::Lg, "Large" }
                    }
                    
                    // States
                    div {
                        style: "display: flex; flex-wrap: wrap; gap: 8px;",
                        Button { disabled: true, "Disabled" }
                        Button { 
                            variant: ButtonVariant::Primary,
                            full_width: true,
                            "Full Width" 
                        }
                    }
                }
            }
        }
    }
}

/// Input showcase section
#[component]
fn InputShowcase() -> Element {
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut error = use_signal(|| false);
    
    rsx! {
        Card {
            CardHeader {
                title: "Form Inputs",
                subtitle: Some("Input fields with validation".to_string()),
            }
            CardContent {
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    
                    InputGroup {
                        label: "Email Address",
                        value: email(),
                        placeholder: Some("you@example.com".to_string()),
                        input_type: InputType::Email,
                        required: true,
                        hint: Some("We'll never share your email.".to_string()),
                        onchange: move |v| email.set(v),
                    }
                    
                    InputGroup {
                        label: "Password",
                        value: password(),
                        placeholder: Some("Enter password".to_string()),
                        input_type: InputType::Password,
                        error: if error() && password().len() < 8 {
                            Some("Password must be at least 8 characters".to_string())
                        } else {
                            None
                        },
                        onchange: move |v| {
                            password.set(v);
                            error.set(true);
                        },
                    }
                    
                    Button {
                        variant: ButtonVariant::Primary,
                        full_width: true,
                        onclick: move |_| {},
                        "Submit"
                    }
                }
            }
        }
    }
}

/// Badge showcase section
#[component]
fn BadgeShowcase() -> Element {
    rsx! {
        Card {
            CardHeader {
                title: "Badges",
                subtitle: Some("Status indicators and labels".to_string()),
            }
            CardContent {
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    
                    // Variants
                    div {
                        style: "display: flex; flex-wrap: wrap; gap: 8px;",
                        Badge { "Default" }
                        Badge { variant: BadgeVariant::Secondary, "Secondary" }
                        Badge { variant: BadgeVariant::Success, icon: Some("check".to_string()), "Success" }
                        Badge { variant: BadgeVariant::Warning, "Warning" }
                        Badge { variant: BadgeVariant::Destructive, "Error" }
                        Badge { variant: BadgeVariant::Outline, "Outline" }
                        Badge { variant: BadgeVariant::Ghost, "Ghost" }
                    }
                    
                    // Status badges
                    div {
                        style: "display: flex; flex-wrap: wrap; gap: 8px;",
                        StatusBadge { status: "Active".to_string(), status_type: StatusType::Success }
                        StatusBadge { status: "Pending".to_string(), status_type: StatusType::Warning }
                        StatusBadge { status: "Failed".to_string(), status_type: StatusType::Error }
                        StatusBadge { status: "Info".to_string(), status_type: StatusType::Info }
                    }
                }
            }
        }
    }
}

/// Card showcase section
#[component]
fn CardShowcase() -> Element {
    rsx! {
        Card {
            CardHeader {
                title: "Card Variants",
                subtitle: Some("Different card styles and layouts".to_string()),
            }
            CardContent {
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    
                    Card {
                        variant: CardVariant::Default,
                        padding: Some("12px".to_string()),
                        "Default card with border"
                    }
                    
                    Card {
                        variant: CardVariant::Muted,
                        padding: Some("12px".to_string()),
                        "Muted card with subtle background"
                    }
                    
                    Card {
                        variant: CardVariant::Elevated,
                        padding: Some("12px".to_string()),
                        "Elevated card with shadow"
                    }
                    
                    Card {
                        variant: CardVariant::Outlined,
                        padding: Some("12px".to_string()),
                        "Outlined card with thicker border"
                    }
                }
            }
        }
    }
}

/// Icon showcase section
#[component]
fn IconShowcase() -> Element {
    let icons = vec![
        ("home", IconColor::Current),
        ("user", IconColor::Primary),
        ("settings", IconColor::Secondary),
        ("search", IconColor::Muted),
        ("bell", IconColor::Warning),
        ("heart", IconColor::Destructive),
        ("star", IconColor::Success),
        ("trash", IconColor::Destructive),
        ("edit", IconColor::Primary),
        ("check", IconColor::Success),
        ("x", IconColor::Destructive),
        ("arrow-right", IconColor::Current),
        ("plus", IconColor::Success),
        ("minus", IconColor::Destructive),
        ("menu", IconColor::Current),
    ];
    
    rsx! {
        Card {
            CardHeader {
                title: "Icons",
                subtitle: Some("Built-in icon library with 30+ icons".to_string()),
            }
            CardContent {
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    
                    // Icon grid
                    div {
                        style: "display: flex; flex-wrap: wrap; gap: 16px; justify-content: center;",
                        
                        for (name, color) in icons {
                            div {
                                style: "display: flex; flex-direction: column; align-items: center; gap: 4px; padding: 8px;",
                                Icon {
                                    name: name.to_string(),
                                    size: IconSize::Large,
                                    color: color,
                                }
                                Label {
                                    size: TextSize::ExtraSmall,
                                    color: TextColor::Muted,
                                    "{name}"
                                }
                            }
                        }
                    }
                    
                    // Size comparison
                    div {
                        style: "display: flex; align-items: center; gap: 16px; justify-content: center; margin-top: 16px;",
                        Icon { name: "star".to_string(), size: IconSize::ExtraSmall, color: IconColor::Warning }
                        Icon { name: "star".to_string(), size: IconSize::Small, color: IconColor::Warning }
                        Icon { name: "star".to_string(), size: IconSize::Medium, color: IconColor::Warning }
                        Icon { name: "star".to_string(), size: IconSize::Large, color: IconColor::Warning }
                        Icon { name: "star".to_string(), size: IconSize::ExtraLarge, color: IconColor::Warning }
                    }
                }
            }
        }
    }
}

/// Typography showcase section
#[component]
fn TypographyShowcase() -> Element {
    rsx! {
        Card {
            CardHeader {
                title: "Typography",
                subtitle: Some("Text styles and formatting".to_string()),
            }
            CardContent {
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    
                    Heading { level: HeadingLevel::H1, "Heading 1" }
                    Heading { level: HeadingLevel::H2, "Heading 2" }
                    Heading { level: HeadingLevel::H3, "Heading 3" }
                    Heading { level: HeadingLevel::H4, "Heading 4" }
                    
                    div {
                        style: "border-top: 1px solid #e2e8f0; margin: 8px 0;",
                    }
                    
                    Label { size: TextSize::ExtraSmall, "Extra Small Text" }
                    Label { size: TextSize::Small, "Small Text" }
                    Label { size: TextSize::Base, "Base Text" }
                    Label { size: TextSize::Large, "Large Text" }
                    Label { size: TextSize::ExtraLarge, "Extra Large Text" }
                    
                    div {
                        style: "border-top: 1px solid #e2e8f0; margin: 8px 0;",
                    }
                    
                    Label { weight: TextWeight::Normal, "Normal Weight" }
                    Label { weight: TextWeight::Medium, "Medium Weight" }
                    Label { weight: TextWeight::Semibold, "Semibold Weight" }
                    Label { weight: TextWeight::Bold, "Bold Weight" }
                    
                    div {
                        style: "border-top: 1px solid #e2e8f0; margin: 8px 0;",
                    }
                    
                    MutedText { "This is muted/secondary text" }
                }
            }
        }
    }
}

/// Interactive demo section
#[component]
fn InteractiveDemo() -> Element {
    let mut count = use_signal(|| 0);
    let mut message = use_signal(|| "Click the buttons!".to_string());
    
    rsx! {
        Card {
            CardHeader {
                title: "Interactive Demo",
                subtitle: Some("State management with signals".to_string()),
            }
            CardContent {
                div {
                    style: "text-align: center;",
                    
                    Heading {
                        level: HeadingLevel::H2,
                        "{count}"
                    }
                    
                    Label {
                        size: TextSize::Small,
                        color: TextColor::Muted,
                        "{message}"
                    }
                    
                    div {
                        style: "display: flex; justify-content: center; gap: 8px; margin-top: 16px;",
                        
                        Button {
                            variant: ButtonVariant::Secondary,
                            onclick: move |_| {
                                count -= 1;
                                message.set("Decremented!".to_string());
                            },
                            Icon {
                                name: "minus".to_string(),
                                size: IconSize::Small,
                                color: IconColor::Current,
                            }
                        }
                        
                        Button {
                            variant: ButtonVariant::Destructive,
                            onclick: move |_| {
                                count.set(0);
                                message.set("Reset!".to_string());
                            },
                            "Reset"
                        }
                        
                        Button {
                            variant: ButtonVariant::Primary,
                            onclick: move |_| {
                                count += 1;
                                message.set("Incremented!".to_string());
                            },
                            Icon {
                                name: "plus".to_string(),
                                size: IconSize::Small,
                                color: IconColor::Current,
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Theme showcase section
#[component]
fn ThemeShowcase() -> Element {
    rsx! {
        Card {
            CardHeader {
                title: "Theme System",
                subtitle: Some("Light, dark, and custom themes".to_string()),
            }
            CardContent {
                div {
                    style: "display: flex; flex-direction: column; gap: 16px; align-items: center;",
                    
                    Label {
                        size: TextSize::Small,
                        "Toggle between light and dark mode:"
                    }
                    
                    ThemeToggle {}
                    
                    div {
                        style: "border-top: 1px solid #e2e8f0; margin: 8px 0; width: 100%;",
                    }
                    
                    Label {
                        size: TextSize::ExtraSmall,
                        color: TextColor::Muted,
                        "The theme context automatically propagates to all child components"
                    }
                }
            }
        }
    }
}

/// Footer section
#[component]
fn ShowcaseFooter() -> Element {
    rsx! {
        footer {
            style: "margin-top: 48px; padding: 32px; text-align: center; border-top: 1px solid #e2e8f0;",
            
            MutedText {
                "Built with ❤️ using Dioxus UI System"
            }
            
            div {
                style: "margin-top: 8px;",
                Label {
                    size: TextSize::ExtraSmall,
                    color: TextColor::Muted,
                    "Cross-platform Rust UI components"
                }
            }
        }
    }
}

/// Navigation header for apps that need routing
#[component]
pub fn AppHeader() -> Element {
    let nav_items = vec![
        NavItem {
            label: "Home".to_string(),
            href: "/".to_string(),
            icon: Some("home".to_string()),
            active: true,
        },
        NavItem {
            label: "Components".to_string(),
            href: "/components".to_string(),
            icon: Some("settings".to_string()),
            active: false,
        },
    ];
    
    rsx! {
        Header {
            brand_title: "Dioxus UI",
            nav_items: nav_items,
            actions: rsx! {
                ThemeToggle {}
            }
        }
    }
}
