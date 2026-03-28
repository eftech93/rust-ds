//! Toggle atom component
//!
//! A two-state button that can be either on or off. Used for features like
//! bold/italic in text editors, or any feature that requires a simple on/off state.
//! Similar to a checkbox but styled as a button.

use crate::styles::Style;
use crate::theme::tokens::Color;
use crate::theme::{use_style, use_theme};
use dioxus::prelude::*;

/// Toggle variant styles
#[derive(Default, Clone, PartialEq, Debug)]
pub enum ToggleVariant {
    /// Default filled style
    #[default]
    Default,
    /// Outlined style with border
    Outline,
    /// Subtle ghost style
    Ghost,
}

/// Toggle sizes
#[derive(Default, Clone, PartialEq, Debug)]
pub enum ToggleSize {
    /// Small toggle
    Sm,
    /// Medium (default) toggle
    #[default]
    Md,
    /// Large toggle
    Lg,
}

/// Toggle properties
#[derive(Props, Clone, PartialEq)]
pub struct ToggleProps {
    /// Toggle content
    pub children: Element,
    /// Whether the toggle is pressed/activated
    #[props(default)]
    pub pressed: bool,
    /// Callback when pressed state changes
    #[props(default)]
    pub on_pressed_change: Option<EventHandler<bool>>,
    /// Visual variant
    #[props(default)]
    pub variant: ToggleVariant,
    /// Toggle size
    #[props(default)]
    pub size: ToggleSize,
    /// Disabled state
    #[props(default)]
    pub disabled: bool,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Toggle atom component
///
/// A two-state button that can be either pressed (on) or not pressed (off).
///
/// # Example
/// ```rust,ignore
/// use dioxus::prelude::*;
/// use dioxus_ui_system::atoms::{Toggle, ToggleVariant, ToggleSize};
///
/// rsx! {
///     Toggle {
///         pressed: true,
///         on_pressed_change: move |pressed| println!("Pressed: {}", pressed),
///         variant: ToggleVariant::Outline,
///         size: ToggleSize::Md,
///         "Bold"
///     }
/// }
/// ```
#[component]
pub fn Toggle(props: ToggleProps) -> Element {
    let _theme = use_theme();

    let mut is_pressed = use_signal(|| props.pressed);
    let mut is_hovered = use_signal(|| false);
    let mut is_focused = use_signal(|| false);

    // Sync with prop changes
    use_effect(move || {
        is_pressed.set(props.pressed);
    });

    let pressed = is_pressed();
    let disabled = props.disabled;
    let variant = props.variant.clone();
    let size = props.size.clone();

    // Memoized styles
    let style = use_style(move |t| {
        let base = Style::new()
            .inline_flex()
            .items_center()
            .justify_center()
            .gap(&t.spacing, "sm")
            .rounded(&t.radius, "md")
            .text(&t.typography, "sm")
            .font_weight(500)
            .line_height(1.0)
            .transition("all 150ms cubic-bezier(0.4, 0, 0.2, 1)")
            .select_none()
            .whitespace_nowrap()
            .cursor(if disabled { "not-allowed" } else { "pointer" });

        // Apply opacity for disabled state
        let base = if disabled {
            base.opacity(0.5)
        } else {
            base.opacity(1.0)
        };

        // Size styles
        let sized = match size {
            ToggleSize::Sm => base.px(&t.spacing, "sm").h_px(32),
            ToggleSize::Md => base.px(&t.spacing, "md").h_px(40),
            ToggleSize::Lg => base.px(&t.spacing, "lg").h_px(48),
        };

        // Variant styles based on pressed state
        let (bg, fg, border) = match variant {
            ToggleVariant::Default => {
                if pressed {
                    let bg = if is_hovered() && !disabled {
                        t.colors.primary.darken(0.1)
                    } else {
                        t.colors.primary.clone()
                    };
                    (bg, t.colors.primary_foreground.clone(), None)
                } else {
                    let bg = if is_hovered() && !disabled {
                        t.colors.muted.darken(0.05)
                    } else {
                        t.colors.muted.clone()
                    };
                    (bg, t.colors.muted_foreground.clone(), None)
                }
            }
            ToggleVariant::Outline => {
                if pressed {
                    let bg = if is_hovered() && !disabled {
                        t.colors.accent.darken(0.05)
                    } else {
                        t.colors.accent.clone()
                    };
                    let border_color = if is_hovered() && !disabled {
                        t.colors.primary.darken(0.1)
                    } else {
                        t.colors.primary.clone()
                    };
                    (bg, t.colors.foreground.clone(), Some(border_color))
                } else {
                    let bg = if is_hovered() && !disabled {
                        t.colors.accent.clone()
                    } else {
                        Color::new_rgba(0, 0, 0, 0.0)
                    };
                    (
                        bg,
                        t.colors.foreground.clone(),
                        Some(t.colors.border.clone()),
                    )
                }
            }
            ToggleVariant::Ghost => {
                if pressed {
                    let bg = if is_hovered() && !disabled {
                        t.colors.accent.darken(0.05)
                    } else {
                        t.colors.accent.clone()
                    };
                    (bg, t.colors.foreground.clone(), None)
                } else {
                    let bg = if is_hovered() && !disabled {
                        t.colors.accent.clone()
                    } else {
                        Color::new_rgba(0, 0, 0, 0.0)
                    };
                    (bg, t.colors.muted_foreground.clone(), None)
                }
            }
        };

        let mut final_style = sized.bg(&bg).text_color(&fg);

        // Add border for outline variant
        if let Some(border_color) = border {
            final_style = final_style.border(1, &border_color);
        }

        // Add focus ring
        if is_focused() && !disabled {
            final_style = Style {
                box_shadow: Some(format!("0 0 0 2px {}", t.colors.ring.to_rgba())),
                ..final_style
            };
        }

        final_style.build()
    });

    // Combine with custom styles
    let final_style = if let Some(custom) = &props.style {
        format!("{} {}", style(), custom)
    } else {
        style()
    };

    let class = props.class.clone().unwrap_or_default();

    let handle_click = move |_| {
        if !disabled {
            let new_pressed = !is_pressed();
            is_pressed.set(new_pressed);
            if let Some(handler) = &props.on_pressed_change {
                handler.call(new_pressed);
            }
        }
    };

    rsx! {
        button {
            r#type: "button",
            role: "switch",
            aria_pressed: "{pressed}",
            disabled: disabled,
            style: "{final_style}",
            class: "{class}",
            onclick: handle_click,
            onmouseenter: move |_| if !disabled { is_hovered.set(true) },
            onmouseleave: move |_| is_hovered.set(false),
            onfocus: move |_| is_focused.set(true),
            onblur: move |_| is_focused.set(false),
            {props.children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_variant_equality() {
        assert_eq!(ToggleVariant::Default, ToggleVariant::Default);
        assert_ne!(ToggleVariant::Default, ToggleVariant::Outline);
        assert_ne!(ToggleVariant::Outline, ToggleVariant::Ghost);
    }

    #[test]
    fn test_toggle_size_equality() {
        assert_eq!(ToggleSize::Md, ToggleSize::Md);
        assert_ne!(ToggleSize::Sm, ToggleSize::Lg);
    }
}
