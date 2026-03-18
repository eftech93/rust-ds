//! Common types and utilities for chart components

use crate::theme::tokens::Color;

/// A single data point for charts
#[derive(Clone, PartialEq, Debug)]
pub struct ChartDataPoint {
    /// Label for the data point (shown on axis/tooltip)
    pub label: String,
    /// Numeric value
    pub value: f64,
    /// Optional color override for this data point
    pub color: Option<Color>,
}

impl ChartDataPoint {
    /// Create a new data point
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        Self {
            label: label.into(),
            value,
            color: None,
        }
    }
    
    /// Create a new data point with color
    pub fn with_color(label: impl Into<String>, value: f64, color: Color) -> Self {
        Self {
            label: label.into(),
            value,
            color: Some(color),
        }
    }
}

/// A data series for multi-series charts
#[derive(Clone, PartialEq, Debug)]
pub struct ChartSeries {
    /// Series name (shown in legend)
    pub name: String,
    /// Series color
    pub color: Color,
    /// Data points
    pub data: Vec<ChartDataPoint>,
}

impl ChartSeries {
    /// Create a new data series
    pub fn new(name: impl Into<String>, color: Color, data: Vec<ChartDataPoint>) -> Self {
        Self {
            name: name.into(),
            color,
            data,
        }
    }
    
    /// Get the minimum value in this series
    pub fn min_value(&self) -> f64 {
        self.data.iter().map(|p| p.value).fold(f64::INFINITY, f64::min)
    }
    
    /// Get the maximum value in this series
    pub fn max_value(&self) -> f64 {
        self.data.iter().map(|p| p.value).fold(f64::NEG_INFINITY, f64::max)
    }
}

/// Chart margin/padding
#[derive(Clone, PartialEq, Debug)]
pub struct ChartMargin {
    pub top: u16,
    pub right: u16,
    pub bottom: u16,
    pub left: u16,
}

impl Default for ChartMargin {
    fn default() -> Self {
        Self {
            top: 20,
            right: 20,
            bottom: 40,
            left: 50,
        }
    }
}

impl ChartMargin {
    /// Create uniform margins
    pub fn uniform(margin: u16) -> Self {
        Self {
            top: margin,
            right: margin,
            bottom: margin,
            left: margin,
        }
    }
    
    /// Create margins with only horizontal/vertical
    pub fn symmetric(vertical: u16, horizontal: u16) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }
}

/// Axis configuration
#[derive(Clone, PartialEq, Debug)]
pub struct ChartAxis {
    /// Show the axis line
    pub show_line: bool,
    /// Show tick marks
    pub show_ticks: bool,
    /// Show grid lines
    pub show_grid: bool,
    /// Number of ticks/grid lines
    pub tick_count: u8,
    /// Format for tick labels
    pub label_format: Option<fn(f64) -> String>,
    /// Minimum value (auto-calculated if None)
    pub min: Option<f64>,
    /// Maximum value (auto-calculated if None)
    pub max: Option<f64>,
}

impl Default for ChartAxis {
    fn default() -> Self {
        Self {
            show_line: true,
            show_ticks: true,
            show_grid: true,
            tick_count: 5,
            label_format: None,
            min: None,
            max: None,
        }
    }
}

impl ChartAxis {
    /// Create a hidden axis
    pub fn hidden() -> Self {
        Self {
            show_line: false,
            show_ticks: false,
            show_grid: false,
            tick_count: 0,
            label_format: None,
            min: None,
            max: None,
        }
    }
}

/// Legend position
#[derive(Default, Clone, PartialEq, Debug)]
pub enum LegendPosition {
    /// No legend
    None,
    /// Top of chart
    #[default]
    Top,
    /// Bottom of chart
    Bottom,
    /// Left side of chart
    Left,
    /// Right side of chart
    Right,
}

/// Tooltip configuration
#[derive(Clone, PartialEq, Debug)]
pub struct ChartTooltip {
    /// Show tooltip on hover
    pub enabled: bool,
    /// Format tooltip content (takes data point and optional series name)
    pub formatter: Option<fn(&ChartDataPoint, Option<&str>) -> String>,
    /// Custom tooltip values (if not using calculated values)
    /// Key is "series_index:data_index" for lookup
    pub custom_values: Option<std::collections::HashMap<String, String>>,
    /// Show series name in tooltip (for multi-series charts)
    pub show_series_name: bool,
    /// Show value in tooltip
    pub show_value: bool,
    /// Value formatter (used when show_value is true and no custom formatter)
    pub value_format: Option<fn(f64) -> String>,
}

impl Default for ChartTooltip {
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: None,
            custom_values: None,
            show_series_name: true,
            show_value: true,
            value_format: None,
        }
    }
}

impl ChartTooltip {
    /// Create a tooltip with custom formatter
    pub fn with_formatter(formatter: fn(&ChartDataPoint, Option<&str>) -> String) -> Self {
        Self {
            enabled: true,
            formatter: Some(formatter),
            ..Default::default()
        }
    }
    
    /// Create a tooltip with custom values
    pub fn with_custom_values(values: std::collections::HashMap<String, String>) -> Self {
        Self {
            enabled: true,
            custom_values: Some(values),
            ..Default::default()
        }
    }
    
    /// Disable tooltip
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Default::default()
        }
    }
    
    /// Get tooltip content for a data point
    pub fn get_content(&self, point: &ChartDataPoint, series_name: Option<&str>) -> String {
        // Use custom formatter if provided
        if let Some(formatter) = self.formatter {
            return formatter(point, series_name);
        }
        
        // Build tooltip content
        let mut parts = Vec::new();
        
        // Series name
        if self.show_series_name && series_name.is_some() {
            parts.push(series_name.unwrap().to_string());
        }
        
        // Label
        parts.push(point.label.clone());
        
        // Value
        if self.show_value {
            let value_str = if let Some(format) = self.value_format {
                format(point.value)
            } else {
                format_compact_number(point.value)
            };
            parts.push(format!(": {}", value_str));
        }
        
        parts.join("")
    }
}

/// Animation configuration
#[derive(Clone, PartialEq, Debug)]
pub struct ChartAnimation {
    /// Enable animations
    pub enabled: bool,
    /// Animation duration in milliseconds
    pub duration_ms: u16,
    /// Easing function
    pub easing: AnimationEasing,
}

impl Default for ChartAnimation {
    fn default() -> Self {
        Self {
            enabled: true,
            duration_ms: 500,
            easing: AnimationEasing::EaseOut,
        }
    }
}

/// Animation easing functions
#[derive(Default, Clone, PartialEq, Debug)]
pub enum AnimationEasing {
    #[default]
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
}

impl AnimationEasing {
    pub fn as_css(&self) -> &'static str {
        match self {
            AnimationEasing::Linear => "linear",
            AnimationEasing::Ease => "ease",
            AnimationEasing::EaseIn => "ease-in",
            AnimationEasing::EaseOut => "ease-out",
            AnimationEasing::EaseInOut => "ease-in-out",
        }
    }
}

/// Calculate nice round numbers for axis ticks
pub fn calculate_nice_ticks(min: f64, max: f64, count: u8) -> Vec<f64> {
    if min == max {
        return vec![min];
    }
    
    let range = max - min;
    let step = nice_number(range / count as f64, false);
    let nice_min = (min / step).floor() * step;
    let nice_max = (max / step).ceil() * step;
    
    let mut ticks = Vec::new();
    let mut current = nice_min;
    while current <= nice_max + step / 2.0 {
        ticks.push(current);
        current += step;
    }
    
    ticks
}

/// Round a number to a nice value
fn nice_number(x: f64, round: bool) -> f64 {
    let exp = x.log10().floor() as i32;
    let f = x / 10.0_f64.powi(exp);
    
    let nf = if round {
        if f < 1.5 { 1.0 } else if f < 3.0 { 2.0 } else if f < 7.0 { 5.0 } else { 10.0 }
    } else {
        if f <= 1.0 { 1.0 } else if f <= 2.0 { 2.0 } else if f <= 5.0 { 5.0 } else { 10.0 }
    };
    
    nf * 10.0_f64.powi(exp)
}

/// Format a number as a compact string
pub fn format_compact_number(value: f64) -> String {
    if value.abs() >= 1_000_000_000.0 {
        format!("{:.1}B", value / 1_000_000_000.0)
    } else if value.abs() >= 1_000_000.0 {
        format!("{:.1}M", value / 1_000_000.0)
    } else if value.abs() >= 1_000.0 {
        format!("{:.1}K", value / 1_000.0)
    } else {
        format!("{:.0}", value)
    }
}

/// Format a number as currency
pub fn format_currency(value: f64) -> String {
    format!("${:.2}", value)
}

/// Format a number as percentage
pub fn format_percentage(value: f64) -> String {
    format!("{:.1}%", value)
}

/// Convert a Color to CSS rgba string
pub fn color_to_css(color: &Color) -> String {
    color.to_rgba()
}

/// Calculate path for a smooth line (catmull-rom spline simplified)
pub fn calculate_smooth_line(points: &[(f64, f64)]) -> String {
    if points.len() < 2 {
        return String::new();
    }
    
    let mut path = format!("M {},{} ", points[0].0, points[0].1);
    
    for i in 1..points.len() {
        let prev = if i > 0 { points[i - 1] } else { points[0] };
        let curr = points[i];
        let next = if i < points.len() - 1 { points[i + 1] } else { curr };
        
        let cp1x = prev.0 + (curr.0 - prev.0) * 0.5;
        let cp1y = prev.1;
        let cp2x = curr.0 - (next.0 - prev.0) * 0.5;
        let cp2y = curr.1;
        
        path.push_str(&format!("C {},{} {},{} {},{} ", cp1x, cp1y, cp2x, cp2y, curr.0, curr.1));
    }
    
    path
}

/// Calculate area path (line path closed to baseline)
pub fn calculate_area_path(points: &[(f64, f64)], baseline_y: f64) -> String {
    if points.is_empty() {
        return String::new();
    }
    
    let mut path = format!("M {},{} ", points[0].0, baseline_y);
    path.push_str(&format!("L {},{} ", points[0].0, points[0].1));
    
    for i in 1..points.len() {
        path.push_str(&format!("L {},{} ", points[i].0, points[i].1));
    }
    
    path.push_str(&format!("L {},{} Z", points[points.len() - 1].0, baseline_y));
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chart_data_point() {
        let point = ChartDataPoint::new("Jan", 100.0);
        assert_eq!(point.label, "Jan");
        assert_eq!(point.value, 100.0);
        assert!(point.color.is_none());
    }

    #[test]
    fn test_chart_data_point_with_color() {
        let color = Color::new(255, 0, 0);
        let point = ChartDataPoint::with_color("Feb", 200.0, color.clone());
        assert_eq!(point.color, Some(color));
    }

    #[test]
    fn test_chart_series() {
        let data = vec![
            ChartDataPoint::new("A", 10.0),
            ChartDataPoint::new("B", 20.0),
            ChartDataPoint::new("C", 30.0),
        ];
        let series = ChartSeries::new("Test", Color::new(0, 0, 255), data);
        
        assert_eq!(series.name, "Test");
        assert_eq!(series.min_value(), 10.0);
        assert_eq!(series.max_value(), 30.0);
    }

    #[test]
    fn test_nice_ticks() {
        let ticks = calculate_nice_ticks(0.0, 100.0, 5);
        assert!(!ticks.is_empty());
        assert!(ticks[0] <= 0.0);
        assert!(ticks[ticks.len() - 1] >= 100.0);
    }

    #[test]
    fn test_format_compact_number() {
        assert_eq!(format_compact_number(1500.0), "1.5K");
        assert_eq!(format_compact_number(1500000.0), "1.5M");
        assert_eq!(format_compact_number(1500000000.0), "1.5B");
        assert_eq!(format_compact_number(150.0), "150");
    }
}
