//! Theme documentation pages

use dioxus::prelude::*;
use dioxus_ui_system::prelude::*;
use dioxus_ui_system::atoms::{Box, VStack, HStack, Button, ButtonVariant, ButtonSize};
use dioxus_ui_system::theme::{ThemeTokens, Color, ThemeMode};

/// Local storage key for custom theme
const CUSTOM_THEME_STORAGE_KEY: &str = "dioxus_ui_custom_theme";

#[component]
pub fn ThemesPage() -> Element {
    rsx! {
        VStack {
            style: "gap: 32px;",
            
            Box {
                h1 { style: "margin: 0 0 12px 0; font-size: 32px; font-weight: 800;", "Themes" }
                p { style: "margin: 0; font-size: 16px; color: rgb(100,116,139);", 
                    "Comprehensive theming system with preset themes and full customization support." }
            }
            
            Box { style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px;",
                DocCard { title: "Overview", description: "Understand the theme system", route: "/themes/overview" }
                DocCard { title: "Design Tokens", description: "Learn about design tokens", route: "/themes/tokens" }
                DocCard { title: "Theme Builder", description: "Visually create and export custom themes", route: "/themes/custom" }
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
        VStack {
            style: "gap: 32px;",
            
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
                    HStack { gap: SpacingSize::Md, style: "flex-wrap: wrap;",
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
        VStack {
            style: "gap: 32px;",
            
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

/// Custom theme builder state (serializable)
#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CustomThemeState {
    // Identity
    pub name: String,
    
    // Colors
    pub background: String,
    pub foreground: String,
    pub primary: String,
    pub secondary: String,
    pub muted: String,
    pub muted_foreground: String,
    pub accent: String,
    pub destructive: String,
    pub border: String,
    
    // Spacing (in pixels)
    pub spacing_unit: u8,
    pub spacing_xs: u8,
    pub spacing_sm: u8,
    pub spacing_md: u8,
    pub spacing_lg: u8,
    pub spacing_xl: u8,
    
    // Typography (in pixels)
    pub font_size_sm: u16,
    pub font_size_base: u16,
    pub font_size_lg: u16,
    pub font_size_xl: u16,
    pub font_weight_normal: u16,
    pub font_weight_medium: u16,
    pub font_weight_semibold: u16,
    pub font_weight_bold: u16,
    pub line_height_normal: u16,
    
    // Border Radius (in pixels)
    pub radius_sm: u8,
    pub radius_md: u8,
    pub radius_lg: u8,
    pub radius_xl: u8,
    
    // Shadows
    pub shadow_sm: String,
    pub shadow_md: String,
    pub shadow_lg: String,
}

impl Default for CustomThemeState {
    fn default() -> Self {
        Self {
            name: "My Custom Theme".to_string(),
            background: "#ffffff".to_string(),
            foreground: "#0f172a".to_string(),
            primary: "#2563eb".to_string(),
            secondary: "#64748b".to_string(),
            muted: "#f1f5f9".to_string(),
            muted_foreground: "#64748b".to_string(),
            accent: "#f1f5f9".to_string(),
            destructive: "#ef4444".to_string(),
            border: "#e2e8f0".to_string(),
            spacing_unit: 4,
            spacing_xs: 4,
            spacing_sm: 8,
            spacing_md: 16,
            spacing_lg: 24,
            spacing_xl: 32,
            font_size_sm: 14,
            font_size_base: 16,
            font_size_lg: 18,
            font_size_xl: 20,
            font_weight_normal: 400,
            font_weight_medium: 500,
            font_weight_semibold: 600,
            font_weight_bold: 700,
            line_height_normal: 15,
            radius_sm: 6,
            radius_md: 8,
            radius_lg: 12,
            radius_xl: 16,
            shadow_sm: "0 1px 2px 0 rgb(0 0 0 / 0.05)".to_string(),
            shadow_md: "0 4px 6px -1px rgb(0 0 0 / 0.1)".to_string(),
            shadow_lg: "0 10px 15px -3px rgb(0 0 0 / 0.1)".to_string(),
        }
    }
}

impl CustomThemeState {
    fn to_theme_tokens(&self) -> ThemeTokens {
        let mut theme = ThemeTokens::light();
        theme.mode = ThemeMode::Brand(self.name.clone());
        
        // Apply colors
        theme.colors.background = hex_to_color(&self.background);
        theme.colors.foreground = hex_to_color(&self.foreground);
        theme.colors.primary = hex_to_color(&self.primary);
        theme.colors.primary_foreground = if is_light_color(&self.primary) {
            Color::new(0, 0, 0)
        } else {
            Color::new(255, 255, 255)
        };
        theme.colors.secondary = hex_to_color(&self.secondary);
        theme.colors.secondary_foreground = hex_to_color(&self.foreground);
        theme.colors.muted = hex_to_color(&self.muted);
        theme.colors.muted_foreground = hex_to_color(&self.muted_foreground);
        theme.colors.accent = hex_to_color(&self.accent);
        theme.colors.accent_foreground = hex_to_color(&self.foreground);
        theme.colors.destructive = hex_to_color(&self.destructive);
        theme.colors.border = hex_to_color(&self.border);
        theme.colors.ring = hex_to_color(&self.primary);
        theme.colors.card = if is_light_color(&self.background) {
            Color::new(255, 255, 255)
        } else {
            Color::new(30, 41, 59)
        };
        theme.colors.card_foreground = hex_to_color(&self.foreground);
        theme.colors.popover = if is_light_color(&self.background) {
            Color::new(255, 255, 255)
        } else {
            Color::new(30, 41, 59)
        };
        theme.colors.popover_foreground = hex_to_color(&self.foreground);
        
        // Apply spacing (u16 values)
        theme.spacing.xs = self.spacing_xs as u16;
        theme.spacing.sm = self.spacing_sm as u16;
        theme.spacing.md = self.spacing_md as u16;
        theme.spacing.lg = self.spacing_lg as u16;
        theme.spacing.xl = self.spacing_xl as u16;
        
        // Apply typography - update each Typography struct
        let line_height = self.line_height_normal as f32 / 10.0;
        
        theme.typography.xs.size = self.font_size_sm.saturating_sub(2);
        theme.typography.xs.weight = self.font_weight_normal;
        theme.typography.xs.line_height = line_height;
        
        theme.typography.sm.size = self.font_size_sm;
        theme.typography.sm.weight = self.font_weight_normal;
        theme.typography.sm.line_height = line_height;
        
        theme.typography.base.size = self.font_size_base;
        theme.typography.base.weight = self.font_weight_normal;
        theme.typography.base.line_height = line_height;
        
        theme.typography.lg.size = self.font_size_lg;
        theme.typography.lg.weight = self.font_weight_normal;
        theme.typography.lg.line_height = line_height;
        
        theme.typography.xl.size = self.font_size_xl;
        theme.typography.xl.weight = self.font_weight_medium;
        theme.typography.xl.line_height = line_height;
        
        theme.typography.xxl.size = self.font_size_xl.saturating_add(4);
        theme.typography.xxl.weight = self.font_weight_semibold;
        theme.typography.xxl.line_height = 1.2;
        
        theme.typography.h1.size = self.font_size_xl.saturating_add(12);
        theme.typography.h1.weight = self.font_weight_bold;
        theme.typography.h1.line_height = 1.2;
        
        theme.typography.h2.size = self.font_size_xl.saturating_add(8);
        theme.typography.h2.weight = self.font_weight_bold;
        theme.typography.h2.line_height = 1.25;
        
        theme.typography.h3.size = self.font_size_xl.saturating_add(4);
        theme.typography.h3.weight = self.font_weight_semibold;
        theme.typography.h3.line_height = 1.3;
        
        theme.typography.h4.size = self.font_size_xl;
        theme.typography.h4.weight = self.font_weight_semibold;
        theme.typography.h4.line_height = 1.35;
        
        // Apply border radius (u16 values)
        theme.radius.sm = self.radius_sm as u16;
        theme.radius.md = self.radius_md as u16;
        theme.radius.lg = self.radius_lg as u16;
        theme.radius.xl = self.radius_xl as u16;
        
        // Apply shadows
        theme.shadows.sm = self.shadow_sm.clone();
        theme.shadows.md = self.shadow_md.clone();
        theme.shadows.lg = self.shadow_lg.clone();
        
        theme
    }

    fn generate_rust_code(&self) -> String {
        let bg = hex_to_rgb(&self.background);
        let fg = hex_to_rgb(&self.foreground);
        let primary = hex_to_rgb(&self.primary);
        let primary_fg = if is_light_color(&self.primary) { (0, 0, 0) } else { (255, 255, 255) };
        let secondary = hex_to_rgb(&self.secondary);
        let muted = hex_to_rgb(&self.muted);
        let muted_fg = hex_to_rgb(&self.muted_foreground);
        let accent = hex_to_rgb(&self.accent);
        let destructive = hex_to_rgb(&self.destructive);
        let border = hex_to_rgb(&self.border);
        let card = if is_light_color(&self.background) { (255, 255, 255) } else { (30, 41, 59) };
        
        format!(r#"//! Custom Theme for Dioxus UI
//! 
//! Theme Name: {name}
//! Generated by Dioxus UI Theme Builder

use dioxus_ui_system::theme::{{
    ThemeTokens, Color, ThemeMode,
    SpacingTokens, SpacingScale,
    TypographyTokens, TypographyScale,
    RadiusTokens, RadiusScale,
    ShadowTokens, ShadowScale
}};

/// Your custom theme
pub fn custom_theme() -> ThemeTokens {{
    let mut theme = ThemeTokens::light();
    theme.mode = ThemeMode::Brand("{mode}".into());
    
    // ==================== COLORS ====================
    
    // Background colors
    theme.colors.background = Color::new({bg_r}, {bg_g}, {bg_b});
    theme.colors.foreground = Color::new({fg_r}, {fg_g}, {fg_b});
    
    // Primary colors
    theme.colors.primary = Color::new({primary_r}, {primary_g}, {primary_b});
    theme.colors.primary_foreground = Color::new({primary_fg_r}, {primary_fg_g}, {primary_fg_b});
    
    // Secondary colors
    theme.colors.secondary = Color::new({secondary_r}, {secondary_g}, {secondary_b});
    theme.colors.secondary_foreground = Color::new({fg_r}, {fg_g}, {fg_b});
    
    // Muted colors
    theme.colors.muted = Color::new({muted_r}, {muted_g}, {muted_b});
    theme.colors.muted_foreground = Color::new({muted_fg_r}, {muted_fg_g}, {muted_fg_b});
    
    // Accent colors
    theme.colors.accent = Color::new({accent_r}, {accent_g}, {accent_b});
    theme.colors.accent_foreground = Color::new({fg_r}, {fg_g}, {fg_b});
    
    // Destructive colors
    theme.colors.destructive = Color::new({destructive_r}, {destructive_g}, {destructive_b});
    
    // UI colors
    theme.colors.border = Color::new({border_r}, {border_g}, {border_b});
    theme.colors.ring = Color::new({primary_r}, {primary_g}, {primary_b});
    
    // Card and popover
    theme.colors.card = Color::new({card_r}, {card_g}, {card_b});
    theme.colors.card_foreground = Color::new({fg_r}, {fg_g}, {fg_b});
    theme.colors.popover = Color::new({card_r}, {card_g}, {card_b});
    theme.colors.popover_foreground = Color::new({fg_r}, {fg_g}, {fg_b});
    
    // ==================== SPACING ====================
    
    theme.spacing = SpacingTokens {{
        unit: {spacing_unit},
        scale: SpacingScale {{
            xs: {spacing_xs},
            sm: {spacing_sm},
            md: {spacing_md},
            lg: {spacing_lg},
            xl: {spacing_xl},
        }},
    }};
    
    // ==================== TYPOGRAPHY ====================
    
    theme.typography = TypographyTokens {{
        font_family: "Inter, system-ui, sans-serif".to_string(),
        scale: TypographyScale {{
            sm: {font_size_sm},
            base: {font_size_base},
            lg: {font_size_lg},
            xl: {font_size_xl},
        }},
        font_weight_normal: {font_weight_normal},
        font_weight_medium: {font_weight_medium},
        font_weight_semibold: {font_weight_semibold},
        font_weight_bold: {font_weight_bold},
        line_height_normal: {line_height_normal},
    }};
    
    // ==================== BORDER RADIUS ====================
    
    theme.radius = RadiusTokens {{
        sm: {radius_sm},
        md: {radius_md},
        lg: {radius_lg},
        xl: {radius_xl},
    }};
    
    // ==================== SHADOWS ====================
    
    theme.shadows = ShadowTokens {{
        sm: "{shadow_sm}".to_string(),
        md: "{shadow_md}".to_string(),
        lg: "{shadow_lg}".to_string(),
    }};
    
    theme
}}

/// Use in your app:
/// 
/// ThemeProvider {{
///     initial_theme: Some(custom_theme()),
///     // Your app content
/// }}
"#,
            name = self.name,
            mode = self.name.to_lowercase().replace(" ", "_"),
            bg_r = bg.0, bg_g = bg.1, bg_b = bg.2,
            fg_r = fg.0, fg_g = fg.1, fg_b = fg.2,
            primary_r = primary.0, primary_g = primary.1, primary_b = primary.2,
            primary_fg_r = primary_fg.0, primary_fg_g = primary_fg.1, primary_fg_b = primary_fg.2,
            secondary_r = secondary.0, secondary_g = secondary.1, secondary_b = secondary.2,
            muted_r = muted.0, muted_g = muted.1, muted_b = muted.2,
            muted_fg_r = muted_fg.0, muted_fg_g = muted_fg.1, muted_fg_b = muted_fg.2,
            accent_r = accent.0, accent_g = accent.1, accent_b = accent.2,
            destructive_r = destructive.0, destructive_g = destructive.1, destructive_b = destructive.2,
            border_r = border.0, border_g = border.1, border_b = border.2,
            card_r = card.0, card_g = card.1, card_b = card.2,
            spacing_unit = self.spacing_unit,
            spacing_xs = self.spacing_xs,
            spacing_sm = self.spacing_sm,
            spacing_md = self.spacing_md,
            spacing_lg = self.spacing_lg,
            spacing_xl = self.spacing_xl,
            font_size_sm = self.font_size_sm,
            font_size_base = self.font_size_base,
            font_size_lg = self.font_size_lg,
            font_size_xl = self.font_size_xl,
            font_weight_normal = self.font_weight_normal,
            font_weight_medium = self.font_weight_medium,
            font_weight_semibold = self.font_weight_semibold,
            font_weight_bold = self.font_weight_bold,
            line_height_normal = self.line_height_normal,
            radius_sm = self.radius_sm,
            radius_md = self.radius_md,
            radius_lg = self.radius_lg,
            radius_xl = self.radius_xl,
            shadow_sm = self.shadow_sm,
            shadow_md = self.shadow_md,
            shadow_lg = self.shadow_lg,
        )
    }
}

fn hex_to_color(hex: &str) -> Color {
    let (r, g, b) = hex_to_rgb(hex);
    Color::new(r, g, b)
}

fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);
    (r, g, b)
}

fn is_light_color(hex: &str) -> bool {
    let (r, g, b) = hex_to_rgb(hex);
    let luminance = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) / 255.0;
    luminance > 0.5
}

#[component]
pub fn CustomThemePage() -> Element {
    let theme = use_theme();
    let mut show_code = use_signal(|| false);
    
    // Load from local storage or use default
    let mut theme_state = use_signal(|| {
        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok()).flatten() {
            if let Ok(Some(json)) = storage.get_item(CUSTOM_THEME_STORAGE_KEY) {
                if let Ok(state) = serde_json::from_str::<CustomThemeState>(&json) {
                    return state;
                }
            }
        }
        CustomThemeState::default()
    });
    
    // Apply custom theme on load and when state changes
    use_effect(move || {
        let custom_theme = theme_state().to_theme_tokens();
        theme.set_theme.call(custom_theme);
        
        // Save to local storage
        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok()).flatten() {
            if let Ok(json) = serde_json::to_string(&theme_state()) {
                let _ = storage.set_item(CUSTOM_THEME_STORAGE_KEY, &json);
            }
        }
    });
    
    let generated_code = use_memo(move || theme_state().generate_rust_code());
    
    rsx! {
        VStack {
            style: "gap: 32px;",
            
            Box {
                h1 { style: "margin: 0 0 12px 0; font-size: 32px; font-weight: 800;", "Theme Builder" }
                p { style: "margin: 0; font-size: 16px; color: rgb(100,116,139);", 
                    "Create, preview, and export your custom theme. Your theme is automatically saved to local storage." }
            }
            
            // Theme Name Input
            Section { title: "Theme Name",
                input {
                    r#type: "text",
                    value: "{theme_state().name}",
                    style: "width: 100%; max-width: 400px; padding: 10px 12px; border: 1px solid rgb(226,232,240); border-radius: 8px; font-size: 16px;",
                    oninput: move |e| {
                        let mut new_state = theme_state();
                        new_state.name = e.value();
                        theme_state.set(new_state);
                    },
                }
            }
            
            // Color Controls
            Section { title: "Colors",
                p { style: "margin: 0 0 16px 0; color: rgb(100,116,139); font-size: 14px;",
                    "Click on any color to edit. The preview updates automatically." }
                
                ColorGrid {
                    ColorPicker {
                        label: "Background",
                        value: theme_state().background.clone(),
                        on_change: move |c: String| {
                            let mut new_state = theme_state();
                            new_state.background = c;
                            theme_state.set(new_state);
                        },
                    }
                    ColorPicker {
                        label: "Foreground (Text)",
                        value: theme_state().foreground.clone(),
                        on_change: move |c: String| {
                            let mut new_state = theme_state();
                            new_state.foreground = c;
                            theme_state.set(new_state);
                        },
                    }
                    ColorPicker {
                        label: "Primary",
                        value: theme_state().primary.clone(),
                        on_change: move |c: String| {
                            let mut new_state = theme_state();
                            new_state.primary = c;
                            theme_state.set(new_state);
                        },
                    }
                    ColorPicker {
                        label: "Secondary",
                        value: theme_state().secondary.clone(),
                        on_change: move |c: String| {
                            let mut new_state = theme_state();
                            new_state.secondary = c;
                            theme_state.set(new_state);
                        },
                    }
                    ColorPicker {
                        label: "Muted",
                        value: theme_state().muted.clone(),
                        on_change: move |c: String| {
                            let mut new_state = theme_state();
                            new_state.muted = c;
                            theme_state.set(new_state);
                        },
                    }
                    ColorPicker {
                        label: "Muted Foreground",
                        value: theme_state().muted_foreground.clone(),
                        on_change: move |c: String| {
                            let mut new_state = theme_state();
                            new_state.muted_foreground = c;
                            theme_state.set(new_state);
                        },
                    }
                    ColorPicker {
                        label: "Accent",
                        value: theme_state().accent.clone(),
                        on_change: move |c: String| {
                            let mut new_state = theme_state();
                            new_state.accent = c;
                            theme_state.set(new_state);
                        },
                    }
                    ColorPicker {
                        label: "Destructive",
                        value: theme_state().destructive.clone(),
                        on_change: move |c: String| {
                            let mut new_state = theme_state();
                            new_state.destructive = c;
                            theme_state.set(new_state);
                        },
                    }
                    ColorPicker {
                        label: "Border",
                        value: theme_state().border.clone(),
                        on_change: move |c: String| {
                            let mut new_state = theme_state();
                            new_state.border = c;
                            theme_state.set(new_state);
                        },
                    }
                }
            }
            
            // Spacing Controls
            Section { title: "Spacing",
                p { style: "margin: 0 0 16px 0; color: rgb(100,116,139); font-size: 14px;",
                    "Control the spacing scale used throughout your theme (in pixels):" }
                
                NumberGrid {
                    NumberInput {
                        label: "Unit",
                        value: theme_state().spacing_unit as u16,
                        min: 1,
                        max: 16,
                        on_change: move |v| {
                            let mut new_state = theme_state();
                            new_state.spacing_unit = v as u8;
                            theme_state.set(new_state);
                        },
                    }
                    NumberInput {
                        label: "XS",
                        value: theme_state().spacing_xs as u16,
                        min: 0,
                        max: 32,
                        on_change: move |v| {
                            let mut new_state = theme_state();
                            new_state.spacing_xs = v as u8;
                            theme_state.set(new_state);
                        },
                    }
                    NumberInput {
                        label: "SM",
                        value: theme_state().spacing_sm as u16,
                        min: 0,
                        max: 32,
                        on_change: move |v| {
                            let mut new_state = theme_state();
                            new_state.spacing_sm = v as u8;
                            theme_state.set(new_state);
                        },
                    }
                    NumberInput {
                        label: "MD",
                        value: theme_state().spacing_md as u16,
                        min: 0,
                        max: 64,
                        on_change: move |v| {
                            let mut new_state = theme_state();
                            new_state.spacing_md = v as u8;
                            theme_state.set(new_state);
                        },
                    }
                    NumberInput {
                        label: "LG",
                        value: theme_state().spacing_lg as u16,
                        min: 0,
                        max: 64,
                        on_change: move |v| {
                            let mut new_state = theme_state();
                            new_state.spacing_lg = v as u8;
                            theme_state.set(new_state);
                        },
                    }
                    NumberInput {
                        label: "XL",
                        value: theme_state().spacing_xl as u16,
                        min: 0,
                        max: 128,
                        on_change: move |v| {
                            let mut new_state = theme_state();
                            new_state.spacing_xl = v as u8;
                            theme_state.set(new_state);
                        },
                    }
                }
            }
            
            // Typography Controls
            Section { title: "Typography",
                p { style: "margin: 0 0 16px 0; color: rgb(100,116,139); font-size: 14px;",
                    "Configure font sizes and weights:" }
                
                NumberGrid {
                    NumberInput {
                        label: "Font Size SM (px)",
                        value: theme_state().font_size_sm,
                        min: 8,
                        max: 32,
                        on_change: move |v| {
                            let mut new_state = theme_state();
                            new_state.font_size_sm = v;
                            theme_state.set(new_state);
                        },
                    }
                    NumberInput {
                        label: "Font Size Base (px)",
                        value: theme_state().font_size_base,
                        min: 10,
                        max: 32,
                        on_change: move |v| {
                            let mut new_state = theme_state();
                            new_state.font_size_base = v;
                            theme_state.set(new_state);
                        },
                    }
                    NumberInput {
                        label: "Font Size LG (px)",
                        value: theme_state().font_size_lg,
                        min: 12,
                        max: 48,
                        on_change: move |v| {
                            let mut new_state = theme_state();
                            new_state.font_size_lg = v;
                            theme_state.set(new_state);
                        },
                    }
                    NumberInput {
                        label: "Font Size XL (px)",
                        value: theme_state().font_size_xl,
                        min: 14,
                        max: 64,
                        on_change: move |v| {
                            let mut new_state = theme_state();
                            new_state.font_size_xl = v;
                            theme_state.set(new_state);
                        },
                    }
                }
                
                NumberGrid {
                    NumberInput {
                        label: "Weight Normal",
                        value: theme_state().font_weight_normal,
                        min: 100,
                        max: 900,
                        step: 100,
                        on_change: move |v| {
                            let mut new_state = theme_state();
                            new_state.font_weight_normal = v as u16;
                            theme_state.set(new_state);
                        },
                    }
                    NumberInput {
                        label: "Weight Medium",
                        value: theme_state().font_weight_medium,
                        min: 100,
                        max: 900,
                        step: 100,
                        on_change: move |v| {
                            let mut new_state = theme_state();
                            new_state.font_weight_medium = v as u16;
                            theme_state.set(new_state);
                        },
                    }
                    NumberInput {
                        label: "Weight Semibold",
                        value: theme_state().font_weight_semibold,
                        min: 100,
                        max: 900,
                        step: 100,
                        on_change: move |v| {
                            let mut new_state = theme_state();
                            new_state.font_weight_semibold = v as u16;
                            theme_state.set(new_state);
                        },
                    }
                    NumberInput {
                        label: "Weight Bold",
                        value: theme_state().font_weight_bold,
                        min: 100,
                        max: 900,
                        step: 100,
                        on_change: move |v| {
                            let mut new_state = theme_state();
                            new_state.font_weight_bold = v as u16;
                            theme_state.set(new_state);
                        },
                    }
                }
                
                NumberGrid {
                    NumberInput {
                        label: "Line Height",
                        value: theme_state().line_height_normal,
                        min: 10,
                        max: 30,
                        on_change: move |v| {
                            let mut new_state = theme_state();
                            new_state.line_height_normal = v;
                            theme_state.set(new_state);
                        },
                    }
                }
            }
            
            // Border Radius Controls
            Section { title: "Border Radius",
                p { style: "margin: 0 0 16px 0; color: rgb(100,116,139); font-size: 14px;",
                    "Control corner roundness (in pixels):" }
                
                NumberGrid {
                    NumberInput {
                        label: "Small",
                        value: theme_state().radius_sm as u16,
                        min: 0,
                        max: 32,
                        on_change: move |v| {
                            let mut new_state = theme_state();
                            new_state.radius_sm = v as u8;
                            theme_state.set(new_state);
                        },
                    }
                    NumberInput {
                        label: "Medium",
                        value: theme_state().radius_md as u16,
                        min: 0,
                        max: 32,
                        on_change: move |v| {
                            let mut new_state = theme_state();
                            new_state.radius_md = v as u8;
                            theme_state.set(new_state);
                        },
                    }
                    NumberInput {
                        label: "Large",
                        value: theme_state().radius_lg as u16,
                        min: 0,
                        max: 48,
                        on_change: move |v| {
                            let mut new_state = theme_state();
                            new_state.radius_lg = v as u8;
                            theme_state.set(new_state);
                        },
                    }
                    NumberInput {
                        label: "Extra Large",
                        value: theme_state().radius_xl as u16,
                        min: 0,
                        max: 64,
                        on_change: move |v| {
                            let mut new_state = theme_state();
                            new_state.radius_xl = v as u8;
                            theme_state.set(new_state);
                        },
                    }
                }
            }
            
            // Shadows Controls
            Section { title: "Shadows",
                p { style: "margin: 0 0 16px 0; color: rgb(100,116,139); font-size: 14px;",
                    "Configure shadow effects (CSS box-shadow values):" }
                
                ShadowInput {
                    label: "Small Shadow",
                    value: theme_state().shadow_sm.clone(),
                    on_change: move |v| {
                        let mut new_state = theme_state();
                        new_state.shadow_sm = v;
                        theme_state.set(new_state);
                    },
                }
                ShadowInput {
                    label: "Medium Shadow",
                    value: theme_state().shadow_md.clone(),
                    on_change: move |v| {
                        let mut new_state = theme_state();
                        new_state.shadow_md = v;
                        theme_state.set(new_state);
                    },
                }
                ShadowInput {
                    label: "Large Shadow",
                    value: theme_state().shadow_lg.clone(),
                    on_change: move |v| {
                        let mut new_state = theme_state();
                        new_state.shadow_lg = v;
                        theme_state.set(new_state);
                    },
                }
            }
            
            // Live Preview
            Section { title: "Live Preview",
                p { style: "margin: 0 0 16px 0; color: rgb(100,116,139); font-size: 14px;",
                    "See how your theme looks with real components:" }
                
                ThemePreview {}
            }
            
            // Export Code
            Section { title: "Export Theme",
                p { style: "margin: 0 0 16px 0; color: rgb(100,116,139); font-size: 14px;",
                    "Copy this code to your project as custom_theme.rs:" }
                
                Button {
                    variant: ButtonVariant::Primary,
                    size: ButtonSize::Sm,
                    onclick: move |_| show_code.set(!show_code()),
                    if show_code() { "Hide Code" } else { "Show Generated Code" }
                }
                
                if show_code() {
                    Box {
                        style: "margin-top: 16px; position: relative;",
                        
                        Button {
                            variant: ButtonVariant::Secondary,
                            size: ButtonSize::Sm,
                            style: "position: absolute; top: 8px; right: 8px;",
                            onclick: move |_| {
                                // Copy to clipboard
                                if let Some(window) = web_sys::window() {
                                    let _ = window.navigator().clipboard().write_text(&generated_code());
                                }
                            },
                            "Copy to Clipboard"
                        }
                        
                        pre {
                            style: "background: rgb(15,23,42); color: rgb(226,232,240); padding: 16px; border-radius: 8px; font-size: 13px; overflow-x: auto; max-height: 500px; overflow-y: auto;",
                            code { "{generated_code()}" }
                        }
                    }
                }
            }
            
            // Reset Button
            Section { title: "Reset",
                p { style: "margin: 0 0 16px 0; color: rgb(100,116,139); font-size: 14px;",
                    "Reset to default theme:" }
                
                HStack { gap: SpacingSize::Sm,
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        onclick: move |_| {
                            theme_state.set(CustomThemeState::default());
                        },
                        "Reset to Default"
                    }
                    
                    Button {
                        variant: ButtonVariant::Destructive,
                        size: ButtonSize::Sm,
                        onclick: move |_| {
                            // Clear local storage
                            if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok()).flatten() {
                                let _ = storage.remove_item(CUSTOM_THEME_STORAGE_KEY);
                            }
                            theme_state.set(CustomThemeState::default());
                        },
                        "Clear Saved Theme"
                    }
                }
            }
        }
    }
}

#[component]
fn ColorGrid(children: Element) -> Element {
    rsx! {
        div {
            style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: 16px;",
            {children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ColorPickerProps {
    label: String,
    value: String,
    on_change: EventHandler<String>,
}

#[component]
fn ColorPicker(props: ColorPickerProps) -> Element {
    let mut show_picker = use_signal(|| false);
    let value = props.value.clone();
    
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 12px; padding: 12px; border: 1px solid rgb(226,232,240); border-radius: 8px; background: white; box-sizing: border-box; min-height: 72px;",
            
            // Color swatch button
            div {
                style: "width: 40px; height: 40px; border-radius: 6px; overflow: hidden; cursor: pointer; flex-shrink: 0; box-shadow: inset 0 0 0 1px rgba(0,0,0,0.1); background-color: {value}; position: relative;",
                onclick: move |_| show_picker.set(true),
                
                // Hidden color input that we trigger programmatically
                input {
                    r#type: "color",
                    value: "{value}",
                    style: "position: absolute; top: 0; left: 0; opacity: 0; width: 1px; height: 1px; pointer-events: none;",
                    style: if show_picker() { "pointer-events: auto;" } else { "" },
                    oninput: move |e| {
                        props.on_change.call(e.value());
                        show_picker.set(false);
                    },
                    onchange: move |_| show_picker.set(false),
                }
            }
            
            // Label and hex input
            div {
                style: "flex: 1; min-width: 0;",
                
                div {
                    style: "font-size: 13px; font-weight: 500; margin-bottom: 4px; color: rgb(71,85,105); white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                    "{props.label}"
                }
                
                input {
                    r#type: "text",
                    value: "{value}",
                    style: "width: 100%; padding: 4px 8px; border: 1px solid rgb(226,232,240); border-radius: 4px; font-size: 12px; font-family: monospace; box-sizing: border-box;",
                    oninput: move |e| props.on_change.call(e.value()),
                }
            }
        }
    }
}

#[component]
fn ThemePreview() -> Element {
    rsx! {
        Card {
            variant: CardVariant::Default,
            padding: Some("24px".to_string()),
            
            VStack { gap: SpacingSize::Lg,
                // Typography showcase
                VStack { gap: SpacingSize::Sm,
                    h4 { style: "margin: 0; font-size: 14px; color: rgb(100,116,139);", "Typography" }
                    div {
                        style: "padding: 16px; background: rgb(248,250,252); border-radius: 8px;",
                        
                        VStack { gap: SpacingSize::Xs,
                            span { style: "font-size: var(--font-size-sm);", "Small text (sm)" }
                            span { style: "font-size: var(--font-size-base); font-weight: var(--font-weight-normal);", "Base text (normal)" }
                            span { style: "font-size: var(--font-size-base); font-weight: var(--font-weight-medium);", "Medium weight" }
                            span { style: "font-size: var(--font-size-lg); font-weight: var(--font-weight-semibold);", "Large semibold" }
                            span { style: "font-size: var(--font-size-xl); font-weight: var(--font-weight-bold);", "Extra Large bold" }
                        }
                    }
                }
                
                Separator {}
                
                // Primary buttons showcase
                VStack { gap: SpacingSize::Sm,
                    h4 { style: "margin: 0; font-size: 14px; color: rgb(100,116,139);", "Buttons" }
                    HStack { gap: SpacingSize::Sm, style: "flex-wrap: wrap;",
                        Button { variant: ButtonVariant::Primary, "Primary" }
                        Button { variant: ButtonVariant::Secondary, "Secondary" }
                        Button { variant: ButtonVariant::Destructive, "Destructive" }
                        Button { variant: ButtonVariant::Ghost, "Ghost" }
                    }
                }
                
                Separator {}
                
                // Spacing showcase
                VStack { gap: SpacingSize::Sm,
                    h4 { style: "margin: 0; font-size: 14px; color: rgb(100,116,139);", "Spacing Scale" }
                    div {
                        style: "padding: 16px; background: rgb(248,250,252); border-radius: 8px;",
                        
                        VStack { gap: SpacingSize::Xs,
                            div { style: "display: flex; align-items: center; gap: 8px;",
                                span { style: "width: 60px; font-size: 12px; color: rgb(100,116,139);", "XS:" }
                                div { style: "height: 20px; width: var(--spacing-xs); background: rgb(59,130,246); border-radius: 2px;" }
                            }
                            div { style: "display: flex; align-items: center; gap: 8px;",
                                span { style: "width: 60px; font-size: 12px; color: rgb(100,116,139);", "SM:" }
                                div { style: "height: 20px; width: var(--spacing-sm); background: rgb(59,130,246); border-radius: 2px;" }
                            }
                            div { style: "display: flex; align-items: center; gap: 8px;",
                                span { style: "width: 60px; font-size: 12px; color: rgb(100,116,139);", "MD:" }
                                div { style: "height: 20px; width: var(--spacing-md); background: rgb(59,130,246); border-radius: 2px;" }
                            }
                            div { style: "display: flex; align-items: center; gap: 8px;",
                                span { style: "width: 60px; font-size: 12px; color: rgb(100,116,139);", "LG:" }
                                div { style: "height: 20px; width: var(--spacing-lg); background: rgb(59,130,246); border-radius: 2px;" }
                            }
                            div { style: "display: flex; align-items: center; gap: 8px;",
                                span { style: "width: 60px; font-size: 12px; color: rgb(100,116,139);", "XL:" }
                                div { style: "height: 20px; width: var(--spacing-xl); background: rgb(59,130,246); border-radius: 2px;" }
                            }
                        }
                    }
                }
                
                Separator {}
                
                // Border radius showcase
                VStack { gap: SpacingSize::Sm,
                    h4 { style: "margin: 0; font-size: 14px; color: rgb(100,116,139);", "Border Radius" }
                    HStack { gap: SpacingSize::Md, style: "flex-wrap: wrap; align-items: center;",
                        div { style: "width: 60px; height: 40px; background: rgb(59,130,246); border-radius: var(--radius-sm); display: flex; align-items: center; justify-content: center; color: white; font-size: 12px;", "SM" }
                        div { style: "width: 60px; height: 40px; background: rgb(59,130,246); border-radius: var(--radius-md); display: flex; align-items: center; justify-content: center; color: white; font-size: 12px;", "MD" }
                        div { style: "width: 60px; height: 40px; background: rgb(59,130,246); border-radius: var(--radius-lg); display: flex; align-items: center; justify-content: center; color: white; font-size: 12px;", "LG" }
                        div { style: "width: 60px; height: 40px; background: rgb(59,130,246); border-radius: var(--radius-xl); display: flex; align-items: center; justify-content: center; color: white; font-size: 12px;", "XL" }
                    }
                }
                
                Separator {}
                
                // Badges showcase
                VStack { gap: SpacingSize::Sm,
                    h4 { style: "margin: 0; font-size: 14px; color: rgb(100,116,139);", "Badges" }
                    HStack { gap: SpacingSize::Sm, style: "flex-wrap: wrap;",
                        Badge { "Default" }
                        Badge { variant: BadgeVariant::Secondary, "Secondary" }
                        Badge { variant: BadgeVariant::Success, "Success" }
                        Badge { variant: BadgeVariant::Warning, "Warning" }
                        Badge { variant: BadgeVariant::Destructive, "Destructive" }
                    }
                }
                
                Separator {}
                
                // Alert showcase
                VStack { gap: SpacingSize::Sm,
                    h4 { style: "margin: 0; font-size: 14px; color: rgb(100,116,139);", "Alert" }
                    Alert {
                        variant: AlertVariant::Default,
                        title: Some("Default Alert".to_string()),
                        "This shows how alerts appear with your theme."
                    }
                    Alert {
                        variant: AlertVariant::Success,
                        title: Some("Success".to_string()),
                        "Your theme has been updated!"
                    }
                }
                
                Separator {}
                
                // Input showcase
                VStack { gap: SpacingSize::Sm,
                    h4 { style: "margin: 0; font-size: 14px; color: rgb(100,116,139);", "Input" }
                    InputGroup {
                        label: "Example Input",
                        value: "Hello World".to_string(),
                        placeholder: Some("Type something...".to_string()),
                        onchange: move |_| {},
                    }
                }
                
                Separator {}
                
                // Card showcase
                VStack { gap: SpacingSize::Sm,
                    h4 { style: "margin: 0; font-size: 14px; color: rgb(100,116,139);", "Card Variants" }
                    HStack { gap: SpacingSize::Sm, style: "flex-wrap: wrap;",
                        Card { variant: CardVariant::Default, padding: Some("16px".to_string()), "Default" }
                        Card { variant: CardVariant::Muted, padding: Some("16px".to_string()), "Muted" }
                        Card { variant: CardVariant::Elevated, padding: Some("16px".to_string()), "Elevated" }
                    }
                }
            }
        }
    }
}

#[component]
fn NumberGrid(children: Element) -> Element {
    rsx! {
        div {
            style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(180px, 1fr)); gap: 12px; margin-bottom: 16px;",
            {children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct NumberInputProps {
    label: String,
    value: u16,
    min: u16,
    max: u16,
    #[props(default = 1)]
    step: u16,
    on_change: EventHandler<u16>,
}

#[component]
fn NumberInput(props: NumberInputProps) -> Element {
    rsx! {
        div {
            style: "padding: 12px; border: 1px solid rgb(226,232,240); border-radius: 8px; background: white; min-width: 0;",
            
            div {
                style: "font-size: 13px; font-weight: 500; margin-bottom: 8px; color: rgb(71,85,105); white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                "{props.label}"
            }
            
            div {
                style: "display: flex; align-items: center; gap: 8px; min-width: 0;",
                
                input {
                    r#type: "range",
                    min: "{props.min}",
                    max: "{props.max}",
                    step: "{props.step}",
                    value: "{props.value}",
                    style: "flex: 1; min-width: 0; cursor: pointer;",
                    oninput: move |e| {
                        if let Ok(val) = e.value().parse::<u16>() {
                            props.on_change.call(val);
                        }
                    },
                }
                
                input {
                    r#type: "number",
                    min: "{props.min}",
                    max: "{props.max}",
                    step: "{props.step}",
                    value: "{props.value}",
                    style: "width: 50px; flex-shrink: 0; padding: 4px 4px; border: 1px solid rgb(226,232,240); border-radius: 4px; font-size: 13px; text-align: center;",
                    oninput: move |e| {
                        if let Ok(val) = e.value().parse::<u16>() {
                            props.on_change.call(val);
                        }
                    },
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ShadowInputProps {
    label: String,
    value: String,
    on_change: EventHandler<String>,
}

#[component]
fn ShadowInput(props: ShadowInputProps) -> Element {
    let value = props.value.clone();
    
    rsx! {
        div {
            style: "padding: 12px; border: 1px solid rgb(226,232,240); border-radius: 8px; background: white; margin-bottom: 12px;",
            
            div {
                style: "font-size: 13px; font-weight: 500; margin-bottom: 8px; color: rgb(71,85,105);",
                "{props.label}"
            }
            
            div {
                style: "display: flex; align-items: center; gap: 12px;",
                
                // Preview box with shadow
                div {
                    style: "width: 60px; height: 40px; background: white; border-radius: 4px; flex-shrink: 0;",
                    style: "box-shadow: {value};",
                }
                
                input {
                    r#type: "text",
                    value: "{value}",
                    style: "flex: 1; padding: 8px 12px; border: 1px solid rgb(226,232,240); border-radius: 6px; font-size: 13px; font-family: monospace;",
                    oninput: move |e| props.on_change.call(e.value()),
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
        VStack {
            style: "gap: 32px;",
            
            h1 { style: "margin: 0; font-size: 32px; font-weight: 800;", "Preset Themes" }
            
            Section { title: "Available Themes",
                p { "Dioxus UI includes 7 preset themes ready to use. Click on any theme to apply it:" }
                
                Box { style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px;",
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
                    VStack { gap: SpacingSize::Md,
                        HStack { gap: SpacingSize::Md, style: "flex-wrap: wrap;",
                            Button { variant: ButtonVariant::Primary, "Primary" }
                            Button { variant: ButtonVariant::Secondary, "Secondary" }
                            Button { variant: ButtonVariant::Destructive, "Destructive" }
                            Button { variant: ButtonVariant::Ghost, "Ghost" }
                        }
                        HStack { gap: SpacingSize::Md, style: "flex-wrap: wrap; align-items: center;",
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
            VStack { gap: SpacingSize::Md, {children} }
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
