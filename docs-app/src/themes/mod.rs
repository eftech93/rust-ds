//! Theme documentation pages

use dioxus::prelude::*;
use dioxus_ui_system::prelude::*;

#[component]
pub fn ThemesPage() -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 32px;",
            
            div {
                h1 { style: "margin: 0 0 12px 0; font-size: 32px; font-weight: 800;", "Themes" }
                p { style: "margin: 0; font-size: 16px; color: rgb(100,116,139);", 
                    "Comprehensive theming system with preset themes and full customization support." }
            }
            
            div { style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px;",
                DocCard { title: "Overview", description: "Understand the theme system", route: "/themes/overview" }
                DocCard { title: "Design Tokens", description: "Learn about design tokens", route: "/themes/tokens" }
                DocCard { title: "Custom Themes", description: "Create your own themes", route: "/themes/custom" }
                DocCard { title: "Preset Themes", description: "Explore built-in themes", route: "/themes/presets" }
            }
        }
    }
}

#[component]
fn DocCard(title: String, description: String, route: String) -> Element {
    rsx! {
        Link {
            to: route,
            style: "padding: 20px; background: white; border: 1px solid rgb(226,232,240); border-radius: 12px; text-decoration: none; color: inherit; display: block;",
            h3 { style: "margin: 0 0 8px 0; font-size: 16px; font-weight: 600;", "{title}" }
            p { style: "margin: 0; font-size: 14px; color: rgb(100,116,139);", "{description}" }
        }
    }
}

#[component]
pub fn ThemeOverviewPage() -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 32px;",
            
            h1 { style: "margin: 0; font-size: 32px; font-weight: 800;", "Theme System Overview" }
            
            Section { title: "Introduction",
                p { "Dioxus UI uses a type-safe theme system based on design tokens. Design tokens are named values that represent the visual properties of your design system." }
                
                ul {
                    li { "7 preset themes included (Light, Dark, Rose, Blue, Green, Violet, Orange)" }
                    li { "Type-safe design tokens in Rust" }
                    li { "Runtime theme switching" }
                    li { "Custom theme creation" }
                    li { "Automatic CSS variable generation" }
                }
            }
            
            Section { title: "Using Themes",
                p { "Wrap your app with ThemeProvider:" }
                CodeBlock { code: "#[component]
fn App() -> Element {{
    rsx! {{
        ThemeProvider {{
            // Your app here
        }}
    }}
}}".to_string() }
            }
            
            Section { title: "Theme Controls",
                p { "Add theme toggle and selector to your app:" }
                ExampleBox {
                    div { style: "display: flex; gap: 12px; flex-wrap: wrap;",
                        ThemeToggle {}
                        ThemeSelector {}
                    }
                }
            }
        }
    }
}

#[component]
pub fn ThemeTokensPage() -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 32px;",
            
            h1 { style: "margin: 0; font-size: 32px; font-weight: 800;", "Design Tokens" }
            
            Section { title: "Color Tokens",
                p { "Colors define the visual identity of your application:" }
                TokenTable {
                    tokens: vec![
                        ("background", "Page background color"),
                        ("foreground", "Primary text color"),
                        ("primary", "Primary actions and highlights"),
                        ("secondary", "Secondary elements"),
                        ("muted", "Muted text and backgrounds"),
                        ("accent", "Accent highlights"),
                        ("destructive", "Errors and danger states"),
                        ("border", "Borders and dividers"),
                    ]
                }
            }
            
            Section { title: "Spacing Tokens",
                p { "Consistent spacing throughout your application:" }
                TokenTable {
                    tokens: vec![
                        ("xs (4px)", "Extra small spacing"),
                        ("sm (8px)", "Small spacing"),
                        ("md (16px)", "Medium spacing"),
                        ("lg (24px)", "Large spacing"),
                        ("xl (32px)", "Extra large spacing"),
                    ]
                }
            }
            
            Section { title: "Typography Tokens",
                p { "Text styles and font settings:" }
                TokenTable {
                    tokens: vec![
                        ("font_family", "Primary font family"),
                        ("font_size_sm", "Small text (14px)"),
                        ("font_size_base", "Base text (16px)"),
                        ("font_size_lg", "Large text (18px)"),
                        ("font_weight_normal", "Normal weight (400)"),
                        ("font_weight_bold", "Bold weight (700)"),
                    ]
                }
            }
        }
    }
}

#[component]
pub fn CustomThemePage() -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 32px;",
            
            h1 { style: "margin: 0; font-size: 32px; font-weight: 800;", "Creating Custom Themes" }
            
            Section { title: "Brand Theme",
                p { "Create a theme based on a primary color:" }
                CodeBlock { code: r#"use dioxus_ui_system::theme::{ThemeTokens, Color};

fn my_theme() -> ThemeTokens {
    ThemeTokens::brand(
        Color::new(220, 38, 38),  // Primary red color
        "acme"                     // Brand name
    )
}

// Use in app
ThemeProvider { initial_theme: Some(my_theme()) }"#.to_string() }
            }
            
            Section { title: "Fully Custom Theme",
                p { "Build a theme from scratch with full control:" }
                CodeBlock { code: r#"use dioxus_ui_system::theme::{
    ThemeTokens, ColorTokens, SpacingTokens, 
    TypographyTokens, ShadowTokens, Color
};

fn custom_theme() -> ThemeTokens {
    ThemeTokens {
        colors: ColorTokens {
            background: Color::new(250, 250, 250),
            foreground: Color::new(15, 23, 42),
            primary: Color::new(37, 99, 235),
            secondary: Color::new(100, 116, 139),
            muted: Color::new(241, 245, 249),
            accent: Color::new(241, 245, 249),
            destructive: Color::new(239, 68, 68),
            border: Color::new(226, 232, 240),
            input: Color::new(226, 232, 240),
            ring: Color::new(37, 99, 235),
        },
        spacing: SpacingTokens {
            unit: 4,
            scale: vec![0, 4, 8, 12, 16, 24, 32, 48, 64],
        },
        typography: TypographyTokens {
            font_family: "Inter, system-ui, sans-serif".to_string(),
            font_size_sm: 14,
            font_size_base: 16,
            font_size_lg: 18,
            font_size_xl: 20,
            font_weight_normal: 400,
            font_weight_medium: 500,
            font_weight_semibold: 600,
            font_weight_bold: 700,
            line_height_tight: 1.25,
            line_height_normal: 1.5,
            line_height_relaxed: 1.75,
        },
        shadows: ShadowTokens {
            sm: "0 1px 2px 0 rgb(0 0 0 / 0.05)".to_string(),
            md: "0 4px 6px -1px rgb(0 0 0 / 0.1)".to_string(),
            lg: "0 10px 15px -3px rgb(0 0 0 / 0.1)".to_string(),
        },
        radii: RadiusTokens {
            sm: 6,
            md: 8,
            lg: 12,
            xl: 16,
        },
    }
}"#.to_string() }
            }
            
            Section { title: "Dark Theme Tips",
                p { "When creating dark themes, invert the contrast:" }
                ul {
                    li { "Background: Dark colors (rgb(15,23,42))" }
                    li { "Foreground: Light colors (rgb(248,250,252))" }
                    li { "Keep primary/accent colors vibrant" }
                    li { "Use lighter borders for subtle definition" }
                }
            }
        }
    }
}

#[component]
pub fn PresetThemesPage() -> Element {
    let theme = use_theme();
    let mut selected_theme = use_signal(|| "light".to_string());
    
    // Update selected theme when theme changes
    use_effect(move || {
        let current = theme.tokens.read();
        let name = match current.mode {
            dioxus_ui_system::theme::ThemeMode::Light => "light",
            dioxus_ui_system::theme::ThemeMode::Dark => "dark",
            dioxus_ui_system::theme::ThemeMode::Brand(ref n) => match n.as_str() {
                "rose" => "rose",
                "blue" => "blue",
                "green" => "green",
                "violet" => "violet",
                "orange" => "orange",
                _ => "custom",
            },
        };
        selected_theme.set(name.to_string());
    });
    
    let presets = vec![
        ("light", "Light", "Clean and bright default theme", "rgb(255,255,255)", "rgb(15,23,42)", "rgb(59,130,246)"),
        ("dark", "Dark", "Easy on the eyes for low light", "rgb(15,23,42)", "rgb(248,250,252)", "rgb(59,130,246)"),
        ("rose", "Rose", "Warm and inviting pink tones", "rgb(255,241,242)", "rgb(136,19,55)", "rgb(225,29,72)"),
        ("blue", "Blue", "Professional and trustworthy", "rgb(239,246,255)", "rgb(30,58,138)", "rgb(37,99,235)"),
        ("green", "Green", "Fresh and natural feel", "rgb(240,253,244)", "rgb(20,83,45)", "rgb(22,163,74)"),
        ("violet", "Violet", "Creative and imaginative", "rgb(245,243,255)", "rgb(76,29,149)", "rgb(124,58,237)"),
        ("orange", "Orange", "Energetic and vibrant", "rgb(255,247,237)", "rgb(124,45,18)", "rgb(234,88,12)"),
    ];
    
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 32px;",
            
            h1 { style: "margin: 0; font-size: 32px; font-weight: 800;", "Preset Themes" }
            
            Section { title: "Available Themes",
                p { "Dioxus UI includes 7 preset themes ready to use. Click on any theme to apply it:" }
                
                div { style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px;",
                    for (id, name, description, bg, text, accent) in presets.clone() {
                        ThemeCard {
                            id: id.to_string(),
                            name: name.to_string(),
                            description: description.to_string(),
                            bg: bg.to_string(),
                            text: text.to_string(),
                            accent: accent.to_string(),
                            is_selected: selected_theme() == id,
                            on_select: move |_| {
                                selected_theme.set(id.to_string());
                                theme.set_theme_by_name.call(id.to_string());
                            },
                        }
                    }
                }
            }
            
            Section { title: "Live Preview",
                p { "See how your selected theme looks with real components:" }
                ExampleBox {
                    div { style: "display: flex; flex-direction: column; gap: 16px;",
                        div { style: "display: flex; gap: 12px; flex-wrap: wrap;",
                            Button { variant: ButtonVariant::Primary, "Primary" }
                            Button { variant: ButtonVariant::Secondary, "Secondary" }
                            Button { variant: ButtonVariant::Destructive, "Destructive" }
                            Button { variant: ButtonVariant::Ghost, "Ghost" }
                        }
                        div { style: "display: flex; gap: 12px; flex-wrap: wrap; align-items: center;",
                            Badge { "Default" }
                            Badge { variant: BadgeVariant::Secondary, "Secondary" }
                            Badge { variant: BadgeVariant::Success, "Success" }
                            Badge { variant: BadgeVariant::Destructive, "Error" }
                        }
                        Alert { 
                            variant: AlertVariant::Default, 
                            title: Some("Theme Preview".to_string()),
                            "This is how alerts appear with the current theme."
                        }
                    }
                }
            }
            
            Section { title: "Applying Themes Programmatically",
                p { "Switch between themes in your code:" }
                CodeBlock { code: "// Get current theme context
let theme = use_theme();

// Apply a preset theme by name
theme.set_theme_by_name.call(\"rose\".to_string());

// Or set a specific theme directly
theme.set_theme.call(ThemeTokens::dark());

// Toggle between light and dark
theme.toggle_mode.call(());".to_string() }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ThemeCardProps {
    id: String,
    name: String,
    description: String,
    bg: String,
    text: String,
    accent: String,
    is_selected: bool,
    on_select: EventHandler<()>,
}

#[component]
fn ThemeCard(props: ThemeCardProps) -> Element {
    let border_style = if props.is_selected {
        "border: 2px solid rgb(59,130,246); box-shadow: 0 0 0 3px rgba(59,130,246,0.1);"
    } else {
        "border: 1px solid rgb(226,232,240);"
    };
    
    let badge = if props.is_selected {
        rsx! {
            div {
                style: "position: absolute; top: -8px; right: -8px; background: rgb(59,130,246); color: white; border-radius: 50%; width: 24px; height: 24px; display: flex; align-items: center; justify-content: center; font-size: 12px; font-weight: bold;",
                "✓"
            }
        }
    } else {
        rsx! {}
    };
    
    let hover_style = if !props.is_selected {
        "&:hover { border-color: rgb(156,163,175); }"
    } else {
        ""
    };
    
    rsx! {
        div {
            style: "position: relative; cursor: pointer; border-radius: 12px; overflow: hidden; transition: all 0.2s; {border_style} {hover_style}",
            onclick: move |_| props.on_select.call(()),
            
            {badge}
            
            div {
                style: "height: 80px; background: {props.bg}; display: flex; align-items: center; justify-content: center; gap: 8px;",
                div { style: "width: 24px; height: 24px; border-radius: 6px; background: {props.text};" }
                div { style: "width: 24px; height: 24px; border-radius: 6px; background: {props.accent};" }
            }
            
            div {
                style: "padding: 16px;",
                h3 { style: "margin: 0 0 4px 0; font-size: 16px; font-weight: 600;", "{props.name}" }
                p { style: "margin: 0; font-size: 13px; color: rgb(100,116,139);", "{props.description}" }
            }
        }
    }
}

// Shared Components

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
fn CodeBlock(code: String) -> Element {
    rsx! {
        pre {
            style: "background: rgb(15,23,42); color: rgb(226,232,240); padding: 16px; border-radius: 8px; font-size: 14px; overflow-x: auto;",
            code { "{code}" }
        }
    }
}

#[component]
fn ExampleBox(children: Element) -> Element {
    rsx! {
        Card { variant: CardVariant::Default, padding: Some("24px".to_string()), {children} }
    }
}

#[component]
fn TokenTable(tokens: Vec<(&'static str, &'static str)>) -> Element {
    rsx! {
        table {
            style: "width: 100%; border-collapse: collapse; font-size: 14px;",
            
            thead {
                tr {
                    style: "background: rgb(248,250,252);",
                    th { style: "text-align: left; padding: 12px; border-bottom: 1px solid rgb(226,232,240); font-weight: 600;", "Token" }
                    th { style: "text-align: left; padding: 12px; border-bottom: 1px solid rgb(226,232,240); font-weight: 600;", "Description" }
                }
            }
            
            tbody {
                for (token, desc) in tokens {
                    tr {
                        td { style: "padding: 12px; border-bottom: 1px solid rgb(241,245,249); font-family: monospace; font-size: 13px;", "{token}" }
                        td { style: "padding: 12px; border-bottom: 1px solid rgb(241,245,249);", "{desc}" }
                    }
                }
            }
        }
    }
}
