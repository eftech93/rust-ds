//! Pie Chart and Donut Chart components
//!
//! Circular charts for showing part-to-whole relationships.

use dioxus::prelude::*;
use crate::theme::use_theme;
use crate::organisms::charts::common::*;
use crate::theme::tokens::Color;

/// Pie chart variant
#[derive(Default, Clone, PartialEq, Debug)]
pub enum PieChartVariant {
    /// Standard pie chart (default)
    #[default]
    Pie,
    /// Donut chart (hollow center)
    Donut,
    /// Gauge chart (semi-circle)
    Gauge,
}

/// Pie chart properties
#[derive(Props, Clone, PartialEq)]
pub struct PieChartProps {
    /// Chart title
    #[props(default)]
    pub title: Option<String>,
    /// Data points
    pub data: Vec<ChartDataPoint>,
    /// Chart width
    #[props(default = "100%".to_string())]
    pub width: String,
    /// Chart height
    #[props(default = "300px".to_string())]
    pub height: String,
    /// Chart variant
    #[props(default)]
    pub variant: PieChartVariant,
    /// Inner radius (for donut, as percentage of outer radius)
    #[props(default = 0.6)]
    pub inner_radius_pct: f32,
    /// Start angle (in degrees, 0 = 3 o'clock)
    #[props(default = 0.0)]
    pub start_angle: f32,
    /// Corner radius for slices
    #[props(default = 0)]
    pub corner_radius: u8,
    /// Gap between slices (in degrees)
    #[props(default = 1.0)]
    pub slice_gap: f32,
    /// Show labels on slices
    #[props(default = false)]
    pub show_labels: bool,
    /// Show percentage labels
    #[props(default = true)]
    pub show_percentages: bool,
    /// Label formatter
    #[props(default)]
    pub label_format: Option<fn(&ChartDataPoint, f64) -> String>,
    /// Legend position
    #[props(default)]
    pub legend_position: LegendPosition,
    /// Tooltip configuration
    #[props(default)]
    pub tooltip: ChartTooltip,
    /// Animation configuration
    #[props(default)]
    pub animation: ChartAnimation,
    /// Click handler for slices
    #[props(default)]
    pub on_slice_click: Option<EventHandler<ChartDataPoint>>,
    /// Custom styles
    #[props(default)]
    pub style: Option<String>,
    /// Colors for slices (uses theme colors if not provided)
    #[props(default)]
    pub colors: Option<Vec<Color>>,
}

/// Pie/Donut chart component
#[component]
pub fn PieChart(props: PieChartProps) -> Element {
    let theme = use_theme();
    let tokens = theme.tokens.read();
    
    // Tooltip state
    let mut tooltip_state = use_signal(|| None as Option<(i32, i32, String)>);
    
    if props.data.is_empty() {
        return rsx! {
            div {
                style: "width: {props.width}; height: {props.height}; display: flex; align-items: center; justify-content: center;",
                "No data"
            }
        };
    }
    
    // Calculate total value
    let total: f64 = props.data.iter().map(|p| p.value.abs()).sum();
    
    if total == 0.0 {
        return rsx! {
            div {
                style: "width: {props.width}; height: {props.height}; display: flex; align-items: center; justify-content: center;",
                "No valid data"
            }
        };
    }
    
    // Default colors from theme
    let default_colors = vec![
        tokens.colors.primary.clone(),
        Color::new(59, 130, 246),   // Blue
        Color::new(34, 197, 94),    // Green
        Color::new(234, 179, 8),    // Yellow
        Color::new(239, 68, 68),    // Red
        Color::new(168, 85, 247),   // Purple
        Color::new(236, 72, 153),   // Pink
        Color::new(20, 184, 166),   // Teal
        Color::new(249, 115, 22),   // Orange
        Color::new(99, 102, 241),   // Indigo
    ];
    
    let colors = props.colors.clone().unwrap_or(default_colors);
    
    // Calculate dimensions
    let svg_width = 400;
    let svg_height = if props.variant == PieChartVariant::Gauge { 240 } else { 400 };
    let center_x = svg_width as f64 / 2.0;
    let center_y = if props.variant == PieChartVariant::Gauge { 
        svg_height as f64 * 0.85 
    } else { 
        svg_height as f64 / 2.0 
    };
    let radius = if props.variant == PieChartVariant::Gauge {
        (svg_width.min(svg_height) as f64 / 2.0) * 0.8
    } else {
        (svg_width.min(svg_height) as f64 / 2.0) * 0.85
    };
    let inner_radius = radius * props.inner_radius_pct as f64;
    
    // Calculate angles for each slice
    let (start_angle_rad, end_angle_rad) = if props.variant == PieChartVariant::Gauge {
        (std::f64::consts::PI, 0.0)
    } else {
        let start = props.start_angle as f64 * std::f64::consts::PI / 180.0;
        (start, start + 2.0 * std::f64::consts::PI)
    };
    
    let total_angle = end_angle_rad - start_angle_rad;
    let gap_rad = props.slice_gap as f64 * std::f64::consts::PI / 180.0;
    
    // Tooltip and related props
    let tooltip = props.tooltip.clone();
    
    // Prepare slice data
    let mut current_angle = start_angle_rad;
    let mut slice_data = Vec::new();
    
    for (idx, point) in props.data.iter().enumerate() {
        let percentage = point.value.abs() / total;
        let angle = total_angle * percentage;
        let sweep_angle = angle - gap_rad.max(0.0);
        
        let start_a = current_angle;
        let end_a = current_angle + sweep_angle.max(0.01);
        
        // Calculate path
        let path = if props.variant == PieChartVariant::Donut || props.variant == PieChartVariant::Gauge {
            create_arc_path(center_x, center_y, radius, inner_radius, start_a, end_a)
        } else {
            create_pie_slice_path(center_x, center_y, radius, start_a, end_a)
        };
        
        let color = point.color.clone().unwrap_or_else(|| {
            colors.get(idx % colors.len()).cloned().unwrap_or_else(|| tokens.colors.primary.clone())
        });
        let color_css = color.to_rgba();
        
        // Calculate label position
        let mid_angle = (start_a + end_a) / 2.0;
        let label_radius = (radius + inner_radius) / 2.0;
        let label_x = center_x + mid_angle.cos() * label_radius;
        let label_y = center_y + mid_angle.sin() * label_radius;
        
        // Calculate tooltip position (center of slice)
        let tooltip_x = center_x + mid_angle.cos() * ((radius + inner_radius) / 2.0);
        let tooltip_y = center_y + mid_angle.sin() * ((radius + inner_radius) / 2.0);
        
        let pct = percentage * 100.0;
        let label_text = if let Some(formatter) = props.label_format {
            formatter(point, pct)
        } else if props.show_percentages {
            format!("{:.1}%", pct)
        } else {
            point.label.clone()
        };
        
        // Generate tooltip content
        let tooltip_content = tooltip.get_content(point, None);
        
        slice_data.push((idx, path, color_css, label_x, label_y, label_text, point.clone(), tooltip_content, tooltip_x, tooltip_y));
        current_angle += angle;
    }
    
    let container_style = format!(
        "width: {}; height: {}; display: flex; align-items: center; justify-content: center; font-family: system-ui, -apple-system, sans-serif; position: relative; {}",
        props.width,
        props.height,
        props.style.as_deref().unwrap_or("")
    );
    
    let flex_direction = match props.legend_position {
        LegendPosition::Top => "column",
        LegendPosition::Bottom => "column-reverse",
        LegendPosition::Left => "row-reverse",
        LegendPosition::Right => "row",
        _ => "column",
    };
    
    let show_labels = props.show_labels;
    let show_percentages = props.show_percentages;
    let on_slice_click = props.on_slice_click.clone();
    let legend_position = props.legend_position.clone();
    let title = props.title.clone();
    let variant = props.variant.clone();
    
    // Pre-compute center text for donut/gauge
    let center_text = if (variant == PieChartVariant::Donut || variant == PieChartVariant::Gauge) && show_percentages {
        let main_value = props.data.first().map(|p| p.value).unwrap_or(0.0);
        let main_pct = (main_value / total * 100.0) as i32;
        let label = if variant == PieChartVariant::Gauge {
            format!("{}%", main_pct)
        } else {
            format_compact_number(total)
        };
        let y_offset = if variant == PieChartVariant::Gauge { -10.0 } else { 0.0 };
        Some((label, y_offset))
    } else {
        None
    };
    
    // Pre-compute legend items: (idx, color_css, label)
    let legend_items: Vec<(usize, String, String)> = if legend_position != LegendPosition::None {
        props.data.iter().enumerate().map(|(idx, point)| {
            let color = point.color.clone().unwrap_or_else(|| {
                colors.get(idx % colors.len()).cloned().unwrap_or_else(|| tokens.colors.primary.clone())
            });
            let color_css = color.to_rgba();
            (idx, color_css, point.label.clone())
        }).collect()
    } else {
        vec![]
    };
    
    let legend_style = match legend_position {
        LegendPosition::Right => "display: flex; flex-direction: column; gap: 8px; margin-left: 16px;",
        LegendPosition::Left => "display: flex; flex-direction: column; gap: 8px; margin-right: 16px;",
        LegendPosition::Top => "display: flex; flex-wrap: wrap; gap: 12px; margin-bottom: 16px; justify-content: center;",
        LegendPosition::Bottom => "display: flex; flex-wrap: wrap; gap: 12px; margin-top: 16px; justify-content: center;",
        _ => "",
    };
    
    // Tooltip styling
    let tooltip_bg = tokens.colors.popover.to_rgba();
    let tooltip_fg = tokens.colors.popover_foreground.to_rgba();
    let tooltip_border = tokens.colors.border.to_rgba();
    
    rsx! {
        div {
            style: "{container_style}; flex-direction: {flex_direction};",
            
            // Tooltip
            if tooltip.enabled {
                if let Some((x, y, content)) = tooltip_state() {
                    div {
                        style: "position: fixed; left: {x}px; top: {y}px; transform: translate(-50%, -100%); margin-top: -8px; padding: 8px 12px; background: {tooltip_bg}; color: {tooltip_fg}; border: 1px solid {tooltip_border}; border-radius: 6px; font-size: 12px; font-weight: 500; white-space: nowrap; pointer-events: none; z-index: 10000; box-shadow: 0 4px 6px -1px rgba(0,0,0,0.1);",
                        "{content}"
                    }
                }
            }
            
            // Legend
            if legend_position != LegendPosition::None {
                div {
                    style: "{legend_style}",
                    for (idx, color_css, label) in legend_items {
                        div {
                            key: "{idx}",
                            style: "display: flex; align-items: center; gap: 6px; font-size: 12px;",
                            div {
                                style: "width: 12px; height: 12px; border-radius: 2px; background-color: {color_css};"
                            }
                            span { "{label}" }
                        }
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
                        x: "{svg_width / 2}",
                        y: "20",
                        "text-anchor": "middle",
                        "font-size": "16",
                        "font-weight": "bold",
                        fill: "{tokens.colors.foreground.to_rgba()}",
                        "{t}"
                    }
                }
                
                // Slices
                for (idx, path, color_css, label_x, label_y, label_text, point, tooltip_content, _tooltip_x, _tooltip_y) in slice_data.clone() {
                    PieSlice {
                        idx: idx,
                        path: path,
                        color_css: color_css,
                        label_x: label_x,
                        label_y: label_y,
                        label_text: label_text,
                        show_labels: show_labels,
                        point: point,
                        tooltip_content: tooltip_content,
                        on_slice_click: on_slice_click.clone(),
                        tooltip_enabled: tooltip.enabled,
                        on_tooltip_show: Some(EventHandler::new(move |(x, y, content): (i32, i32, String)| {
                            tooltip_state.set(Some((x, y, content)));
                        })),
                        on_tooltip_hide: Some(EventHandler::new(move |_| {
                            tooltip_state.set(None);
                        })),
                    }
                }
                
                // Center text
                if let Some((label, y_offset)) = center_text {
                    text {
                        x: "{center_x}",
                        y: "{center_y + y_offset}",
                        "text-anchor": "middle",
                        "dominant-baseline": "middle",
                        "font-size": if variant == PieChartVariant::Gauge { "32" } else { "24" },
                        "font-weight": "bold",
                        fill: "{tokens.colors.foreground.to_rgba()}",
                        "{label}"
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct PieSliceProps {
    idx: usize,
    path: String,
    color_css: String,
    label_x: f64,
    label_y: f64,
    label_text: String,
    show_labels: bool,
    point: ChartDataPoint,
    tooltip_content: String,
    on_slice_click: Option<EventHandler<ChartDataPoint>>,
    tooltip_enabled: bool,
    on_tooltip_show: Option<EventHandler<(i32, i32, String)>>,
    on_tooltip_hide: Option<EventHandler<()>>,
}

#[component]
fn PieSlice(props: PieSliceProps) -> Element {
    let on_click = props.on_slice_click.clone();
    let point = props.point.clone();
    let tooltip_content = props.tooltip_content.clone();
    
    rsx! {
        g {
            key: "{props.idx}",
            path {
                d: "{props.path}",
                fill: "{props.color_css}",
                stroke: "white",
                "stroke-width": "1",
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
            if props.show_labels {
                text {
                    x: "{props.label_x}",
                    y: "{props.label_y}",
                    "text-anchor": "middle",
                    "dominant-baseline": "middle",
                    "font-size": "11",
                    "font-weight": "bold",
                    fill: "white",
                    "{props.label_text}"
                }
            }
        }
    }
}

/// Create a pie slice path
fn create_pie_slice_path(cx: f64, cy: f64, r: f64, start_angle: f64, end_angle: f64) -> String {
    let x1 = cx + r * start_angle.cos();
    let y1 = cy + r * start_angle.sin();
    let x2 = cx + r * end_angle.cos();
    let y2 = cy + r * end_angle.sin();
    
    let large_arc = if end_angle - start_angle > std::f64::consts::PI { 1 } else { 0 };
    
    format!(
        "M {},{} L {},{} A {},{} 0 {},1 {},{} Z",
        cx, cy, x1, y1, r, r, large_arc, x2, y2
    )
}

/// Create an arc path (for donut/gauge)
fn create_arc_path(cx: f64, cy: f64, outer_r: f64, inner_r: f64, start_angle: f64, end_angle: f64) -> String {
    let outer_x1 = cx + outer_r * start_angle.cos();
    let outer_y1 = cy + outer_r * start_angle.sin();
    let outer_x2 = cx + outer_r * end_angle.cos();
    let outer_y2 = cy + outer_r * end_angle.sin();
    
    let inner_x1 = cx + inner_r * start_angle.cos();
    let inner_y1 = cy + inner_r * start_angle.sin();
    let inner_x2 = cx + inner_r * end_angle.cos();
    let inner_y2 = cy + inner_r * end_angle.sin();
    
    let large_arc = if end_angle - start_angle > std::f64::consts::PI { 1 } else { 0 };
    
    format!(
        "M {},{} L {},{} A {},{} 0 {},1 {},{} L {},{} A {},{} 0 {},0 {},{} Z",
        inner_x1, inner_y1,
        outer_x1, outer_y1,
        outer_r, outer_r, large_arc, outer_x2, outer_y2,
        inner_x2, inner_y2,
        inner_r, inner_r, large_arc, inner_x1, inner_y1
    )
}

/// Donut chart convenience component
#[component]
pub fn DonutChart(
    data: Vec<ChartDataPoint>,
    #[props(default)]
    title: Option<String>,
    #[props(default = "100%".to_string())]
    width: String,
    #[props(default = "300px".to_string())]
    height: String,
    #[props(default = 0.6)]
    inner_radius_pct: f32,
    #[props(default = true)]
    show_center_text: bool,
    #[props(default)]
    legend_position: LegendPosition,
    #[props(default)]
    on_slice_click: Option<EventHandler<ChartDataPoint>>,
    #[props(default)]
    style: Option<String>,
    #[props(default)]
    colors: Option<Vec<Color>>,
    #[props(default)]
    tooltip: ChartTooltip,
) -> Element {
    rsx! {
        PieChart {
            data: data,
            title: title,
            width: width,
            height: height,
            variant: PieChartVariant::Donut,
            inner_radius_pct: inner_radius_pct,
            show_percentages: show_center_text,
            legend_position: legend_position,
            on_slice_click: on_slice_click,
            style: style,
            colors: colors,
            tooltip: tooltip,
        }
    }
}

/// Gauge chart convenience component
#[component]
pub fn GaugeChart(
    data: Vec<ChartDataPoint>,
    #[props(default)]
    title: Option<String>,
    #[props(default = "100%".to_string())]
    width: String,
    #[props(default = "200px".to_string())]
    height: String,
    #[props(default = 0.7)]
    inner_radius_pct: f32,
    #[props(default)]
    legend_position: LegendPosition,
    #[props(default)]
    on_slice_click: Option<EventHandler<ChartDataPoint>>,
    #[props(default)]
    style: Option<String>,
    #[props(default)]
    colors: Option<Vec<Color>>,
    #[props(default)]
    tooltip: ChartTooltip,
) -> Element {
    rsx! {
        PieChart {
            data: data,
            title: title,
            width: width,
            height: height,
            variant: PieChartVariant::Gauge,
            inner_radius_pct: inner_radius_pct,
            show_percentages: true,
            legend_position: legend_position,
            on_slice_click: on_slice_click,
            style: style,
            colors: colors,
            tooltip: tooltip,
        }
    }
}
