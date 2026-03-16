//! Styling system for Dioxus UI
//!
//! Provides a type-safe, fluent API for building CSS styles in pure Rust.
//! Similar to Tailwind CSS but with compile-time safety and full IDE support.

pub mod builder;

pub use builder::Style;

/// Utility macro for conditional class composition
///
/// # Example
/// ```rust,ignore
/// use dioxus_ui_system::cx;
///
/// let is_active = true;
/// let is_disabled = false;
/// let classes = cx!(
///     true => "base-class",
///     is_active => "active",
///     is_disabled => "disabled",
/// );
/// ```
#[macro_export]
macro_rules! cx {
    ($($condition:expr => $class:expr),* $(,)?) => {{
        let mut classes = String::new();
        $(if $condition {
            classes.push_str($class);
            classes.push(' ');
        })*
        classes.trim_end().to_string()
    }};
}

/// Utility macro for conditional styles
///
/// # Example
/// ```rust,ignore
/// use dioxus_ui_system::style_if;
///
/// let is_active = true;
/// let style = style_if!(is_active => "background: blue;");
/// ```
#[macro_export]
macro_rules! style_if {
    ($condition:expr => $style:expr) => {{
        if $condition {
            $style
        } else {
            ""
        }
    }};
}
