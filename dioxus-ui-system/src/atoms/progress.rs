//! Progress atom component
//!
//! Linear and circular progress indicators.

use crate::theme::use_theme;
use dioxus::prelude::*;

pub fn default_width() -> String {
    "100%".to_string()
}

/// Progress indicator variant
#[derive(Default, Clone, PartialEq, Debug)]
pub enum ProgressVariant {
    #[default]
    Linear,
    Circular,
}

/// Progress size
#[derive(Default, Clone, PartialEq, Debug)]
pub enum ProgressSize {
    #[default]
    Sm, // Small
    Md, // Medium
    Lg, // Large
}

impl ProgressSize {
    fn to_height(&self) -> u8 {
        match self {
            ProgressSize::Sm => 4,
            ProgressSize::Md => 8,
            ProgressSize::Lg => 12,
        }
    }

    fn to_diameter(&self) -> u8 {
        match self {
            ProgressSize::Sm => 16,
            ProgressSize::Md => 32,
            ProgressSize::Lg => 48,
        }
    }

    fn to_stroke(&self) -> u8 {
        match self {
            ProgressSize::Sm => 2,
            ProgressSize::Md => 3,
            ProgressSize::Lg => 4,
        }
    }
}

/// Progress indicator properties
#[derive(Props, Clone, PartialEq)]
pub struct ProgressProps {
    /// Current progress value (0-100)
    pub value: Option<f32>,
    /// Maximum value (default: 100)
    #[props(default = 100.0)]
    pub max: f32,
    /// Variant (linear or circular)
    #[props(default = ProgressVariant::Linear)]
    pub variant: ProgressVariant,
    /// Size variant
    #[props(default = ProgressSize::Md)]
    pub size: ProgressSize,
    /// Color for the progress bar/track
    pub color: Option<String>,
    /// Background track color
    pub track_color: Option<String>,
    /// Show percentage label
    #[props(default = false)]
    pub show_label: bool,
    /// Label position (inline for linear, inside for circular)
    #[props(default = LabelPosition::Right)]
    pub label_position: LabelPosition,
    /// Width for linear progress
    #[props(default = default_width())]
    pub width: String,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Indeterminate state (loading without specific progress)
    #[props(default = false)]
    pub indeterminate: bool,
    /// Element id
    #[props(default)]
    pub id: Option<String>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Accessible name announced by screen readers (`aria-label`).
    ///
    /// Defaults to `"Loading"` for indeterminate indicators and `"Progress"`
    /// otherwise.
    #[props(default)]
    pub aria_label: Option<String>,
    /// Human-readable value announced instead of the percentage
    /// (`aria-valuetext`), e.g. `"Step 2 of 5"`. Ignored when indeterminate.
    #[props(default)]
    pub value_text: Option<String>,
}

/// ARIA value attributes for a progress bar: `(aria-valuenow, aria-valuetext)`.
///
/// Returns `None` for indeterminate indicators (no value or `indeterminate`),
/// which must omit `aria-valuenow`. The value is clamped to `0..=max`.
fn progress_aria_values(
    value: Option<f32>,
    max: f32,
    indeterminate: bool,
    value_text: Option<String>,
) -> Option<(String, String)> {
    let value = value.filter(|_| !indeterminate)?;
    let now = value.clamp(0.0, max.max(0.0));
    let text = value_text.unwrap_or_else(|| {
        let pct = if max > 0.0 { now / max * 100.0 } else { 0.0 };
        format!("{pct:.0}%")
    });
    Some((now.to_string(), text))
}

/// Label position
#[derive(Default, Clone, PartialEq, Debug)]
pub enum LabelPosition {
    #[default]
    Right,
    Inside,
    Bottom,
}

/// Progress indicator component
///
/// # Accessibility
/// The root element has `role="progressbar"` with `aria-valuemin="0"`,
/// `aria-valuemax` (the `max` prop) and, for determinate indicators,
/// `aria-valuenow` plus `aria-valuetext` (the percentage, or `value_text`).
/// Indeterminate indicators omit `aria-valuenow`/`aria-valuetext` and set
/// `aria-busy="true"`. The accessible name comes from `aria_label`.
#[component]
pub fn Progress(props: ProgressProps) -> Element {
    let theme = use_theme();

    let is_indeterminate = props.indeterminate || props.value.is_none();
    let (aria_valuenow, aria_valuetext) = progress_aria_values(
        props.value,
        props.max,
        props.indeterminate,
        props.value_text.clone(),
    )
    .unzip();
    let aria_valuemax = props.max.to_string();
    let aria_busy = is_indeterminate.then_some("true");
    let aria_label = props.aria_label.clone().unwrap_or_else(|| {
        if is_indeterminate {
            "Loading".to_string()
        } else {
            "Progress".to_string()
        }
    });

    let color = props
        .color
        .unwrap_or_else(|| theme.tokens.read().colors.primary.to_rgba());

    let track_color = props
        .track_color
        .unwrap_or_else(|| theme.tokens.read().colors.muted.to_rgba());

    let class_css = props
        .class
        .as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();

    let style_css = props
        .style
        .as_ref()
        .map(|s| format!(" {}", s))
        .unwrap_or_default();

    let id_css = props.id.clone().unwrap_or_default();

    // Calculate percentage
    let percentage = if props.indeterminate || props.value.is_none() {
        0.0
    } else {
        let val = props.value.unwrap();
        ((val / props.max) * 100.0).clamp(0.0, 100.0)
    };

    let label_text = format!("{:.0}%", percentage);

    match props.variant {
        ProgressVariant::Linear => {
            let height = props.size.to_height();
            let _indeterminate_css = if props.indeterminate {
                "background: linear-gradient(90deg, transparent, {color}, transparent); background-size: 200% 100%; animation: progress-indeterminate 1.5s ease-in-out infinite;"
            } else {
                ""
            };

            rsx! {
                div {
                    class: "progress progress-linear{class_css}",
                    role: "progressbar",
                    "aria-label": "{aria_label}",
                    "aria-valuemin": "0",
                    "aria-valuemax": "{aria_valuemax}",
                    "aria-valuenow": aria_valuenow,
                    "aria-valuetext": aria_valuetext,
                    "aria-busy": aria_busy,
                    style: "display: flex; align-items: center; gap: 8px; width: {props.width};{style_css}",
                    id: "{id_css}",

                    div {
                        style: "flex: 1; height: {height}px; background: {track_color}; border-radius: 9999px; overflow: hidden;",

                        if props.indeterminate {
                            div {
                                style: "height: 100%; width: 50%; background: {color}; border-radius: 9999px; animation: progress-indeterminate 1.5s ease-in-out infinite;",
                            }
                        } else {
                            div {
                                style: "height: 100%; width: {percentage}%; background: {color}; border-radius: 9999px; transition: width 0.3s ease;",
                            }
                        }
                    }

                    if props.show_label {
                        span {
                            "aria-hidden": "true",
                            style: "font-size: 12px; color: {theme.tokens.read().colors.foreground.to_rgba()}; min-width: 40px; text-align: right;",
                            "{label_text}"
                        }
                    }
                }

                style { "{{"
                    "@keyframes progress-indeterminate {{"
                    "0% {{ transform: translateX(-100%); }}"
                    "100% {{ transform: translateX(200%); }}"
                    "}}"
                "}}" }
            }
        }
        ProgressVariant::Circular => {
            let diameter = props.size.to_diameter();
            let stroke = props.size.to_stroke();
            let radius = (diameter as f32 - stroke as f32) / 2.0;
            let circumference = 2.0 * std::f32::consts::PI * radius;
            let stroke_dashoffset = if props.indeterminate {
                circumference * 0.25
            } else {
                circumference * (1.0 - percentage / 100.0)
            };

            let center = diameter as f32 / 2.0;

            rsx! {
                div {
                    class: "progress progress-circular{class_css}",
                    role: "progressbar",
                    "aria-label": "{aria_label}",
                    "aria-valuemin": "0",
                    "aria-valuemax": "{aria_valuemax}",
                    "aria-valuenow": aria_valuenow,
                    "aria-valuetext": aria_valuetext,
                    "aria-busy": aria_busy,
                    style: "display: inline-flex; align-items: center; justify-content: center; position: relative;{style_css}",
                    id: "{id_css}",

                    svg {
                        "aria-hidden": "true",
                        width: "{diameter}px",
                        height: "{diameter}px",
                        view_box: "0 0 {diameter} {diameter}",
                        style: if props.indeterminate { "animation: rotate 1s linear infinite;" } else { "" },

                        // Background track
                        circle {
                            cx: "{center}",
                            cy: "{center}",
                            r: "{radius}",
                            fill: "none",
                            stroke: "{track_color}",
                            stroke_width: "{stroke}",
                        }

                        // Progress arc
                        circle {
                            cx: "{center}",
                            cy: "{center}",
                            r: "{radius}",
                            fill: "none",
                            stroke: "{color}",
                            stroke_width: "{stroke}",
                            stroke_linecap: "round",
                            stroke_dasharray: "{circumference}",
                            stroke_dashoffset: "{stroke_dashoffset}",
                            style: if props.indeterminate {
                                "animation: circular-progress 1.5s ease-in-out infinite;"
                            } else {
                                "transition: stroke-dashoffset 0.3s ease; transform: rotate(-90deg); transform-origin: center;"
                            },
                        }
                    }

                    if props.show_label {
                        span {
                            "aria-hidden": "true",
                            style: format!("position: absolute; font-size: {}px; color: {}; font-weight: 500;", diameter / 4, theme.tokens.read().colors.foreground.to_rgba()),
                            "{label_text}"
                        }
                    }
                }

                style { "@keyframes rotate {{ from {{ transform: rotate(0deg); }} to {{ transform: rotate(360deg); }} }} @keyframes circular-progress {{ 0% {{ stroke-dasharray: 1, 200; stroke-dashoffset: 0; }} 50% {{ stroke-dasharray: 89, 200; stroke-dashoffset: -35; }} 100% {{ stroke-dasharray: 89, 200; stroke-dashoffset: -124; }} }}" }
            }
        }
    }
}

/// Step progress properties (for multi-step processes)
#[derive(Props, Clone, PartialEq)]
pub struct StepProgressProps {
    /// Total number of steps
    pub total_steps: usize,
    /// Current step (0-indexed)
    pub current_step: usize,
    /// Step labels
    #[props(default)]
    pub labels: Vec<String>,
    /// Color for completed/current steps
    pub color: Option<String>,
    /// Show step labels
    #[props(default = true)]
    pub show_labels: bool,
}

/// Step progress indicator
#[component]
pub fn StepProgress(props: StepProgressProps) -> Element {
    let theme = use_theme();

    let color = props
        .color
        .unwrap_or_else(|| theme.tokens.read().colors.primary.to_rgba());

    let muted_color = theme.tokens.read().colors.muted.to_rgba();

    rsx! {
        div {
            class: "step-progress",
            style: "display: flex; align-items: center; width: 100%;",

            for i in 0..props.total_steps {
                {
                    let is_completed = i < props.current_step;
                    let is_current = i == props.current_step;
                    let step_color = if is_completed || is_current { color.clone() } else { muted_color.clone() };
                    let bg_color = if is_completed || is_current { color.clone() } else { "transparent".to_string() };
                    let border_color = step_color.clone();
                    let text_color = if is_completed || is_current { "white".to_string() } else { muted_color.clone() };

                    rsx! {
                        div {
                            key: "step-{i}",
                            style: "display: flex; flex-direction: column; align-items: center; flex: 1; position: relative;",

                            // Step circle
                            div {
                                style: "width: 32px; height: 32px; border-radius: 50%; background: {bg_color}; border: 2px solid {border_color}; display: flex; align-items: center; justify-content: center; font-size: 14px; font-weight: 600; color: {text_color}; transition: all 0.3s ease;",

                                if is_completed {
                                    "✓"
                                } else {
                                    "{i + 1}"
                                }
                            }

                            // Label
                            if props.show_labels && i < props.labels.len() {
                                span {
                                    style: "margin-top: 8px; font-size: 12px; color: {step_color}; text-align: center;",
                                    "{props.labels[i]}"
                                }
                            }

                            // Connector line (except for last step)
                            if i < props.total_steps - 1 {
                                div {
                                    style: format!("position: absolute; top: 16px; left: calc(50% + 20px); right: calc(-50% + 20px); height: 2px; background: {}; z-index: -1;", if i < props.current_step { color.clone() } else { muted_color.clone() }),
                                }
                            }
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

    fn render(app: fn() -> Element) -> String {
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
        dioxus_ssr::render(&dom)
    }

    #[test]
    fn aria_values_clamp_and_format() {
        assert_eq!(
            progress_aria_values(Some(42.0), 100.0, false, None),
            Some(("42".to_string(), "42%".to_string()))
        );
        assert_eq!(
            progress_aria_values(Some(15.0), 10.0, false, None),
            Some(("10".to_string(), "100%".to_string()))
        );
        assert_eq!(
            progress_aria_values(Some(1.0), 4.0, false, Some("1 of 4".into())),
            Some(("1".to_string(), "1 of 4".to_string()))
        );
        assert_eq!(progress_aria_values(Some(5.0), 100.0, true, None), None);
        assert_eq!(progress_aria_values(None, 100.0, false, None), None);
    }

    #[test]
    fn linear_progress_exposes_progressbar_values() {
        let html = render(|| rsx! { Progress { value: Some(30.0), aria_label: "Upload" } });
        assert!(html.contains("role=\"progressbar\""), "{html}");
        assert!(html.contains("aria-valuenow=\"30\""), "{html}");
        assert!(html.contains("aria-valuemin=\"0\""), "{html}");
        assert!(html.contains("aria-valuemax=\"100\""), "{html}");
        assert!(html.contains("aria-valuetext=\"30%\""), "{html}");
        assert!(html.contains("aria-label=\"Upload\""), "{html}");
        assert!(!html.contains("aria-busy"), "{html}");
    }

    #[test]
    fn indeterminate_circular_progress_omits_valuenow() {
        let html = render(|| {
            rsx! { Progress { variant: ProgressVariant::Circular, indeterminate: true } }
        });
        assert!(html.contains("role=\"progressbar\""), "{html}");
        assert!(!html.contains("aria-valuenow"), "{html}");
        assert!(!html.contains("aria-valuetext"), "{html}");
        assert!(html.contains("aria-busy=\"true\""), "{html}");
        assert!(html.contains("aria-label=\"Loading\""), "{html}");
    }
}
