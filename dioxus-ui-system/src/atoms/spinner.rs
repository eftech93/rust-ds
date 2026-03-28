//! Spinner atom component
//!
//! Loading spinners and indicators.

use crate::theme::use_theme;
use dioxus::prelude::*;

/// CSS keyframes for spinner animations
const SPINNER_STYLES: &str = r#"
@keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
}

@keyframes bounce {
    0%, 100% { transform: translateY(0); }
    50% { transform: translateY(-25%); }
}

@keyframes pulse {
    0%, 100% { opacity: 1; transform: scale(1); }
    50% { opacity: 0.5; transform: scale(0.9); }
}

@keyframes bars {
    0%, 100% { transform: scaleY(0.3); }
    50% { transform: scaleY(1); }
}
"#;

pub fn default_loading_label() -> String {
    "Loading".to_string()
}

/// Spinner variant
#[derive(Default, Clone, PartialEq, Debug)]
pub enum SpinnerVariant {
    #[default]
    Circular,
    Dots,
    Pulse,
    Bars,
}

/// Spinner size
#[derive(Default, Clone, PartialEq, Debug)]
pub enum SpinnerSize {
    Xs, // 12px
    Sm, // 16px
    #[default]
    Md, // 24px
    Lg, // 32px
    Xl, // 48px
}

impl SpinnerSize {
    fn to_px(&self) -> u8 {
        match self {
            SpinnerSize::Xs => 12,
            SpinnerSize::Sm => 16,
            SpinnerSize::Md => 24,
            SpinnerSize::Lg => 32,
            SpinnerSize::Xl => 48,
        }
    }
}

/// Spinner properties
#[derive(Props, Clone, PartialEq)]
pub struct SpinnerProps {
    /// Spinner variant/style
    #[props(default = SpinnerVariant::Circular)]
    pub variant: SpinnerVariant,
    /// Size variant
    #[props(default = SpinnerSize::Md)]
    pub size: SpinnerSize,
    /// Color override
    pub color: Option<String>,
    /// Accessibility label
    #[props(default = default_loading_label())]
    pub label: String,
    /// Speed multiplier (1.0 = default)
    #[props(default = 1.0)]
    pub speed: f32,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Spinner loading indicator component
#[component]
pub fn Spinner(props: SpinnerProps) -> Element {
    let theme = use_theme();

    let color = props
        .color
        .unwrap_or_else(|| theme.tokens.read().colors.primary.to_rgba());

    let size = props.size.to_px();
    let class_css = props
        .class
        .as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();

    let animation_duration = format!("{:.2}s", 1.0 / props.speed);

    match props.variant {
        SpinnerVariant::Circular => {
            let stroke_width = size / 8;
            let radius = (size - stroke_width) / 2;
            let circumference = 2.0 * std::f32::consts::PI * radius as f32;

            rsx! {
                style { dangerous_inner_html: "{SPINNER_STYLES}" }

                span {
                    class: "spinner spinner-circular{class_css}",
                    role: "status",
                    aria_label: "{props.label}",

                    svg {
                        width: "{size}px",
                        height: "{size}px",
                        view_box: "0 0 {size} {size}",
                        style: "animation: spin {animation_duration} linear infinite;",

                        circle {
                            cx: "{size / 2}",
                            cy: "{size / 2}",
                            r: "{radius}",
                            fill: "none",
                            stroke: "{color}",
                            stroke_width: "{stroke_width}",
                            stroke_linecap: "round",
                            stroke_dasharray: "{circumference}",
                            stroke_dashoffset: "{circumference * 0.75}",
                        }
                    }

                    span {
                        class: "sr-only",
                        style: "position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0, 0, 0, 0); white-space: nowrap; border: 0;",
                        "{props.label}"
                    }
                }
            }
        }
        SpinnerVariant::Dots => {
            let dot_size = size / 4;
            let gap = dot_size / 2;

            rsx! {
                style { dangerous_inner_html: "{SPINNER_STYLES}" }

                span {
                    class: "spinner spinner-dots{class_css}",
                    role: "status",
                    aria_label: "{props.label}",
                    style: "display: inline-flex; align-items: center; gap: {gap}px;",

                    for i in 0..3 {
                        span {
                            key: "{i}",
                            style: format!("width: {dot_size}px; height: {dot_size}px; background: {color}; border-radius: 50%; animation: bounce {animation_duration} ease-in-out {:.2}s infinite;", i as f32 * 0.16),
                        }
                    }

                    span {
                        class: "sr-only",
                        style: "position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0, 0, 0, 0); white-space: nowrap; border: 0;",
                        "{props.label}"
                    }
                }
            }
        }
        SpinnerVariant::Pulse => {
            rsx! {
                style { dangerous_inner_html: "{SPINNER_STYLES}" }

                span {
                    class: "spinner spinner-pulse{class_css}",
                    role: "status",
                    aria_label: "{props.label}",

                    span {
                        style: "display: inline-block; width: {size}px; height: {size}px; background: {color}; border-radius: 50%; animation: pulse {animation_duration} ease-in-out infinite;",
                    }

                    span {
                        class: "sr-only",
                        style: "position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0, 0, 0, 0); white-space: nowrap; border: 0;",
                        "{props.label}"
                    }
                }
            }
        }
        SpinnerVariant::Bars => {
            let bar_width = size / 6;
            let bar_height = size;

            rsx! {
                style { dangerous_inner_html: "{SPINNER_STYLES}" }

                span {
                    class: "spinner spinner-bars{class_css}",
                    role: "status",
                    aria_label: "{props.label}",
                    style: "display: inline-flex; align-items: center; gap: 2px; height: {bar_height}px;",

                    for i in 0..5 {
                        span {
                            key: "{i}",
                            style: format!("width: {bar_width}px; height: 100%; background: {color}; animation: bars {animation_duration} ease-in-out {:.1}s infinite;", i as f32 * 0.1),
                        }
                    }

                    span {
                        class: "sr-only",
                        style: "position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0, 0, 0, 0); white-space: nowrap; border: 0;",
                        "{props.label}"
                    }
                }
            }
        }
    }
}

/// Loading overlay properties
#[derive(Props, Clone, PartialEq)]
pub struct LoadingOverlayProps {
    /// Whether the overlay is visible
    pub visible: bool,
    /// Spinner size
    #[props(default = SpinnerSize::Lg)]
    pub spinner_size: SpinnerSize,
    /// Message to display below spinner
    #[props(default)]
    pub message: Option<String>,
    /// Background color (with opacity)
    #[props(default)]
    pub background: Option<String>,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Children elements (content that gets overlaid)
    pub children: Element,
}

/// Loading overlay component
#[component]
pub fn LoadingOverlay(props: LoadingOverlayProps) -> Element {
    let theme = use_theme();

    let bg_color = props.background.unwrap_or_else(|| {
        format!(
            "{}80",
            theme
                .tokens
                .read()
                .colors
                .background
                .to_rgba()
                .trim_start_matches('#')
        )
    });

    let class_css = props
        .class
        .as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();

    rsx! {
        div {
            class: "loading-overlay-container{class_css}",
            style: "position: relative;",

            {props.children}

            if props.visible {
                div {
                    class: "loading-overlay",
                    style: "position: absolute; inset: 0; display: flex; flex-direction: column; align-items: center; justify-content: center; background: {bg_color}; backdrop-filter: blur(2px); z-index: 50; transition: opacity 0.2s ease;",

                    Spinner {
                        size: props.spinner_size,
                    }

                    if let Some(message) = props.message {
                        span {
                            style: "margin-top: 16px; font-size: 14px; color: {theme.tokens.read().colors.foreground.to_rgba()};",
                            "{message}"
                        }
                    }
                }
            }
        }
    }
}
