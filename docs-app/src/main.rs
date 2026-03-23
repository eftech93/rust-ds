//! Dioxus UI Documentation Site
//!
//! A comprehensive documentation site with multi-page navigation,
//! component examples, and theme building guides.

#![allow(unused_braces)]

use dioxus::prelude::*;
use dioxus_ui_system::prelude::*;

mod components;
mod guides;
mod themes;

fn main() {
    dioxus::logger::init(tracing::Level::INFO).unwrap();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap"
        }
        
        ThemeProvider {
            Router::<Route> {}
        }
    }
}

/// Main route enum for documentation
#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    #[layout(DocsLayout)]
    Home {},
    
    // Atoms
    #[route("/atoms")]
    AtomsPage {},
    #[route("/atoms/box")]
    BoxPage {},
    #[route("/atoms/button")]
    ButtonPage {},
    #[route("/atoms/input")]
    InputPage {},
    #[route("/atoms/label")]
    LabelPage {},
    #[route("/atoms/icon")]
    IconPage {},
    #[route("/atoms/checkbox")]
    CheckboxPage {},
    #[route("/atoms/radio")]
    RadioPage {},
    #[route("/atoms/switch")]
    SwitchPage {},
    #[route("/atoms/select")]
    SelectPage {},
    #[route("/atoms/textarea")]
    TextAreaPage {},
    #[route("/atoms/step")]
    StepPage {},
    #[route("/atoms/heading")]
    HeadingPage {},
    #[route("/atoms/divider")]
    DividerPage {},
    #[route("/atoms/progress")]
    ProgressPage {},
    #[route("/atoms/spinner")]
    SpinnerPage {},
    #[route("/atoms/skeleton")]
    SkeletonAtomPage {},
    #[route("/atoms/rating")]
    RatingPage {},
    #[route("/atoms/datepicker")]
    DatePickerPage {},
    #[route("/atoms/slider")]
    SliderPage {},
    #[route("/atoms/tag")]
    TagPage {},
    
    // Molecules
    #[route("/molecules")]
    MoleculesPage {},
    #[route("/molecules/card")]
    CardPage {},
    #[route("/molecules/badge")]
    BadgePage {},
    #[route("/molecules/alert")]
    AlertPage {},
    #[route("/molecules/avatar")]
    AvatarPage {},
    #[route("/molecules/dialog")]
    DialogPage {},
    #[route("/molecules/dropdown")]
    DropdownPage {},
    #[route("/molecules/popover")]
    PopoverPage {},
    #[route("/molecules/tooltip")]
    TooltipPage {},
    #[route("/molecules/separator")]
    SeparatorPage {},
    #[route("/molecules/skeleton")]
    SkeletonPage {},
    #[route("/molecules/stepper")]
    StepperPage {},
    #[route("/molecules/toast")]
    ToastPage {},
    #[route("/molecules/combobox")]
    ComboboxPage {},
    #[route("/molecules/media-object")]
    MediaObjectPage {},
    #[route("/molecules/pagination")]
    PaginationPage {},
    #[route("/molecules/list-item")]
    ListItemPage {},
    
    // Organisms
    #[route("/organisms")]
    OrganismsPage {},
    #[route("/organisms/header")]
    HeaderPage {},
    #[route("/organisms/layout")]
    LayoutPage {},
    #[route("/organisms/tabs")]
    TabsPage {},
    #[route("/organisms/accordion")]
    AccordionPage {},
    #[route("/organisms/cards")]
    CardsPage {},
    #[route("/organisms/data-table")]
    DataTablePage {},
    #[route("/organisms/stepper-wizard")]
    StepperWizardPage {},
    #[route("/organisms/charts")]
    ChartsPage {},
    #[route("/organisms/footer")]
    FooterPage {},
    #[route("/organisms/notification-center")]
    NotificationCenterPage {},
    #[route("/organisms/hero")]
    HeroPage {},
    #[route("/organisms/file-upload")]
    FileUploadPage {},
    #[route("/organisms/confirmation-dialog")]
    ConfirmationDialogPage {},
    
    // Themes
    #[route("/themes")]
    ThemesPage {},
    #[route("/themes/overview")]
    ThemeOverviewPage {},
    #[route("/themes/tokens")]
    ThemeTokensPage {},
    #[route("/themes/custom")]
    CustomThemePage {},
    #[route("/themes/presets")]
    PresetThemesPage {},
    
    // Guides
    #[route("/guides")]
    GuidesPage {},
    #[route("/guides/quickstart")]
    QuickStartPage {},
    #[route("/guides/styling")]
    StylingPage {},
    #[route("/guides/forms")]
    FormsPage {},
    #[route("/guides/layouts")]
    LayoutsPage {},
}

/// Docs layout with custom sidebar using Dioxus Router
#[component]
fn DocsLayout() -> Element {
    rsx! {
        div {
            style: "display: flex; min-height: 100vh;",
            
            Sidebar {}
            
            div {
                style: "flex: 1; margin-left: 260px; padding: 32px; max-width: 900px;",
                Outlet::<Route> {}
            }
        }
    }
}

/// Sidebar navigation component with collapsible sections
#[component]
fn Sidebar() -> Element {
    let current_route = use_route::<Route>();
    let theme = use_theme();
    
    // Reactive CSS variables that update when theme changes
    let sidebar_style = use_memo(move || {
        let t = theme.tokens.read();
        format!(
            "width: 260px; min-width: 260px; height: 100vh; position: fixed; left: 0; top: 0; overflow-y: auto; padding: 24px 16px; box-sizing: border-box; background: {}; border-right: 1px solid {}; --sb-text: {}; --sb-primary: {}; --sb-muted: {};",
            t.colors.background.to_rgba(),
            t.colors.border.to_rgba(),
            t.colors.foreground.to_rgba(),
            t.colors.primary.to_rgba(),
            t.colors.muted.to_rgba(),
        )
    });
    
    rsx! {
        // Static CSS rules - variables come from parent inline style
        style {{ r#"
            .nav-link { display: block; padding: 6px 12px; margin: 2px 0; border-radius: 6px; font-size: 14px; text-decoration: none; color: var(--sb-text); transition: background 0.15s ease; }
            .nav-link:hover { background: color-mix(in srgb, var(--sb-primary) 15%, transparent); }
            .nav-link.active { background: var(--sb-primary); color: #ffffff; font-weight: 500; }
            .nav-link.active:hover { opacity: 0.9; }
        "# }}
        
        aside {
            style: "{sidebar_style}",
            style: "width: 260px; min-width: 260px; height: 100vh; position: fixed; left: 0; top: 0; overflow-y: auto; padding: 24px 16px; box-sizing: border-box;",
            
            Link {
                to: Route::Home {},
                style: "display: block; font-size: 20px; font-weight: 700; color: var(--sb-text); text-decoration: none; margin-bottom: 32px; padding: 0 8px;",
                "Dioxus UI"
            }
            
            // Getting Started
            NavSection { title: "Getting Started", current_route: current_route.clone(), items: vec![
                ("Home", Route::Home {}),
                ("Quick Start", Route::QuickStartPage {}),
            ]}
            
            // Atoms
            NavSection { title: "Atoms", current_route: current_route.clone(), items: vec![
                ("Overview", Route::AtomsPage {}),
                ("Box", Route::BoxPage {}),
                ("Button", Route::ButtonPage {}),
                ("Checkbox", Route::CheckboxPage {}),
                ("DatePicker", Route::DatePickerPage {}),
                ("Divider", Route::DividerPage {}),
                ("Heading", Route::HeadingPage {}),
                ("Icon", Route::IconPage {}),
                ("Input", Route::InputPage {}),
                ("Label", Route::LabelPage {}),
                ("Progress", Route::ProgressPage {}),
                ("Radio", Route::RadioPage {}),
                ("Rating", Route::RatingPage {}),
                ("Select", Route::SelectPage {}),
                ("Skeleton", Route::SkeletonAtomPage {}),
                ("Slider", Route::SliderPage {}),
                ("Spinner", Route::SpinnerPage {}),
                ("Step", Route::StepPage {}),
                ("Switch", Route::SwitchPage {}),
                ("Tag", Route::TagPage {}),
                ("TextArea", Route::TextAreaPage {}),
            ]}
            
            // Molecules
            NavSection { title: "Molecules", current_route: current_route.clone(), items: vec![
                ("Overview", Route::MoleculesPage {}),
                ("Alert", Route::AlertPage {}),
                ("Avatar", Route::AvatarPage {}),
                ("Badge", Route::BadgePage {}),
                ("Card", Route::CardPage {}),
                ("Combobox", Route::ComboboxPage {}),
                ("Dialog", Route::DialogPage {}),
                ("Dropdown", Route::DropdownPage {}),
                ("List Item", Route::ListItemPage {}),
                ("Media Object", Route::MediaObjectPage {}),
                ("Pagination", Route::PaginationPage {}),
                ("Separator", Route::SeparatorPage {}),
                ("Skeleton", Route::SkeletonPage {}),
                ("Stepper", Route::StepperPage {}),
                ("Toast", Route::ToastPage {}),
                ("Tooltip", Route::TooltipPage {}),
            ]}
            
            // Organisms
            NavSection { title: "Organisms", current_route: current_route.clone(), items: vec![
                ("Overview", Route::OrganismsPage {}),
                ("Accordion", Route::AccordionPage {}),
                ("Cards", Route::CardsPage {}),
                ("Charts", Route::ChartsPage {}),
                ("Confirmation Dialog", Route::ConfirmationDialogPage {}),
                ("DataTable", Route::DataTablePage {}),
                ("File Upload", Route::FileUploadPage {}),
                ("Footer", Route::FooterPage {}),
                ("Header", Route::HeaderPage {}),
                ("Hero", Route::HeroPage {}),
                ("Layout", Route::LayoutPage {}),
                ("Notification Center", Route::NotificationCenterPage {}),
                ("Stepper Wizard", Route::StepperWizardPage {}),
                ("Tabs", Route::TabsPage {}),
            ]}
            
            // Themes & Guides
            NavSection { title: "Themes", current_route: current_route.clone(), items: vec![
                ("Overview", Route::ThemesPage {}),
                ("Design Tokens", Route::ThemeTokensPage {}),
                ("Custom Themes", Route::CustomThemePage {}),
                ("Preset Themes", Route::PresetThemesPage {}),
            ]}
            
            NavSection { title: "Guides", current_route: current_route.clone(), items: vec![
                ("Overview", Route::GuidesPage {}),
                ("Styling", Route::StylingPage {}),
                ("Forms", Route::FormsPage {}),
                ("Layouts", Route::LayoutsPage {}),
            ]}
        }
    }
}

/// Navigation section with collapsible items
#[component]
fn NavSection(title: String, current_route: Route, items: Vec<(&'static str, Route)>) -> Element {
    let mut expanded = use_signal(|| true);
    let is_active_section = items.iter().any(|(_, r)| *r == current_route);
    
    rsx! {
        div {
            style: "margin-bottom: 16px;",
            
            div {
                style: "display: flex; align-items: center; justify-content: space-between; padding: 8px 12px; cursor: pointer; border-radius: 6px; transition: background 0.15s;",
                onclick: move |_| expanded.set(!expanded()),
                
                h3 {
                    style: if is_active_section { 
                        "margin: 0; font-size: 12px; font-weight: 600; color: var(--sb-text); text-transform: uppercase; letter-spacing: 0.05em;" 
                    } else { 
                        "margin: 0; font-size: 12px; font-weight: 600; color: var(--sb-muted); text-transform: uppercase; letter-spacing: 0.05em;" 
                    },
                    "{title}"
                }
                
                span {
                    style: if expanded() { 
                        "font-size: 10px; color: var(--sb-muted); transition: transform 0.2s;" 
                    } else { 
                        "font-size: 10px; color: var(--sb-muted); transition: transform 0.2s; transform: rotate(-90deg);" 
                    },
                    "▼"
                }
            }
            
            if expanded() {
                div {
                    style: "margin-top: 4px;",
                    
                    for (label, route) in items {
                        NavLink {
                            label: label.to_string(),
                            route: route.clone(),
                        }
                    }
                }
            }
        }
    }
}

/// Individual navigation link using Link's active_class prop
#[component]
fn NavLink(label: String, route: Route) -> Element {
    rsx! {
        Link {
            key: "{label}",
            to: route,
            class: "nav-link",
            active_class: "active",
            "{label}"
        }
    }
}


// Page Components

#[component]
fn Home() -> Element {
    rsx! {
        VStack {
            style: "gap: 32px;",
            
            Box {
                h1 { style: "margin: 0 0 16px 0; font-size: 40px; font-weight: 800;", "Dioxus UI" }
                p { style: "margin: 0; font-size: 18px; color: rgb(100,116,139); line-height: 1.6;", 
                    "A pure Rust design system for building beautiful, type-safe user interfaces with Dioxus." }
            }
            
            Box {
                style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px;",
                
                DocCard { icon: "⚛️", title: "60+ Components", description: "Atoms, molecules, and organisms following Atomic Design principles." }
                DocCard { icon: "🎨", title: "7 Theme Presets", description: "Light, dark, and brand themes with full customization." }
                DocCard { icon: "📱", title: "Cross-Platform", description: "Works on Web, Desktop, and Mobile." }
                DocCard { icon: "🔒", title: "Type-Safe", description: "No CSS files - all styles are compile-time checked Rust code." }
            }
            
            Box {
                style: "padding: 24px; background: rgb(241,245,249); border-radius: 12px;",
                
                h2 { style: "font-size: 20px; font-weight: 700; margin-bottom: 16px;", "Quick Example" }
                
                pre {
                    style: "background: rgb(15,23,42); color: rgb(226,232,240); padding: 16px; border-radius: 8px; font-size: 14px; overflow-x: auto;",
                    code {
                        "use dioxus::prelude::*;\nuse dioxus_ui_system::prelude::*;\n\n#[component]\nfn App() -> Element {{\n    rsx! {{\n        ThemeProvider {{\n            Card {{\n                CardHeader {{\n                    title: \"Hello\",\n                    subtitle: Some(\"World\".to_string()),\n                }}\n                CardContent {{\n                    Button {{ variant: ButtonVariant::Primary, \"Click me\" }}\n                }}\n            }}\n        }}\n    }}\n}}"
                    }
                }
            }
        }
    }
}

#[component]
fn DocCard(icon: String, title: String, description: String) -> Element {
    rsx! {
        Box {
            style: "padding: 20px; background: white; border: 1px solid rgb(226,232,240); border-radius: 12px;",
            
            Box { style: "font-size: 28px; margin-bottom: 12px;", "{icon}" }
            h3 { style: "margin: 0 0 8px 0; font-size: 16px; font-weight: 600;", "{title}" }
            p { style: "margin: 0; font-size: 14px; color: rgb(100,116,139); line-height: 1.5;", "{description}" }
        }
    }
}


// Route Page Components - These map routes to the actual component modules

// Atoms
#[component]
fn AtomsPage() -> Element { components::atoms::AtomsPage() }
#[component]
fn BoxPage() -> Element { components::atoms::BoxPage() }
#[component]
fn ButtonPage() -> Element { components::atoms::ButtonPage() }
#[component]
fn InputPage() -> Element { components::atoms::InputPage() }
#[component]
fn LabelPage() -> Element { components::atoms::LabelPage() }
#[component]
fn IconPage() -> Element { components::atoms::IconPage() }
#[component]
fn CheckboxPage() -> Element { components::atoms::CheckboxPage() }
#[component]
fn RadioPage() -> Element { components::atoms::RadioPage() }
#[component]
fn SwitchPage() -> Element { components::atoms::SwitchPage() }
#[component]
fn SelectPage() -> Element { components::atoms::SelectPage() }
#[component]
fn TextAreaPage() -> Element { components::atoms::TextAreaPage() }
#[component]
fn StepPage() -> Element { components::atoms::StepPage() }
#[component]
fn HeadingPage() -> Element { components::atoms::HeadingPage() }
#[component]
fn DividerPage() -> Element { components::atoms::DividerPage() }
#[component]
fn ProgressPage() -> Element { components::atoms::ProgressPage() }
#[component]
fn SpinnerPage() -> Element { components::atoms::SpinnerPage() }
#[component]
fn SkeletonAtomPage() -> Element { components::atoms::SkeletonAtomPage() }
#[component]
fn RatingPage() -> Element { components::atoms::RatingPage() }
#[component]
fn DatePickerPage() -> Element { components::atoms::DatePickerPage() }
#[component]
fn SliderPage() -> Element { components::atoms::SliderPage() }
#[component]
fn TagPage() -> Element { components::atoms::TagPage() }

// Molecules
#[component]
fn MoleculesPage() -> Element { components::molecules::MoleculesPage() }
#[component]
fn CardPage() -> Element { components::molecules::CardPage() }
#[component]
fn BadgePage() -> Element { components::molecules::BadgePage() }
#[component]
fn AlertPage() -> Element { components::molecules::AlertPage() }
#[component]
fn AvatarPage() -> Element { components::molecules::AvatarPage() }
#[component]
fn DialogPage() -> Element { components::molecules::DialogPage() }
#[component]
fn DropdownPage() -> Element { components::molecules::DropdownPage() }
#[component]
fn PopoverPage() -> Element { components::molecules::PopoverPage() }
#[component]
fn TooltipPage() -> Element { components::molecules::TooltipPage() }
#[component]
fn SeparatorPage() -> Element { components::molecules::SeparatorPage() }
#[component]
fn SkeletonPage() -> Element { components::molecules::SkeletonMoleculePage() }
#[component]
fn StepperPage() -> Element { components::molecules::StepperPage() }
#[component]
fn ToastPage() -> Element { components::molecules::ToastPage() }
#[component]
fn ComboboxPage() -> Element { components::molecules::ComboboxPage() }
#[component]
fn MediaObjectPage() -> Element { components::molecules::MediaObjectPage() }
#[component]
fn PaginationPage() -> Element { components::molecules::PaginationPage() }
#[component]
fn ListItemPage() -> Element { components::molecules::ListItemPage() }

// Organisms
#[component]
fn OrganismsPage() -> Element { components::organisms::OrganismsPage() }
#[component]
fn HeaderPage() -> Element { components::organisms::HeaderPage() }
#[component]
fn LayoutPage() -> Element { components::organisms::LayoutPage() }
#[component]
fn TabsPage() -> Element { components::organisms::TabsPage() }
#[component]
fn AccordionPage() -> Element { components::organisms::AccordionPage() }
#[component]
fn CardsPage() -> Element { components::organisms::CardsPage() }
#[component]
fn DataTablePage() -> Element { components::organisms::DataTablePage() }
#[component]
fn StepperWizardPage() -> Element { components::organisms::StepperWizardPage() }
#[component]
fn ChartsPage() -> Element { components::organisms::ChartsPage() }
#[component]
fn FooterPage() -> Element { components::organisms::FooterPage() }
#[component]
fn NotificationCenterPage() -> Element { components::organisms::NotificationCenterPage() }
#[component]
fn HeroPage() -> Element { components::organisms::HeroPage() }
#[component]
fn FileUploadPage() -> Element { components::organisms::FileUploadPage() }
#[component]
fn ConfirmationDialogPage() -> Element { components::organisms::ConfirmationDialogPage() }

// Themes
#[component]
fn ThemesPage() -> Element { themes::ThemesPage() }
#[component]
fn ThemeOverviewPage() -> Element { themes::ThemeOverviewPage() }
#[component]
fn ThemeTokensPage() -> Element { themes::ThemeTokensPage() }
#[component]
fn CustomThemePage() -> Element { themes::CustomThemePage() }
#[component]
fn PresetThemesPage() -> Element { themes::PresetThemesPage() }

// Guides
#[component]
fn GuidesPage() -> Element { guides::GuidesPage() }
#[component]
fn QuickStartPage() -> Element { guides::QuickStartPage() }
#[component]
fn StylingPage() -> Element { guides::StylingPage() }
#[component]
fn FormsPage() -> Element { guides::FormsPage() }
#[component]
fn LayoutsPage() -> Element { guides::LayoutsPage() }
