//! Chart components
//!
//! Pure Rust chart components for data visualization.
//! Uses SVG for rendering without external dependencies.

mod common;
mod bar_chart;
mod line_chart;
mod pie_chart;
mod sparkline;

// Re-export common types
pub use common::{
    ChartDataPoint,
    ChartSeries,
    ChartMargin,
    ChartAxis,
    LegendPosition,
    ChartTooltip,
    ChartAnimation,
    AnimationEasing,
    calculate_nice_ticks,
    format_compact_number,
    format_currency,
    format_percentage,
    color_to_css,
    calculate_smooth_line,
    calculate_area_path,
};

// Re-export bar chart
pub use bar_chart::{
    BarChart,
    BarChartProps,
    BarChartVariant,
};

// Re-export line chart
pub use line_chart::{
    LineChart,
    LineChartProps,
    LineChartVariant,
};

// Re-export pie chart
pub use pie_chart::{
    PieChart,
    PieChartProps,
    PieChartVariant,
    DonutChart,
    GaugeChart,
};

// Re-export sparkline
pub use sparkline::{
    Sparkline,
    SparklineProps,
    SparklineVariant,
    TrendIndicator,
};

/// Predefined color palettes for charts
pub mod palettes {
    use crate::theme::tokens::Color;

    /// Default color palette
    pub fn default() -> Vec<Color> {
        vec![
            Color::new(15, 23, 42),     // Slate 900
            Color::new(59, 130, 246),   // Blue 500
            Color::new(34, 197, 94),    // Green 500
            Color::new(234, 179, 8),    // Yellow 500
            Color::new(239, 68, 68),    // Red 500
            Color::new(168, 85, 247),   // Purple 500
            Color::new(236, 72, 153),   // Pink 500
            Color::new(20, 184, 166),   // Teal 500
            Color::new(249, 115, 22),   // Orange 500
            Color::new(99, 102, 241),   // Indigo 500
        ]
    }

    /// Warm color palette
    pub fn warm() -> Vec<Color> {
        vec![
            Color::new(239, 68, 68),    // Red 500
            Color::new(249, 115, 22),   // Orange 500
            Color::new(234, 179, 8),    // Yellow 500
            Color::new(236, 72, 153),   // Pink 500
            Color::new(168, 85, 247),   // Purple 500
        ]
    }

    /// Cool color palette
    pub fn cool() -> Vec<Color> {
        vec![
            Color::new(59, 130, 246),   // Blue 500
            Color::new(20, 184, 166),   // Teal 500
            Color::new(34, 197, 94),    // Green 500
            Color::new(99, 102, 241),   // Indigo 500
            Color::new(6, 182, 212),    // Cyan 500
        ]
    }

    /// Monochrome palette (shades of blue)
    pub fn monochrome() -> Vec<Color> {
        vec![
            Color::new(30, 58, 138),    // Blue 900
            Color::new(37, 99, 235),    // Blue 600
            Color::new(59, 130, 246),   // Blue 500
            Color::new(96, 165, 250),   // Blue 400
            Color::new(147, 197, 253),  // Blue 300
            Color::new(191, 219, 254),  // Blue 200
        ]
    }

    /// Pastel color palette
    pub fn pastel() -> Vec<Color> {
        vec![
            Color::new(252, 165, 165),  // Red 300
            Color::new(253, 186, 116),  // Orange 300
            Color::new(253, 224, 71),   // Yellow 300
            Color::new(134, 239, 172),  // Green 300
            Color::new(103, 232, 249),  // Cyan 300
            Color::new(147, 197, 253),  // Blue 300
            Color::new(196, 181, 253),  // Purple 300
            Color::new(249, 168, 212),  // Pink 300
        ]
    }

    /// High contrast palette (accessibility)
    pub fn high_contrast() -> Vec<Color> {
        vec![
            Color::new(0, 0, 0),        // Black
            Color::new(255, 0, 0),      // Red
            Color::new(0, 255, 0),      // Green
            Color::new(0, 0, 255),      // Blue
            Color::new(255, 255, 0),    // Yellow
            Color::new(255, 0, 255),    // Magenta
            Color::new(0, 255, 255),    // Cyan
            Color::new(255, 255, 255),  // White
        ]
    }
}

/// Utility functions for chart data preparation
pub mod utils {
    use super::{ChartDataPoint, ChartSeries};
    use crate::theme::tokens::Color;

    /// Create a simple data series from values with auto-generated labels
    pub fn series_from_values(name: impl Into<String>, color: Color, values: Vec<f64>) -> ChartSeries {
        let data: Vec<ChartDataPoint> = values
            .into_iter()
            .enumerate()
            .map(|(idx, value)| ChartDataPoint::new(format!("Item {}", idx + 1), value))
            .collect();
        
        ChartSeries::new(name, color, data)
    }

    /// Create a data series from label-value pairs
    pub fn series_from_pairs(
        name: impl Into<String>, 
        color: Color, 
        pairs: Vec<(impl Into<String>, f64)>
    ) -> ChartSeries {
        let data: Vec<ChartDataPoint> = pairs
            .into_iter()
            .map(|(label, value)| ChartDataPoint::new(label, value))
            .collect();
        
        ChartSeries::new(name, color, data)
    }

    /// Generate time series labels
    pub fn time_labels(count: usize, interval: TimeInterval) -> Vec<String> {
        use std::collections::VecDeque;
        
        let mut labels = VecDeque::new();
        
        for i in (0..count).rev() {
            let label = match interval {
                TimeInterval::Hours => format!("{}h ago", i),
                TimeInterval::Days => format!("{}d ago", i),
                TimeInterval::Weeks => format!("{}w ago", i),
                TimeInterval::Months => format!("{}m ago", i),
            };
            labels.push_front(label);
        }
        
        labels.into()
    }

    /// Time interval for generating labels
    #[derive(Clone, Copy)]
    pub enum TimeInterval {
        Hours,
        Days,
        Weeks,
        Months,
    }

    /// Calculate percentage change between two values
    pub fn percent_change(old: f64, new: f64) -> f64 {
        if old == 0.0 {
            0.0
        } else {
            ((new - old) / old) * 100.0
        }
    }

    /// Calculate moving average
    pub fn moving_average(data: &[f64], window: usize) -> Vec<f64> {
        if window == 0 || data.is_empty() {
            return data.to_vec();
        }
        
        data.windows(window)
            .map(|w| w.iter().sum::<f64>() / w.len() as f64)
            .collect()
    }

    /// Normalize data to 0-100 range
    pub fn normalize_to_percent(data: &[f64]) -> Vec<f64> {
        if data.is_empty() {
            return vec![];
        }
        
        let min = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let range = max - min;
        
        if range == 0.0 {
            return vec![50.0; data.len()];
        }
        
        data.iter()
            .map(|&v| ((v - min) / range) * 100.0)
            .collect()
    }
}
