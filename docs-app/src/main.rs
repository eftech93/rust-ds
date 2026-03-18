//! Dioxus UI Documentation Site
//!
//! A comprehensive documentation site with multi-page navigation,
//! component examples, and theme building guides.

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

/// Docs layout with sidebar navigation
#[component]
fn DocsLayout() -> Element {
    rsx! {
        div {
            style: "font-family: 'Inter', system-ui, -apple-system, sans-serif; min-height: 100vh; display: flex; flex-direction: column;",
            
            // Header
            Header {
                brand_title: "Dioxus UI Docs",
                nav_items: vec![],
                actions: rsx! {
                    ThemeToggle {}
                },
            }
            
            // Main content area with sidebar
            div {
                style: "display: flex; flex: 1;",
                
                // Sidebar Navigation
                Sidebar {}
                
                // Content area
                div {
                    style: "flex: 1; padding: 32px; max-width: 900px; margin: 0 auto;",
                    Outlet::<Route> {}
                }
            }
        }
    }
}

/// Sidebar navigation component
#[component]
fn Sidebar() -> Element {
    let current_route = use_route::<Route>();
    
    rsx! {
        aside {
            style: "width: 260px; background: rgb(248,250,252); border-right: 1px solid rgb(226,232,240); padding: 24px 16px; overflow-y: auto; position: sticky; top: 64px; height: calc(100vh - 64px);",
            
            h2 {
                style: "margin: 0 0 24px 0; font-size: 14px; font-weight: 700; color: rgb(148,163,184); text-transform: uppercase; letter-spacing: 0.05em;",
                "Navigation"
            }
            
            nav {
                style: "display: flex; flex-direction: column; gap: 4px;",
                
                // Getting Started
                NavSection { title: "Getting Started", items: vec![
                    ("Home", Route::Home {}),
                    ("Quick Start", Route::QuickStartPage {}),
                ], current_route: current_route.clone() }
                
                // Atoms
                NavSection { title: "Atoms", items: vec![
                    ("Overview", Route::AtomsPage {}),
                    ("Box", Route::BoxPage {}),
                    ("Button", Route::ButtonPage {}),
                    ("Input", Route::InputPage {}),
                    ("Label", Route::LabelPage {}),
                    ("Icon", Route::IconPage {}),
                    ("Checkbox", Route::CheckboxPage {}),
                    ("Radio", Route::RadioPage {}),
                    ("Switch", Route::SwitchPage {}),
                    ("Select", Route::SelectPage {}),
                    ("TextArea", Route::TextAreaPage {}),
                    ("Step", Route::StepPage {}),
                ], current_route: current_route.clone() }
                
                // Molecules
                NavSection { title: "Molecules", items: vec![
                    ("Overview", Route::MoleculesPage {}),
                    ("Card", Route::CardPage {}),
                    ("Badge", Route::BadgePage {}),
                    ("Alert", Route::AlertPage {}),
                    ("Avatar", Route::AvatarPage {}),
                    ("Dialog", Route::DialogPage {}),
                    ("Dropdown", Route::DropdownPage {}),
                    ("Tooltip", Route::TooltipPage {}),
                    ("Separator", Route::SeparatorPage {}),
                    ("Skeleton", Route::SkeletonPage {}),
                    ("Stepper", Route::StepperPage {}),
                ], current_route: current_route.clone() }
                
                // Organisms
                NavSection { title: "Organisms", items: vec![
                    ("Overview", Route::OrganismsPage {}),
                    ("Header", Route::HeaderPage {}),
                    ("Layout", Route::LayoutPage {}),
                    ("Tabs", Route::TabsPage {}),
                    ("Accordion", Route::AccordionPage {}),
                    ("Cards", Route::CardsPage {}),
                    ("DataTable", Route::DataTablePage {}),
                    ("Stepper Wizard", Route::StepperWizardPage {}),
                    ("Charts", Route::ChartsPage {}),
                ], current_route: current_route.clone() }
                
                // Themes
                NavSection { title: "Themes", items: vec![
                    ("Overview", Route::ThemesPage {}),
                    ("Design Tokens", Route::ThemeTokensPage {}),
                    ("Custom Themes", Route::CustomThemePage {}),
                    ("Preset Themes", Route::PresetThemesPage {}),
                ], current_route: current_route.clone() }
                
                // Guides
                NavSection { title: "Guides", items: vec![
                    ("All Guides", Route::GuidesPage {}),
                    ("Styling", Route::StylingPage {}),
                    ("Forms", Route::FormsPage {}),
                    ("Layouts", Route::LayoutsPage {}),
                ], current_route: current_route.clone() }
            }
        }
    }
}

/// Navigation section in sidebar
#[component]
fn NavSection(title: String, items: Vec<(&'static str, Route)>, current_route: Route) -> Element {
    rsx! {
        div {
            style: "margin-bottom: 16px;",
            
            h3 {
                style: "margin: 0 0 8px 0; font-size: 12px; font-weight: 600; color: rgb(100,116,139); padding-left: 12px;",
                "{title}"
            }
            
            div {
                style: "display: flex; flex-direction: column; gap: 2px;",
                
                for (label, route) in items {
                    Link {
                        to: route.clone(),
                        style: if current_route == route { 
                            "padding: 6px 12px; border-radius: 6px; background: rgb(15,23,42); color: white; font-size: 14px; text-decoration: none; display: block;"
                        } else { 
                            "padding: 6px 12px; border-radius: 6px; color: rgb(71,85,105); font-size: 14px; text-decoration: none; display: block; &:hover {{ background: rgb(226,232,240); }}"
                        },
                        "{label}"
                    }
                }
            }
        }
    }
}

// Page Components

#[component]
fn Home() -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 32px;",
            
            div {
                h1 { style: "margin: 0 0 16px 0; font-size: 40px; font-weight: 800;", "Dioxus UI" }
                p { style: "margin: 0; font-size: 18px; color: rgb(100,116,139); line-height: 1.6;", 
                    "A pure Rust design system for building beautiful, type-safe user interfaces with Dioxus." }
            }
            
            div {
                style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px;",
                
                DocCard { icon: "⚛️", title: "60+ Components", description: "Atoms, molecules, and organisms following Atomic Design principles." }
                DocCard { icon: "🎨", title: "7 Theme Presets", description: "Light, dark, and brand themes with full customization." }
                DocCard { icon: "📱", title: "Cross-Platform", description: "Works on Web, Desktop, and Mobile." }
                DocCard { icon: "🔒", title: "Type-Safe", description: "No CSS files - all styles are compile-time checked Rust code." }
            }
            
            div {
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
        div {
            style: "padding: 20px; background: white; border: 1px solid rgb(226,232,240); border-radius: 12px;",
            
            div { style: "font-size: 28px; margin-bottom: 12px;", "{icon}" }
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
