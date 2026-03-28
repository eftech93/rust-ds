//! Theme system for Dioxus UI
//!
//! Provides comprehensive theme support including:
//! - Design tokens (colors, spacing, typography, shadows)
//! - Light/dark/brand theme modes
//! - Theme context for reactive theme switching
//! - Utility hooks for theme-aware styling

pub mod context;
pub mod tokens;

// Re-export commonly used items
pub use context::{use_style, use_theme, ThemeContext, ThemeProvider, ThemeSelector, ThemeToggle};
pub use tokens::{
    Color, ColorScale, RadiusScale, ShadowScale, SpacingScale, ThemeMode, ThemeTokens, Typography,
    TypographyScale,
};
