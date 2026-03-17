//! Theme system for Dioxus UI
//!
//! Provides comprehensive theme support including:
//! - Design tokens (colors, spacing, typography, shadows)
//! - Light/dark/brand theme modes
//! - Theme context for reactive theme switching
//! - Utility hooks for theme-aware styling

pub mod tokens;
pub mod context;

// Re-export commonly used items
pub use tokens::{
    ThemeTokens, ThemeMode, ColorScale, Color, SpacingScale, 
    RadiusScale, TypographyScale, Typography, ShadowScale
};
pub use context::{ThemeContext, ThemeProvider, use_theme, use_style, ThemeToggle, ThemeSelector};
