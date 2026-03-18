//! Bar Chart component
//!
//! A flexible bar chart for comparing categorical data.

use dioxus::prelude::*;
use crate::theme::use_theme;
use crate::organisms::charts::common::*;
use crate::theme::tokens::Color;

/// Bar chart variant
#[derive(Default, Clone, PartialEq, Debug)]
pub enum BarChartVariant {
    /// Vertical bars (default)
    #[default]
    Vertical,
    /// Horizontal bars
    Horizontal,
    /// Stacked bars (for multi-series)
    Stacked,
    /// Grouped bars (for multi-series)
    Grouped,
}

/// Bar chart properties
#[derive(Props, Clone, PartialEq)]
pub struct BarChartProps {
    /// Chart title
    #[props(default)]
    pub title: Option<String>,
    /// Single series data (for simple charts)
    #[props(default)]
    pub data: Option<Vec<ChartDataPoint>>,
    /// Multiple series data (for complex charts)
    #[props(default)]
    pub series: Option<Vec<ChartSeries>>,
    /// Chart width (e.g., "100%", "600px")
    #[props(default = "100%".to_string())]
    pub width: String,
    /// Chart height
    #[props(default = "300px".to_string())]
    pub height: String,
    /// Chart variant
    #[props(default)]
    pub variant: BarChartVariant,
    /// Chart margins
    #[props(default)]
    pub margin: ChartMargin,
    /// X-axis configuration
    #[props(default)]
    pub x_axis: ChartAxis,
    /// Y-axis configuration
    #[props(default)]
    pub y_axis: ChartAxis,
    /// Bar color (for single series)
    #[props(default)]
    pub bar_color: Option<Color>,
    /// Bar corner radius
    #[props(default = 4)]
    pub bar_radius: u8,
    /// Gap between bars (0.0 - 1.0, as percentage of bar width)
    #[props(default = 0.2)]
    pub bar_gap: f32,
    /// Show values on bars
    #[props(default = false)]
    pub show_values: bool,
    /// Value formatter
    #[props(default)]
    pub value_format: Option<fn(f64) -> String>,
    /// Legend position
    #[props(default)]
    pub legend_position: LegendPosition,
    /// Tooltip configuration
    #[props(default)]
    pub tooltip: ChartTooltip,
    /// Animation configuration
    #[props(default)]
    pub animation: ChartAnimation,
    /// Click handler for bars
    #[props(default)]
    pub on_bar_click: Option<EventHandler<ChartDataPoint>>,
    /// Custom styles
    #[props(default)]
    pub style: Option<String>,
}

/// Bar chart component
#[component]
pub fn BarChart(props: BarChartProps) -> Element {
    let theme = use_theme();
    let tokens = theme.tokens.read();
    
    // Tooltip state
    let mut tooltip_state = use_signal(|| None as Option<(i32, i32, String)>);
    
    // Collect all data
    let all_series: Vec<ChartSeries> = if let Some(series) = &props.series {
        series.clone()
    } else if let Some(data) = &props.data {
        vec![ChartSeries::new(
            "Series 1",
            props.bar_color.clone().unwrap_or_else(|| tokens.colors.primary.clone()),
            data.clone()
        )]
    } else {
        vec![]
    };
    
    if all_series.is_empty() || all_series[0].data.is_empty() {
        return rsx! {
            div {
                style: "width: {props.width}; height: {props.height}; display: flex; align-items: center; justify-content: center;",
                "No data"
            }
        };
    }
    
    // Calculate dimensions
    let margin = props.margin.clone();
    let svg_width = 800;
    let svg_height = 400;
    let chart_width = svg_width - margin.left - margin.right;
    let chart_height = svg_height - margin.top - margin.bottom;
    
    // Calculate value range
    let (y_min, y_max) = match props.variant {
        BarChartVariant::Stacked => {
            let mut min_val = f64::INFINITY;
            let mut max_val = f64::NEG_INFINITY;
            let data_len = all_series[0].data.len();
            for i in 0..data_len {
                let sum: f64 = all_series.iter().map(|s| s.data[i].value).sum();
                min_val = min_val.min(sum);
                max_val = max_val.max(sum);
            }
            (0.0_f64.min(min_val), max_val)
        }
        _ => {
            let all_values: Vec<f64> = all_series.iter()
                .flat_map(|s| s.data.iter().map(|p| p.value))
                .collect();
            (
                all_values.iter().fold(f64::INFINITY, |a, &b| a.min(b)),
                all_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
            )
        }
    };
    
    let y_min = props.y_axis.min.unwrap_or(y_min);
    let y_max = props.y_axis.max.unwrap_or(y_max * 1.1);
    
    // Calculate Y ticks
    let y_ticks = calculate_nice_ticks(y_min, y_max, props.y_axis.tick_count);
    
    // Calculate scales
    let y_scale = move |value: f64| -> f64 {
        let range = y_max - y_min;
        if range == 0.0 {
            chart_height as f64 / 2.0
        } else {
            chart_height as f64 - ((value - y_min) / range * chart_height as f64)
        }
    };
    
    // Calculate bar dimensions
    let data_len = all_series[0].data.len();
    let category_width = chart_width as f64 / data_len as f64;
    
    let (bar_width, group_gap) = match props.variant {
        BarChartVariant::Grouped => {
            let series_count = all_series.len();
            let total_gap = category_width * props.bar_gap as f64;
            let available_width = category_width - total_gap;
            let bw = available_width / series_count as f64;
            (bw, total_gap / 2.0)
        }
        _ => {
            let gap = category_width * props.bar_gap as f64;
            (category_width - gap, gap / 2.0)
        }
    };
    
    // Base zero line
    let zero_y = y_scale(0.0);
    let zero_line_y = zero_y + margin.top as f64;
    let x2_for_grid = margin.left + chart_width;
    
    let container_style = format!(
        "width: {}; height: {}; font-family: system-ui, -apple-system, sans-serif; position: relative; {}",
        props.width,
        props.height,
        props.style.as_deref().unwrap_or("")
    );
    
    let title = props.title.clone();
    let variant = props.variant.clone();
    let y_axis = props.y_axis.clone();
    let animation = props.animation.clone();
    let on_bar_click = props.on_bar_click.clone();
    let bar_radius = props.bar_radius;
    let tooltip = props.tooltip.clone();
    
    // Pre-compute all bar data
    let mut bars = Vec::new();
    for (series_idx, series) in all_series.iter().enumerate() {
        for (data_idx, point) in series.data.iter().enumerate() {
            let x = margin.left as f64 + data_idx as f64 * category_width;
            
            let (bar_x, bar_y, bar_w, bar_h) = match variant {
                BarChartVariant::Vertical | BarChartVariant::Stacked | BarChartVariant::Grouped => {
                    let bx = match variant {
                        BarChartVariant::Grouped => x + group_gap + series_idx as f64 * bar_width,
                        _ => x + group_gap,
                    };
                    
                    let (by, bh) = if variant == BarChartVariant::Stacked {
                        let stack_bottom: f64 = all_series[..series_idx].iter()
                            .map(|s| s.data[data_idx].value)
                            .sum();
                        let stack_top = stack_bottom + point.value;
                        let y_top = y_scale(stack_top.max(stack_bottom));
                        let y_bottom = y_scale(stack_bottom.min(stack_top));
                        (y_top, y_bottom - y_top)
                    } else {
                        let y_val = y_scale(point.value);
                        if point.value >= 0.0 {
                            (y_val, zero_y - y_val)
                        } else {
                            (zero_y, y_val - zero_y)
                        }
                    };
                    
                    (bx, by, bar_width, bh)
                }
                BarChartVariant::Horizontal => {
                    let by = x + group_gap;
                    let bx = margin.left as f64;
                    let bw = (point.value / y_max * chart_width as f64).max(0.0);
                    let bh = bar_width;
                    (bx, by, bw, bh)
                }
            };
            
            let color = point.color.clone().unwrap_or_else(|| series.color.clone());
            let radius = bar_radius as f64;
            
            let path = if variant == BarChartVariant::Horizontal {
                format!(
                    "M {bx},{by} L {x2},{by} Q {x3},{by} {x3},{y2} L {x3},{y3} Q {x3},{y4} {x2},{y4} Z",
                    bx = bar_x,
                    by = bar_y,
                    x2 = bar_x + bar_w - radius,
                    x3 = bar_x + bar_w,
                    y2 = bar_y + radius,
                    y3 = bar_y + bar_h - radius,
                    y4 = bar_y + bar_h,
                )
            } else {
                format!(
                    "M {bx},{by} L {x2},{by} Q {x3},{by} {x3},{y2} L {x3},{y3} Q {x3},{y4} {x2},{y4} L {bx},{y4} Z",
                    bx = bar_x,
                    by = bar_y,
                    x2 = bar_x + bar_w,
                    y2 = bar_y + radius,
                    y3 = bar_y + bar_h - radius,
                    x3 = bar_x + bar_w - radius,
                    y4 = bar_y + bar_h,
                )
            };
            
            let color_css = color.to_rgba();
            let tooltip_content = tooltip.get_content(point, Some(&series.name));
            bars.push((series_idx, data_idx, path, color_css, point.clone(), bar_x, bar_y, bar_w, bar_h, tooltip_content, series.name.clone()));
        }
    }
    
    // Pre-compute Y axis labels: (y_position, label)
    let y_labels: Vec<(f64, String)> = y_ticks.iter().map(|&tick| {
        let y_pos = y_scale(tick) + margin.top as f64;
        let label = if let Some(formatter) = y_axis.label_format {
            formatter(tick)
        } else {
            format_compact_number(tick)
        };
        (y_pos, label)
    }).collect();
    
    // Pre-compute X axis labels: (x_position, label_text)
    let x_labels: Vec<(f64, String)> = all_series[0].data.iter().enumerate().map(|(idx, point)| {
        let x_pos = margin.left as f64 + idx as f64 * category_width + category_width / 2.0;
        (x_pos, point.label.clone())
    }).collect();
    
    // Title position
    let title_x = svg_width / 2;
    let x_labels_y = margin.top + chart_height + 20;
    
    // Tooltip styling
    let tooltip_bg = tokens.colors.popover.to_rgba();
    let tooltip_fg = tokens.colors.popover_foreground.to_rgba();
    let tooltip_border = tokens.colors.border.to_rgba();
    
    rsx! {
        div {
            style: "{container_style}",
            
            // Tooltip
            if tooltip.enabled {
                if let Some((x, y, content)) = tooltip_state() {
                    div {
                        style: "position: fixed; left: {x}px; top: {y}px; transform: translate(-50%, -100%); margin-top: -8px; padding: 8px 12px; background: {tooltip_bg}; color: {tooltip_fg}; border: 1px solid {tooltip_border}; border-radius: 6px; font-size: 12px; font-weight: 500; white-space: nowrap; pointer-events: none; z-index: 10000; box-shadow: 0 4px 6px -1px rgba(0,0,0,0.1);",
                        "{content}"
                    }
                }
            }
            
            svg {
                view_box: "0 0 {svg_width} {svg_height}",
                width: "100%",
                height: "100%",
                preserve_aspect_ratio: "xMidYMid meet",
                
                // Title
                if let Some(t) = title {
                    text {
                        x: "{title_x}",
                        y: "20",
                        "text-anchor": "middle",
                        "font-size": "16",
                        "font-weight": "bold",
                        fill: "{tokens.colors.foreground.to_rgba()}",
                        "{t}"
                    }
                }
                
                // Grid lines
                if y_axis.show_grid {
                    for (y_pos, _label) in y_labels.clone() {
                        line {
                            x1: "{margin.left}",
                            y1: "{y_pos}",
                            x2: "{x2_for_grid}",
                            y2: "{y_pos}",
                            stroke: "{tokens.colors.border.to_rgba()}",
                            "stroke-width": "1",
                            "stroke-dasharray": "2,2",
                        }
                    }
                }
                
                // Zero line
                if y_min < 0.0 && y_max > 0.0 {
                    line {
                        x1: "{margin.left}",
                        y1: "{zero_line_y}",
                        x2: "{x2_for_grid}",
                        y2: "{zero_line_y}",
                        stroke: "{tokens.colors.foreground.to_rgba()}",
                        "stroke-width": "2",
                    }
                }
                
                // Bars - render using BarElement component pattern
                BarElements {
                    bars: bars,
                    animation_enabled: animation.enabled,
                    on_bar_click: on_bar_click,
                    tooltip_enabled: tooltip.enabled,
                    on_tooltip_show: EventHandler::new(move |(x, y, content): (i32, i32, String)| {
                        tooltip_state.set(Some((x, y, content)));
                    }),
                    on_tooltip_hide: EventHandler::new(move |_| {
                        tooltip_state.set(None);
                    }),
                }
                
                // Y axis labels
                for (y_pos, label) in y_labels {
                    text {
                        x: "{margin.left - 10}",
                        y: "{y_pos}",
                        "text-anchor": "end",
                        "dominant-baseline": "middle",
                        "font-size": "12",
                        fill: "{tokens.colors.muted_foreground.to_rgba()}",
                        "{label}"
                    }
                }
                
                // X axis labels
                for (x_pos, label) in x_labels {
                    text {
                        x: "{x_pos}",
                        y: "{x_labels_y}",
                        "text-anchor": "middle",
                        "dominant-baseline": "top",
                        "font-size": "12",
                        fill: "{tokens.colors.muted_foreground.to_rgba()}",
                        "{label}"
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct BarElementsProps {
    bars: Vec<(usize, usize, String, String, ChartDataPoint, f64, f64, f64, f64, String, String)>,
    animation_enabled: bool,
    on_bar_click: Option<EventHandler<ChartDataPoint>>,
    tooltip_enabled: bool,
    on_tooltip_show: Option<EventHandler<(i32, i32, String)>>,
    on_tooltip_hide: Option<EventHandler<()>>,
}

#[component]
fn BarElements(props: BarElementsProps) -> Element {
    rsx! {
        for (series_idx, data_idx, path, color_css, point, bar_x, bar_y, bar_w, bar_h, tooltip_content, _series_name) in props.bars {
            BarPath {
                key: "{series_idx}-{data_idx}",
                path: path,
                color_css: color_css,
                animation_enabled: props.animation_enabled,
                point: point,
                on_bar_click: props.on_bar_click.clone(),
                tooltip_enabled: props.tooltip_enabled,
                bar_x: bar_x,
                bar_y: bar_y,
                bar_w: bar_w,
                bar_h: bar_h,
                tooltip_content: tooltip_content,
                on_tooltip_show: props.on_tooltip_show.clone(),
                on_tooltip_hide: props.on_tooltip_hide.clone(),
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct BarPathProps {
    path: String,
    color_css: String,
    animation_enabled: bool,
    point: ChartDataPoint,
    on_bar_click: Option<EventHandler<ChartDataPoint>>,
    tooltip_enabled: bool,
    bar_x: f64,
    bar_y: f64,
    bar_w: f64,
    bar_h: f64,
    tooltip_content: String,
    on_tooltip_show: Option<EventHandler<(i32, i32, String)>>,
    on_tooltip_hide: Option<EventHandler<()>>,
}

#[component]
fn BarPath(props: BarPathProps) -> Element {
    let on_click = props.on_bar_click.clone();
    let point = props.point.clone();
    let tooltip_content = props.tooltip_content.clone();
    
    // Calculate center of bar for tooltip position
    let center_x = (props.bar_x + props.bar_w / 2.0) as i32;
    let center_y = props.bar_y as i32;
    
    rsx! {
        path {
            d: "{props.path}",
            fill: "{props.color_css}",
            opacity: if props.animation_enabled { "0" } else { "1" },
            onclick: move |_| {
                if let Some(handler) = &on_click {
                    handler.call(point.clone());
                }
            },
            onmouseenter: move |e: Event<MouseData>| {
                if props.tooltip_enabled {
                    if let Some(handler) = &props.on_tooltip_show {
                        let coords = e.data().page_coordinates();
                        handler.call((coords.x as i32, coords.y as i32, tooltip_content.clone()));
                    }
                }
            },
            onmouseleave: move |_| {
                if props.tooltip_enabled {
                    if let Some(handler) = &props.on_tooltip_hide {
                        handler.call(());
                    }
                }
            },
        }
    }
}
