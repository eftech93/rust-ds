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
    /// Callback to set theme by name (preset)
    pub set_theme_by_name: Callback<String>,
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

    let set_theme_by_name = Callback::new(move |name: String| {
        if let Some(new_theme) = ThemeTokens::by_name(&name) {
            tokens.set(new_theme);
        }
    });

    use_context_provider(|| ThemeContext {
        tokens,
        set_theme,
        toggle_mode,
        set_theme_by_name,
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

/// Theme selector dropdown component
///
/// Allows users to select from all available preset themes
#[component]
pub fn ThemeSelector() -> Element {
    let theme = use_theme();
    let mut is_open = use_signal(|| false);
    let current_mode = use_style(|t| t.mode.clone());
    
    let presets = ThemeTokens::presets();
    
    let current_name = match current_mode() {
        super::tokens::ThemeMode::Light => "Light",
        super::tokens::ThemeMode::Dark => "Dark",
        super::tokens::ThemeMode::Brand(name) => match name.as_str() {
            "rose" => "Rose",
            "blue" => "Blue",
            "green" => "Green",
            "violet" => "Violet",
            "orange" => "Orange",
            _ => "Custom",
        },
    };

    rsx! {
        div {
            style: "position: relative; display: inline-block;",
            
            // Trigger button
            button {
                style: "display: flex; align-items: center; gap: 8px; padding: 8px 12px; border-radius: 6px; border: 1px solid #e2e8f0; background: white; cursor: pointer;",
                onclick: move |_| is_open.toggle(),
                
                span { "🎨" }
                span { "{current_name}" }
                span { "▼" }
            }
            
            // Dropdown
            if is_open() {
                div {
                    style: "position: absolute; top: calc(100% + 4px); right: 0; min-width: 150px; background: white; border-radius: 8px; border: 1px solid #e2e8f0; box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1); z-index: 100;",
                    
                    for (name, _) in presets {
                        button {
                            style: "display: block; width: 100%; padding: 8px 12px; text-align: left; background: none; border: none; cursor: pointer; border-radius: 6px; margin: 2px;",
                            style: if current_name.to_lowercase() == name { "background: #f1f5f9;" } else { "" },
                            onclick: move |_| {
                                theme.set_theme_by_name.call(name.to_string());
                                is_open.set(false);
                            },
                            "{name.chars().next().unwrap().to_uppercase()}{&name[1..]}"
                        }
                    }
                }
            }
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
