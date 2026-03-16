//! Theme context for Dioxus
//!
//! Provides a context provider and hooks for accessing and modifying the theme
//! throughout the component tree.

use dioxus::prelude::*;
use super::tokens::ThemeTokens;

/// Theme context providing access to current theme and theme switching
#[derive(Clone)]
pub struct ThemeContext {
    /// Current theme tokens (reactive signal)
    pub tokens: Signal<ThemeTokens>,
    /// Callback to set a new theme
    pub set_theme: Callback<ThemeTokens>,
    /// Callback to toggle between light and dark modes
    pub toggle_mode: Callback<()>,
}

impl ThemeContext {
    /// Get the current theme tokens value
    pub fn current(&self) -> ThemeTokens {
        self.tokens.read().clone()
    }
}

/// Hook to access the theme context
///
/// # Panics
/// Panics if called outside of a ThemeProvider
///
/// # Example
/// ```rust,ignore
/// use dioxus_ui_system::theme::use_theme;
///
/// fn MyComponent() -> Element {
///     let theme = use_theme();
///     let bg_color = theme.tokens.read().colors.background.to_rgba();
///     
///     rsx! {
///         div { style: "background-color: {bg_color}", "Hello" }
///     }
/// }
/// ```
pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>()
}

/// Hook for computing memoized styles based on theme
///
/// # Type Parameters
/// * `F` - Function type that computes a value from theme tokens
/// * `R` - Return type (must be PartialEq for memoization)
///
/// # Example
/// ```rust,ignore
/// use dioxus_ui_system::theme::use_style;
///
/// fn MyComponent() -> Element {
///     let bg_color = use_style(|tokens| tokens.colors.background.to_rgba());
///     
///     rsx! {
///         div { style: "background-color: {bg_color}", "Hello" }
///     }
/// }
/// ```
pub fn use_style<F, R>(f: F) -> Memo<R>
where
    F: Fn(&ThemeTokens) -> R + 'static,
    R: PartialEq + 'static,
{
    let theme = use_theme();
    use_memo(move || f(&theme.tokens.read()))
}

/// Theme provider component
///
/// Wraps children with theme context. Must be placed near the root of your app.
///
/// # Properties
/// * `children` - Child elements to render
/// * `initial_theme` - Optional initial theme (defaults to light theme)
///
/// # Example
/// ```rust,ignore
/// use dioxus_ui_system::theme::{ThemeProvider, ThemeTokens};
///
/// fn App() -> Element {
///     rsx! {
///         ThemeProvider {
///             Home {}
///         }
///     }
/// }
/// ```
#[component]
pub fn ThemeProvider(
    children: Element,
    #[props(default)]
    initial_theme: Option<ThemeTokens>,
) -> Element {
    let initial = initial_theme.unwrap_or_else(ThemeTokens::light);
    let mut tokens = use_signal(|| initial);

    let set_theme = Callback::new(move |new_theme: ThemeTokens| {
        tokens.set(new_theme);
    });

    let toggle_mode = Callback::new(move |()| {
        tokens.with_mut(|current| {
            let new_theme = match current.mode {
                super::tokens::ThemeMode::Light => ThemeTokens::dark(),
                super::tokens::ThemeMode::Dark => ThemeTokens::light(),
                super::tokens::ThemeMode::Brand(_) => ThemeTokens::light(),
            };
            *current = new_theme;
        });
    });

    use_context_provider(|| ThemeContext {
        tokens,
        set_theme,
        toggle_mode,
    });

    rsx! { {children} }
}

/// Theme toggle button component
///
/// A convenience component that toggles between light and dark modes
#[component]
pub fn ThemeToggle() -> Element {
    let theme = use_theme();
    let mode = use_style(|t| t.mode.clone());

    let button_text = match mode() {
        super::tokens::ThemeMode::Light => "🌙",
        super::tokens::ThemeMode::Dark => "☀️",
        super::tokens::ThemeMode::Brand(_) => "🎨",
    };

    rsx! {
        button {
            onclick: move |_| theme.toggle_mode.call(()),
            "{button_text}"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::tokens::ThemeMode;

    #[test]
    fn test_theme_context_creation() {
        // Note: This is a basic test - full testing requires dioxus testing utilities
        let tokens = ThemeTokens::light();
        assert!(matches!(tokens.mode, ThemeMode::Light));
    }
}
