//! Shared components for all example apps
//!
//! This crate provides a comprehensive showcase of all UI components
//! that can be used across different platform examples.

pub mod layout_showcase;

use dioxus::prelude::*;
use dioxus_ui_system::atoms::{AlignItems, Box, HStack, JustifyContent, SpacingSize, VStack};
use dioxus_ui_system::atoms::{
    AspectRatio, AspectRatios, DatePicker, InputTag, NumberInput, Rating, Slider, Toggle,
    ToggleVariant,
};
use dioxus_ui_system::atoms::{StepSize, StepState, TextWeight};
use dioxus_ui_system::molecules::{
    use_sonner, Collapsible, Combobox, ComboboxOption, Command, CommandEmpty, CommandGroup,
    CommandInput, CommandItem, CommandList, ContextMenu, ContextMenuContent, ContextMenuItem,
    ContextMenuTrigger, HoverCard, HoverCardContent, HoverCardHeader, ListGroup, ListItem,
    ListItemVariant, MultiSelect, OtpInput, Pagination, PaginationInfo, QrCode, QrCodeLevel, Sheet,
    Sonner, TimePicker, ToastPosition, ToggleGroup, ToggleGroupType, ToggleItem,
};
use dioxus_ui_system::molecules::{HorizontalStepper, StepItem, VerticalStepper};
use dioxus_ui_system::organisms::*;
use dioxus_ui_system::organisms::{
    Calendar, CalendarMode, CarouselOptions, Direction, KanbanCard, KanbanColumn, MinimalRichText,
    ResizableHandle, ResizablePanel, ResizablePanelGroup, SimpleCarousel, SimpleKanban,
    SimpleTimeline, TimelineEvent, Tour, TourStep, Tree, TreeNodeData, WizardStep,
};
use dioxus_ui_system::organisms::{ColumnAlign, FilterOption, TableColumn, TableFilter};
use dioxus_ui_system::prelude::*;
pub use layout_showcase::{LayoutShowcase, LayoutShowcaseInner};

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
        Box {
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
                CardsOrganismsShowcase {}
                DataTableShowcase {}
                StepperShowcase {}
                IconShowcase {}
                TypographyShowcase {}
                NewFormControlsShowcase {}
                AlertShowcase {}
                AvatarShowcase {}
                DialogShowcase {}
                TabsShowcase {}
                AccordionShowcase {}
                SkeletonShowcase {}
                TooltipPopoverShowcase {}
                DropdownMenuShowcase {}
                InteractiveDemo {}
                ChartShowcase {}
                ThemeShowcase {}
                NewComponentsShowcase {}
                NewAtomsShowcase {}
                NewMoleculesShowcase {}
                NewOrganismsShowcase {}
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
                "A pure Rust design system for Dioxus with 100+ components"
            }

            HStack {
                justify: JustifyContent::Center,
                gap: SpacingSize::Sm,
                style: "margin-top: 16px;",
                Badge {
                    variant: BadgeVariant::Success,
                    icon: Some("check".to_string()),
                    "v0.3.0"
                }
                Badge {
                    variant: BadgeVariant::Secondary,
                    "Multi-Theme"
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
                VStack {
                    gap: SpacingSize::Md,

                    // Variants
                    HStack {
                        style: "flex-wrap: wrap;",
                        gap: SpacingSize::Sm,
                        Button { variant: ButtonVariant::Primary, "Primary" }
                        Button { variant: ButtonVariant::Secondary, "Secondary" }
                        Button { variant: ButtonVariant::Ghost, "Ghost" }
                        Button { variant: ButtonVariant::Destructive, "Destructive" }
                        Button { variant: ButtonVariant::Link, "Link" }
                    }

                    // Sizes
                    HStack {
                        align: AlignItems::Center,
                        style: "flex-wrap: wrap;",
                        gap: SpacingSize::Sm,
                        Button { size: ButtonSize::Sm, "Small" }
                        Button { size: ButtonSize::Md, "Medium" }
                        Button { size: ButtonSize::Lg, "Large" }
                    }

                    // States
                    HStack {
                        style: "flex-wrap: wrap;",
                        gap: SpacingSize::Sm,
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
                VStack {
                    gap: SpacingSize::Md,

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
                VStack {
                    gap: SpacingSize::Md,

                    // Variants
                    HStack {
                        style: "flex-wrap: wrap;",
                        gap: SpacingSize::Sm,
                        Badge { "Default" }
                        Badge { variant: BadgeVariant::Secondary, "Secondary" }
                        Badge { variant: BadgeVariant::Success, icon: Some("check".to_string()), "Success" }
                        Badge { variant: BadgeVariant::Warning, "Warning" }
                        Badge { variant: BadgeVariant::Destructive, "Error" }
                        Badge { variant: BadgeVariant::Outline, "Outline" }
                        Badge { variant: BadgeVariant::Ghost, "Ghost" }
                    }

                    // Status badges
                    HStack {
                        style: "flex-wrap: wrap;",
                        gap: SpacingSize::Sm,
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
                VStack {
                    gap: SpacingSize::Md,

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

/// Card Organisms showcase - demonstrates all card organism variations
#[component]
fn CardsOrganismsShowcase() -> Element {
    let mut notification_visible = use_signal(|| true);

    rsx! {
        Card {
            CardHeader {
                title: "Card Organisms",
                subtitle: Some("10+ pre-built card patterns".to_string()),
            }
            CardContent {
                VStack {
                    gap: SpacingSize::Lg,

                    // Action Card
                    ActionCard {
                        title: "Deploy Project",
                        description: "Your project is ready to deploy to production. Click the button below to start the deployment process.",
                        action_label: "Deploy Now",
                        on_action: move |_| println!("Deploying project..."),
                        icon: Some("rocket".to_string()),
                    }

                    // Dual Action Card
                    DualActionCard {
                        title: "Save Changes?",
                        description: "You have unsaved changes. Would you like to save them before leaving?",
                        primary_label: "Save",
                        secondary_label: "Discard",
                        on_primary: move |_| println!("Saving..."),
                        on_secondary: move |_| println!("Discarding..."),
                    }

                    // Profile Card
                    ProfileCard {
                        name: "Sarah Chen".to_string(),
                        role: Some("Senior Engineer".to_string()),
                        avatar_url: None,
                        description: Some("Full-stack developer passionate about Rust and WebAssembly.".to_string()),
                        action_label: "Connect".to_string(),
                        on_action: Some(EventHandler::new(move |_| println!("Connecting to Sarah..."))),
                        stats: vec![
                            ("Projects".to_string(), "24".to_string()),
                            ("Followers".to_string(), "1.2K".to_string()),
                        ],
                    }

                    // Stat Cards Row
                    div {
                        style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(140px, 1fr)); gap: 12px;",

                        StatCard {
                            label: "Revenue",
                            value: "$48.2K",
                            change: Some("+12%".to_string()),
                            change_positive: Some(true),
                            icon: Some("trending-up".to_string()),
                            icon_bg: "#dbeafe".to_string(),
                        }

                        StatCard {
                            label: "Users",
                            value: "2,420",
                            change: Some("+5%".to_string()),
                            change_positive: Some(true),
                            icon: Some("users".to_string()),
                            icon_bg: "#dcfce7".to_string(),
                        }

                        StatCard {
                            label: "Bounce Rate",
                            value: "42%",
                            change: Some("-3%".to_string()),
                            change_positive: Some(true),
                            icon: Some("activity".to_string()),
                            icon_bg: "#fef3c7".to_string(),
                        }
                    }

                    // Notification Cards
                    if notification_visible() {
                        NotificationCard {
                            title: "Success!".to_string(),
                            message: "Your changes have been saved successfully.".to_string(),
                            notification_type: NotificationType::Success,
                            on_dismiss: Some(EventHandler::new(move |_| notification_visible.set(false))),
                        }
                    }

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
                                "Click to expand and see more options..."
                            }
                        },
                        expanded_content: rsx! {
                            VStack {
                                gap: SpacingSize::Md,
                                style: "padding-top: 12px;",

                                Label { "API Key" }
                                InputGroup {
                                    label: "",
                                    value: "sk-xxxxxxxxxxxx".to_string(),
                                    input_type: InputType::Password,
                                    onchange: move |_| {},
                                }

                                Checkbox {
                                    checked: true,
                                    label: Some("Enable notifications".to_string()),
                                    onchange: move |_| {},
                                }
                            }
                        },
                        default_expanded: false,
                    }

                    // Pricing Cards Row
                    div {
                        style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 16px;",

                        PricingCard {
                            plan: "Starter".to_string(),
                            price: "$9".to_string(),
                            period: "/month".to_string(),
                            description: Some("Perfect for individuals".to_string()),
                            features: vec![
                                "5 Projects".to_string(),
                                "10GB Storage".to_string(),
                                "Basic Support".to_string(),
                            ],
                            cta_label: "Get Started".to_string(),
                            on_cta: EventHandler::new(move |_| println!("Selected Starter")),
                            recommended: false,
                        }

                        PricingCard {
                            plan: "Pro".to_string(),
                            price: "$29".to_string(),
                            period: "/month".to_string(),
                            description: Some("For growing teams".to_string()),
                            features: vec![
                                "Unlimited Projects".to_string(),
                                "100GB Storage".to_string(),
                                "Priority Support".to_string(),
                                "Advanced Analytics".to_string(),
                            ],
                            cta_label: "Upgrade".to_string(),
                            on_cta: EventHandler::new(move |_| println!("Selected Pro")),
                            recommended: true,
                        }
                    }

                    // Image Card
                    ImageCard {
                        image_url: "https://images.unsplash.com/photo-1618005182384-a83a8bd57fbe?w=400&h=300&fit=crop".to_string(),
                        image_alt: "Abstract Design".to_string(),
                        title: "Abstract Art".to_string(),
                        description: "A beautiful abstract design with flowing colors.".to_string(),
                        action_label: Some("View Gallery".to_string()),
                        on_action: Some(EventHandler::new(move |_| println!("Viewing gallery..."))),
                        aspect_ratio: "16/9".to_string(),
                    }

                    // Image Action Card
                    ImageActionCard {
                        image_url: "https://images.unsplash.com/photo-1558655146-9f40138edfeb?w=400&h=250&fit=crop".to_string(),
                        title: "Design System".to_string(),
                        description: "Learn how to build scalable design systems for your team.".to_string(),
                        primary_label: "Read".to_string(),
                        secondary_label: "Save".to_string(),
                        on_primary: EventHandler::new(move |_| println!("Reading article...")),
                        on_secondary: EventHandler::new(move |_| println!("Saving article...")),
                        badge: Some("New".to_string()),
                    }

                    // Horizontal Card
                    HorizontalCard {
                        image_url: "https://images.unsplash.com/photo-1498050108023-c5249f4df085?w=200&h=200&fit=crop".to_string(),
                        title: "Featured Article".to_string(),
                        description: "Discover the latest trends in UI design and how to apply them to your projects.".to_string(),
                        action_label: Some("Read".to_string()),
                        on_action: Some(EventHandler::new(move |_| println!("Reading..."))),
                    }

                    // Media Card
                    MediaCard {
                        title: "Getting Started Video".to_string(),
                        media_type: MediaType::Video,
                        media_url: "https://images.unsplash.com/photo-1492691527719-9d1e07e534b4?w=400&h=225&fit=crop".to_string(),
                        creator: Some("Dioxus UI Team".to_string()),
                        duration: Some("5:32".to_string()),
                        on_play: Some(EventHandler::new(move |_| println!("Playing video..."))),
                        on_like: Some(EventHandler::new(move |_| println!("Liked!"))),
                        on_share: Some(EventHandler::new(move |_| println!("Shared!"))),
                    }
                }
            }
        }
    }
}

/// Stepper showcase section
#[component]
fn StepperShowcase() -> Element {
    let mut active_step = use_signal(|| 1);
    let mut wizard_step = use_signal(|| 0);

    let steps = vec![
        StepItem::new("Account")
            .with_description("Login details")
            .with_icon("user"),
        StepItem::new("Personal")
            .with_description("Your info")
            .with_icon("settings"),
        StepItem::new("Review")
            .with_description("Verify data")
            .with_icon("check"),
        StepItem::new("Complete")
            .with_description("Done!")
            .with_icon("star"),
    ];
    let steps_len = steps.len();

    let wizard_steps = vec![
        WizardStep::new("Account Setup").with_description("Create your account"),
        WizardStep::new("Profile").with_description("Add your details"),
        WizardStep::new("Preferences").with_description("Set your preferences"),
    ];

    rsx! {
        Card {
            CardHeader {
                title: "Stepper",
                subtitle: Some("Multi-step progress indicators".to_string()),
            }
            CardContent {
                VStack {
                    gap: SpacingSize::Xl,

                    // Horizontal Stepper
                    VStack {
                        gap: SpacingSize::Sm,

                        Label { size: TextSize::Small, color: TextColor::Muted, "Horizontal Stepper" }

                        HorizontalStepper {
                            steps: steps.clone(),
                            active_step: active_step(),
                            size: StepSize::Md,
                            on_step_click: Some(EventHandler::new(move |step: usize| {
                                active_step.set(step);
                            })),
                        }
                    }

                    // Step Navigation
                    HStack {
                        justify: JustifyContent::Center,
                        gap: SpacingSize::Sm,

                        Button {
                            variant: ButtonVariant::Secondary,
                            size: ButtonSize::Sm,
                            disabled: active_step() == 0,
                            onclick: move |_| if active_step() > 0 { active_step -= 1 },
                            "Previous"
                        }

                        Button {
                            variant: ButtonVariant::Primary,
                            size: ButtonSize::Sm,
                            disabled: active_step() >= steps_len - 1,
                            onclick: move |_| if active_step() < steps_len - 1 { active_step += 1 },
                            "Next"
                        }
                    }

                    Separator {}

                    // Vertical Stepper
                    VStack {
                        gap: SpacingSize::Sm,

                        Label { size: TextSize::Small, color: TextColor::Muted, "Vertical Stepper" }

                        div {
                            style: "max-width: 300px;",

                            VerticalStepper {
                                steps: vec![
                                    StepItem::new("Upload").with_description("File uploaded successfully").with_state(StepState::Completed),
                                    StepItem::new("Process").with_description("Processing your file...").with_state(StepState::Active),
                                    StepItem::new("Review").with_description("Pending"),
                                    StepItem::new("Publish").with_description("Pending"),
                                ],
                                active_step: 1,
                                size: StepSize::Md,
                            }
                        }
                    }

                    Separator {}

                    // Compact Stepper
                    VStack {
                        gap: SpacingSize::Sm,

                        Label { size: TextSize::Small, color: TextColor::Muted, "Compact Stepper" }

                        CompactStepper {
                            steps: steps.clone(),
                            active_step: active_step(),
                            size: StepSize::Sm,
                        }
                    }

                    Separator {}

                    // Wizard
                    VStack {
                        gap: SpacingSize::Sm,

                        Label { size: TextSize::Small, color: TextColor::Muted, "Wizard with Content" }

                        Wizard {
                            steps: wizard_steps,
                            active_step: wizard_step(),
                            on_step_change: EventHandler::new(move |step: usize| wizard_step.set(step)),
                            on_finish: EventHandler::new(move |_| println!("Wizard completed!")),
                            title: Some("Setup Wizard".to_string()),

                            Box {
                                style: "padding: 16px; background: #f8fafc; border-radius: 8px;",

                                match wizard_step() {
                                    0 => rsx! {
                                        VStack {
                                            gap: SpacingSize::Md,
                                            Label { "Step 1: Account Setup" }
                                            p { style: "margin: 0; color: #64748b;", "Please enter your account details." }
                                            InputGroup {
                                                label: "Email",
                                                value: "".to_string(),
                                                input_type: InputType::Email,
                                                onchange: move |_| {},
                                            }
                                        }
                                    },
                                    1 => rsx! {
                                        VStack {
                                            gap: SpacingSize::Md,
                                            Label { "Step 2: Profile" }
                                            p { style: "margin: 0; color: #64748b;", "Tell us about yourself." }
                                            InputGroup {
                                                label: "Full Name",
                                                value: "".to_string(),
                                                onchange: move |_| {},
                                            }
                                        }
                                    },
                                    2 => rsx! {
                                        VStack {
                                            gap: SpacingSize::Md,
                                            Label { "Step 3: Preferences" }
                                            p { style: "margin: 0; color: #64748b;", "Choose your preferences." }
                                            Checkbox {
                                                checked: true,
                                                label: Some("Enable notifications".to_string()),
                                                onchange: move |_| {},
                                            }
                                        }
                                    },
                                    _ => rsx! { "Unknown step" },
                                }
                            }
                        }
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
                VStack {
                    gap: SpacingSize::Md,

                    // Icon grid
                    HStack {
                        style: "flex-wrap: wrap;",
                        justify: JustifyContent::Center,
                        gap: SpacingSize::Md,

                        for (name, color) in icons {
                            VStack {
                                align: AlignItems::Center,
                                gap: SpacingSize::Xs,
                                style: "padding: 8px;",
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
                    HStack {
                        align: AlignItems::Center,
                        justify: JustifyContent::Center,
                        gap: SpacingSize::Md,
                        style: "margin-top: 16px;",
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
                VStack {
                    gap: SpacingSize::Md,

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

/// New form controls showcase
#[component]
fn NewFormControlsShowcase() -> Element {
    let mut checked = use_signal(|| false);
    let mut radio_value = use_signal(|| "option1".to_string());
    let mut switch_on = use_signal(|| true);
    let mut selected = use_signal(|| "".to_string());
    let mut textarea = use_signal(|| String::new());

    let options = vec![
        SelectOption::new("", "Select an option..."),
        SelectOption::new("option1", "Option 1"),
        SelectOption::new("option2", "Option 2"),
        SelectOption::new("option3", "Option 3"),
    ];

    rsx! {
        Card {
            CardHeader {
                title: "New Form Controls",
                subtitle: Some("Checkbox, Radio, Switch, Select, TextArea".to_string()),
            }
            CardContent {
                VStack {
                    gap: SpacingSize::Md,

                    // Checkbox
                    Checkbox {
                        checked: checked(),
                        label: Some("Accept terms and conditions".to_string()),
                        onchange: move |v| checked.set(v),
                    }

                    // Radio
                    VStack {
                        gap: SpacingSize::Sm,

                        Label { "Select an option:" }

                        Radio {
                            name: "radio-group".to_string(),
                            value: "option1".to_string(),
                            checked: radio_value() == "option1",
                            label: Some("Option 1".to_string()),
                            onchange: move |_| radio_value.set("option1".to_string()),
                        }
                        Radio {
                            name: "radio-group".to_string(),
                            value: "option2".to_string(),
                            checked: radio_value() == "option2",
                            label: Some("Option 2".to_string()),
                            onchange: move |_| radio_value.set("option2".to_string()),
                        }
                        Radio {
                            name: "radio-group".to_string(),
                            value: "option3".to_string(),
                            checked: radio_value() == "option3",
                            label: Some("Option 3".to_string()),
                            onchange: move |_| radio_value.set("option3".to_string()),
                        }
                    }

                    Separator {}

                    // Switch
                    Switch {
                        checked: switch_on(),
                        label: Some(if switch_on() { "Notifications ON" } else { "Notifications OFF" }.to_string()),
                        onchange: move |v| switch_on.set(v),
                    }

                    Separator {}

                    // Select
                    Label { "Choose from dropdown:" }
                    Select {
                        value: selected(),
                        options: options,
                        placeholder: Some("Select...".to_string()),
                        onchange: move |v| selected.set(v),
                    }

                    Separator {}

                    // TextArea
                    Label { "Your message:" }
                    TextArea {
                        value: textarea(),
                        placeholder: Some("Type your message here...".to_string()),
                        rows: 3,
                        max_length: Some(200),
                        onchange: move |v| textarea.set(v),
                    }
                }
            }
        }
    }
}

/// Alert showcase section
#[component]
fn AlertShowcase() -> Element {
    rsx! {
        Card {
            CardHeader {
                title: "Alerts",
                subtitle: Some("Status messages and notifications".to_string()),
            }
            CardContent {
                VStack {
                    gap: SpacingSize::Md,

                    Alert {
                        variant: AlertVariant::Default,
                        title: Some("Note".to_string()),
                        "This is a default alert with some information."
                    }

                    Alert {
                        variant: AlertVariant::Success,
                        title: Some("Success".to_string()),
                        icon: Some("check-circle".to_string()),
                        "Your changes have been saved successfully."
                    }

                    Alert {
                        variant: AlertVariant::Warning,
                        title: Some("Warning".to_string()),
                        icon: Some("alert-triangle".to_string()),
                        "Please review your settings before continuing."
                    }

                    Alert {
                        variant: AlertVariant::Destructive,
                        title: Some("Error".to_string()),
                        icon: Some("alert-triangle".to_string()),
                        "Something went wrong. Please try again."
                    }
                }
            }
        }
    }
}

/// Avatar showcase section
#[component]
fn AvatarShowcase() -> Element {
    rsx! {
        Card {
            CardHeader {
                title: "Avatars",
                subtitle: Some("User profile images with fallbacks".to_string()),
            }
            CardContent {
                VStack {
                    gap: SpacingSize::Md,

                    // Different sizes
                    HStack {
                        align: AlignItems::Center,
                        style: "flex-wrap: wrap;",
                        gap: SpacingSize::Md,

                        Avatar {
                            size: AvatarSize::Xs,
                            name: Some("John Doe".to_string()),
                            src: None,
                            alt: "".to_string(),
                            fallback: None,
                            style: None,
                            class: None,
                        }
                        Avatar {
                            size: AvatarSize::Sm,
                            name: Some("Jane Smith".to_string()),
                            src: None,
                            alt: "".to_string(),
                            fallback: None,
                            style: None,
                            class: None,
                        }
                        Avatar {
                            size: AvatarSize::Md,
                            name: Some("Bob Wilson".to_string()),
                            src: None,
                            alt: "".to_string(),
                            fallback: None,
                            style: None,
                            class: None,
                        }
                        Avatar {
                            size: AvatarSize::Lg,
                            name: Some("Alice Brown".to_string()),
                            src: None,
                            alt: "".to_string(),
                            fallback: None,
                            style: None,
                            class: None,
                        }
                        Avatar {
                            size: AvatarSize::Xl,
                            name: Some("Charlie Davis".to_string()),
                            src: None,
                            alt: "".to_string(),
                            fallback: None,
                            style: None,
                            class: None,
                        }
                    }

                    Separator {}

                    // With image
                    HStack {
                        align: AlignItems::Center,
                        gap: SpacingSize::Md,

                        Label { "With initials fallback:" }
                        Avatar {
                            size: AvatarSize::Lg,
                            name: Some("Sarah Connor".to_string()),
                            src: None,
                            alt: "".to_string(),
                            fallback: None,
                            style: None,
                            class: None,
                        }
                    }
                }
            }
        }
    }
}

/// Dialog showcase section
#[component]
fn DialogShowcase() -> Element {
    let mut dialog_open = use_signal(|| false);
    let mut alert_open = use_signal(|| false);

    rsx! {
        Card {
            CardHeader {
                title: "Dialogs",
                subtitle: Some("Modal windows and alerts".to_string()),
            }
            CardContent {
                VStack {
                    gap: SpacingSize::Md,

                    Button {
                        variant: ButtonVariant::Primary,
                        onclick: move |_| dialog_open.set(true),
                        "Open Dialog"
                    }

                    Button {
                        variant: ButtonVariant::Destructive,
                        onclick: move |_| alert_open.set(true),
                        "Open Alert Dialog"
                    }

                    Dialog {
                        open: dialog_open(),
                        on_close: move |_| dialog_open.set(false),
                        title: Some("Example Dialog".to_string()),
                        description: Some("This is a dialog component with rich content.".to_string()),

                        p {
                            style: "margin: 0; font-size: 14px; color: #64748b; line-height: 1.5;",
                            "Dialogs are great for displaying important information or getting user confirmation for actions."
                        }

                        DialogFooter {
                            align: DialogFooterAlign::End,

                            Button {
                                variant: ButtonVariant::Ghost,
                                onclick: move |_| dialog_open.set(false),
                                "Cancel"
                            }

                            Button {
                                variant: ButtonVariant::Primary,
                                onclick: move |_| dialog_open.set(false),
                                "Confirm"
                            }
                        }
                    }

                    AlertDialog {
                        open: alert_open(),
                        on_close: move |_| alert_open.set(false),
                        title: "Are you sure?".to_string(),
                        description: "This action cannot be undone. This will permanently delete your account.".to_string(),
                        on_confirm: move |_| alert_open.set(false),
                        destructive: true,
                    }
                }
            }
        }
    }
}

/// Tabs showcase section
#[component]
fn TabsShowcase() -> Element {
    let mut active_tab = use_signal(|| "account".to_string());

    let tabs = vec![
        TabItem::new("account", "Account").with_icon("settings"),
        TabItem::new("password", "Password"),
        TabItem::new("notifications", "Notifications"),
    ];

    rsx! {
        Card {
            CardHeader {
                title: "Tabs",
                subtitle: Some("Tabbed content navigation".to_string()),
            }
            CardContent {
                VStack {
                    gap: SpacingSize::Md,

                    Tabs {
                        tabs: tabs,
                        active_tab: active_tab(),
                        on_change: move |id| active_tab.set(id),

                        TabPanel {
                            if active_tab() == "account" {
                                "Manage your account settings and preferences."
                            } else if active_tab() == "password" {
                                "Change your password and security settings."
                            } else {
                                "Configure your notification preferences."
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Accordion showcase section
#[component]
fn AccordionShowcase() -> Element {
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
            "Yes. It comes with default styles that match the other components.",
        ),
        AccordionItem::new(
            "item3",
            "Is it animated?",
            "Yes. It's animated by default, but you can disable it if you prefer.",
        ),
    ];

    rsx! {
        Card {
            CardHeader {
                title: "Accordion",
                subtitle: Some("Collapsible content sections".to_string()),
            }
            CardContent {
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

/// Skeleton showcase section
#[component]
fn SkeletonShowcase() -> Element {
    let mut loading = use_signal(|| true);

    rsx! {
        Card {
            CardHeader {
                title: "Skeleton",
                subtitle: Some("Loading placeholders".to_string()),
            }
            CardContent {
                VStack {
                    gap: SpacingSize::Md,

                    Switch {
                        checked: loading(),
                        label: Some("Show Skeleton".to_string()),
                        onchange: move |v| loading.set(v),
                    }

                    if loading() {
                        VStack {
                            gap: SpacingSize::Md,

                            HStack {
                                align: AlignItems::Center,
                                gap: SpacingSize::Md,
                                SkeletonCircle {
                                    size: "48".to_string(),
                                    animate: true,
                                    style: None,
                                }
                                VStack {
                                    style: "flex: 1;",
                                    gap: SpacingSize::Sm,
                                    Skeleton {
                                        width: Some("150px".to_string()),
                                        height: None,
                                        animated: true,
                                        shape: SkeletonShape::Rectangle,
                                        animation: SkeletonAnimation::Pulse,
                                        color: None,
                                        highlight_color: None,
                                        class: None,
                                    }
                                    Skeleton {
                                        width: Some("100px".to_string()),
                                        height: None,
                                        animated: true,
                                        shape: SkeletonShape::Rectangle,
                                        animation: SkeletonAnimation::Pulse,
                                        color: None,
                                        highlight_color: None,
                                        class: None,
                                    }
                                }
                            }

                            SkeletonText {
                                lines: 3,
                                animate: true,
                                last_line_width: 60,
                                style: None,
                            }
                        }
                    } else {
                        HStack {
                            align: AlignItems::Center,
                            gap: SpacingSize::Md,

                            Avatar {
                                size: AvatarSize::Lg,
                                name: Some("Loading Complete".to_string()),
                                src: None,
                                alt: "".to_string(),
                                fallback: None,
                                style: None,
                                class: None,
                            }

                            div {
                                p { style: "margin: 0; font-weight: 600;", "Content Loaded" }
                                p { style: "margin: 0; color: #64748b; font-size: 14px;", "The skeleton has been replaced with actual content." }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Tooltip and Popover showcase
#[component]
fn TooltipPopoverShowcase() -> Element {
    rsx! {
        Card {
            CardHeader {
                title: "Tooltips & Popovers",
                subtitle: Some("Contextual information displays".to_string()),
            }
            CardContent {
                HStack {
                    style: "flex-wrap: wrap;",
                    align: AlignItems::Center,
                    gap: SpacingSize::Md,

                    SimpleTooltip {
                        text: "This is a tooltip".to_string(),
                        placement: TooltipPlacement::Top,

                        Button {
                            variant: ButtonVariant::Secondary,
                            "Hover for Tooltip"
                        }
                    }

                    Popover {
                        trigger: rsx! {
                            Button {
                                variant: ButtonVariant::Secondary,
                                "Click for Popover"
                            }
                        },

                        PopoverHeader {
                            title: "Popover Title".to_string(),
                            description: Some("This is a popover with more detailed content.".to_string()),
                        }

                        "Popovers can contain rich content, forms, and other interactive elements."

                        PopoverFooter {
                            Button {
                                variant: ButtonVariant::Primary,
                                "Action"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Dropdown menu showcase
#[component]
fn DropdownMenuShowcase() -> Element {
    let mut last_action = use_signal(|| "No action yet".to_string());

    let items = vec![
        DropdownMenuItem::new("edit", "Edit").with_icon("edit"),
        DropdownMenuItem::new("duplicate", "Duplicate").with_icon("copy"),
        DropdownMenuItem::new("delete", "Delete")
            .with_icon("trash")
            .disabled(),
    ];

    rsx! {
        Card {
            CardHeader {
                title: "Dropdown Menu",
                subtitle: Some("Contextual action menus".to_string()),
            }
            CardContent {
                VStack {
                    gap: SpacingSize::Md,

                    DropdownMenu {
                        trigger: rsx! {
                            Button {
                                variant: ButtonVariant::Secondary,
                                "Open Menu ▼"
                            }
                        },
                        items: items,
                        on_select: move |value| last_action.set(format!("Selected: {}", value)),
                    }

                    Label {
                        color: TextColor::Muted,
                        "{last_action}"
                    }
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
                Box {
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

                    HStack {
                        justify: JustifyContent::Center,
                        gap: SpacingSize::Sm,
                        style: "margin-top: 16px;",

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

/// Chart showcase with tooltip examples
#[component]
fn ChartShowcase() -> Element {
    use dioxus_ui_system::organisms::charts::*;

    let bar_data = vec![
        ChartDataPoint::new("Jan", 100.0),
        ChartDataPoint::new("Feb", 150.0),
        ChartDataPoint::new("Mar", 200.0),
        ChartDataPoint::new("Apr", 180.0),
        ChartDataPoint::new("May", 250.0),
        ChartDataPoint::new("Jun", 300.0),
    ];

    let pie_data = vec![
        ChartDataPoint::new("Desktop", 45.0),
        ChartDataPoint::new("Mobile", 35.0),
        ChartDataPoint::new("Tablet", 20.0),
    ];

    rsx! {
        Card {
            CardHeader {
                title: "Charts with Tooltips",
                subtitle: Some("Hover over chart elements to see tooltips".to_string()),
            }
            CardContent {
                VStack {
                    gap: SpacingSize::Lg,

                    // Bar chart with tooltips
                    div {
                        BarChart {
                            title: Some("Monthly Revenue (with tooltips)".to_string()),
                            data: Some(bar_data.clone()),
                            width: "100%".to_string(),
                            height: "200px".to_string(),
                            tooltip: ChartTooltip::with_formatter(|point, _| {
                                format!("{}: ${:.0}K", point.label, point.value)
                            }),
                        }
                    }

                    // Pie chart with tooltips
                    HStack {
                        justify: JustifyContent::Center,
                        DonutChart {
                            data: pie_data.clone(),
                            width: "200px".to_string(),
                            height: "200px".to_string(),
                            show_center_text: true,
                            tooltip: ChartTooltip::with_formatter(|point, _| {
                                format!("{}: {:.0}%", point.label, point.value)
                            }),
                        }
                    }

                    Label {
                        size: TextSize::ExtraSmall,
                        color: TextColor::Muted,
                        "Charts support custom tooltips, multiple series, and various chart types"
                    }
                }
            }
        }
    }
}

/// DataTable showcase with search and filters
#[component]
fn DataTableShowcase() -> Element {
    use std::collections::HashMap;

    #[derive(Clone, PartialEq)]
    struct Product {
        id: String,
        name: String,
        category: String,
        price: f64,
        stock: i32,
        status: String,
    }

    let all_products = vec![
        Product {
            id: "1".to_string(),
            name: "Wireless Headphones".to_string(),
            category: "Electronics".to_string(),
            price: 99.99,
            stock: 45,
            status: "In Stock".to_string(),
        },
        Product {
            id: "2".to_string(),
            name: "Running Shoes".to_string(),
            category: "Sports".to_string(),
            price: 129.99,
            stock: 12,
            status: "Low Stock".to_string(),
        },
        Product {
            id: "3".to_string(),
            name: "Coffee Maker".to_string(),
            category: "Home".to_string(),
            price: 79.99,
            stock: 0,
            status: "Out of Stock".to_string(),
        },
        Product {
            id: "4".to_string(),
            name: "Laptop Stand".to_string(),
            category: "Electronics".to_string(),
            price: 49.99,
            stock: 78,
            status: "In Stock".to_string(),
        },
        Product {
            id: "5".to_string(),
            name: "Yoga Mat".to_string(),
            category: "Sports".to_string(),
            price: 29.99,
            stock: 34,
            status: "In Stock".to_string(),
        },
        Product {
            id: "6".to_string(),
            name: "Desk Lamp".to_string(),
            category: "Home".to_string(),
            price: 39.99,
            stock: 5,
            status: "Low Stock".to_string(),
        },
        Product {
            id: "7".to_string(),
            name: "Bluetooth Speaker".to_string(),
            category: "Electronics".to_string(),
            price: 59.99,
            stock: 23,
            status: "In Stock".to_string(),
        },
        Product {
            id: "8".to_string(),
            name: "Water Bottle".to_string(),
            category: "Sports".to_string(),
            price: 19.99,
            stock: 0,
            status: "Out of Stock".to_string(),
        },
    ];

    #[derive(Clone, PartialEq)]
    struct ProductRef(Product);

    fn render_price(product: &ProductRef) -> Element {
        let price = product.0.price;
        rsx! {
            Label {
                weight: TextWeight::Semibold,
                "{price:.2}"
            }
        }
    }

    fn render_status(product: &ProductRef) -> Element {
        let (bg, color) = match product.0.status.as_str() {
            "In Stock" => ("rgb(220,252,231)", "rgb(22,163,74)"),
            "Low Stock" => ("rgb(254,249,195)", "rgb(161,98,7)"),
            _ => ("rgb(254,226,226)", "rgb(220,38,38)"),
        };
        rsx! {
            span {
                style: "padding: 4px 10px; background: {bg}; color: {color}; border-radius: 9999px; font-size: 12px; font-weight: 500;",
                "{product.0.status}"
            }
        }
    }

    let columns = vec![
        TableColumn {
            key: "name".to_string(),
            header: "Product".to_string(),
            width: Some("180px".to_string()),
            align: ColumnAlign::Left,
            sortable: true,
            render: None,
        },
        TableColumn {
            key: "category".to_string(),
            header: "Category".to_string(),
            width: Some("120px".to_string()),
            align: ColumnAlign::Left,
            sortable: true,
            render: None,
        },
        TableColumn {
            key: "price".to_string(),
            header: "Price".to_string(),
            width: Some("100px".to_string()),
            align: ColumnAlign::Right,
            sortable: true,
            render: Some(|p: &ProductRef| render_price(p)),
        },
        TableColumn {
            key: "stock".to_string(),
            header: "Stock".to_string(),
            width: Some("80px".to_string()),
            align: ColumnAlign::Center,
            sortable: true,
            render: None,
        },
        TableColumn {
            key: "status".to_string(),
            header: "Status".to_string(),
            width: Some("120px".to_string()),
            align: ColumnAlign::Center,
            sortable: false,
            render: Some(|p: &ProductRef| render_status(p)),
        },
    ];

    let filters = vec![
        TableFilter {
            key: "category".to_string(),
            label: "All Categories".to_string(),
            options: vec![
                FilterOption {
                    label: "Electronics".to_string(),
                    value: "Electronics".to_string(),
                },
                FilterOption {
                    label: "Sports".to_string(),
                    value: "Sports".to_string(),
                },
                FilterOption {
                    label: "Home".to_string(),
                    value: "Home".to_string(),
                },
            ],
        },
        TableFilter {
            key: "status".to_string(),
            label: "All Status".to_string(),
            options: vec![
                FilterOption {
                    label: "In Stock".to_string(),
                    value: "In Stock".to_string(),
                },
                FilterOption {
                    label: "Low Stock".to_string(),
                    value: "Low Stock".to_string(),
                },
                FilterOption {
                    label: "Out of Stock".to_string(),
                    value: "Out of Stock".to_string(),
                },
            ],
        },
    ];

    // State for search and filters
    let mut search_query = use_signal(|| "".to_string());
    let mut active_filters = use_signal(|| HashMap::<String, String>::new());

    // Filter the data
    let filtered_products: Vec<ProductRef> = all_products
        .clone()
        .into_iter()
        .map(ProductRef)
        .filter(|p| {
            let search_lower = search_query().to_lowercase();
            let matches_search = search_lower.is_empty()
                || p.0.name.to_lowercase().contains(&search_lower)
                || p.0.category.to_lowercase().contains(&search_lower);

            let matches_category = active_filters()
                .get("category")
                .map_or(true, |v| v == &p.0.category);
            let matches_status = active_filters()
                .get("status")
                .map_or(true, |v| v == &p.0.status);

            matches_search && matches_category && matches_status
        })
        .collect();

    rsx! {
        Card {
            CardHeader {
                title: "DataTable with Search & Filters",
                subtitle: Some("Filter by category, status, or search by name".to_string()),
            }
            CardContent {
                DataTable {
                    data: filtered_products,
                    columns: columns,
                    key_extractor: |p: &ProductRef| p.0.id.clone(),
                    empty_message: "No products match your criteria",
                    loading: false,
                    search_query: Some(search_query()),
                    on_search_change: Some(EventHandler::new(move |q: String| search_query.set(q))),
                    search_placeholder: "Search products...",
                    filters: filters,
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
                subtitle: Some("7 preset themes: Light, Dark, Rose, Blue, Green, Violet, Orange".to_string()),
            }
            CardContent {
                VStack {
                    align: AlignItems::Center,
                    gap: SpacingSize::Md,

                    Label {
                        size: TextSize::Small,
                        "Select a theme:"
                    }

                    ThemeSelector {}

                    Separator {}

                    Label {
                        size: TextSize::Small,
                        "Or toggle dark mode:"
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

/// New Atom Components Showcase
#[component]
fn NewAtomsShowcase() -> Element {
    let mut rating = use_signal(|| 3.5_f32);
    let mut slider_value = use_signal(|| 50.0);
    let mut date = use_signal(|| "2024-01-15".to_string());
    let mut tags = use_signal(|| vec!["Rust".to_string(), "Dioxus".to_string()]);
    let mut toggle_on = use_signal(|| false);
    let mut number = use_signal(|| 42.0);

    rsx! {
        Card {
            overflow_hidden: false,
            CardHeader {
                title: "New Atom Components",
                subtitle: Some("Rating, DatePicker, Slider, Tag, Toggle, NumberInput, AspectRatio".to_string()),
            }
            CardContent {
                VStack {
                    gap: SpacingSize::Lg,

                    // Rating Demo
                    VStack {
                        gap: SpacingSize::Sm,
                        Label { size: TextSize::Small, "Rating (Read-only & Interactive)" }
                        HStack {
                            gap: SpacingSize::Md,
                            Rating { value: 4.5, max: 5, size: 20, interactive: false }
                            Rating {
                                value: rating(),
                                max: 5,
                                size: 20,
                                interactive: true,
                                on_change: Some(EventHandler::new(move |v: f32| rating.set(v))),
                            }
                        }
                        Label { size: TextSize::ExtraSmall, color: TextColor::Muted, "Value: {rating:.1}" }
                    }

                    Separator {}

                    // Slider Demo
                    VStack {
                        gap: SpacingSize::Sm,
                        Label { size: TextSize::Small, "Slider" }
                        Slider {
                            value: slider_value(),
                            min: 0.0,
                            max: 100.0,
                            step: 1.0,
                            on_change: move |v| slider_value.set(v),
                        }
                        Label { size: TextSize::ExtraSmall, color: TextColor::Muted, "Value: {slider_value:.0}%" }
                    }

                    Separator {}

                    // DatePicker Demo
                    VStack {
                        gap: SpacingSize::Sm,
                        Label { size: TextSize::Small, "DatePicker" }
                        DatePicker {
                            value: date(),
                            on_change: move |v| date.set(v),
                            placeholder: Some("Select a date...".to_string()),
                        }
                    }

                    Separator {}

                    // Tag Input Demo
                    VStack {
                        gap: SpacingSize::Sm,
                        Label { size: TextSize::Small, "Tag Input" }
                        InputTag {
                            tags: tags(),
                            on_change: move |t: Vec<String>| tags.set(t),
                            placeholder: Some("Type and press Enter...".to_string()),
                        }
                    }

                    Separator {}

                    // Toggle Demo
                    VStack {
                        gap: SpacingSize::Sm,
                        Label { size: TextSize::Small, "Toggle" }
                        Toggle {
                            pressed: toggle_on(),
                            on_pressed_change: move |v| toggle_on.set(v),
                            variant: ToggleVariant::Default,
                            if toggle_on() { "ON" } else { "OFF" }
                        }
                    }

                    Separator {}

                    // NumberInput Demo
                    VStack {
                        gap: SpacingSize::Sm,
                        Label { size: TextSize::Small, "NumberInput" }
                        NumberInput {
                            value: number(),
                            min: 0.0,
                            max: 100.0,
                            step: 1.0,
                            on_change: move |v| number.set(v),
                        }
                    }

                    Separator {}

                    // AspectRatio Demo
                    VStack {
                        gap: SpacingSize::Sm,
                        Label { size: TextSize::Small, "AspectRatio (16:9)" }
                        AspectRatio {
                            ratio: AspectRatios::WIDESCREEN, // 16:9
                            Box {
                                style: "background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-weight: 600;",
                                        "16:9 Aspect Ratio"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// New Molecule Components Showcase
#[component]
fn NewMoleculesShowcase() -> Element {
    let mut otp_value = use_signal(|| "".to_string());
    let mut time = use_signal(|| Some("14:30".to_string()));
    let mut combo_value = use_signal(|| "".to_string());
    let mut page = use_signal(|| 1);
    let page_size = use_signal(|| 10);

    let combo_options = vec![
        ComboboxOption::new("next", "Next.js"),
        ComboboxOption::new("react", "React"),
        ComboboxOption::new("vue", "Vue.js"),
        ComboboxOption::new("angular", "Angular"),
        ComboboxOption::new("svelte", "Svelte"),
    ];

    rsx! {
        Card {
            overflow_hidden: false,
            CardHeader {
                title: "New Molecule Components",
                subtitle: Some("OTP Input, TimePicker, Combobox, Pagination, QRCode, ListItem".to_string()),
            }
            CardContent {
                VStack {
                    gap: SpacingSize::Lg,

                    // OTP Input Demo
                    VStack {
                        gap: SpacingSize::Sm,
                        Label { size: TextSize::Small, "OTP Input" }
                        OtpInput {
                            value: otp_value(),
                            length: 6,
                            on_change: move |v| otp_value.set(v),
                            on_complete: move |v| println!("OTP Complete: {}", v),
                        }
                        Label { size: TextSize::ExtraSmall, color: TextColor::Muted, "Value: {otp_value}" }
                    }

                    Separator {}

                    // TimePicker Demo
                    VStack {
                        gap: SpacingSize::Sm,
                        Label { size: TextSize::Small, "TimePicker" }
                        TimePicker {
                            value: time(),
                            on_change: move |v| time.set(v),
                            use_24h: true,
                        }
                    }

                    Separator {}

                    // Combobox Demo
                    VStack {
                        gap: SpacingSize::Sm,
                        Label { size: TextSize::Small, "Combobox" }
                        Combobox {
                            options: combo_options,
                            value: combo_value(),
                            on_change: move |v| combo_value.set(v),
                            placeholder: "Search frameworks...",
                        }
                    }

                    Separator {}

                    // Pagination Demo
                    VStack {
                        gap: SpacingSize::Sm,
                        Label { size: TextSize::Small, "Pagination" }
                        Pagination {
                            current_page: page(),
                            total_pages: 10,
                            on_change: move |p| page.set(p),
                        }
                        PaginationInfo {
                            current_page: page(),
                            page_size: page_size(),
                            total_items: 95,
                        }
                    }

                    Separator {}

                    // List Demo
                    VStack {
                        gap: SpacingSize::Sm,
                        Label { size: TextSize::Small, "Action List" }
                        ListGroup {
                            title: Some("Menu".to_string()),
                            ListItem {
                                variant: ListItemVariant::Default,
                                on_click: Some(EventHandler::new(move |_| println!("Profile clicked"))),
                                leading: Some(rsx! { Icon { name: "user".to_string(), size: IconSize::Small, color: IconColor::Muted } }),
                                title: "Profile".to_string(),
                            }
                            ListItem {
                                variant: ListItemVariant::Default,
                                on_click: Some(EventHandler::new(move |_| println!("Settings clicked"))),
                                leading: Some(rsx! { Icon { name: "settings".to_string(), size: IconSize::Small, color: IconColor::Muted } }),
                                title: "Settings".to_string(),
                            }
                            ListItem {
                                variant: ListItemVariant::Default,
                                on_click: Some(EventHandler::new(move |_| println!("Logout clicked"))),
                                leading: Some(rsx! { Icon { name: "log-out".to_string(), size: IconSize::Small, color: IconColor::Destructive } }),
                                title: "Logout".to_string(),
                            }
                        }
                    }

                    Separator {}

                    // QRCode Demo
                    VStack {
                        gap: SpacingSize::Sm,
                        Label { size: TextSize::Small, "QRCode" }
                        HStack {
                            gap: SpacingSize::Md,
                            QrCode {
                                value: "https://dioxuslabs.com".to_string(),
                                size: 128,
                                level: QrCodeLevel::Medium,
                            }
                            VStack {
                                gap: SpacingSize::Xs,
                                Label { size: TextSize::Small, "Scan to visit" }
                                Label { size: TextSize::ExtraSmall, color: TextColor::Muted, "dioxuslabs.com" }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// New Organism Components Showcase
#[component]
fn NewOrganismsShowcase() -> Element {
    let _carousel_index = use_signal(|| 0);
    let mut tour_open = use_signal(|| false);
    let mut calendar_date = use_signal(|| Some("2024-01-15".to_string()));

    let carousel_items = vec![
        rsx! {
            div { style: "height: 200px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); border-radius: 12px; display: flex; align-items: center; justify-content: center; color: white; font-size: 24px; font-weight: 600;",
                "Slide 1"
            }
        },
        rsx! {
            div { style: "height: 200px; background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%); border-radius: 12px; display: flex; align-items: center; justify-content: center; color: white; font-size: 24px; font-weight: 600;",
                "Slide 2"
            }
        },
        rsx! {
            div { style: "height: 200px; background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%); border-radius: 12px; display: flex; align-items: center; justify-content: center; color: white; font-size: 24px; font-weight: 600;",
                "Slide 3"
            }
        },
    ];

    let tree_data = vec![
        TreeNodeData::new("root1", "Documents").with_children(vec![
            TreeNodeData::new("doc1", "Resume.pdf"),
            TreeNodeData::new("doc2", "Cover Letter.pdf"),
            TreeNodeData::new("folder1", "Projects").with_children(vec![
                TreeNodeData::new("proj1", "Project Alpha"),
                TreeNodeData::new("proj2", "Project Beta"),
            ]),
        ]),
        TreeNodeData::new("root2", "Images").with_children(vec![
            TreeNodeData::new("img1", "photo1.jpg"),
            TreeNodeData::new("img2", "photo2.jpg"),
        ]),
    ];

    let timeline_events = vec![
        TimelineEvent::new("Project Started").with_description("Initial planning and setup"),
        TimelineEvent::new("Development").with_description("Building core features"),
        TimelineEvent::new("Testing").with_description("QA and bug fixes"),
        TimelineEvent::new("Launch"),
    ];

    let kanban_columns = vec![
        KanbanColumn::new("todo", "To Do").with_cards(vec![
            KanbanCard::new("task1", "Design Homepage").with_tags(vec!["Design".to_string()]),
            KanbanCard::new("task2", "Setup Database").with_tags(vec!["Backend".to_string()]),
        ]),
        KanbanColumn::new("inprogress", "In Progress").with_cards(vec![KanbanCard::new(
            "task3",
            "API Development",
        )
        .with_tags(vec!["Backend".to_string(), "High".to_string()])]),
        KanbanColumn::new("done", "Done")
            .with_cards(vec![KanbanCard::new("task4", "Project Setup")]),
    ];

    let tour_steps = vec![
        TourStep::new("step1", "Welcome", "Welcome to the Dioxus UI System tour!"),
        TourStep::new("step2", "Components", "Explore 85+ components"),
        TourStep::new("step3", "Themes", "Switch between 7 themes"),
    ];

    rsx! {
        Card {
            overflow_hidden: false,
            CardHeader {
                title: "New Organism Components",
                subtitle: Some("Resizable, Carousel, Tree, Timeline, Menubar, RichText, Kanban, Calendar".to_string()),
            }
            CardContent {
                VStack {
                    gap: SpacingSize::Lg,

                    // Carousel Demo
                    VStack {
                        gap: SpacingSize::Sm,
                        Label { size: TextSize::Small, "Carousel" }
                        SimpleCarousel {
                            items: carousel_items,
                            opts: CarouselOptions::new().with_autoplay_ms(3000),
                            show_dots: true,
                        }
                    }

                    Separator {}

                    // Calendar Demo
                    VStack {
                        gap: SpacingSize::Sm,
                        Label { size: TextSize::Small, "Calendar" }
                        Calendar {
                            value: calendar_date(),
                            on_change: move |date| calendar_date.set(date),
                            mode: CalendarMode::Single,
                        }
                    }

                    Separator {}

                    // Tree Demo
                    VStack {
                        gap: SpacingSize::Sm,
                        Label { size: TextSize::Small, "Tree" }
                        div {
                            style: "max-height: 200px; overflow: auto;",
                            Tree {
                                data: tree_data,
                                on_select: move |id| println!("Selected tree node: {}", id),
                            }
                        }
                    }

                    Separator {}

                    // Timeline Demo
                    VStack {
                        gap: SpacingSize::Sm,
                        Label { size: TextSize::Small, "Timeline" }
                        SimpleTimeline {
                            events: timeline_events,
                        }
                    }

                    Separator {}

                    // Kanban Demo
                    VStack {
                        gap: SpacingSize::Sm,
                        Label { size: TextSize::Small, "Kanban Board" }
                        div {
                            style: "overflow-x: auto;",
                            SimpleKanban {
                                columns: kanban_columns,
                                on_card_click: Some(EventHandler::new(move |card_id: String| println!("Clicked card: {}", card_id))),
                            }
                        }
                    }

                    Separator {}

                    // RichText Demo
                    VStack {
                        gap: SpacingSize::Sm,
                        Label { size: TextSize::Small, "Rich Text Editor" }
                        MinimalRichText {
                            placeholder: "Type something...",
                            on_change: move |html| println!("HTML: {}", html),
                        }
                    }

                    Separator {}

                    // Resizable Demo
                    VStack {
                        gap: SpacingSize::Sm,
                        Label { size: TextSize::Small, "Resizable Panels" }
                        ResizablePanelGroup {
                            direction: Direction::Horizontal,
                            ResizablePanel {
                                default_size: 30.0,
                                min_size: 20.0,
                                Box {
                                    style: "padding: 16px; background: #f3f4f6; border-radius: 8px; height: 100px;",
                                    "Sidebar (resize me)"
                                }
                            }
                            ResizableHandle {}
                            ResizablePanel {
                                default_size: 70.0,
                                min_size: 30.0,
                                Box {
                                    style: "padding: 16px; background: #e5e7eb; border-radius: 8px; height: 100px;",
                                    "Main Content"
                                }
                            }
                        }
                    }

                    Separator {}

                    // Tour Demo
                    VStack {
                        gap: SpacingSize::Sm,
                        Label { size: TextSize::Small, "Tour / Onboarding" }
                        Button {
                            onclick: move |_| tour_open.set(true),
                            "Start Tour"
                        }
                    }
                }
            }
        }

        // Tour component
        Tour {
            steps: tour_steps,
            open: tour_open(),
            on_open_change: move |_| tour_open.set(false),
        }
    }
}

/// New Phase 1-4 Components Showcase
#[component]
fn NewComponentsShowcase() -> Element {
    let mut sheet_open = use_signal(|| false);
    let mut command_value = use_signal(|| "".to_string());
    let mut selected_frameworks = use_signal(|| vec!["react".to_string()]);
    let mut sonner = use_sonner();
    let toasts_sig = sonner.toasts_signal();
    let toasts_list = toasts_sig();

    let frameworks = vec![
        SelectOption::new("react", "React"),
        SelectOption::new("vue", "Vue"),
        SelectOption::new("angular", "Angular"),
        SelectOption::new("svelte", "Svelte"),
        SelectOption::new("solid", "Solid"),
    ];

    rsx! {
        Card {
            overflow_hidden: false,
            CardHeader {
                title: "New Components (Phase 1-4)",
                subtitle: Some("Command, Sheet, MultiSelect, ContextMenu, HoverCard, Sonner, Collapsible, ToggleGroup".to_string()),
            }
            CardContent {
                VStack {
                    gap: SpacingSize::Lg,

                    // Sheet Demo
                    VStack {
                        gap: SpacingSize::Sm,
                        Label { size: TextSize::Small, "Sheet (Side Panel)" }
                        Button {
                            variant: ButtonVariant::Primary,
                            onclick: move |_| sheet_open.set(true),
                            "Open Sheet"
                        }
                        Sheet {
                            open: sheet_open(),
                            on_open_change: move |o| sheet_open.set(o),
                            title: "Example Sheet",
                            description: Some("This is a sheet component that slides in from the right.".to_string()),
                            VStack { gap: SpacingSize::Md,
                                p { "Sheets are great for displaying secondary content without leaving the current page." }
                                Button { onclick: move |_| sheet_open.set(false), "Close Sheet" }
                            }
                        }
                    }

                    Separator {}

                    // Sonner Demo
                    VStack {
                        gap: SpacingSize::Sm,
                        Label { size: TextSize::Small, "Sonner (Toast Notifications)" }
                        HStack {
                            gap: SpacingSize::Sm,
                            Button { onclick: move |_| { sonner.toast("Hello World!"); }, "Show Toast" }
                            Button { variant: ButtonVariant::Secondary, onclick: move |_| { sonner.success("Success!"); }, "Success" }
                            Button { variant: ButtonVariant::Destructive, onclick: move |_| { sonner.error("Error!"); }, "Error" }
                        }
                        // Sonner renders at page level in real usage
                    }

                    Separator {}

                    // MultiSelect Demo
                    VStack {
                        gap: SpacingSize::Sm,
                        Label { size: TextSize::Small, "MultiSelect" }
                        MultiSelect {
                            options: frameworks.clone(),
                            value: selected_frameworks(),
                            on_change: move |v| selected_frameworks.set(v),
                            placeholder: "Select frameworks...",
                        }
                    }

                    Separator {}

                    // ToggleGroup Demo
                    VStack {
                        gap: SpacingSize::Sm,
                        Label { size: TextSize::Small, "ToggleGroup" }
                        ToggleGroup {
                            group_type: ToggleGroupType::Multiple,
                            value: vec!["bold".to_string()],
                            on_value_change: move |v| println!("Selected: {:?}", v),
                            ToggleItem { value: "bold", "B" }
                            ToggleItem { value: "italic", "I" }
                            ToggleItem { value: "underline", "U" }
                        }
                    }

                    Separator {}

                    // Collapsible Demo
                    VStack {
                        gap: SpacingSize::Sm,
                        Label { size: TextSize::Small, "Collapsible" }
                        Collapsible {
                            trigger: rsx! { "Click to expand" },
                            "This content is hidden by default and revealed when you click the trigger."
                        }
                    }

                    Separator {}

                    // HoverCard Demo
                    VStack {
                        gap: SpacingSize::Sm,
                        Label { size: TextSize::Small, "HoverCard" }
                        HoverCard {
                            trigger: rsx! { span { style: "color: #3b82f6; cursor: pointer; text-decoration: underline;", "@username" } },
                            HoverCardHeader {
                                title: "John Doe".to_string(),
                                description: Some("Software Engineer @ Company".to_string()),
                            }
                            HoverCardContent {
                                "Passionate about building great user experiences with Rust and Dioxus."
                            }
                        }
                    }

                    Separator {}

                    // ContextMenu Demo
                    VStack {
                        gap: SpacingSize::Sm,
                        Label { size: TextSize::Small, "ContextMenu (Right-click)" }
                        ContextMenu {
                            ContextMenuTrigger {
                                Box {
                                    style: "padding: 20px; background: #f3f4f6; border-radius: 8px; text-align: center; cursor: context-menu;",
                                    "Right-click here"
                                }
                            }
                            ContextMenuContent {
                                ContextMenuItem { on_click: move |_| println!("Cut"), "Cut" }
                                ContextMenuItem { on_click: move |_| println!("Copy"), "Copy" }
                                ContextMenuItem { on_click: move |_| println!("Paste"), "Paste" }
                            }
                        }
                    }

                    Separator {}

                    // Command Demo
                    VStack {
                        gap: SpacingSize::Sm,
                        Label { size: TextSize::Small, "Command Palette" }
                        Box { style: "max-width: 400px; border: 1px solid #e5e7eb; border-radius: 8px;",
                            Command {
                                on_select: move |_| println!("Command selected"),
                                CommandInput {
                                    placeholder: "Search commands...",
                                    value: command_value(),
                                    on_value_change: move |v| command_value.set(v),
                                }
                                CommandList {
                                    CommandEmpty { "No results found." }
                                    CommandGroup {
                                        heading: "Suggestions",
                                        CommandItem { value: "calendar", on_select: move |_| {}, "Calendar" }
                                        CommandItem { value: "search", on_select: move |_| {}, "Search" }
                                        CommandItem { value: "settings", on_select: move |_| {}, "Settings" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Sonner rendered at page level
        Sonner {
            toasts: toasts_list.clone(),
            position: ToastPosition::BottomRight,
            on_dismiss: move |id: String| sonner.dismiss(&id),
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
                    "100+ Cross-platform Rust UI components"
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
                ThemeSelector {}
            }
        }
    }
}
