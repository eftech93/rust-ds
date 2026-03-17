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
                CodeBlock { code: "#[component]\nfn App() -> Element {{\n    rsx! {{\n        ThemeProvider {{\n            // Your app here\n        }}\n    }}\n}}".to_string() }
            }
            
            Section { title: "Theme Controls",
                p { "Add theme toggle and selector to your app:" }
                ExampleBox {
                    div { style: "display: flex; gap: 12px; flex-wrap: wrap;",
                        ThemeToggle {}
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
                CodeBlock { code: "use dioxus_ui_system::theme::{{ThemeTokens, Color}};\n\nfn my_theme() -> ThemeTokens {{\n    ThemeTokens::brand(\n        Color::new(220, 38, 38),  // Primary red color\n        \"acme\"                     // Brand name\n    )\n}}\n\n// Use in app\nThemeProvider {{ initial_theme: Some(my_theme()) }}".to_string() }
            }
            
            Section { title: "Fully Custom Theme",
                p { "Build a theme from scratch with full control:" }
                CodeBlock { code: "use dioxus_ui_system::theme::{{\n    ThemeTokens, ColorTokens, SpacingTokens, \n    TypographyTokens, ShadowTokens, Color\n}};\n\nfn custom_theme() -> ThemeTokens {{\n    ThemeTokens {{\n        colors: ColorTokens {{\n            background: Color::new(250, 250, 250),\n            foreground: Color::new(15, 23, 42),\n            primary: Color::new(37, 99, 235),\n            secondary: Color::new(100, 116, 139),\n            muted: Color::new(241, 245, 249),\n            accent: Color::new(241, 245, 249),\n            destructive: Color::new(239, 68, 68),\n            border: Color::new(226, 232, 240),\n            input: Color::new(226, 232, 240),\n            ring: Color::new(37, 99, 235),\n        }},\n        spacing: SpacingTokens {{\n            unit: 4,\n            scale: vec![0, 4, 8, 12, 16, 24, 32, 48, 64],\n        }},\n        typography: TypographyTokens {{\n            font_family: \"Inter, system-ui, sans-serif\".to_string(),\n            font_size_sm: 14,\n            font_size_base: 16,\n            font_size_lg: 18,\n            font_size_xl: 20,\n            font_weight_normal: 400,\n            font_weight_medium: 500,\n            font_weight_semibold: 600,\n            font_weight_bold: 700,\n            line_height_tight: 1.25,\n            line_height_normal: 1.5,\n            line_height_relaxed: 1.75,\n        }},\n        shadows: ShadowTokens {{\n            sm: \"0 1px 2px 0 rgb(0 0 0 / 0.05)\".to_string(),\n            md: \"0 4px 6px -1px rgb(0 0 0 / 0.1)\".to_string(),\n            lg: \"0 10px 15px -3px rgb(0 0 0 / 0.1)\".to_string(),\n        }},\n        radii: RadiusTokens {{\n            sm: 6,\n            md: 8,\n            lg: 12,\n            xl: 16,\n        }},\n    }}\n}}".to_string() }
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
    let _theme = use_theme();
    
    let presets = vec![
        ("Light", "rgb(255,255,255)", "rgb(15,23,42)", "rgb(59,130,246)"),
        ("Dark", "rgb(15,23,42)", "rgb(248,250,252)", "rgb(59,130,246)"),
        ("Rose", "rgb(255,241,242)", "rgb(136,19,55)", "rgb(225,29,72)"),
        ("Blue", "rgb(239,246,255)", "rgb(30,58,138)", "rgb(37,99,235)"),
        ("Green", "rgb(240,253,244)", "rgb(20,83,45)", "rgb(22,163,74)"),
        ("Violet", "rgb(245,243,255)", "rgb(76,29,149)", "rgb(124,58,237)"),
        ("Orange", "rgb(255,247,237)", "rgb(124,45,18)", "rgb(234,88,12)"),
    ];
    
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 32px;",
            
            h1 { style: "margin: 0; font-size: 32px; font-weight: 800;", "Preset Themes" }
            
            Section { title: "Available Themes",
                p { "Dioxus UI includes 7 preset themes ready to use:" }
                
                div { style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px;",
                    for (name, bg, text, accent) in presets {
                        div {
                            style: "border: 1px solid rgb(226,232,240); border-radius: 12px; overflow: hidden;",
                            
                            div {
                                style: "height: 80px; background: {bg}; display: flex; align-items: center; justify-content: center; gap: 8px;",
                                div { style: "width: 24px; height: 24px; border-radius: 6px; background: {text};" }
                                div { style: "width: 24px; height: 24px; border-radius: 6px; background: {accent};" }
                            }
                            
                            div {
                                style: "padding: 16px;",
                                h3 { style: "margin: 0 0 4px 0; font-size: 16px; font-weight: 600;", "{name}" }
                                p { style: "margin: 0; font-size: 13px; color: rgb(100,116,139);", "Ready to use preset" }
                            }
                        }
                    }
                }
            }
            
            Section { title: "Applying Themes",
                p { "Switch between themes using ThemeSelector or set programmatically:" }
                CodeBlock { code: "// Get current theme\nlet theme = use_theme();\n\n// Set specific theme\nlet light = ThemeTokens::light();\nlet dark = ThemeTokens::dark();\n\n// In your app\nThemeProvider {{\n    initial_theme: Some(ThemeTokens::rose()),\n    children\n}}".to_string() }
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
