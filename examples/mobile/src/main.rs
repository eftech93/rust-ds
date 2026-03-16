//! Mobile Example with Component Showcase

use dioxus::prelude::*;
use dioxus_ui_system::prelude::*;

fn main() {
    dioxus::logger::init(tracing::Level::INFO).unwrap();
    println!("Starting Mobile Example");
    dioxus::mobile::launch(App);
}

/// App entry - provides ThemeProvider
#[component]
fn App() -> Element {
    rsx! {
        ThemeProvider {
            MobileApp {}
        }
    }
}

/// Main mobile app with navigation
#[component]
fn MobileApp() -> Element {
    // Simple navigation state
    let mut current_page = use_signal(|| Page::Welcome);
    
    rsx! {
        div {
            style: "font-family: system-ui, -apple-system, sans-serif; min-height: 100vh; background: #f8fafc;",
            
            // Status bar area
            div {
                style: "height: env(safe-area-inset-top, 44px); background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);",
            }
            
            // Navigation bar
            div {
                style: "background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 16px; padding-top: 12px; display: flex; align-items: center; justify-content: space-between;",
                
                if current_page() != Page::Welcome {
                    button {
                        style: "background: none; border: none; color: white; font-size: 16px; padding: 8px;",
                        onclick: move |_| current_page.set(Page::Welcome),
                        "← Back"
                    }
                } else {
                    div { style: "width: 60px;", "" }
                }
                
                Label {
                    size: TextSize::Large,
                    weight: TextWeight::Semibold,
                    color: TextColor::Inverse,
                    "Dioxus UI"
                }
                
                div { style: "width: 60px;", "" }
            }
            
            // Main content area with safe area padding
            div {
                style: "padding: 16px; padding-bottom: max(16px, env(safe-area-inset-bottom, 20px)); overflow-y: auto;",
                
                match current_page() {
                    Page::Welcome => rsx! { WelcomePage { on_get_started: move || current_page.set(Page::Components) } },
                    Page::Components => rsx! { ComponentsPage {} },
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
enum Page {
    Welcome,
    Components,
}

/// Welcome/Landing page
#[component]
fn WelcomePage(on_get_started: EventHandler<()>) -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 70vh; gap: 24px;",
            
            // Logo/Icon
            div {
                style: "width: 120px; height: 120px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); border-radius: 24px; display: flex; align-items: center; justify-content: center; box-shadow: 0 10px 25px rgba(102, 126, 234, 0.3);",
                
                Icon {
                    name: "star".to_string(),
                    size: IconSize::ExtraLarge,
                    color: IconColor::Inverse,
                }
            }
            
            // Title
            Heading {
                level: HeadingLevel::H1,
                "Dioxus UI"
            }
            
            // Subtitle
            MutedText {
                size: TextSize::Large,
                "Pure Rust Design System"
            }
            
            // Description
            Card {
                variant: CardVariant::Default,
                padding: Some("20px".to_string()),
                
                div {
                    style: "text-align: center;",
                    
                    Label {
                        size: TextSize::Small,
                        color: TextColor::Muted,
                        "A comprehensive UI component library for Dioxus with support for Web, Desktop, and Mobile platforms."
                    }
                    
                    div {
                        style: "margin-top: 16px; display: flex; gap: 8px; justify-content: center; flex-wrap: wrap;",
                        
                        Badge { variant: BadgeVariant::Success, icon: Some("check".to_string()), "Rust" }
                        Badge { variant: BadgeVariant::Secondary, "Cross-platform" }
                        Badge { variant: BadgeVariant::Outline, "Type-safe" }
                    }
                }
            }
            
            // Get Started Button
            div {
                style: "margin-top: 32px; width: 100%; max-width: 300px;",
                
                Button {
                    variant: ButtonVariant::Primary,
                    size: ButtonSize::Lg,
                    full_width: true,
                    onclick: move |_| on_get_started.call(()),
                    
                    div {
                        style: "display: flex; align-items: center; justify-content: center; gap: 8px;",
                        
                        "Get Started"
                        Icon {
                            name: "arrow-right".to_string(),
                            size: IconSize::Small,
                            color: IconColor::Current,
                        }
                    }
                }
            }
            
            // Version
            div {
                style: "margin-top: auto; padding-top: 32px;",
                
                Label {
                    size: TextSize::ExtraSmall,
                    color: TextColor::Muted,
                    "v0.1.0"
                }
            }
        }
    }
}

/// Components showcase page
#[component]
fn ComponentsPage() -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 16px; padding-bottom: 100px;",
            
            // Header
            div {
                style: "text-align: center; margin-bottom: 8px;",
                
                Heading {
                    level: HeadingLevel::H2,
                    "Components"
                }
                
                MutedText {
                    size: TextSize::Small,
                    "Browse our UI component library"
                }
            }
            
            // Button Showcase
            ComponentSection { title: "Buttons", 
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    
                    Button { variant: ButtonVariant::Primary, full_width: true, "Primary Button" }
                    Button { variant: ButtonVariant::Secondary, full_width: true, "Secondary Button" }
                    Button { variant: ButtonVariant::Ghost, full_width: true, "Ghost Button" }
                    Button { variant: ButtonVariant::Destructive, full_width: true, "Destructive" }
                    
                    div {
                        style: "display: flex; gap: 8px;",
                        Button { size: ButtonSize::Sm, "Small" }
                        Button { size: ButtonSize::Md, "Medium" }
                        Button { size: ButtonSize::Lg, "Large" }
                    }
                }
            }
            
            // Input Showcase
            ComponentSection { title: "Inputs", 
                InputShowcase {}
            }
            
            // Badge Showcase
            ComponentSection { title: "Badges",
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 8px;",
                    
                    Badge { "Default" }
                    Badge { variant: BadgeVariant::Secondary, "Secondary" }
                    Badge { variant: BadgeVariant::Success, icon: Some("check".to_string()), "Success" }
                    Badge { variant: BadgeVariant::Warning, "Warning" }
                    Badge { variant: BadgeVariant::Destructive, "Error" }
                    Badge { variant: BadgeVariant::Outline, "Outline" }
                }
            }
            
            // Card Showcase
            ComponentSection { title: "Cards",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    
                    Card {
                        variant: CardVariant::Default,
                        padding: Some("16px".to_string()),
                        "Default card with border"
                    }
                    
                    Card {
                        variant: CardVariant::Elevated,
                        padding: Some("16px".to_string()),
                        "Elevated card with shadow"
                    }
                }
            }
            
            // Icons Showcase
            ComponentSection { title: "Icons",
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 16px; justify-content: center;",
                    
                    Icon { name: "home".to_string(), size: IconSize::Large, color: IconColor::Primary }
                    Icon { name: "user".to_string(), size: IconSize::Large, color: IconColor::Secondary }
                    Icon { name: "settings".to_string(), size: IconSize::Large, color: IconColor::Muted }
                    Icon { name: "heart".to_string(), size: IconSize::Large, color: IconColor::Destructive }
                    Icon { name: "star".to_string(), size: IconSize::Large, color: IconColor::Warning }
                    Icon { name: "check".to_string(), size: IconSize::Large, color: IconColor::Success }
                }
            }
            
            // Typography Showcase
            ComponentSection { title: "Typography",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    
                    Heading { level: HeadingLevel::H3, "Heading 3" }
                    Heading { level: HeadingLevel::H4, "Heading 4" }
                    
                    Label { size: TextSize::Large, "Large text" }
                    Label { size: TextSize::Base, "Base text" }
                    Label { size: TextSize::Small, "Small text" }
                    
                    MutedText { "This is muted/secondary text" }
                }
            }
            
            // Interactive Demo
            ComponentSection { title: "Interactive",
                CounterDemo {}
            }
        }
    }
}

/// Section wrapper for components
#[component]
fn ComponentSection(title: String, children: Element) -> Element {
    rsx! {
        Card {
            variant: CardVariant::Default,
            full_width: true,
            
            CardHeader {
                title: title,
            }
            
            CardContent {
                {children}
            }
        }
    }
}

/// Input showcase with state
#[component]
fn InputShowcase() -> Element {
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 16px;",
            
            InputGroup {
                label: "Email",
                value: email(),
                placeholder: Some("you@example.com".to_string()),
                input_type: InputType::Email,
                onchange: move |v| email.set(v),
            }
            
            InputGroup {
                label: "Password",
                value: password(),
                placeholder: Some("Enter password".to_string()),
                input_type: InputType::Password,
                error: if password().len() > 0 && password().len() < 8 {
                    Some("Must be at least 8 characters".to_string())
                } else {
                    None
                },
                onchange: move |v| password.set(v),
            }
            
            Button {
                variant: ButtonVariant::Primary,
                full_width: true,
                onclick: move |_| {
                    println!("Login: {} / {}", email(), password());
                },
                "Sign In"
            }
        }
    }
}

/// Counter interactive demo
#[component]
fn CounterDemo() -> Element {
    let mut count = use_signal(|| 0);
    
    rsx! {
        div {
            style: "text-align: center; padding: 16px;",
            
            Heading {
                level: HeadingLevel::H2,
                "{count}"
            }
            
            div {
                style: "display: flex; gap: 12px; justify-content: center; margin-top: 16px;",
                
                Button {
                    variant: ButtonVariant::Secondary,
                    onclick: move |_| count -= 1,
                    Icon { name: "minus".to_string(), size: IconSize::Small, color: IconColor::Current }
                }
                
                Button {
                    variant: ButtonVariant::Destructive,
                    onclick: move |_| count.set(0),
                    "Reset"
                }
                
                Button {
                    variant: ButtonVariant::Primary,
                    onclick: move |_| count += 1,
                    Icon { name: "plus".to_string(), size: IconSize::Small, color: IconColor::Current }
                }
            }
        }
    }
}
