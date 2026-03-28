//! Button atom component
//!
//! A highly customizable button component with multiple variants and sizes.
//! Uses Rust-native hover/active states (no CSS pseudo-classes).

use crate::styles::Style;
use crate::theme::tokens::Color;
use crate::theme::{use_style, use_theme};
use dioxus::prelude::*;

/// Button variant styles
#[derive(Default, Clone, PartialEq, Debug)]
pub enum ButtonVariant {
    /// Primary action button
    #[default]
    Primary,
    /// Secondary action button
    Secondary,
    /// Ghost/outline button
    Ghost,
    /// Destructive action button
    Destructive,
    /// Link-style button
    Link,
}

/// Button sizes
#[derive(Default, Clone, PartialEq, Debug)]
pub enum ButtonSize {
    /// Small button
    Sm,
    /// Medium (default) button
    #[default]
    Md,
    /// Large button
    Lg,
    /// Icon-only button
    Icon,
}

/// Button properties
#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    /// Button content
    pub children: Element,
    /// Visual variant
    #[props(default)]
    pub variant: ButtonVariant,
    /// Button size
    #[props(default)]
    pub size: ButtonSize,
    /// Disabled state
    #[props(default)]
    pub disabled: bool,
    /// Full width button
    #[props(default)]
    pub full_width: bool,
    /// Click handler
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
    /// Button type
    #[props(default)]
    pub button_type: ButtonType,
}

/// HTML button type
#[derive(Default, Clone, PartialEq)]
pub enum ButtonType {
    #[default]
    Button,
    Submit,
    Reset,
}

impl ButtonType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonType::Button => "button",
            ButtonType::Submit => "submit",
            ButtonType::Reset => "reset",
        }
    }
}

/// Button atom component
///
/// # Example
/// ```rust,ignore
/// use dioxus::prelude::*;
/// use dioxus_ui_system::atoms::{Button, ButtonVariant, ButtonSize};
///
/// rsx! {
///     Button {
///         variant: ButtonVariant::Primary,
///         size: ButtonSize::Md,
///         onclick: move |_| println!("Clicked!"),
///         "Click me"
///     }
/// }
/// ```
#[component]
pub fn Button(props: ButtonProps) -> Element {
    let _theme = use_theme();

    let variant = props.variant.clone();
    let size = props.size.clone();
    let disabled = props.disabled;
    let full_width = props.full_width;

    // Interactive states
    let mut is_hovered = use_signal(|| false);
    let mut is_pressed = use_signal(|| false);
    let mut is_focused = use_signal(|| false);

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

        // Full width
        let base = if full_width { base.w_full() } else { base };

        // Size styles
        let sized = match size {
            ButtonSize::Sm => base.p(&t.spacing, "sm").h_px(32),
            ButtonSize::Md => base.p(&t.spacing, "md").h_px(40),
            ButtonSize::Lg => base.p(&t.spacing, "lg").h_px(48),
            ButtonSize::Icon => base.p(&t.spacing, "md").rounded(&t.radius, "md"),
        };

        // Variant styles with hover states
        let (bg, fg, border, shadow) = match variant {
            ButtonVariant::Primary => {
                let bg = if is_hovered() && !disabled {
                    t.colors.primary.darken(0.1)
                } else {
                    t.colors.primary.clone()
                };
                let shadow = if is_focused() && !disabled {
                    format!("0 0 0 2px {}", t.colors.background.to_rgba())
                } else {
                    String::new()
                };
                (bg, t.colors.primary_foreground.clone(), None, shadow)
            }
            ButtonVariant::Secondary => {
                let bg = if is_hovered() && !disabled {
                    t.colors.secondary.darken(0.05)
                } else {
                    t.colors.secondary.clone()
                };
                (
                    bg,
                    t.colors.secondary_foreground.clone(),
                    None,
                    String::new(),
                )
            }
            ButtonVariant::Ghost => {
                let bg = if is_hovered() && !disabled {
                    t.colors.accent.clone()
                } else {
                    Color::new_rgba(0, 0, 0, 0.0)
                };
                let border = if is_focused() && !disabled {
                    Some(format!("1px solid {}", t.colors.ring.to_rgba()))
                } else {
                    None
                };
                (bg, t.colors.foreground.clone(), border, String::new())
            }
            ButtonVariant::Destructive => {
                let bg = if is_hovered() && !disabled {
                    t.colors.destructive.darken(0.1)
                } else {
                    t.colors.destructive.clone()
                };
                (bg, Color::new(255, 255, 255), None, String::new())
            }
            ButtonVariant::Link => {
                let fg = if is_hovered() && !disabled {
                    t.colors.primary.darken(0.1)
                } else {
                    t.colors.primary.clone()
                };
                (Color::new_rgba(0, 0, 0, 0.0), fg, None, String::new())
            }
        };

        let mut final_style = sized.bg(&bg).text_color(&fg);

        if let Some(b) = border {
            final_style = Style {
                border: Some(b),
                ..final_style
            };
        }

        if !shadow.is_empty() {
            final_style = Style {
                box_shadow: Some(shadow),
                ..final_style
            };
        }

        final_style.build()
    });

    // Transform for pressed state
    let transform = if is_pressed() && !disabled {
        "transform: scale(0.98);"
    } else {
        ""
    };

    // Combine with custom styles
    let final_style = if let Some(custom) = &props.style {
        format!("{} {}{}", style(), custom, transform)
    } else {
        format!("{}{}", style(), transform)
    };

    let class = props.class.clone().unwrap_or_default();

    rsx! {
        button {
            r#type: props.button_type.as_str(),
            style: "{final_style}",
            class: "{class}",
            disabled: disabled,
            onmouseenter: move |_| if !disabled { is_hovered.set(true) },
            onmouseleave: move |_| { is_hovered.set(false); is_pressed.set(false); },
            onmousedown: move |_| if !disabled { is_pressed.set(true) },
            onmouseup: move |_| if !disabled { is_pressed.set(false) },
            onfocus: move |_| is_focused.set(true),
            onblur: move |_| is_focused.set(false),
            onclick: move |e| {
                if let Some(handler) = &props.onclick {
                    if !disabled {
                        handler.call(e);
                    }
                }
            },
            {props.children}
        }
    }
}

/// Icon button component (convenience wrapper)
#[component]
pub fn IconButton(
    icon: Element,
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(default)] disabled: bool,
    #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    rsx! {
        Button {
            variant: variant,
            size: ButtonSize::Icon,
            disabled: disabled,
            onclick: onclick,
            {icon}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_variant_equality() {
        assert_eq!(ButtonVariant::Primary, ButtonVariant::Primary);
        assert_ne!(ButtonVariant::Primary, ButtonVariant::Secondary);
    }

    #[test]
    fn test_button_size_equality() {
        assert_eq!(ButtonSize::Md, ButtonSize::Md);
        assert_ne!(ButtonSize::Sm, ButtonSize::Lg);
    }
}
