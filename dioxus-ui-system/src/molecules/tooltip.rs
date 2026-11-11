//! Tooltip molecule component
//!
//! A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it.

use crate::styles::Style;
use crate::theme::{use_style, use_theme};
use dioxus::prelude::*;

/// Tooltip properties
#[derive(Props, Clone, PartialEq)]
pub struct TooltipProps {
    /// Trigger element (the element that shows the tooltip)
    pub children: Element,
    /// Tooltip content
    pub content: String,
    /// Tooltip placement
    #[props(default)]
    pub placement: TooltipPlacement,
    /// Delay before showing tooltip (in ms)
    #[props(default = 200)]
    pub delay: u64,
    /// Delay before hiding tooltip (in ms)
    #[props(default = 0)]
    pub hide_delay: u64,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Element id
    #[props(default)]
    pub id: Option<String>,
    /// Custom CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Tooltip placement options
#[derive(Default, Clone, PartialEq)]
pub enum TooltipPlacement {
    /// Top placement (default)
    #[default]
    Top,
    /// Top-start placement
    TopStart,
    /// Top-end placement
    TopEnd,
    /// Right placement
    Right,
    /// Right-start placement
    RightStart,
    /// Right-end placement
    RightEnd,
    /// Bottom placement
    Bottom,
    /// Bottom-start placement
    BottomStart,
    /// Bottom-end placement
    BottomEnd,
    /// Left placement
    Left,
    /// Left-start placement
    LeftStart,
    /// Left-end placement
    LeftEnd,
}

/// Tooltip component
#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    let _theme = use_theme();
    let mut is_visible = use_signal(|| false);
    // Bumped on every enter/leave; a pending timer only applies if the
    // generation it captured is still current (cancels stale show/hide).
    let mut generation = use_signal(|| 0u64);

    let position_style = match props.placement {
        TooltipPlacement::Top => {
            "bottom: calc(100% + 6px); left: 50%; transform: translateX(-50%);"
        }
        TooltipPlacement::TopStart => "bottom: calc(100% + 6px); left: 0;",
        TooltipPlacement::TopEnd => "bottom: calc(100% + 6px); right: 0;",
        TooltipPlacement::Right => "left: calc(100% + 6px); top: 50%; transform: translateY(-50%);",
        TooltipPlacement::RightStart => "left: calc(100% + 6px); top: 0;",
        TooltipPlacement::RightEnd => "left: calc(100% + 6px); bottom: 0;",
        TooltipPlacement::Bottom => {
            "top: calc(100% + 6px); left: 50%; transform: translateX(-50%);"
        }
        TooltipPlacement::BottomStart => "top: calc(100% + 6px); left: 0;",
        TooltipPlacement::BottomEnd => "top: calc(100% + 6px); right: 0;",
        TooltipPlacement::Left => "right: calc(100% + 6px); top: 50%; transform: translateY(-50%);",
        TooltipPlacement::LeftStart => "right: calc(100% + 6px); top: 0;",
        TooltipPlacement::LeftEnd => "right: calc(100% + 6px); bottom: 0;",
    };

    let tooltip_style = use_style(|t| {
        Style::new()
            .absolute()
            .px(&t.spacing, "sm")
            .py(&t.spacing, "xs")
            .rounded(&t.radius, "md")
            .bg(&t.colors.foreground)
            .text_color(&t.colors.background)
            .font_size(12)
            .font_weight(500)
            .whitespace_nowrap()
            .z_index(100)
            .build()
    });

    let mut schedule = move |visible: bool, delay_ms: u64| {
        generation += 1;
        let current = *generation.peek();
        if delay_ms == 0 {
            is_visible.set(visible);
            return;
        }
        // Tasks spawned here are owned by this component and dropped on unmount.
        spawn(async move {
            sleep_ms(delay_ms).await;
            if *generation.peek() == current {
                is_visible.set(visible);
            }
        });
    };

    let (show_delay, hide_delay) = (props.delay, props.hide_delay);
    let mut show_tooltip = move || schedule(true, show_delay);
    let mut hide_tooltip = move || schedule(false, hide_delay);

    rsx! {
        span {
            id: "{props.id.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            style: "position: relative; display: inline-block;",
            onmouseenter: move |_| show_tooltip(),
            onmouseleave: move |_| hide_tooltip(),
            onfocus: move |_| show_tooltip(),
            onblur: move |_| hide_tooltip(),

            {props.children}

            if is_visible() {
                div {
                    role: "tooltip",
                    style: "{tooltip_style} {position_style} {props.style.clone().unwrap_or_default()}",
                    "{props.content}"
                }
            }
        }
    }
}

/// Resolve after `ms` milliseconds (browser `setTimeout`).
#[cfg(all(feature = "web", target_arch = "wasm32"))]
async fn sleep_ms(ms: u64) {
    let promise = js_sys::Promise::new(&mut |resolve, _reject| {
        if let Some(window) = web_sys::window() {
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                &resolve,
                ms.min(i32::MAX as u64) as i32,
            );
        }
    });
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}

/// Non-web targets have no timer runtime here: apply immediately.
#[cfg(not(all(feature = "web", target_arch = "wasm32")))]
async fn sleep_ms(_ms: u64) {}

/// Simple tooltip that wraps any element
#[derive(Props, Clone, PartialEq)]
pub struct SimpleTooltipProps {
    /// Element to wrap
    pub children: Element,
    /// Tooltip text
    pub text: String,
    /// Tooltip placement
    #[props(default)]
    pub placement: TooltipPlacement,
    /// Element id
    #[props(default)]
    pub id: Option<String>,
    /// Custom CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Simple tooltip wrapper
#[component]
pub fn SimpleTooltip(props: SimpleTooltipProps) -> Element {
    rsx! {
        Tooltip {
            id: "{props.id.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            style: "{props.style.clone().unwrap_or_default()}",
            content: props.text,
            placement: props.placement,
            {props.children}
        }
    }
}
