//! Mobile Example with Component Showcase

use dioxus::prelude::*;
use dioxus_ui_system::atoms::{
    AlignItems, Box, HStack, JustifyContent, SpacingSize, StepSize, VStack,
};
use dioxus_ui_system::molecules::{HorizontalStepper, StepItem};
use dioxus_ui_system::organisms::*;
use dioxus_ui_system::prelude::*;

fn main() {
    dioxus::logger::init(tracing::Level::INFO).unwrap();
    println!("Starting Mobile Example");
    dioxus::launch(App);
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
    let bg_color = use_style(|t| t.colors.background.to_rgba());

    rsx! {
        Box {
            style: "font-family: system-ui, -apple-system, sans-serif; min-height: 100vh; background: {bg_color}; transition: background 200ms ease;",

            // Status bar area
            div {
                style: "height: env(safe-area-inset-top, 44px); background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);",
            }

            // Navigation bar
            HStack {
                style: "background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 16px; padding-top: 12px;",
                align: AlignItems::Center,
                justify: JustifyContent::SpaceBetween,

                if current_page() == Page::Welcome {
                    div { style: "width: 60px;", "" }
                } else {
                    button {
                        style: "background: none; border: none; color: white; font-size: 16px; padding: 8px;",
                        onclick: move |_| current_page.set(Page::Welcome),
                        "← Back"
                    }
                }

                Label {
                    size: TextSize::Large,
                    weight: TextWeight::Semibold,
                    color: TextColor::Inverse,
                    "Dioxus UI"
                }

                // Theme Toggle
                HStack {
                    style: "width: 60px;",
                    justify: JustifyContent::End,

                    MobileThemeToggle {}
                }
            }

            // Main content area with safe area padding
            div {
                style: "padding: 16px; padding-bottom: max(16px, env(safe-area-inset-bottom, 20px)); overflow-y: auto;",

                match current_page() {
                    Page::Welcome => rsx! {
                        WelcomePage {
                            on_get_started: move || current_page.set(Page::Components),
                            on_view_layouts: move || current_page.set(Page::Layouts),
                        }
                    },
                    Page::Components => rsx! { ComponentsPage { on_back: move || current_page.set(Page::Welcome) } },
                    Page::Layouts => rsx! { LayoutsPage { on_back: move || current_page.set(Page::Welcome) } },
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
enum Page {
    Welcome,
    Components,
    Layouts,
}

/// Mobile theme toggle button
#[component]
fn MobileThemeToggle() -> Element {
    let theme = use_theme();
    let mode = use_style(|t| t.mode.clone());

    let (_icon, label) = match mode() {
        dioxus_ui_system::theme::ThemeMode::Light => ("moon", "🌙"),
        dioxus_ui_system::theme::ThemeMode::Dark => ("sun", "☀️"),
        dioxus_ui_system::theme::ThemeMode::Brand(_) => ("palette", "🎨"),
    };

    rsx! {
        button {
            style: "background: rgba(255,255,255,0.2); border: none; border-radius: 8px; padding: 8px; cursor: pointer; font-size: 18px; color: white;",
            onclick: move |_| theme.toggle_mode.call(()),
            "{label}"
        }
    }
}

/// Mobile theme selector button
#[derive(Props, Clone, PartialEq)]
struct MobileThemeButtonProps {
    theme_name: String,
    emoji: String,
    label: String,
}

#[component]
fn MobileThemeButton(props: MobileThemeButtonProps) -> Element {
    let theme = use_theme();

    rsx! {
        button {
            style: "display: flex; flex-direction: column; align-items: center; gap: 4px; padding: 8px; border-radius: 8px; border: 1px solid #e2e8f0; background: white; cursor: pointer; min-width: 50px;",
            onclick: move |_| theme.set_theme_by_name.call(props.theme_name.clone()),

            span { style: "font-size: 20px;", "{props.emoji}" }
            span { style: "font-size: 11px; color: #64748b;", "{props.label}" }
        }
    }
}

/// Welcome/Landing page
#[derive(Props, Clone, PartialEq)]
struct WelcomePageProps {
    on_get_started: EventHandler<()>,
    on_view_layouts: EventHandler<()>,
}

#[component]
fn WelcomePage(props: WelcomePageProps) -> Element {
    rsx! {
        VStack {
            align: AlignItems::Center,
            justify: JustifyContent::Center,
            gap: SpacingSize::Lg,
            style: "min-height: 70vh;",

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
                        "A comprehensive UI component library for Dioxus with 60+ components for Web, Desktop, and Mobile platforms."
                    }

                    div {
                        style: "margin-top: 16px; display: flex; gap: 8px; justify-content: center; flex-wrap: wrap;",

                        Badge { variant: BadgeVariant::Success, icon: Some("check".to_string()), "Rust" }
                        Badge { variant: BadgeVariant::Secondary, "Cross-platform" }
                        Badge { variant: BadgeVariant::Outline, "Type-safe" }
                    }
                }
            }

            // Theme Selector
            Card {
                variant: CardVariant::Muted,
                padding: Some("16px".to_string()),

                div {
                    style: "text-align: center;",

                    Label {
                        size: TextSize::Small,
                        weight: TextWeight::Medium,
                        "Choose Theme"
                    }

                    div {
                        style: "margin-top: 12px; display: flex; gap: 8px; justify-content: center; flex-wrap: wrap;",

                        MobileThemeButton { theme_name: "light", emoji: "☀️", label: "Light" }
                        MobileThemeButton { theme_name: "dark", emoji: "🌙", label: "Dark" }
                        MobileThemeButton { theme_name: "rose", emoji: "🌹", label: "Rose" }
                        MobileThemeButton { theme_name: "blue", emoji: "🔵", label: "Blue" }
                        MobileThemeButton { theme_name: "green", emoji: "🟢", label: "Green" }
                        MobileThemeButton { theme_name: "violet", emoji: "🟣", label: "Violet" }
                        MobileThemeButton { theme_name: "orange", emoji: "🟠", label: "Orange" }
                    }
                }
            }

            // Action Buttons
            VStack {
                style: "margin-top: 32px; width: 100%; max-width: 300px;",
                gap: SpacingSize::Md,

                Button {
                    variant: ButtonVariant::Primary,
                    size: ButtonSize::Lg,
                    full_width: true,
                    onclick: move |_| props.on_get_started.call(()),

                    HStack {
                        align: AlignItems::Center,
                        justify: JustifyContent::Center,
                        gap: SpacingSize::Sm,

                        Icon {
                            name: "box".to_string(),
                            size: IconSize::Small,
                            color: IconColor::Current,
                        }
                        "Browse Components"
                    }
                }

                Button {
                    variant: ButtonVariant::Secondary,
                    size: ButtonSize::Lg,
                    full_width: true,
                    onclick: move |_| props.on_view_layouts.call(()),

                    div {
                        style: "display: flex; align-items: center; justify-content: center; gap: 8px;",

                        Icon {
                            name: "layout".to_string(),
                            size: IconSize::Small,
                            color: IconColor::Current,
                        }
                        "View Layouts"
                    }
                }
            }

            // Version
            div {
                style: "margin-top: auto; padding-top: 32px;",

                Label {
                    size: TextSize::ExtraSmall,
                    color: TextColor::Muted,
                    "v0.2.0"
                }
            }
        }
    }
}

/// Layouts showcase page with interactive layout demo
#[derive(Props, Clone, PartialEq)]
struct LayoutsPageProps {
    on_back: EventHandler<()>,
}

#[component]
fn LayoutsPage(props: LayoutsPageProps) -> Element {
    let mut current_layout = use_signal(|| LayoutType::Sidebar);

    // Navigation items for the layout demo
    let nav_items = vec![
        LayoutNavItem::new("home", "Home", "#")
            .with_icon("home")
            .active(true),
        LayoutNavItem::new("components", "Components", "#").with_icon("box"),
        LayoutNavItem::new("settings", "Settings", "#").with_icon("settings"),
        LayoutNavItem::new("profile", "Profile", "#").with_icon("user"),
    ];

    // Brand element
    let brand = rsx! {
        HStack {
            align: AlignItems::Center,
            gap: SpacingSize::Sm,
            Icon {
                name: "star".to_string(),
                size: IconSize::Medium,
                color: IconColor::Primary,
            }
            span {
                style: "font-weight: 700;",
                "Dioxus"
            }
        }
    };

    // Actions (theme selector)
    let actions = rsx! {
        ThemeToggle {}
    };

    rsx! {
        VStack {
            gap: SpacingSize::Md,
            style: "padding-bottom: 100px;",

            // Header with back button
            HStack {
                align: AlignItems::Center,
                gap: SpacingSize::Md,
                style: "margin-bottom: 8px;",

                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Icon,
                    onclick: move |_| props.on_back.call(()),
                    Icon {
                        name: "arrow-left".to_string(),
                        size: IconSize::Medium,
                        color: IconColor::Current,
                    }
                }

                Box {
                    style: "flex: 1; text-align: center; padding-right: 48px;",

                    Heading {
                        level: HeadingLevel::H2,
                        "Layout Demo"
                    }

                    MutedText {
                        size: TextSize::Small,
                        "Tap a layout to try it"
                    }
                }
            }

            // Layout Type Selector
            ComponentSection { title: "Select Layout",
                div {
                    style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 8px;",

                    LayoutButton {
                        icon: "sidebar",
                        label: "Sidebar",
                        is_active: current_layout() == LayoutType::Sidebar,
                        onclick: move || current_layout.set(LayoutType::Sidebar),
                    }

                    LayoutButton {
                        icon: "layout",
                        label: "TopNav",
                        is_active: current_layout() == LayoutType::TopNav,
                        onclick: move || current_layout.set(LayoutType::TopNav),
                    }

                    LayoutButton {
                        icon: "menu",
                        label: "Drawer",
                        is_active: current_layout() == LayoutType::Drawer,
                        onclick: move || current_layout.set(LayoutType::Drawer),
                    }

                    LayoutButton {
                        icon: "maximize",
                        label: "Full",
                        is_active: current_layout() == LayoutType::FullWidth,
                        onclick: move || current_layout.set(LayoutType::FullWidth),
                    }
                }
            }

            // Live Layout Preview
            ComponentSection { title: "Live Preview",
                Box {
                    style: "border: 2px solid #e2e8f0; border-radius: 12px; overflow: hidden; height: 400px; position: relative;",

                    // Scale down the layout to fit in the preview
                    Box {
                        style: "transform: scale(0.5); transform-origin: top left; width: 200%; height: 200%;",

                        Layout {
                            layout_type: current_layout(),
                            nav_items: nav_items.clone(),
                            brand: Some(brand),
                            title: Some("Preview".to_string()),
                            actions: Some(actions),
                            collapsible: true,
                            sidebar_collapsed: false,
                            sidebar_width: 200,
                            header_height: 56,

                            // Sample content
                            Box {
                                style: "padding: 16px;",

                                Heading {
                                    level: HeadingLevel::H3,
                                    "Dashboard"
                                }

                                p {
                                    style: "color: #64748b; margin-top: 8px;",
                                    "This is how the "
                                    strong { "{layout_type_name(current_layout())}" }
                                    " layout looks with content."
                                }

                                HStack {
                                    style: "margin-top: 16px; flex-wrap: wrap;",
                                    gap: SpacingSize::Sm,

                                    Card {
                                        variant: CardVariant::Default,
                                        padding: Some("12px".to_string()),
                                        "Card 1"
                                    }

                                    Card {
                                        variant: CardVariant::Default,
                                        padding: Some("12px".to_string()),
                                        "Card 2"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Layout Description
            ComponentSection { title: "About This Layout",
                Card {
                    variant: CardVariant::Muted,
                    padding: Some("16px".to_string()),

                    p {
                        style: "margin: 0; color: #64748b; font-size: 14px; line-height: 1.5;",
                        "{layout_description(current_layout())}"
                    }
                }
            }

            // Theme Info
            ComponentSection { title: "Available Themes",
                VStack {
                    gap: SpacingSize::Sm,

                    ThemeRow { name: "Light", color: "#ffffff" }
                    ThemeRow { name: "Dark", color: "#0f172a" }
                    ThemeRow { name: "Rose", color: "#e11d48" }
                    ThemeRow { name: "Blue", color: "#2563eb" }
                    ThemeRow { name: "Green", color: "#16a34a" }
                    ThemeRow { name: "Violet", color: "#7c3aed" }
                    ThemeRow { name: "Orange", color: "#ea580c" }
                }
            }
        }
    }
}

/// Layout button component
#[derive(Props, Clone, PartialEq)]
struct LayoutButtonProps {
    icon: String,
    label: String,
    is_active: bool,
    onclick: EventHandler<()>,
}

#[component]
fn LayoutButton(props: LayoutButtonProps) -> Element {
    let bg_color = if props.is_active {
        "#0f172a"
    } else {
        "#f1f5f9"
    };
    let text_color = if props.is_active { "white" } else { "#0f172a" };
    let border = if props.is_active {
        "2px solid #0f172a"
    } else {
        "2px solid #e2e8f0"
    };

    rsx! {
        button {
            style: "display: flex; flex-direction: column; align-items: center; gap: 8px; padding: 16px; border-radius: 12px; border: {border}; background: {bg_color}; color: {text_color}; cursor: pointer; transition: all 150ms;",
            onclick: move |_| props.onclick.call(()),

            Icon {
                name: props.icon,
                size: IconSize::Large,
                color: if props.is_active { IconColor::Inverse } else { IconColor::Primary },
            }

            span {
                style: "font-size: 13px; font-weight: 500;",
                "{props.label}"
            }
        }
    }
}

/// Get layout type name
fn layout_type_name(layout: LayoutType) -> &'static str {
    match layout {
        LayoutType::Sidebar => "Sidebar",
        LayoutType::TopNav => "Top Navigation",
        LayoutType::Drawer => "Drawer",
        LayoutType::FullWidth => "Full Width",
    }
}

/// Get layout description
fn layout_description(layout: LayoutType) -> &'static str {
    match layout {
        LayoutType::Sidebar => "The Sidebar layout features a collapsible side navigation panel that stays visible while scrolling. Perfect for dashboards and admin interfaces with many navigation items.",
        LayoutType::TopNav => "The Top Navigation layout places the menu horizontally at the top of the page. Great for websites and apps with fewer navigation items where you want maximum content space.",
        LayoutType::Drawer => "The Drawer layout uses a hamburger menu to reveal a slide-out navigation panel. Ideal for mobile apps or when you want to maximize screen real estate.",
        LayoutType::FullWidth => "The Full Width layout removes navigation entirely, giving you complete control of the page. Perfect for landing pages, focused workflows, or when you want custom navigation.",
    }
}

/// Layout card component
#[derive(Props, Clone, PartialEq)]
struct LayoutCardProps {
    icon: String,
    title: String,
    description: String,
}

#[component]
fn LayoutCard(props: LayoutCardProps) -> Element {
    rsx! {
        Card {
            variant: CardVariant::Default,
            padding: Some("16px".to_string()),

            HStack {
                align: AlignItems::Start,
                gap: SpacingSize::Md,

                div {
                    style: "width: 40px; height: 40px; background: #f1f5f9; border-radius: 8px; display: flex; align-items: center; justify-content: center; flex-shrink: 0;",

                    Icon {
                        name: props.icon,
                        size: IconSize::Medium,
                        color: IconColor::Primary,
                    }
                }

                div {
                    Heading {
                        level: HeadingLevel::H4,
                        "{props.title}"
                    }

                    p {
                        style: "margin: 4px 0 0 0; color: #64748b; font-size: 13px; line-height: 1.4;",
                        "{props.description}"
                    }
                }
            }
        }
    }
}

/// Theme row component
#[derive(Props, Clone, PartialEq)]
struct ThemeRowProps {
    name: String,
    color: String,
}

#[component]
fn ThemeRow(props: ThemeRowProps) -> Element {
    rsx! {
        HStack {
            align: AlignItems::Center,
            gap: SpacingSize::Md,
            style: "padding: 8px 0;",

            div {
                style: "width: 24px; height: 24px; border-radius: 6px; background: {props.color}; border: 1px solid #e2e8f0;",
            }

            Label {
                "{props.name}"
            }
        }
    }
}

/// Components showcase page
#[derive(Props, Clone, PartialEq)]
struct ComponentsPageProps {
    on_back: EventHandler<()>,
}

#[component]
fn ComponentsPage(props: ComponentsPageProps) -> Element {
    rsx! {
        VStack {
            gap: SpacingSize::Md,
            style: "padding-bottom: 100px;",

            // Header with back button
            HStack {
                align: AlignItems::Center,
                gap: SpacingSize::Md,
                style: "margin-bottom: 8px;",

                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Icon,
                    onclick: move |_| props.on_back.call(()),
                    Icon {
                        name: "arrow-left".to_string(),
                        size: IconSize::Medium,
                        color: IconColor::Current,
                    }
                }

                Box {
                    style: "flex: 1; text-align: center; padding-right: 48px;",

                    Heading {
                        level: HeadingLevel::H2,
                        "Components"
                    }

                    MutedText {
                        size: TextSize::Small,
                        "Browse our UI component library"
                    }
                }
            }

            // Button Showcase
            ComponentSection { title: "Buttons",
                VStack {
                    gap: SpacingSize::Md,

                    Button { variant: ButtonVariant::Primary, full_width: true, "Primary Button" }
                    Button { variant: ButtonVariant::Secondary, full_width: true, "Secondary Button" }
                    Button { variant: ButtonVariant::Ghost, full_width: true, "Ghost Button" }
                    Button { variant: ButtonVariant::Destructive, full_width: true, "Destructive" }

                    HStack {
                        gap: SpacingSize::Sm,
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
                VStack {
                    gap: SpacingSize::Md,

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

            // Card Organisms Showcase
            ComponentSection { title: "Card Organisms",
                VStack {
                    gap: SpacingSize::Md,

                    // Action Card
                    ActionCard {
                        title: "Deploy Project",
                        description: "Your project is ready to deploy to production.",
                        action_label: "Deploy Now",
                        on_action: move |_| println!("Deploying..."),
                        icon: Some("rocket".to_string()),
                    }

                    // Profile Card
                    ProfileCard {
                        name: "Sarah Chen".to_string(),
                        role: Some("Senior Engineer".to_string()),
                        avatar_url: None,
                        description: Some("Full-stack Rust developer".to_string()),
                        action_label: "Connect".to_string(),
                        on_action: Some(EventHandler::new(move |_| println!("Connecting..."))),
                        stats: vec![
                            ("Projects".to_string(), "24".to_string()),
                            ("Followers".to_string(), "1.2K".to_string()),
                        ],
                    }

                    // Stat Cards Row
                    div {
                        style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 8px;",

                        StatCard {
                            label: "Revenue",
                            value: "$48K",
                            change: Some("+12%".to_string()),
                            change_positive: Some(true),
                            icon: Some("trending-up".to_string()),
                            icon_bg: "#dbeafe".to_string(),
                        }

                        StatCard {
                            label: "Users",
                            value: "2.4K",
                            change: Some("+5%".to_string()),
                            change_positive: Some(true),
                            icon: Some("users".to_string()),
                            icon_bg: "#dcfce7".to_string(),
                        }
                    }

                    // Notification Card
                    NotificationCard {
                        title: "Update Available".to_string(),
                        message: "A new version of the app is available.".to_string(),
                        notification_type: NotificationType::Info,
                        on_dismiss: None,
                    }

                    // Expandable Card
                    ExpandableCard {
                        title: "Advanced Settings",
                        preview: rsx! {
                            p { style: "margin: 0; color: #64748b; font-size: 14px;",
                                "Tap to expand options..."
                            }
                        },
                        expanded_content: rsx! {
                            VStack {
                                gap: SpacingSize::Md,
                                style: "padding-top: 12px;",

                                Checkbox {
                                    checked: true,
                                    label: Some("Enable notifications".to_string()),
                                    onchange: move |_| {},
                                }

                                Checkbox {
                                    checked: false,
                                    label: Some("Dark mode".to_string()),
                                    onchange: move |_| {},
                                }
                            }
                        },
                        default_expanded: false,
                    }

                    // Pricing Card
                    PricingCard {
                        plan: "Pro".to_string(),
                        price: "$29".to_string(),
                        period: "/month".to_string(),
                        description: Some("For growing teams".to_string()),
                        features: vec![
                            "Unlimited Projects".to_string(),
                            "100GB Storage".to_string(),
                            "Priority Support".to_string(),
                        ],
                        cta_label: "Upgrade".to_string(),
                        on_cta: EventHandler::new(move |_| println!("Upgrading...")),
                        recommended: true,
                    }
                }
            }

            // Icons Showcase
            ComponentSection { title: "Icons",
                HStack {
                    style: "flex-wrap: wrap;",
                    justify: JustifyContent::Center,
                    gap: SpacingSize::Md,

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
                VStack {
                    gap: SpacingSize::Md,

                    Heading { level: HeadingLevel::H3, "Heading 3" }
                    Heading { level: HeadingLevel::H4, "Heading 4" }

                    Label { size: TextSize::Large, "Large text" }
                    Label { size: TextSize::Base, "Base text" }
                    Label { size: TextSize::Small, "Small text" }

                    MutedText { "This is muted/secondary text" }
                }
            }

            // New Form Controls
            ComponentSection { title: "New Form Controls",
                NewFormControlsShowcase {}
            }

            // Alert Showcase
            ComponentSection { title: "Alerts",
                VStack {
                    gap: SpacingSize::Md,

                    Alert {
                        variant: AlertVariant::Default,
                        title: Some("Note".to_string()),
                        "This is a default alert message."
                    }

                    Alert {
                        variant: AlertVariant::Success,
                        title: Some("Success".to_string()),
                        icon: Some("check-circle".to_string()),
                        "Your changes have been saved!"
                    }

                    Alert {
                        variant: AlertVariant::Warning,
                        title: Some("Warning".to_string()),
                        icon: Some("alert-triangle".to_string()),
                        "Please review your settings."
                    }
                }
            }

            // Avatar Showcase
            ComponentSection { title: "Avatars",
                HStack {
                    align: AlignItems::Center,
                    justify: JustifyContent::Center,
                    gap: SpacingSize::Md,

                    Avatar {
                        size: AvatarSize::Xs,
                        name: Some("John".to_string()),
                        src: None, alt: "".to_string(), fallback: None, style: None, class: None,
                    }
                    Avatar {
                        size: AvatarSize::Sm,
                        name: Some("Jane".to_string()),
                        src: None, alt: "".to_string(), fallback: None, style: None, class: None,
                    }
                    Avatar {
                        size: AvatarSize::Md,
                        name: Some("Bob".to_string()),
                        src: None, alt: "".to_string(), fallback: None, style: None, class: None,
                    }
                    Avatar {
                        size: AvatarSize::Lg,
                        name: Some("Alice".to_string()),
                        src: None, alt: "".to_string(), fallback: None, style: None, class: None,
                    }
                }
            }

            // Stepper Showcase
            ComponentSection { title: "Stepper",
                MobileStepperShowcase {}
            }

            // Interactive Demo
            ComponentSection { title: "Interactive",
                CounterDemo {}
            }
        }
    }
}

/// Stepper showcase for mobile
#[component]
fn MobileStepperShowcase() -> Element {
    let mut step = use_signal(|| 1);

    let steps = vec![
        StepItem::new("Account").with_icon("user"),
        StepItem::new("Profile").with_icon("settings"),
        StepItem::new("Done").with_icon("check"),
    ];

    rsx! {
        VStack {
            gap: SpacingSize::Md,

            // Horizontal Stepper
            HorizontalStepper {
                steps: steps,
                active_step: step(),
                size: StepSize::Md,
            }

            // Navigation
            HStack {
                justify: JustifyContent::Center,
                gap: SpacingSize::Sm,
                style: "margin-top: 8px;",

                Button {
                    variant: ButtonVariant::Secondary,
                    size: ButtonSize::Sm,
                    disabled: step() == 0,
                    onclick: move |_| if step() > 0 { step -= 1 },
                    "Back"
                }

                Button {
                    variant: ButtonVariant::Primary,
                    size: ButtonSize::Sm,
                    disabled: step() >= 2,
                    onclick: move |_| if step() < 2 { step += 1 },
                    "Next"
                }
            }

            // Step content
            Card {
                variant: CardVariant::Muted,
                padding: Some("16px".to_string()),

                div {
                    style: "text-align: center;",

                    match step() {
                        0 => rsx! { "Step 1: Create your account" },
                        1 => rsx! { "Step 2: Complete your profile" },
                        2 => rsx! { "Step 3: All done!" },
                        _ => rsx! { "Unknown step" },
                    }
                }
            }
        }
    }
}

/// New form controls showcase
#[component]
fn NewFormControlsShowcase() -> Element {
    let mut checked = use_signal(|| false);
    let mut switch_on = use_signal(|| true);
    let mut selected = use_signal(|| "".to_string());
    let mut textarea = use_signal(|| String::new());

    let options = vec![
        SelectOption::new("", "Select..."),
        SelectOption::new("option1", "Option 1"),
        SelectOption::new("option2", "Option 2"),
        SelectOption::new("option3", "Option 3"),
    ];

    rsx! {
        VStack {
            gap: SpacingSize::Md,

            // Checkbox
            Checkbox {
                checked: checked(),
                label: Some("Accept terms".to_string()),
                onchange: move |v| checked.set(v),
            }

            // Switch
            Switch {
                checked: switch_on(),
                label: Some(if switch_on() { "On" } else { "Off" }.to_string()),
                onchange: move |v| switch_on.set(v),
            }

            // Select
            Label { size: TextSize::Small, "Select:" }
            Select {
                value: selected(),
                options: options,
                placeholder: Some("Choose...".to_string()),
                onchange: move |v| selected.set(v),
            }

            // TextArea
            Label { size: TextSize::Small, "Message:" }
            TextArea {
                value: textarea(),
                placeholder: Some("Type here...".to_string()),
                rows: 3,
                onchange: move |v| textarea.set(v),
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
        VStack {
            gap: SpacingSize::Md,

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
        Box {
            style: "text-align: center; padding: 16px;",

            Heading {
                level: HeadingLevel::H2,
                "{count}"
            }

            HStack {
                justify: JustifyContent::Center,
                gap: SpacingSize::Md,
                style: "margin-top: 16px;",

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
