//! Slider atom component
//!
//! Range slider input for selecting numeric values.

use crate::theme::use_theme;
use dioxus::prelude::*;

/// Slider properties
#[derive(Props, Clone, PartialEq)]
#[allow(unpredictable_function_pointer_comparisons)]
pub struct SliderProps {
    /// Current value
    #[props(default = 0.0)]
    pub value: f64,
    /// Minimum value
    #[props(default = 0.0)]
    pub min: f64,
    /// Maximum value
    #[props(default = 100.0)]
    pub max: f64,
    /// Step increment
    #[props(default = 1.0)]
    pub step: f64,
    /// Change handler
    pub on_change: EventHandler<f64>,
    /// Label
    #[props(default)]
    pub label: Option<String>,
    /// Whether the slider is disabled
    #[props(default = false)]
    pub disabled: bool,
    /// Show current value
    #[props(default = true)]
    pub show_value: bool,
    /// Format function for value display
    #[props(default)]
    pub format_value: Option<fn(f64) -> String>,
    /// Color for the slider track/fill
    pub color: Option<String>,
    /// Size variant
    #[props(default = SliderSize::Md)]
    pub size: SliderSize,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Marks/ticks on the slider
    #[props(default)]
    pub marks: Vec<SliderMark>,
}

/// Slider size
#[derive(Default, Clone, PartialEq, Debug)]
pub enum SliderSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl SliderSize {
    fn to_height(&self) -> u8 {
        match self {
            SliderSize::Sm => 4,
            SliderSize::Md => 6,
            SliderSize::Lg => 8,
        }
    }

    fn to_thumb_size(&self) -> u8 {
        match self {
            SliderSize::Sm => 14,
            SliderSize::Md => 18,
            SliderSize::Lg => 22,
        }
    }
}

/// Slider mark/tick
#[derive(Clone, PartialEq, Debug)]
pub struct SliderMark {
    pub value: f64,
    pub label: Option<String>,
}

/// Slider component
#[component]
pub fn Slider(props: SliderProps) -> Element {
    let theme = use_theme();

    let color = props
        .color
        .unwrap_or_else(|| theme.tokens.read().colors.primary.to_rgba());

    let class_css = props
        .class
        .as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();

    let range = props.max - props.min;
    let percentage = if range > 0.0 {
        ((props.value - props.min) / range * 100.0).clamp(0.0, 100.0)
    } else {
        0.0
    };

    let height = props.size.to_height();
    let thumb_size = props.size.to_thumb_size();

    let track_color = theme.tokens.read().colors.muted.to_rgba();

    // Format value display
    let value_text = props
        .format_value
        .map(|f| f(props.value))
        .unwrap_or_else(|| format!("{:.0}", props.value));

    let disabled_css = if props.disabled {
        "opacity: 0.5; cursor: not-allowed;"
    } else {
        "cursor: pointer;"
    };

    rsx! {
        div {
            class: "slider{class_css}",
            style: "display: flex; flex-direction: column; gap: 8px; {disabled_css}",

            // Label row
            if props.label.is_some() || props.show_value {
                div {
                    style: "display: flex; justify-content: space-between; align-items: center;",

                    if let Some(label) = props.label {
                        label {
                            class: "slider-label",
                            style: "font-size: 14px; font-weight: 500; color: {theme.tokens.read().colors.foreground.to_rgba()};",
                            "{label}"
                        }
                    }

                    if props.show_value {
                        span {
                            class: "slider-value",
                            style: "font-size: 14px; color: {theme.tokens.read().colors.muted.to_rgba()};",
                            "{value_text}"
                        }
                    }
                }
            }

            // Slider input
            div {
                style: "position: relative; height: {thumb_size}px; display: flex; align-items: center;",

                input {
                    type: "range",
                    class: "slider-input",
                    min: "{props.min}",
                    max: "{props.max}",
                    step: "{props.step}",
                    value: "{props.value}",
                    disabled: props.disabled,
                    style: format!("position: absolute; width: 100%; height: 100%; opacity: 0; cursor: {}; z-index: 2;", if props.disabled { "not-allowed" } else { "pointer" }),
                    oninput: move |e: Event<FormData>| {
                        if let Ok(val) = e.value().parse::<f64>() {
                            props.on_change.call(val);
                        }
                    },
                }

                // Visual track
                div {
                    class: "slider-track",
                    style: "position: relative; width: 100%; height: {height}px; background: {track_color}; border-radius: 9999px;",

                    // Fill
                    div {
                        class: "slider-fill",
                        style: "position: absolute; left: 0; top: 0; height: 100%; width: {percentage}%; background: {color}; border-radius: 9999px;",
                    }

                    // Thumb
                    div {
                        class: "slider-thumb",
                        style: "position: absolute; top: 50%; left: {percentage}%; transform: translate(-50%, -50%); width: {thumb_size}px; height: {thumb_size}px; background: white; border: 2px solid {color}; border-radius: 50%; box-shadow: 0 2px 4px rgba(0,0,0,0.1); transition: transform 0.1s ease;",
                    }
                }
            }

            // Marks
            if !props.marks.is_empty() {
                div {
                    class: "slider-marks",
                    style: "position: relative; height: 20px; margin-top: 4px;",

                    for mark in props.marks.iter() {
                        {
                            let mark_pct = if range > 0.0 {
                                ((mark.value - props.min) / range * 100.0).clamp(0.0, 100.0)
                            } else {
                                0.0
                            };

                            rsx! {
                                div {
                                    key: "{mark.value}",
                                    style: "position: absolute; left: {mark_pct}%; transform: translateX(-50%); text-align: center;",

                                    div {
                                        style: "width: 2px; height: 6px; background: {track_color}; margin: 0 auto;",
                                    }

                                    if let Some(label) = mark.label.clone() {
                                        span {
                                            style: "font-size: 10px; color: {theme.tokens.read().colors.muted.to_rgba()}; white-space: nowrap;",
                                            "{label}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Range slider properties (dual handle)
#[derive(Props, Clone, PartialEq)]
pub struct RangeSliderProps {
    /// Lower value
    pub min_value: f64,
    /// Upper value
    pub max_value: f64,
    /// Overall minimum
    #[props(default = 0.0)]
    pub min: f64,
    /// Overall maximum
    #[props(default = 100.0)]
    pub max: f64,
    /// Step increment
    #[props(default = 1.0)]
    pub step: f64,
    /// Change handler
    pub on_change: EventHandler<(f64, f64)>,
    /// Label
    #[props(default)]
    pub label: Option<String>,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Range slider component (dual handle)
#[component]
pub fn RangeSlider(props: RangeSliderProps) -> Element {
    let theme = use_theme();

    let class_css = props
        .class
        .as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();

    let range = props.max - props.min;
    let min_pct = ((props.min_value - props.min) / range * 100.0).clamp(0.0, 100.0);
    let max_pct = ((props.max_value - props.min) / range * 100.0).clamp(0.0, 100.0);

    let color = theme.tokens.read().colors.primary.to_rgba();
    let track_color = theme.tokens.read().colors.muted.to_rgba();

    rsx! {
        div {
            class: "range-slider{class_css}",
            style: "display: flex; flex-direction: column; gap: 8px;",

            if let Some(label) = props.label {
                label {
                    class: "range-slider-label",
                    style: "font-size: 14px; font-weight: 500;",
                    "{label}"
                }
            }

            div {
                style: "display: flex; align-items: center; gap: 12px;",

                // Min value input
                input {
                    type: "number",
                    class: "range-slider-min-input",
                    min: "{props.min}",
                    max: "{props.max_value}",
                    step: "{props.step}",
                    value: "{props.min_value}",
                    style: "width: 70px; padding: 6px; font-size: 14px; border: 1px solid {track_color}; border-radius: 6px;",
                    oninput: move |e: Event<FormData>| {
                        if let Ok(val) = e.value().parse::<f64>() {
                            let new_min = val.min(props.max_value);
                            props.on_change.call((new_min, props.max_value));
                        }
                    },
                }

                // Visual slider
                div {
                    style: "flex: 1; position: relative; height: 20px; display: flex; align-items: center;",

                    div {
                        class: "range-slider-track",
                        style: "width: 100%; height: 6px; background: {track_color}; border-radius: 9999px; position: relative;",

                        // Fill between handles
                        div {
                            class: "range-slider-fill",
                            style: format!("position: absolute; left: {min_pct}%; right: {}%; height: 100%; background: {color}; border-radius: 9999px;", 100.0 - max_pct),
                        }

                        // Min handle
                        div {
                            class: "range-slider-handle range-slider-handle-min",
                            style: "position: absolute; left: {min_pct}%; top: 50%; transform: translate(-50%, -50%); width: 18px; height: 18px; background: white; border: 2px solid {color}; border-radius: 50%; cursor: pointer; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
                        }

                        // Max handle
                        div {
                            class: "range-slider-handle range-slider-handle-max",
                            style: "position: absolute; left: {max_pct}%; top: 50%; transform: translate(-50%, -50%); width: 18px; height: 18px; background: white; border: 2px solid {color}; border-radius: 50%; cursor: pointer; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
                        }
                    }
                }

                // Max value input
                input {
                    type: "number",
                    class: "range-slider-max-input",
                    min: "{props.min_value}",
                    max: "{props.max}",
                    step: "{props.step}",
                    value: "{props.max_value}",
                    style: "width: 70px; padding: 6px; font-size: 14px; border: 1px solid {track_color}; border-radius: 6px;",
                    oninput: move |e: Event<FormData>| {
                        if let Ok(val) = e.value().parse::<f64>() {
                            let new_max = val.max(props.min_value);
                            props.on_change.call((props.min_value, new_max));
                        }
                    },
                }
            }
        }
    }
}
