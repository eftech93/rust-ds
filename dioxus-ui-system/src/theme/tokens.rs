//! Design tokens for the UI system
//!
//! This module provides type-safe design tokens including colors, spacing,
//! typography, and other visual properties that define the theme.

use serde::{Deserialize, Serialize};

/// The main theme tokens structure containing all design tokens
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThemeTokens {
    /// Color palette
    pub colors: ColorScale,
    /// Spacing scale (margins, paddings, gaps)
    pub spacing: SpacingScale,
    /// Border radius scale
    pub radius: RadiusScale,
    /// Typography scale (font sizes, weights, etc.)
    pub typography: TypographyScale,
    /// Box shadows
    pub shadows: ShadowScale,
    /// Current theme mode
    pub mode: ThemeMode,
}

/// Theme mode variants supporting light, dark, and custom brand themes
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ThemeMode {
    /// Light mode
    Light,
    /// Dark mode
    Dark,
    /// Custom brand theme with name
    Brand(String),
}

impl Default for ThemeMode {
    fn default() -> Self {
        ThemeMode::Light
    }
}

/// Color scale with semantic color names
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColorScale {
    /// Primary brand color
    pub primary: Color,
    /// Text color on primary background
    pub primary_foreground: Color,
    /// Secondary accent color
    pub secondary: Color,
    /// Text color on secondary background
    pub secondary_foreground: Color,
    /// Page/element background color
    pub background: Color,
    /// Main text color
    pub foreground: Color,
    /// Muted/subtle background
    pub muted: Color,
    /// Muted text color
    pub muted_foreground: Color,
    /// Border color
    pub border: Color,
    /// Error/destructive color
    pub destructive: Color,
    /// Success color
    pub success: Color,
    /// Warning color
    pub warning: Color,
    /// Accent color for highlights
    pub accent: Color,
    /// Text on accent
    pub accent_foreground: Color,
    /// Card/elevated surface background
    pub card: Color,
    /// Text on card
    pub card_foreground: Color,
    /// Popover/dropdown background
    pub popover: Color,
    /// Text on popover
    pub popover_foreground: Color,
    /// Disabled state
    pub disabled: Color,
    /// Ring/focus indicator
    pub ring: Color,
}

/// RGBA color representation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Color {
    /// Red channel (0-255)
    pub r: u8,
    /// Green channel (0-255)
    pub g: u8,
    /// Blue channel (0-255)
    pub b: u8,
    /// Alpha channel (0.0-1.0)
    pub a: f32,
}

impl Color {
    /// Create a new color with RGB values (alpha = 1.0)
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Create a new color with RGBA values
    pub const fn new_rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Convert to CSS rgba() string
    pub fn to_rgba(&self) -> String {
        format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }

    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        if self.a < 1.0 {
            format!(
                "#{:02x}{:02x}{:02x}{:02x}",
                self.r,
                self.g,
                self.b,
                (self.a * 255.0) as u8
            )
        } else {
            format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
        }
    }

    /// Darken the color by a given amount (0.0-1.0)
    pub fn darken(&self, amount: f32) -> Color {
        let factor = 1.0 - amount.clamp(0.0, 1.0);
        Color {
            r: ((self.r as f32) * factor).clamp(0.0, 255.0) as u8,
            g: ((self.g as f32) * factor).clamp(0.0, 255.0) as u8,
            b: ((self.b as f32) * factor).clamp(0.0, 255.0) as u8,
            a: self.a,
        }
    }

    /// Lighten the color by a given amount (0.0-1.0)
    pub fn lighten(&self, amount: f32) -> Color {
        let factor = amount.clamp(0.0, 1.0);
        Color {
            r: ((self.r as f32) + (255.0 - self.r as f32) * factor).clamp(0.0, 255.0) as u8,
            g: ((self.g as f32) + (255.0 - self.g as f32) * factor).clamp(0.0, 255.0) as u8,
            b: ((self.b as f32) + (255.0 - self.b as f32) * factor).clamp(0.0, 255.0) as u8,
            a: self.a,
        }
    }

    /// Blend with another color (0.0 = self, 1.0 = other)
    pub fn blend(&self, other: &Color, ratio: f32) -> Color {
        let r = ratio.clamp(0.0, 1.0);
        Color {
            r: ((self.r as f32) * (1.0 - r) + (other.r as f32) * r) as u8,
            g: ((self.g as f32) * (1.0 - r) + (other.g as f32) * r) as u8,
            b: ((self.b as f32) * (1.0 - r) + (other.b as f32) * r) as u8,
            a: self.a * (1.0 - r) + other.a * r,
        }
    }

    /// Convert to RGBA tuple
    pub fn to_rgba_tuple(&self) -> (u8, u8, u8, f32) {
        (self.r, self.g, self.b, self.a)
    }
}

/// Spacing scale for margins, paddings, and gaps
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpacingScale {
    pub xs: u16,  // 4px
    pub sm: u16,  // 8px
    pub md: u16,  // 16px
    pub lg: u16,  // 24px
    pub xl: u16,  // 32px
    pub xxl: u16, // 48px
}

impl SpacingScale {
    /// Get spacing value by name
    pub fn get(&self, size: &str) -> u16 {
        match size {
            "xs" => self.xs,
            "sm" => self.sm,
            "md" => self.md,
            "lg" => self.lg,
            "xl" => self.xl,
            "xxl" => self.xxl,
            _ => self.md,
        }
    }
}

/// Border radius scale
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RadiusScale {
    pub none: u16,
    pub sm: u16,
    pub md: u16,
    pub lg: u16,
    pub xl: u16,
    pub full: u16, // 9999 for circles/pills
}

impl RadiusScale {
    /// Get radius value by name
    pub fn get(&self, size: &str) -> u16 {
        match size {
            "none" => self.none,
            "sm" => self.sm,
            "md" => self.md,
            "lg" => self.lg,
            "xl" => self.xl,
            "full" => self.full,
            _ => self.md,
        }
    }
}

/// Typography scale
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TypographyScale {
    pub xs: Typography,
    pub sm: Typography,
    pub base: Typography,
    pub lg: Typography,
    pub xl: Typography,
    pub xxl: Typography,
    pub h1: Typography,
    pub h2: Typography,
    pub h3: Typography,
    pub h4: Typography,
}

/// Individual typography specification
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Typography {
    /// Font size in pixels
    pub size: u16,
    /// Line height as multiplier
    pub line_height: f32,
    /// Font weight (400 = normal, 700 = bold)
    pub weight: u16,
    /// Font family stack
    pub family: String,
    /// Letter spacing in pixels
    pub letter_spacing: Option<f32>,
}

impl TypographyScale {
    /// Get typography by name
    pub fn get(&self, size: &str) -> &Typography {
        match size {
            "xs" => &self.xs,
            "sm" => &self.sm,
            "base" => &self.base,
            "lg" => &self.lg,
            "xl" => &self.xl,
            "xxl" => &self.xxl,
            "h1" => &self.h1,
            "h2" => &self.h2,
            "h3" => &self.h3,
            "h4" => &self.h4,
            _ => &self.base,
        }
    }
}

/// Box shadow scale
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShadowScale {
    pub none: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
    pub inner: String,
}

impl ShadowScale {
    /// Get shadow by name
    pub fn get(&self, size: &str) -> &String {
        match size {
            "none" => &self.none,
            "sm" => &self.sm,
            "md" => &self.md,
            "lg" => &self.lg,
            "xl" => &self.xl,
            "inner" => &self.inner,
            _ => &self.md,
        }
    }

    /// Create a colored shadow based on a color
    pub fn colored(&self, size: &str, color: &Color) -> String {
        let base = self.get(size);
        // Replace the rgba values in the shadow with the color
        format!("{}; box-shadow-color: {}", base, color.to_rgba())
    }
}

impl Default for ShadowScale {
    fn default() -> Self {
        Self {
            none: String::new(),
            sm: "0 1px 2px 0 rgba(0, 0, 0, 0.05)".into(),
            md: "0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)".into(),
            lg: "0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)".into(),
            xl: "0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04)".into(),
            inner: "inset 0 2px 4px 0 rgba(0, 0, 0, 0.06)".into(),
        }
    }
}

// Preset themes implementation
impl ThemeTokens {
    /// Create the default light theme
    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,
            colors: ColorScale {
                primary: Color::new(15, 23, 42),
                primary_foreground: Color::new(248, 250, 252),
                secondary: Color::new(241, 245, 249),
                secondary_foreground: Color::new(15, 23, 42),
                background: Color::new(255, 255, 255),
                foreground: Color::new(15, 23, 42),
                muted: Color::new(248, 250, 252),
                muted_foreground: Color::new(100, 116, 139),
                border: Color::new(226, 232, 240),
                destructive: Color::new(239, 68, 68),
                success: Color::new(34, 197, 94),
                warning: Color::new(234, 179, 8),
                accent: Color::new(241, 245, 249),
                accent_foreground: Color::new(15, 23, 42),
                card: Color::new(255, 255, 255),
                card_foreground: Color::new(15, 23, 42),
                popover: Color::new(255, 255, 255),
                popover_foreground: Color::new(15, 23, 42),
                disabled: Color::new(241, 245, 249),
                ring: Color::new(15, 23, 42),
            },
            spacing: SpacingScale {
                xs: 4,
                sm: 8,
                md: 16,
                lg: 24,
                xl: 32,
                xxl: 48,
            },
            radius: RadiusScale {
                none: 0,
                sm: 4,
                md: 8,
                lg: 12,
                xl: 16,
                full: 9999,
            },
            typography: TypographyScale {
                xs: Typography {
                    size: 12,
                    line_height: 1.0,
                    weight: 400,
                    family: "system-ui, -apple-system, sans-serif".into(),
                    letter_spacing: Some(0.01),
                },
                sm: Typography {
                    size: 14,
                    line_height: 1.25,
                    weight: 400,
                    family: "system-ui, -apple-system, sans-serif".into(),
                    letter_spacing: None,
                },
                base: Typography {
                    size: 16,
                    line_height: 1.5,
                    weight: 400,
                    family: "system-ui, -apple-system, sans-serif".into(),
                    letter_spacing: None,
                },
                lg: Typography {
                    size: 18,
                    line_height: 1.75,
                    weight: 400,
                    family: "system-ui, -apple-system, sans-serif".into(),
                    letter_spacing: None,
                },
                xl: Typography {
                    size: 20,
                    line_height: 1.75,
                    weight: 600,
                    family: "system-ui, -apple-system, sans-serif".into(),
                    letter_spacing: None,
                },
                xxl: Typography {
                    size: 24,
                    line_height: 2.0,
                    weight: 600,
                    family: "system-ui, -apple-system, sans-serif".into(),
                    letter_spacing: None,
                },
                h1: Typography {
                    size: 36,
                    line_height: 1.1,
                    weight: 700,
                    family: "system-ui, -apple-system, sans-serif".into(),
                    letter_spacing: Some(-0.02),
                },
                h2: Typography {
                    size: 30,
                    line_height: 1.2,
                    weight: 700,
                    family: "system-ui, -apple-system, sans-serif".into(),
                    letter_spacing: Some(-0.02),
                },
                h3: Typography {
                    size: 24,
                    line_height: 1.3,
                    weight: 600,
                    family: "system-ui, -apple-system, sans-serif".into(),
                    letter_spacing: None,
                },
                h4: Typography {
                    size: 20,
                    line_height: 1.4,
                    weight: 600,
                    family: "system-ui, -apple-system, sans-serif".into(),
                    letter_spacing: None,
                },
            },
            shadows: ShadowScale::default(),
        }
    }

    /// Create the dark theme
    pub fn dark() -> Self {
        let mut dark = Self::light();
        dark.mode = ThemeMode::Dark;
        dark.colors.background = Color::new(15, 23, 42);
        dark.colors.foreground = Color::new(248, 250, 252);
        dark.colors.muted = Color::new(30, 41, 59);
        dark.colors.muted_foreground = Color::new(148, 163, 184);
        dark.colors.border = Color::new(51, 65, 85);
        dark.colors.primary = Color::new(248, 250, 252);
        dark.colors.primary_foreground = Color::new(15, 23, 42);
        dark.colors.secondary = Color::new(30, 41, 59);
        dark.colors.secondary_foreground = Color::new(248, 250, 252);
        dark.colors.accent = Color::new(30, 41, 59);
        dark.colors.accent_foreground = Color::new(248, 250, 252);
        dark.colors.card = Color::new(15, 23, 42);
        dark.colors.card_foreground = Color::new(248, 250, 252);
        dark.colors.popover = Color::new(15, 23, 42);
        dark.colors.popover_foreground = Color::new(248, 250, 252);
        dark.colors.disabled = Color::new(30, 41, 59);
        dark.colors.ring = Color::new(248, 250, 252);
        dark
    }

    /// Create a brand theme with custom primary color
    pub fn brand(primary: Color, name: &str) -> Self {
        let mut brand = Self::light();
        brand.mode = ThemeMode::Brand(name.into());
        brand.colors.primary = primary.clone();
        brand.colors.primary_foreground = if is_dark_color(&primary) {
            Color::new(255, 255, 255)
        } else {
            Color::new(0, 0, 0)
        };
        brand.colors.ring = primary;
        brand
    }
}

/// Determine if a color is dark (useful for choosing contrasting text)
fn is_dark_color(color: &Color) -> bool {
    let luminance =
        (0.299 * color.r as f32 + 0.587 * color.g as f32 + 0.114 * color.b as f32) / 255.0;
    luminance < 0.5
}

// Preset theme implementations
impl ThemeTokens {
    /// Rose theme - romantic pink/red tones
    pub fn rose() -> Self {
        let mut rose = Self::light();
        rose.mode = ThemeMode::Brand("rose".into());
        rose.colors.primary = Color::new(225, 29, 72); // Rose 600
        rose.colors.primary_foreground = Color::new(255, 255, 255);
        rose.colors.ring = Color::new(225, 29, 72);
        rose.colors.accent = Color::new(255, 228, 230); // Rose 100
        rose.colors.accent_foreground = Color::new(136, 19, 55); // Rose 800
        rose
    }

    /// Blue theme - cool blue tones
    pub fn blue() -> Self {
        let mut blue = Self::light();
        blue.mode = ThemeMode::Brand("blue".into());
        blue.colors.primary = Color::new(37, 99, 235); // Blue 600
        blue.colors.primary_foreground = Color::new(255, 255, 255);
        blue.colors.ring = Color::new(37, 99, 235);
        blue.colors.accent = Color::new(219, 234, 254); // Blue 100
        blue.colors.accent_foreground = Color::new(30, 58, 138); // Blue 800
        blue
    }

    /// Green theme - nature green tones
    pub fn green() -> Self {
        let mut green = Self::light();
        green.mode = ThemeMode::Brand("green".into());
        green.colors.primary = Color::new(22, 163, 74); // Green 600
        green.colors.primary_foreground = Color::new(255, 255, 255);
        green.colors.ring = Color::new(22, 163, 74);
        green.colors.accent = Color::new(220, 252, 231); // Green 100
        green.colors.accent_foreground = Color::new(20, 83, 45); // Green 800
        green
    }

    /// Violet theme - purple tones
    pub fn violet() -> Self {
        let mut violet = Self::light();
        violet.mode = ThemeMode::Brand("violet".into());
        violet.colors.primary = Color::new(124, 58, 237); // Violet 600
        violet.colors.primary_foreground = Color::new(255, 255, 255);
        violet.colors.ring = Color::new(124, 58, 237);
        violet.colors.accent = Color::new(237, 233, 254); // Violet 100
        violet.colors.accent_foreground = Color::new(91, 33, 182); // Violet 800
        violet
    }

    /// Orange theme - warm orange tones
    pub fn orange() -> Self {
        let mut orange = Self::light();
        orange.mode = ThemeMode::Brand("orange".into());
        orange.colors.primary = Color::new(234, 88, 12); // Orange 600
        orange.colors.primary_foreground = Color::new(255, 255, 255);
        orange.colors.ring = Color::new(234, 88, 12);
        orange.colors.accent = Color::new(255, 237, 213); // Orange 100
        orange.colors.accent_foreground = Color::new(154, 52, 18); // Orange 800
        orange
    }

    /// Get all available preset themes
    pub fn presets() -> Vec<(&'static str, ThemeTokens)> {
        vec![
            ("light", Self::light()),
            ("dark", Self::dark()),
            ("rose", Self::rose()),
            ("blue", Self::blue()),
            ("green", Self::green()),
            ("violet", Self::violet()),
            ("orange", Self::orange()),
        ]
    }

    /// Get theme by name
    pub fn by_name(name: &str) -> Option<Self> {
        match name {
            "light" => Some(Self::light()),
            "dark" => Some(Self::dark()),
            "rose" => Some(Self::rose()),
            "blue" => Some(Self::blue()),
            "green" => Some(Self::green()),
            "violet" => Some(Self::violet()),
            "orange" => Some(Self::orange()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_darken() {
        let white = Color::new(255, 255, 255);
        let darkened = white.darken(0.5);
        assert_eq!(darkened.r, 127);
        assert_eq!(darkened.g, 127);
        assert_eq!(darkened.b, 127);
    }

    #[test]
    fn test_color_lighten() {
        let black = Color::new(0, 0, 0);
        let lightened = black.lighten(0.5);
        assert_eq!(lightened.r, 127);
        assert_eq!(lightened.g, 127);
        assert_eq!(lightened.b, 127);
    }

    #[test]
    fn test_color_blend() {
        let red = Color::new(255, 0, 0);
        let blue = Color::new(0, 0, 255);
        let blended = red.blend(&blue, 0.5);
        assert_eq!(blended.r, 127);
        assert_eq!(blended.g, 0);
        assert_eq!(blended.b, 127);
    }

    #[test]
    fn test_is_dark_color() {
        assert!(is_dark_color(&Color::new(0, 0, 0)));
        assert!(!is_dark_color(&Color::new(255, 255, 255)));
    }

    #[test]
    fn test_preset_themes() {
        let presets = ThemeTokens::presets();
        assert_eq!(presets.len(), 7);
        assert!(ThemeTokens::by_name("light").is_some());
        assert!(ThemeTokens::by_name("dark").is_some());
        assert!(ThemeTokens::by_name("rose").is_some());
        assert!(ThemeTokens::by_name("unknown").is_none());
    }
}
