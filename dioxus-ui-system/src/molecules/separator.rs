//! Separator molecule component
//!
//! Visually or semantically separates content.

use crate::styles::Style;
use crate::theme::{use_style, use_theme};
use dioxus::prelude::*;

/// Separator orientation
#[derive(Default, Clone, PartialEq)]
pub enum SeparatorOrientation {
    /// Horizontal separator (default)
    #[default]
    Horizontal,
    /// Vertical separator
    Vertical,
}

/// Separator properties
#[derive(Props, Clone, PartialEq)]
pub struct SeparatorProps {
    /// Separator orientation
    #[props(default)]
    pub orientation: SeparatorOrientation,
    /// Whether the separator is decorative only
    #[props(default = true)]
    pub decorative: bool,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
    /// Element id
    #[props(default)]
    pub id: Option<String>,
}

/// Separator molecule component
#[component]
pub fn Separator(props: SeparatorProps) -> Element {
    let _theme = use_theme();

    let is_horizontal = props.orientation == SeparatorOrientation::Horizontal;

    let separator_style = use_style(move |t| {
        if is_horizontal {
            Style::new()
                .w_full()
                .h_px(1)
                .bg(&t.colors.border)
                .my(&t.spacing, "md")
                .build()
        } else {
            Style::new()
                .w_px(1)
                .h_full()
                .bg(&t.colors.border)
                .mx(&t.spacing, "md")
                .build()
        }
    });

    let decorative = props.decorative;
    let orientation_attr = if is_horizontal {
        "horizontal"
    } else {
        "vertical"
    };

    // Decorative separators are hidden from assistive tech; semantic ones get
    // role="separator" (+ aria-orientation, which is only meaningful with that role).
    let role = if decorative { "none" } else { "separator" };
    let aria_hidden = decorative.then_some("true");
    let aria_orientation = (!decorative).then_some(orientation_attr);

    rsx! {
        div {
            id: "{props.id.clone().unwrap_or_default()}",
            style: "{separator_style} {props.style.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            role: role,
            aria_hidden: aria_hidden,
            aria_orientation: aria_orientation,
            "data-orientation": orientation_attr,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(app: fn() -> Element) -> String {
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
        dioxus_ssr::render(&dom)
    }

    #[test]
    fn decorative_separator_is_hidden_and_style_is_css_only() {
        let html = render(|| rsx! { Separator {} });
        assert!(html.contains("aria-hidden=\"true\""), "{html}");
        assert!(html.contains("role=\"none\""), "{html}");
        assert!(!html.contains("aria-orientation"), "{html}");
        assert!(!html.contains("aria-hidden:"), "{html}");
        assert!(!html.contains("role:"), "{html}");
    }

    #[test]
    fn semantic_vertical_separator_has_role_and_orientation() {
        let html = render(|| {
            rsx! { Separator { decorative: false, orientation: SeparatorOrientation::Vertical } }
        });
        assert!(html.contains("role=\"separator\""), "{html}");
        assert!(html.contains("aria-orientation=\"vertical\""), "{html}");
        assert!(!html.contains("aria-hidden"), "{html}");
    }
}
