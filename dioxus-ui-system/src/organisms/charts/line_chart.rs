//! Line Chart component
//!
//! A flexible line chart for showing trends over time or categories.

#![allow(unpredictable_function_pointer_comparisons)]

use crate::atoms::Box;
use crate::organisms::charts::common::*;
use crate::theme::tokens::Color;
use crate::theme::use_theme;
use dioxus::prelude::*;

/// Line chart variant
#[derive(Default, Clone, PartialEq, Debug)]
pub enum LineChartVariant {
    /// Simple line chart (default)
    #[default]
    Line,
    /// Smooth curved lines
    Smooth,
    /// Step chart (right angles)
    Step,
    /// Area chart (filled below line)
    Area,
    /// Stacked area chart
    StackedArea,
}

/// Line chart properties
#[derive(Props, Clone, PartialEq)]
pub struct LineChartProps {
    /// Chart title
    #[props(default)]
    pub title: Option<String>,
    /// Single series data
    #[props(default)]
    pub data: Option<Vec<ChartDataPoint>>,
    /// Multiple series data
    #[props(default)]
    pub series: Option<Vec<ChartSeries>>,
    /// Chart width
    #[props(default = "100%".to_string())]
    pub width: String,
    /// Chart height
    #[props(default = "300px".to_string())]
    pub height: String,
    /// Chart variant
    #[props(default)]
    pub variant: LineChartVariant,
    /// Chart margins
    #[props(default)]
    pub margin: ChartMargin,
    /// X-axis configuration
    #[props(default)]
    pub x_axis: ChartAxis,
    /// Y-axis configuration
    #[props(default)]
    pub y_axis: ChartAxis,
    /// Line color (for single series)
    #[props(default)]
    pub line_color: Option<Color>,
    /// Line width
    #[props(default = 2)]
    pub line_width: u8,
    /// Show data points
    #[props(default = true)]
    pub show_points: bool,
    /// Point radius
    #[props(default = 4)]
    pub point_radius: u8,
    /// Show values on points
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
    /// Click handler for points
    #[props(default)]
    pub on_point_click: Option<EventHandler<ChartDataPoint>>,
    /// Custom styles
    #[props(default)]
    pub style: Option<String>,
}

/// Line chart component
#[component]
pub fn LineChart(props: LineChartProps) -> Element {
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
            props
                .line_color
                .clone()
                .unwrap_or_else(|| tokens.colors.primary.clone()),
            data.clone(),
        )]
    } else {
        vec![]
    };

    if all_series.is_empty() || all_series[0].data.is_empty() {
        return rsx! {
            Box {
                width: Some(props.width.clone()),
                height: Some(props.height.clone()),
                display: crate::atoms::BoxDisplay::Flex,
                align_items: crate::atoms::AlignItems::Center,
                justify_content: crate::atoms::JustifyContent::Center,
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
    let (min_value, max_value) = match props.variant {
        LineChartVariant::StackedArea => {
            let data_len = all_series[0].data.len();
            let mut min_val = f64::INFINITY;
            let mut max_val = f64::NEG_INFINITY;
            for i in 0..data_len {
                let sum: f64 = all_series.iter().map(|s| s.data[i].value).sum();
                min_val = min_val.min(sum);
                max_val = max_val.max(sum);
            }
            (0.0_f64.min(min_val), max_val)
        }
        _ => {
            let all_values: Vec<f64> = all_series
                .iter()
                .flat_map(|s| s.data.iter().map(|p| p.value))
                .collect();
            (
                all_values.iter().fold(f64::INFINITY, |a, &b| a.min(b)),
                all_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)),
            )
        }
    };

    let y_min = props.y_axis.min.unwrap_or(min_value);
    let y_max = props.y_axis.max.unwrap_or(max_value * 1.1);

    // Calculate scales
    let y_scale = move |value: f64| -> f64 {
        let range = y_max - y_min;
        if range == 0.0 {
            chart_height as f64 / 2.0
        } else {
            chart_height as f64 - ((value - y_min) / range * chart_height as f64)
        }
    };

    let x_scale = move |index: usize, total: usize| -> f64 {
        if total <= 1 {
            0.0
        } else {
            index as f64 / (total - 1) as f64 * chart_width as f64
        }
    };

    // Calculate Y ticks
    let y_ticks = calculate_nice_ticks(y_min, y_max, props.y_axis.tick_count);

    // Generate series paths and points
    let data_len = all_series[0].data.len();

    let container_style = format!(
        "width: {}; height: {}; font-family: system-ui, -apple-system, sans-serif; position: relative; {}",
        props.width,
        props.height,
        props.style.as_deref().unwrap_or("")
    );

    let title = props.title.clone();
    let variant = props.variant.clone();
    let y_axis = props.y_axis.clone();
    let show_points = props.show_points;
    let point_radius = props.point_radius;
    let line_width = props.line_width;
    let on_point_click = props.on_point_click.clone();
    let tooltip = props.tooltip.clone();

    // Pre-compute Y axis data: (y_position, label, x2_for_grid)
    let y_axis_data: Vec<(f64, String, u16)> = y_ticks
        .iter()
        .map(|&tick| {
            let y = margin.top as f64 + y_scale(tick);
            let label = if let Some(formatter) = y_axis.label_format {
                formatter(tick)
            } else {
                format_compact_number(tick)
            };
            let x2 = margin.left + chart_width;
            (y, label, x2)
        })
        .collect();

    // Pre-compute series data with tooltip content
    let mut series_data = Vec::new();
    for (series_idx, series) in all_series.iter().enumerate() {
        let color = series.color.clone();
        let color_css = color.to_rgba();

        let points: Vec<(f64, f64)> = (0..data_len)
            .map(|i| {
                let x = margin.left as f64 + x_scale(i, data_len);
                let y_val = match variant {
                    LineChartVariant::StackedArea => {
                        let stack_bottom: f64 = all_series[..series_idx]
                            .iter()
                            .map(|s| s.data[i].value)
                            .sum();
                        stack_bottom + series.data[i].value
                    }
                    _ => series.data[i].value,
                };
                let y = margin.top as f64 + y_scale(y_val);
                (x, y)
            })
            .collect();

        let path_d = match variant {
            LineChartVariant::Step => {
                let mut d = format!("M {},{} ", points[0].0, points[0].1);
                for i in 1..points.len() {
                    let prev = points[i - 1];
                    let curr = points[i];
                    d.push_str(&format!("L {},{} L {},{} ", curr.0, prev.1, curr.0, curr.1));
                }
                d
            }
            LineChartVariant::Smooth => calculate_smooth_line(&points),
            _ => {
                let mut d = format!("M {},{} ", points[0].0, points[0].1);
                for point in &points[1..] {
                    d.push_str(&format!("L {},{} ", point.0, point.1));
                }
                d
            }
        };

        let area_path = if matches!(
            variant,
            LineChartVariant::Area | LineChartVariant::StackedArea
        ) {
            if variant == LineChartVariant::StackedArea && series_idx > 0 {
                let prev_points: Vec<(f64, f64)> = (0..data_len)
                    .map(|i| {
                        let x = margin.left as f64 + x_scale(i, data_len);
                        let stack_bottom: f64 = all_series[..series_idx]
                            .iter()
                            .map(|s| s.data[i].value)
                            .sum();
                        let y = margin.top as f64 + y_scale(stack_bottom);
                        (x, y)
                    })
                    .collect();

                let mut d = format!("M {},{} ", points[0].0, points[0].1);
                for point in &points[1..] {
                    d.push_str(&format!("L {},{} ", point.0, point.1));
                }
                for point in prev_points.iter().rev() {
                    d.push_str(&format!("L {},{} ", point.0, point.1));
                }
                d.push_str("Z");
                Some(d)
            } else {
                let baseline = margin.top as f64 + chart_height as f64;
                let mut d = format!("M {},{} ", points[0].0, baseline);
                d.push_str(&format!("L {},{} ", points[0].0, points[0].1));
                for point in &points[1..] {
                    d.push_str(&format!("L {},{} ", point.0, point.1));
                }
                d.push_str(&format!("L {},{} ", points[points.len() - 1].0, baseline));
                d.push_str("Z");
                Some(d)
            }
        } else {
            None
        };

        let area_color = format!("rgba({}, {}, {}, 0.2)", color.r, color.g, color.b);
        let show_line = !matches!(
            variant,
            LineChartVariant::Area | LineChartVariant::StackedArea
        ) || series_idx < all_series.len() - 1;

        // Pre-compute tooltip content for each point
        let tooltip_contents: Vec<String> = series
            .data
            .iter()
            .map(|point| tooltip.get_content(point, Some(&series.name)))
            .collect();

        series_data.push((
            series_idx,
            path_d,
            area_path,
            area_color,
            color_css,
            points,
            show_line,
            tooltip_contents,
            series.name.clone(),
        ));
    }

    // Pre-compute X axis labels: (x_position, label_text)
    let x_labels: Vec<(f64, String)> = all_series[0]
        .data
        .iter()
        .enumerate()
        .map(|(idx, point)| {
            let x = margin.left as f64 + x_scale(idx, data_len);
            (x, point.label.clone())
        })
        .collect();

    let title_x = svg_width / 2;
    let x_labels_y = margin.top + chart_height + 20;
    let bg_color = tokens.colors.background.to_rgba();

    // Tooltip styling
    let tooltip_bg = tokens.colors.popover.to_rgba();
    let tooltip_fg = tokens.colors.popover_foreground.to_rgba();
    let tooltip_border = tokens.colors.border.to_rgba();

    rsx! {
        Box {
            width: Some(props.width.clone()),
            height: Some(props.height.clone()),
            style: Some(container_style),

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

                // Y axis elements
                for (y, label, x2) in y_axis_data {
                    // Grid line
                    if y_axis.show_grid {
                        line {
                            x1: "{margin.left}",
                            y1: "{y}",
                            x2: "{x2}",
                            y2: "{y}",
                            stroke: "{tokens.colors.border.to_rgba()}",
                            "stroke-width": "1",
                            "stroke-dasharray": "2,2",
                        }
                    }

                    // Label
                    text {
                        x: "{margin.left - 10}",
                        y: "{y}",
                        "text-anchor": "end",
                        "dominant-baseline": "middle",
                        "font-size": "12",
                        fill: "{tokens.colors.muted_foreground.to_rgba()}",
                        "{label}"
                    }
                }

                // Series (lines, areas, points)
                for (series_idx, path_d, area_path, area_color, color_css, points, show_line, tooltip_contents, series_name) in series_data {
                    LineSeries {
                        path_d: path_d,
                        area_path: area_path,
                        area_color: area_color,
                        color_css: color_css,
                        points: points,
                        show_line: show_line,
                        show_points: show_points && !matches!(variant, LineChartVariant::StackedArea),
                        point_radius: point_radius,
                        line_width: line_width,
                        bg_color: bg_color.clone(),
                        series_data: all_series[series_idx].data.clone(),
                        tooltip_contents: tooltip_contents,
                        series_name: series_name,
                        on_point_click: on_point_click.clone(),
                        tooltip_enabled: tooltip.enabled,
                        on_tooltip_show: Some(EventHandler::new(move |(x, y, content): (i32, i32, String)| {
                            tooltip_state.set(Some((x, y, content)));
                        })),
                        on_tooltip_hide: Some(EventHandler::new(move |_| {
                            tooltip_state.set(None);
                        })),
                    }
                }

                // X axis labels
                for (x, label) in x_labels {
                    text {
                        x: "{x}",
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
struct LineSeriesProps {
    path_d: String,
    area_path: Option<String>,
    area_color: String,
    color_css: String,
    points: Vec<(f64, f64)>,
    show_line: bool,
    show_points: bool,
    point_radius: u8,
    line_width: u8,
    bg_color: String,
    series_data: Vec<ChartDataPoint>,
    tooltip_contents: Vec<String>,
    series_name: String,
    on_point_click: Option<EventHandler<ChartDataPoint>>,
    tooltip_enabled: bool,
    on_tooltip_show: Option<EventHandler<(i32, i32, String)>>,
    on_tooltip_hide: Option<EventHandler<()>>,
}

#[component]
fn LineSeries(props: LineSeriesProps) -> Element {
    rsx! {
        g {
            // Area fill
            if let Some(area_d) = props.area_path {
                path {
                    d: "{area_d}",
                    fill: "{props.area_color}",
                    stroke: "none",
                }
            }

            // Line
            if props.show_line {
                path {
                    d: "{props.path_d}",
                    fill: "none",
                    stroke: "{props.color_css}",
                    "stroke-width": "{props.line_width}",
                    "stroke-linecap": "round",
                    "stroke-linejoin": "round",
                }
            }

            // Data points
            if props.show_points {
                for (i, (x, y)) in props.points.iter().enumerate() {
                    LinePoint {
                        x: *x,
                        y: *y,
                        point_radius: props.point_radius,
                        bg_color: props.bg_color.clone(),
                        color_css: props.color_css.clone(),
                        point: props.series_data[i].clone(),
                        tooltip_content: props.tooltip_contents[i].clone(),
                        series_name: props.series_name.clone(),
                        on_point_click: props.on_point_click.clone(),
                        tooltip_enabled: props.tooltip_enabled,
                        on_tooltip_show: props.on_tooltip_show.clone(),
                        on_tooltip_hide: props.on_tooltip_hide.clone(),
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct LinePointProps {
    x: f64,
    y: f64,
    point_radius: u8,
    bg_color: String,
    color_css: String,
    point: ChartDataPoint,
    tooltip_content: String,
    series_name: String,
    on_point_click: Option<EventHandler<ChartDataPoint>>,
    tooltip_enabled: bool,
    on_tooltip_show: Option<EventHandler<(i32, i32, String)>>,
    on_tooltip_hide: Option<EventHandler<()>>,
}

#[component]
fn LinePoint(props: LinePointProps) -> Element {
    let on_click = props.on_point_click.clone();
    let point = props.point.clone();
    let tooltip_content = props.tooltip_content.clone();

    rsx! {
        circle {
            cx: "{props.x}",
            cy: "{props.y}",
            r: "{props.point_radius}",
            fill: "{props.bg_color}",
            stroke: "{props.color_css}",
            "stroke-width": "2",
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
