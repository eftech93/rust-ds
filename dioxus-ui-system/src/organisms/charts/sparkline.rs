//! Sparkline component
//!
//! Mini line charts for dashboards and inline use.

use dioxus::prelude::*;
use crate::theme::use_theme;
use crate::organisms::charts::common::*;
use crate::theme::tokens::Color;

/// Sparkline variant
#[derive(Default, Clone, PartialEq, Debug)]
pub enum SparklineVariant {
    /// Simple line (default)
    #[default]
    Line,
    /// Smooth curve
    Smooth,
    /// Area fill below line
    Area,
    /// Bar chart style
    Bars,
}

/// Sparkline properties
#[derive(Props, Clone, PartialEq)]
pub struct SparklineProps {
    /// Data values
    pub data: Vec<f64>,
    /// Chart width
    #[props(default = "120px".to_string())]
    pub width: String,
    /// Chart height
    #[props(default = "30px".to_string())]
    pub height: String,
    /// Sparkline variant
    #[props(default)]
    pub variant: SparklineVariant,
    /// Line/bar color
    #[props(default)]
    pub color: Option<Color>,
    /// Fill color for area (uses color with opacity if not set)
    #[props(default)]
    pub fill_color: Option<Color>,
    /// Line width
    #[props(default = 1.5)]
    pub line_width: f32,
    /// Show current value indicator
    #[props(default = false)]
    pub show_last_point: bool,
    /// Show min/max indicators
    #[props(default = false)]
    pub show_min_max: bool,
    /// Animation configuration
    #[props(default)]
    pub animation: ChartAnimation,
    /// Click handler
    #[props(default)]
    pub onclick: Option<EventHandler<()>>,
    /// Custom styles
    #[props(default)]
    pub style: Option<String>,
}

/// Sparkline component
#[component]
pub fn Sparkline(props: SparklineProps) -> Element {
    let theme = use_theme();
    let tokens = theme.tokens.read();
    
    if props.data.is_empty() {
        return rsx! {
            div {
                style: "width: {props.width}; height: {props.height};",
            }
        };
    }
    
    // Get color
    let color = props.color.clone().unwrap_or_else(|| tokens.colors.primary.clone());
    let color_css = color.to_rgba();
    
    // Fill color (default to color with opacity)
    let fill_css = props.fill_color.as_ref().map(|c| c.to_rgba()).unwrap_or_else(|| {
        format!("rgba({}, {}, {}, 0.2)", color.r, color.g, color.b)
    });
    
    // Calculate dimensions
    let svg_width = 120.0;
    let svg_height = 30.0;
    let padding = 2.0;
    
    let chart_width = svg_width - padding * 2.0;
    let chart_height = svg_height - padding * 2.0;
    
    // Calculate value range
    let min_value = props.data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_value = props.data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    
    // Add some padding to the range
    let range = max_value - min_value;
    let y_min = min_value - range * 0.1;
    let y_max = max_value + range * 0.1;
    let effective_range = y_max - y_min;
    
    // Clone data first before using in closures
    let data = props.data.clone();
    let data_len = data.len();
    
    // Scales
    let x_scale = move |idx: usize| -> f64 {
        if data_len <= 1 {
            padding
        } else {
            padding + idx as f64 / (data_len - 1) as f64 * chart_width
        }
    };
    
    let y_scale = move |value: f64| -> f64 {
        if effective_range == 0.0 {
            svg_height / 2.0
        } else {
            svg_height - padding - (value - y_min) / effective_range * chart_height
        }
    };
    
    let container_style = format!(
        "width: {}; height: {}; display: inline-block; {}",
        props.width,
        props.height,
        props.style.as_deref().unwrap_or("")
    );
    
    let variant = props.variant.clone();
    let show_last_point = props.show_last_point;
    let show_min_max = props.show_min_max;
    let line_width = props.line_width;
    let onclick = props.onclick.clone();
    
    // Calculate points for line/area
    let points: Vec<(f64, f64)> = data.iter().enumerate()
        .map(|(idx, &value)| (x_scale(idx), y_scale(value)))
        .collect();
    
    // Line/area path
    let path_d = match variant {
        SparklineVariant::Smooth => calculate_smooth_line(&points),
        SparklineVariant::Bars => String::new(),
        _ => {
            let mut d = format!("M {},{} ", points[0].0, points[0].1);
            for point in &points[1..] {
                d.push_str(&format!("L {},{} ", point.0, point.1));
            }
            d
        }
    };
    
    // Area path
    let area_path = if variant == SparklineVariant::Area {
        let baseline_y = y_scale(y_min);
        let mut d = format!("M {},{} ", points[0].0, baseline_y);
        d.push_str(&format!("L {},{} ", points[0].0, points[0].1));
        for point in &points[1..] {
            d.push_str(&format!("L {},{} ", point.0, point.1));
        }
        d.push_str(&format!("L {},{} Z", points[points.len()-1].0, baseline_y));
        Some(d)
    } else {
        None
    };
    
    // Min/max positions
    let min_pos: Option<(f64, f64)> = if show_min_max {
        data.iter().enumerate()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(idx, _)| points[idx])
    } else { None };
    
    let max_pos: Option<(f64, f64)> = if show_min_max {
        data.iter().enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(idx, _)| points[idx])
    } else { None };
    
    // Last point
    let last_point: Option<(f64, f64)> = if show_last_point {
        points.last().cloned()
    } else { None };
    
    // Bar data: (idx, x, y, width, height)
    let bar_data: Vec<(usize, f64, f64, f64, f64)> = if variant == SparklineVariant::Bars {
        let bar_width = chart_width / data.len() as f64 * 0.7;
        let bar_gap = chart_width / data.len() as f64 * 0.3;
        let baseline_y = y_scale(0.0_f64.max(y_min));
        
        data.iter().enumerate().map(|(idx, &value)| {
            let x = padding + idx as f64 * (bar_width + bar_gap) + bar_gap / 2.0;
            let y = y_scale(value);
            let height = (baseline_y - y).abs();
            let bar_y = y.min(baseline_y);
            (idx, x, bar_y, bar_width, height)
        }).collect()
    } else {
        vec![]
    };
    
    rsx! {
        div {
            style: "{container_style}",
            onclick: move |_| {
                if let Some(handler) = &onclick {
                    handler.call(());
                }
            },
            svg {
                view_box: "0 0 {svg_width} {svg_height}",
                width: "100%",
                height: "100%",
                preserve_aspect_ratio: "none",
                
                // Bars
                if variant == SparklineVariant::Bars {
                    for (idx, x, bar_y, bar_width, height) in bar_data {
                        rect {
                            key: "{idx}",
                            x: "{x}",
                            y: "{bar_y}",
                            width: "{bar_width}",
                            height: "{height}",
                            fill: "{color_css}",
                            rx: "1",
                        }
                    }
                }
                
                // Area fill
                if let Some(area_d) = area_path {
                    path {
                        d: "{area_d}",
                        fill: "{fill_css}",
                        stroke: "none",
                    }
                }
                
                // Line
                if variant != SparklineVariant::Bars {
                    path {
                        d: "{path_d}",
                        fill: "none",
                        stroke: "{color_css}",
                        "stroke-width": "{line_width}",
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                    }
                }
                
                // Min indicator
                if let Some((x, y)) = min_pos {
                    circle {
                        cx: "{x}",
                        cy: "{y}",
                        r: "2",
                        fill: "{tokens.colors.destructive.to_rgba()}",
                    }
                }
                
                // Max indicator
                if let Some((x, y)) = max_pos {
                    circle {
                        cx: "{x}",
                        cy: "{y}",
                        r: "2",
                        fill: "{tokens.colors.success.to_rgba()}",
                    }
                }
                
                // Last point
                if let Some((x, y)) = last_point {
                    circle {
                        cx: "{x}",
                        cy: "{y}",
                        r: "3",
                        fill: "{tokens.colors.background.to_rgba()}",
                        stroke: "{color_css}",
                        "stroke-width": "2",
                    }
                }
            }
        }
    }
}

/// Trend indicator with sparkline
#[component]
pub fn TrendIndicator(
    data: Vec<f64>,
    #[props(default)]
    current_value: Option<f64>,
    #[props(default = "120px".to_string())]
    width: String,
    #[props(default = "40px".to_string())]
    height: String,
    #[props(default)]
    variant: SparklineVariant,
    #[props(default)]
    color: Option<Color>,
    #[props(default = true)]
    show_percentage: bool,
    #[props(default)]
    style: Option<String>,
) -> Element {
    let theme = use_theme();
    let tokens = theme.tokens.read();
    
    if data.len() < 2 {
        return rsx! {
            div {
                style: "width: {width}; height: {height};",
            }
        };
    }
    
    let current = current_value.unwrap_or(*data.last().unwrap());
    let previous = data[data.len() - 2];
    let change = current - previous;
    let pct_change = if previous != 0.0 { (change / previous) * 100.0 } else { 0.0 };
    
    let is_positive = change >= 0.0;
    
    // Use success/destructive colors based on trend
    let trend_color = if is_positive {
        tokens.colors.success.clone()
    } else {
        tokens.colors.destructive.clone()
    };
    
    let sparkline_color = color.clone().unwrap_or_else(|| trend_color.clone());
    
    let arrow = if is_positive { "↑" } else { "↓" };
    let pct_text = format!("{:.1}%", pct_change.abs());
    let color_css = trend_color.to_rgba();
    
    rsx! {
        div {
            style: "display: inline-flex; align-items: center; gap: 8px;",
            Sparkline {
                data: data,
                width: width,
                height: "24px".to_string(),
                variant: variant,
                color: Some(sparkline_color),
                show_last_point: true,
            }
            if show_percentage {
                span {
                    style: "font-size: 12px; font-weight: 500; color: {color_css};",
                    "{arrow} {pct_text}"
                }
            }
        }
    }
}
