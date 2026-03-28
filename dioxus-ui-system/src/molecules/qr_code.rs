//! QR Code molecule component
//!
//! A QR code generator display component.

use crate::styles::Style;
use crate::theme::{use_style, use_theme};
use dioxus::prelude::*;

/// QR Code error correction level
#[derive(Default, Clone, PartialEq, Debug)]
pub enum QrCodeLevel {
    /// Low (~7% correction)
    Low,
    /// Medium (~15% correction)
    #[default]
    Medium,
    /// Quartile (~25% correction)
    Quartile,
    /// High (~30% correction)
    High,
}

impl QrCodeLevel {
    fn as_str(&self) -> &'static str {
        match self {
            QrCodeLevel::Low => "L",
            QrCodeLevel::Medium => "M",
            QrCodeLevel::Quartile => "Q",
            QrCodeLevel::High => "H",
        }
    }
}

/// QR Code properties
#[derive(Props, Clone, PartialEq)]
pub struct QrCodeProps {
    /// Value to encode
    pub value: String,
    /// Size in pixels
    #[props(default = 200)]
    pub size: u32,
    /// Error correction level
    #[props(default)]
    pub level: QrCodeLevel,
    /// Foreground color (default black)
    #[props(default)]
    pub fg_color: Option<String>,
    /// Background color (default white)
    #[props(default)]
    pub bg_color: Option<String>,
    /// Include margin/quiet zone
    #[props(default = true)]
    pub include_margin: bool,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
    /// Title/alt text for accessibility
    #[props(default)]
    pub title: Option<String>,
}

/// QR Code display component
///
/// Generates a QR code using an external API (for simplicity).
/// In production, you might want to use a Rust QR library.
///
/// # Example
/// ```rust,ignore
/// use dioxus::prelude::*;
/// use dioxus_ui_system::molecules::{QrCode, QrCodeLevel};
///
/// rsx! {
///     QrCode {
///         value: "https://example.com",
///         size: 256,
///         level: QrCodeLevel::High,
///     }
/// }
/// ```
#[component]
pub fn QrCode(props: QrCodeProps) -> Element {
    let _theme = use_theme();

    let container_style = use_style(|_t| Style::new().inline_block().build());

    let fg = props
        .fg_color
        .clone()
        .unwrap_or_else(|| "000000".to_string());
    let bg = props
        .bg_color
        .clone()
        .unwrap_or_else(|| "FFFFFF".to_string());

    // Generate QR code using Google Chart API
    // For a pure Rust solution, you'd use the `qrcode` crate
    let api_url = format!(
        "https://api.qrserver.com/v1/create-qr-code/?size={}x{}&data={}&ecc={}&color={}&bgcolor={}&margin={}",
        props.size,
        props.size,
        urlencode(&props.value),
        props.level.as_str(),
        fg,
        bg,
        if props.include_margin { 4 } else { 0 }
    );

    let alt_text = props
        .title
        .clone()
        .unwrap_or_else(|| format!("QR Code: {}", props.value));

    rsx! {
        div {
            style: "{container_style} {props.style.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",

            img {
                src: "{api_url}",
                width: "{props.size}",
                height: "{props.size}",
                alt: "{alt_text}",
                style: "display: block; max-width: 100%; height: auto;",
            }
        }
    }
}

/// SVG-based QR Code component (no external API)
/// This is a simplified placeholder that renders a sample pattern
#[derive(Props, Clone, PartialEq)]
pub struct QrCodeSvgProps {
    pub value: String,
    #[props(default = 200)]
    pub size: u32,
    #[props(default)]
    pub level: QrCodeLevel,
    #[props(default)]
    pub style: Option<String>,
    #[props(default)]
    pub class: Option<String>,
}

/// QR Code SVG component using inline SVG
/// Note: This is a simplified placeholder. For real QR codes,
/// use the `qrcode` crate to generate the actual matrix.
#[component]
pub fn QrCodeSvg(props: QrCodeSvgProps) -> Element {
    let _theme = use_theme();
    let module_count = 21; // Minimum QR version 1 size
    let _module_size = props.size / module_count as u32;

    // Generate a pseudo-random pattern based on the value hash
    // In production, use the `qrcode` crate to generate the actual matrix
    let pattern = generate_placeholder_pattern(&props.value, module_count);

    let container_style = use_style(|_t| Style::new().inline_block().build());

    rsx! {
        div {
            style: "{container_style} {props.style.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",

            svg {
                width: "{props.size}",
                height: "{props.size}",
                view_box: "0 0 {module_count} {module_count}",
                xmlns: "http://www.w3.org/2000/svg",

                // Background
                rect {
                    x: "0",
                    y: "0",
                    width: "{module_count}",
                    height: "{module_count}",
                    fill: "white",
                }

                // QR pattern modules
                for (row_idx, row) in pattern.iter().enumerate() {
                    for (col_idx, &is_dark) in row.iter().enumerate() {
                        if is_dark {
                            rect {
                                key: "{row_idx}-{col_idx}",
                                x: "{col_idx}",
                                y: "{row_idx}",
                                width: "1",
                                height: "1",
                                fill: "black",
                            }
                        }
                    }
                }

                // Position detection patterns (corners)
                // Top-left
                PositionPattern { x: 0, y: 0 }
                // Top-right
                PositionPattern { x: module_count - 7, y: 0 }
                // Bottom-left
                PositionPattern { x: 0, y: module_count - 7 }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct PositionPatternProps {
    x: usize,
    y: usize,
}

#[component]
fn PositionPattern(props: PositionPatternProps) -> Element {
    let x = props.x;
    let y = props.y;

    rsx! {
        // Outer square
        rect { x: "{x}", y: "{y}", width: "7", height: "7", fill: "black" }
        // White border
        rect { x: "{x + 1}", y: "{y + 1}", width: "5", height: "5", fill: "white" }
        // Inner square
        rect { x: "{x + 2}", y: "{y + 2}", width: "3", height: "3", fill: "black" }
    }
}

/// Simple placeholder pattern generator
/// In production, use the `qrcode` crate
fn generate_placeholder_pattern(value: &str, size: usize) -> Vec<Vec<bool>> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    let hash = hasher.finish();

    let mut pattern = vec![vec![false; size]; size];

    // Generate pattern based on hash
    for i in 0..size {
        for j in 0..size {
            // Skip position detection patterns
            if (i < 7 && j < 7) || (i < 7 && j >= size - 7) || (i >= size - 7 && j < 7) {
                continue;
            }
            // Skip timing patterns
            if i == 6 || j == 6 {
                pattern[i][j] = (i + j) % 2 == 0;
                continue;
            }
            // Fill rest with pseudo-random data
            let idx = i * size + j;
            pattern[i][j] = ((hash >> (idx % 64)) & 1) == 1;
        }
    }

    pattern
}

fn urlencode(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            c => format!("%{:02X}", c as u8),
        })
        .collect()
}
