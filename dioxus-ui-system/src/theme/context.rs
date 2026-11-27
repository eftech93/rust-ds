//! Theme context for Dioxus
//!
//! Provides a context provider and hooks for accessing and modifying the theme
//! throughout the component tree.

use super::tokens::{ThemeMode, ThemeTokens};
use crate::a11y::use_unique_id;
use dioxus::prelude::*;

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

    /// Build a standalone context (light theme) that is not shared with any
    /// [`ThemeProvider`]. Used as the fallback when no provider is mounted.
    ///
    /// Must be called inside a component scope (it creates signals/callbacks).
    fn standalone() -> Self {
        let mut tokens = Signal::new(ThemeTokens::light());
        Self {
            tokens,
            set_theme: Callback::new(move |t: ThemeTokens| tokens.set(t)),
            toggle_mode: Callback::new(move |()| {
                tokens.with_mut(|current| {
                    *current = match current.mode {
                        super::tokens::ThemeMode::Dark => ThemeTokens::light(),
                        _ => ThemeTokens::dark(),
                    };
                });
            }),
            set_theme_by_name: Callback::new(move |name: String| {
                if let Some(t) = ThemeTokens::by_name(&name) {
                    tokens.set(t);
                }
            }),
        }
    }
}

/// Hook to access the theme context
///
/// If no [`ThemeProvider`] ancestor exists, this does **not** panic: it returns
/// a component-local context initialised with [`ThemeTokens::light`]. Theme
/// changes made through that fallback only affect the calling component, so
/// wrap your app in a `ThemeProvider` for app-wide theming. Use
/// [`try_use_theme`] if you need to know whether a provider is present.
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
    use_hook(|| try_consume_context::<ThemeContext>().unwrap_or_else(ThemeContext::standalone))
}

/// Hook to access the theme context, returning `None` when no
/// [`ThemeProvider`] ancestor exists.
pub fn try_use_theme() -> Option<ThemeContext> {
    use_hook(try_consume_context::<ThemeContext>)
}

/// Hook for computing memoized styles based on theme
///
/// Like [`use_theme`], falls back to the default light theme when no
/// [`ThemeProvider`] is mounted instead of panicking.
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

#[cfg(all(feature = "web", target_arch = "wasm32"))]
const THEME_STORAGE_KEY: &str = "dioxus-ui-theme";

/// Load theme from localStorage (web only)
#[cfg(all(feature = "web", target_arch = "wasm32"))]
fn load_theme_from_storage() -> Option<ThemeTokens> {
    web_sys::window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
        .and_then(|storage| storage.get_item(THEME_STORAGE_KEY).ok())
        .flatten()
        .and_then(|name| ThemeTokens::by_name(&name))
}

/// Save theme to localStorage (web only)
#[cfg(all(feature = "web", target_arch = "wasm32"))]
fn save_theme_to_storage(theme_name: &str) {
    if let Some(storage) = web_sys::window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
    {
        let _ = storage.set_item(THEME_STORAGE_KEY, theme_name);
    }
}

#[cfg(not(all(feature = "web", target_arch = "wasm32")))]
fn load_theme_from_storage() -> Option<ThemeTokens> {
    None
}

#[cfg(not(all(feature = "web", target_arch = "wasm32")))]
fn save_theme_to_storage(_theme_name: &str) {}

/// Theme provider component
///
/// Wraps children with theme context. Must be placed near the root of your app.
/// Optionally persists theme selection to localStorage on web.
///
/// # Properties
/// * `children` - Child elements to render
/// * `initial_theme` - Optional initial theme (defaults to light theme, or loaded from localStorage if persistence enabled)
/// * `persist_theme` - Whether to save/load theme from localStorage (default: false)
///
/// # Example
/// ```rust,ignore
/// use dioxus_ui_system::theme::{ThemeProvider, ThemeTokens};
///
/// fn App() -> Element {
///     rsx! {
///         // With persistence
///         ThemeProvider { persist_theme: true,
///             Home {}
///         }
///         
///         // Without persistence (default)
///         ThemeProvider {
///             Home {}
///         }
///     }
/// }
/// ```
#[component]
pub fn ThemeProvider(
    children: Element,
    #[props(default)] initial_theme: Option<ThemeTokens>,
    #[props(default = false)] persist_theme: bool,
) -> Element {
    // Determine initial theme: explicit prop > localStorage (if enabled) > default light
    let initial = if persist_theme {
        initial_theme
            .or_else(load_theme_from_storage)
            .unwrap_or_else(ThemeTokens::light)
    } else {
        initial_theme.unwrap_or_else(ThemeTokens::light)
    };

    let mut tokens = use_signal(|| initial);
    let persist = use_signal(|| persist_theme);

    let set_theme = Callback::new(move |new_theme: ThemeTokens| {
        // Save theme name to localStorage only if persistence is enabled
        if persist() {
            let theme_name = match &new_theme.mode {
                super::tokens::ThemeMode::Light => "light",
                super::tokens::ThemeMode::Dark => "dark",
                super::tokens::ThemeMode::Brand(name) => name.as_str(),
            };
            save_theme_to_storage(theme_name);
        }
        tokens.set(new_theme);
    });

    let toggle_mode = Callback::new(move |()| {
        tokens.with_mut(|current| {
            let new_theme = match current.mode {
                super::tokens::ThemeMode::Light => ThemeTokens::dark(),
                super::tokens::ThemeMode::Dark => ThemeTokens::light(),
                super::tokens::ThemeMode::Brand(_) => ThemeTokens::light(),
            };
            // Save to localStorage only if persistence is enabled
            if persist() {
                let theme_name = match &new_theme.mode {
                    super::tokens::ThemeMode::Light => "light",
                    super::tokens::ThemeMode::Dark => "dark",
                    super::tokens::ThemeMode::Brand(name) => name.as_str(),
                };
                save_theme_to_storage(theme_name);
            }
            *current = new_theme;
        });
    });

    let set_theme_by_name = Callback::new(move |name: String| {
        if let Some(new_theme) = ThemeTokens::by_name(&name) {
            if persist() {
                save_theme_to_storage(&name);
            }
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

/// Accessible label for [`ThemeToggle`], describing what pressing it does.
fn toggle_action_label(mode: &ThemeMode) -> &'static str {
    match mode {
        ThemeMode::Light => "Switch to dark theme",
        ThemeMode::Dark | ThemeMode::Brand(_) => "Switch to light theme",
    }
}

/// Theme toggle button component
///
/// A convenience component that toggles between light and dark modes
///
/// # Accessibility
/// The icon-only button has an `aria-label` describing the action
/// (e.g. "Switch to dark theme"); the icon itself is `aria-hidden`.
#[component]
pub fn ThemeToggle() -> Element {
    let theme = use_theme();
    let mode = use_style(|t| t.mode.clone());

    let button_text = match mode() {
        super::tokens::ThemeMode::Light => "🌙",
        super::tokens::ThemeMode::Dark => "☀️",
        super::tokens::ThemeMode::Brand(_) => "🎨",
    };
    let label = toggle_action_label(&mode());

    rsx! {
        button {
            r#type: "button",
            "aria-label": "{label}",
            title: "{label}",
            onclick: move |_| theme.toggle_mode.call(()),
            span { "aria-hidden": "true", "{button_text}" }
        }
    }
}

/// Theme selector dropdown component
///
/// Allows users to select from all available preset themes
///
/// # Accessibility
/// The trigger has `aria-haspopup="listbox"`, `aria-expanded` and
/// `aria-controls`; the popup is a `role="listbox"` whose items are
/// `role="option"` with `aria-selected` marking the active theme. Escape
/// closes the popup.
#[component]
pub fn ThemeSelector() -> Element {
    let theme = use_theme();
    let mut is_open = use_signal(|| false);
    let listbox_id = use_unique_id("theme-selector-listbox");
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
            onkeydown: move |e: KeyboardEvent| {
                if e.key() == Key::Escape && is_open() {
                    e.prevent_default();
                    is_open.set(false);
                }
            },

            // Trigger button
            button {
                r#type: "button",
                "aria-haspopup": "listbox",
                "aria-expanded": "{is_open()}",
                "aria-controls": "{listbox_id}",
                "aria-label": "Theme: {current_name}",
                style: "display: flex; align-items: center; gap: 8px; padding: 8px 12px; border-radius: 6px; border: 1px solid #e2e8f0; background: white; cursor: pointer;",
                onclick: move |_| is_open.toggle(),

                span { "aria-hidden": "true", "🎨" }
                span { "{current_name}" }
                span { "aria-hidden": "true", "▼" }
            }

            // Dropdown
            if is_open() {
                div {
                    id: "{listbox_id}",
                    role: "listbox",
                    "aria-label": "Themes",
                    style: "position: absolute; top: calc(100% + 4px); right: 0; min-width: 150px; background: white; border-radius: 8px; border: 1px solid #e2e8f0; box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1); z-index: 100;",

                    for (name, _) in presets {
                        button {
                            r#type: "button",
                            role: "option",
                            "aria-selected": "{current_name.to_lowercase() == name}",
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

    #[test]
    fn test_theme_context_creation() {
        // Note: This is a basic test - full testing requires dioxus testing utilities
        let tokens = ThemeTokens::light();
        assert!(matches!(tokens.mode, ThemeMode::Light));
    }

    fn render(app: fn() -> Element) -> String {
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
        dioxus_ssr::render(&dom)
    }

    #[test]
    fn use_theme_falls_back_without_provider() {
        fn app() -> Element {
            let theme = use_theme();
            let bg = theme.tokens.read().colors.background.to_rgba();
            let has_provider = try_use_theme().is_some();
            rsx! { div { "data-provider": "{has_provider}", style: "background:{bg}" } }
        }
        let html = render(app);
        let expected = ThemeTokens::light().colors.background.to_rgba();
        assert!(html.contains("data-provider=\"false\""), "{html}");
        assert!(html.contains(&expected), "{html}");
    }

    #[test]
    fn use_style_falls_back_without_provider() {
        fn app() -> Element {
            let mode = use_style(|t| format!("{:?}", t.mode));
            rsx! { span { "{mode}" } }
        }
        assert!(render(app).contains("Light"));
    }

    #[test]
    fn toggle_label_describes_action() {
        assert_eq!(
            toggle_action_label(&ThemeMode::Light),
            "Switch to dark theme"
        );
        assert_eq!(
            toggle_action_label(&ThemeMode::Dark),
            "Switch to light theme"
        );
        assert_eq!(
            toggle_action_label(&ThemeMode::Brand("rose".into())),
            "Switch to light theme"
        );
    }

    #[test]
    fn theme_toggle_has_action_label() {
        let html = render(|| rsx! { ThemeToggle {} });
        assert!(
            html.contains("aria-label=\"Switch to dark theme\""),
            "{html}"
        );
        assert!(html.contains("aria-hidden=\"true\""), "{html}");

        fn dark() -> Element {
            rsx! { ThemeProvider { initial_theme: ThemeTokens::dark(), ThemeToggle {} } }
        }
        let html = render(dark);
        assert!(
            html.contains("aria-label=\"Switch to light theme\""),
            "{html}"
        );
    }

    #[test]
    fn theme_selector_trigger_announces_listbox() {
        let html = render(|| rsx! { ThemeSelector {} });
        assert!(html.contains("aria-haspopup=\"listbox\""), "{html}");
        assert!(html.contains("aria-expanded=\"false\""), "{html}");
        assert!(
            html.contains("aria-controls=\"theme-selector-listbox-"),
            "{html}"
        );
        assert!(html.contains("aria-label=\"Theme: Light\""), "{html}");
        assert!(!html.contains("role=\"listbox\""), "{html}");
    }

    #[test]
    fn try_use_theme_sees_provider() {
        fn inner() -> Element {
            let found = try_use_theme().is_some();
            rsx! { span { "found={found}" } }
        }
        fn app() -> Element {
            rsx! { ThemeProvider { initial_theme: ThemeTokens::dark(), Inner {} } }
        }
        #[allow(non_snake_case)]
        fn Inner() -> Element {
            inner()
        }
        assert!(render(app).contains("found=true"));
    }
}
