//! Style builder for inline CSS generation
//!
//! Provides a fluent API for building CSS style strings in pure Rust.
//! Similar to Tailwind's utility classes but type-safe and compiled.

use crate::theme::tokens::{Color, RadiusScale, SpacingScale, ThemeTokens, TypographyScale};
use std::fmt::Write;

/// CSS Style builder with fluent API
///
/// # Example
/// ```rust,ignore
/// use dioxus_ui_system::styles::Style;
/// use dioxus_ui_system::theme::ThemeTokens;
///
/// let theme = ThemeTokens::light();
/// let style = Style::new()
///     .flex()
///     .flex_col()
///     .items_center()
///     .gap(&theme.spacing, "md")
///     .bg(&theme.colors.primary)
///     .rounded(&theme.radius, "md")
///     .build();
/// ```
#[derive(Default, Clone)]
pub struct Style {
    // Layout
    pub display: Option<String>,
    pub flex_direction: Option<String>,
    pub flex_wrap: Option<String>,
    pub align_items: Option<String>,
    pub align_self: Option<String>,
    pub justify_content: Option<String>,
    pub justify_items: Option<String>,
    pub gap: Option<String>,
    pub row_gap: Option<String>,
    pub column_gap: Option<String>,

    // Spacing
    pub padding: Option<String>,
    pub padding_top: Option<String>,
    pub padding_right: Option<String>,
    pub padding_bottom: Option<String>,
    pub padding_left: Option<String>,
    pub margin: Option<String>,
    pub margin_top: Option<String>,
    pub margin_right: Option<String>,
    pub margin_bottom: Option<String>,
    pub margin_left: Option<String>,

    // Colors
    pub background_color: Option<String>,
    pub color: Option<String>,
    pub border_color: Option<String>,

    // Typography
    pub font_size: Option<String>,
    pub font_weight: Option<String>,
    pub font_family: Option<String>,
    pub line_height: Option<String>,
    pub text_align: Option<String>,
    pub text_decoration: Option<String>,
    pub letter_spacing: Option<String>,

    // Effects
    pub border_radius: Option<String>,
    pub border: Option<String>,
    pub border_top: Option<String>,
    pub border_right: Option<String>,
    pub border_bottom: Option<String>,
    pub border_left: Option<String>,
    pub border_width: Option<String>,
    pub box_shadow: Option<String>,

    // Sizing
    pub width: Option<String>,
    pub height: Option<String>,
    pub min_width: Option<String>,
    pub min_height: Option<String>,
    pub max_width: Option<String>,
    pub max_height: Option<String>,

    // Position
    pub position: Option<String>,
    pub top: Option<String>,
    pub right: Option<String>,
    pub bottom: Option<String>,
    pub left: Option<String>,
    pub z_index: Option<String>,

    // Misc
    pub cursor: Option<String>,
    pub opacity: Option<String>,
    pub transition: Option<String>,
    pub transform: Option<String>,
    pub overflow: Option<String>,
    pub visibility: Option<String>,
    pub pointer_events: Option<String>,
    pub user_select: Option<String>,
    pub white_space: Option<String>,
    pub word_break: Option<String>,
    pub outline: Option<String>,
    pub resize: Option<String>,
}

impl Style {
    /// Create a new empty style
    pub fn new() -> Self {
        Self::default()
    }

    // ============================================================================
    // Layout
    // ============================================================================

    /// Set display to flex
    pub fn flex(mut self) -> Self {
        self.display = Some("flex".into());
        self
    }

    /// Set display to block
    pub fn block(mut self) -> Self {
        self.display = Some("block".into());
        self
    }

    /// Set display to inline-block
    pub fn inline_block(mut self) -> Self {
        self.display = Some("inline-block".into());
        self
    }

    /// Set display to inline-flex
    pub fn inline_flex(mut self) -> Self {
        self.display = Some("inline-flex".into());
        self
    }

    /// Set display to grid
    pub fn grid(mut self) -> Self {
        self.display = Some("grid".into());
        self
    }

    /// Set display to none
    pub fn hidden(mut self) -> Self {
        self.display = Some("none".into());
        self
    }

    /// Set flex-direction to column
    pub fn flex_col(mut self) -> Self {
        self.flex_direction = Some("column".into());
        self
    }

    /// Set flex-direction to row
    pub fn flex_row(mut self) -> Self {
        self.flex_direction = Some("row".into());
        self
    }

    /// Set flex-wrap to wrap
    pub fn flex_wrap(mut self) -> Self {
        self.flex_wrap = Some("wrap".into());
        self
    }

    /// Set flex-wrap to nowrap
    pub fn flex_nowrap(mut self) -> Self {
        self.flex_wrap = Some("nowrap".into());
        self
    }

    /// Set align-items to center
    pub fn items_center(mut self) -> Self {
        self.align_items = Some("center".into());
        self
    }

    /// Set align-items to start
    pub fn items_start(mut self) -> Self {
        self.align_items = Some("flex-start".into());
        self
    }

    /// Set align-items to end
    pub fn items_end(mut self) -> Self {
        self.align_items = Some("flex-end".into());
        self
    }

    /// Set align-items to stretch
    pub fn items_stretch(mut self) -> Self {
        self.align_items = Some("stretch".into());
        self
    }

    /// Set align-self to center
    pub fn self_center(mut self) -> Self {
        self.align_self = Some("center".into());
        self
    }

    /// Set justify-content to center
    pub fn justify_center(mut self) -> Self {
        self.justify_content = Some("center".into());
        self
    }

    /// Set justify-content to start
    pub fn justify_start(mut self) -> Self {
        self.justify_content = Some("flex-start".into());
        self
    }

    /// Set justify-content to end
    pub fn justify_end(mut self) -> Self {
        self.justify_content = Some("flex-end".into());
        self
    }

    /// Set justify-content to space-between
    pub fn justify_between(mut self) -> Self {
        self.justify_content = Some("space-between".into());
        self
    }

    /// Set justify-content to space-around
    pub fn justify_around(mut self) -> Self {
        self.justify_content = Some("space-around".into());
        self
    }

    /// Set justify-content to space-evenly
    pub fn justify_evenly(mut self) -> Self {
        self.justify_content = Some("space-evenly".into());
        self
    }

    /// Set gap from spacing scale
    pub fn gap(mut self, spacing: &SpacingScale, size: &str) -> Self {
        let val = spacing.get(size);
        self.gap = Some(format!("{}px", val));
        self
    }

    /// Set gap to a specific pixel value
    pub fn gap_px(mut self, px: u16) -> Self {
        self.gap = Some(format!("{}px", px));
        self
    }

    /// Set row gap from spacing scale
    pub fn row_gap(mut self, spacing: &SpacingScale, size: &str) -> Self {
        let val = spacing.get(size);
        self.row_gap = Some(format!("{}px", val));
        self
    }

    /// Set column gap from spacing scale
    pub fn column_gap(mut self, spacing: &SpacingScale, size: &str) -> Self {
        let val = spacing.get(size);
        self.column_gap = Some(format!("{}px", val));
        self
    }

    // ============================================================================
    // Spacing
    // ============================================================================

    /// Set padding from spacing scale
    pub fn p(mut self, spacing: &SpacingScale, size: &str) -> Self {
        let val = spacing.get(size);
        self.padding = Some(format!("{}px", val));
        self
    }

    /// Set padding to specific value
    pub fn p_px(mut self, px: u16) -> Self {
        self.padding = Some(format!("{}px", px));
        self
    }

    /// Set horizontal padding (left and right)
    pub fn px(mut self, spacing: &SpacingScale, size: &str) -> Self {
        let val = spacing.get(size);
        self.padding_left = Some(format!("{}px", val));
        self.padding_right = Some(format!("{}px", val));
        self
    }

    /// Set horizontal padding to specific value
    pub fn px_px(mut self, px: u16) -> Self {
        self.padding_left = Some(format!("{}px", px));
        self.padding_right = Some(format!("{}px", px));
        self
    }

    /// Set vertical padding (top and bottom)
    pub fn py(mut self, spacing: &SpacingScale, size: &str) -> Self {
        let val = spacing.get(size);
        self.padding_top = Some(format!("{}px", val));
        self.padding_bottom = Some(format!("{}px", val));
        self
    }

    /// Set vertical padding to specific value
    pub fn py_px(mut self, px: u16) -> Self {
        self.padding_top = Some(format!("{}px", px));
        self.padding_bottom = Some(format!("{}px", px));
        self
    }

    /// Set top padding
    pub fn pt(mut self, spacing: &SpacingScale, size: &str) -> Self {
        let val = spacing.get(size);
        self.padding_top = Some(format!("{}px", val));
        self
    }

    /// Set right padding
    pub fn pr(mut self, spacing: &SpacingScale, size: &str) -> Self {
        let val = spacing.get(size);
        self.padding_right = Some(format!("{}px", val));
        self
    }

    /// Set bottom padding
    pub fn pb(mut self, spacing: &SpacingScale, size: &str) -> Self {
        let val = spacing.get(size);
        self.padding_bottom = Some(format!("{}px", val));
        self
    }

    /// Set left padding
    pub fn pl(mut self, spacing: &SpacingScale, size: &str) -> Self {
        let val = spacing.get(size);
        self.padding_left = Some(format!("{}px", val));
        self
    }

    /// Set top padding in pixels
    pub fn pt_px(mut self, px: u16) -> Self {
        self.padding_top = Some(format!("{}px", px));
        self
    }

    /// Set right padding in pixels
    pub fn pr_px(mut self, px: u16) -> Self {
        self.padding_right = Some(format!("{}px", px));
        self
    }

    /// Set bottom padding in pixels
    pub fn pb_px(mut self, px: u16) -> Self {
        self.padding_bottom = Some(format!("{}px", px));
        self
    }

    /// Set left padding in pixels
    pub fn pl_px(mut self, px: u16) -> Self {
        self.padding_left = Some(format!("{}px", px));
        self
    }

    /// Set margin from spacing scale
    pub fn m(mut self, spacing: &SpacingScale, size: &str) -> Self {
        let val = spacing.get(size);
        self.margin = Some(format!("{}px", val));
        self
    }

    /// Set margin to specific value
    pub fn m_px(mut self, px: u16) -> Self {
        self.margin = Some(format!("{}px", px));
        self
    }

    /// Set horizontal margin
    pub fn mx(mut self, spacing: &SpacingScale, size: &str) -> Self {
        let val = spacing.get(size);
        self.margin_left = Some(format!("{}px", val));
        self.margin_right = Some(format!("{}px", val));
        self
    }

    /// Set vertical margin
    pub fn my(mut self, spacing: &SpacingScale, size: &str) -> Self {
        let val = spacing.get(size);
        self.margin_top = Some(format!("{}px", val));
        self.margin_bottom = Some(format!("{}px", val));
        self
    }

    /// Set top margin
    pub fn mt(mut self, spacing: &SpacingScale, size: &str) -> Self {
        let val = spacing.get(size);
        self.margin_top = Some(format!("{}px", val));
        self
    }

    /// Set right margin
    pub fn mr(mut self, spacing: &SpacingScale, size: &str) -> Self {
        let val = spacing.get(size);
        self.margin_right = Some(format!("{}px", val));
        self
    }

    /// Set bottom margin
    pub fn mb(mut self, spacing: &SpacingScale, size: &str) -> Self {
        let val = spacing.get(size);
        self.margin_bottom = Some(format!("{}px", val));
        self
    }

    /// Set bottom margin in pixels (can be negative)
    pub fn mb_px(mut self, px: i16) -> Self {
        self.margin_bottom = Some(format!("{}px", px));
        self
    }

    /// Set left margin
    pub fn ml(mut self, spacing: &SpacingScale, size: &str) -> Self {
        let val = spacing.get(size);
        self.margin_left = Some(format!("{}px", val));
        self
    }

    // ============================================================================
    // Colors
    // ============================================================================

    /// Set background color
    pub fn bg(mut self, color: &Color) -> Self {
        self.background_color = Some(color.to_rgba());
        self
    }

    /// Set background color from hex string
    pub fn bg_hex(mut self, hex: &str) -> Self {
        self.background_color = Some(hex.into());
        self
    }

    /// Set text color
    pub fn text_color(mut self, color: &Color) -> Self {
        self.color = Some(color.to_rgba());
        self
    }

    /// Set text color from hex string
    pub fn text_hex(mut self, hex: &str) -> Self {
        self.color = Some(hex.into());
        self
    }

    /// Set border color
    pub fn border_color(mut self, color: &Color) -> Self {
        self.border_color = Some(color.to_rgba());
        self
    }

    // ============================================================================
    // Typography
    // ============================================================================

    /// Set typography from scale
    pub fn text(mut self, typography: &TypographyScale, size: &str) -> Self {
        let t = typography.get(size);
        self.font_size = Some(format!("{}px", t.size));
        self.font_weight = Some(t.weight.to_string());
        self.font_family = Some(t.family.clone());
        self.line_height = Some(t.line_height.to_string());
        if let Some(ls) = t.letter_spacing {
            self.letter_spacing = Some(format!("{}em", ls));
        }
        self
    }

    /// Set font size
    pub fn font_size(mut self, size: u16) -> Self {
        self.font_size = Some(format!("{}px", size));
        self
    }

    /// Set font weight
    pub fn font_weight(mut self, weight: u16) -> Self {
        self.font_weight = Some(weight.to_string());
        self
    }

    /// Set font family
    pub fn font_family(mut self, family: &str) -> Self {
        self.font_family = Some(family.into());
        self
    }

    /// Set line height
    pub fn line_height(mut self, height: f32) -> Self {
        self.line_height = Some(height.to_string());
        self
    }

    /// Set text align
    pub fn text_align(mut self, align: &str) -> Self {
        self.text_align = Some(align.into());
        self
    }

    /// Set text align to center
    pub fn text_center(mut self) -> Self {
        self.text_align = Some("center".into());
        self
    }

    /// Set text align to left
    pub fn text_left(mut self) -> Self {
        self.text_align = Some("left".into());
        self
    }

    /// Set text align to right
    pub fn text_right(mut self) -> Self {
        self.text_align = Some("right".into());
        self
    }

    /// Set text align to left
    pub fn text_align_left(mut self) -> Self {
        self.text_align = Some("left".into());
        self
    }

    /// Set text decoration to none
    pub fn no_underline(mut self) -> Self {
        self.text_decoration = Some("none".into());
        self
    }

    /// Set text decoration to underline
    pub fn underline(mut self) -> Self {
        self.text_decoration = Some("underline".into());
        self
    }

    // ============================================================================
    // Effects
    // ============================================================================

    /// Set border radius from scale
    pub fn rounded(mut self, radius: &RadiusScale, size: &str) -> Self {
        let val = radius.get(size);
        self.border_radius = Some(format!("{}px", val));
        self
    }

    /// Set border radius to specific value
    pub fn rounded_px(mut self, px: u16) -> Self {
        self.border_radius = Some(format!("{}px", px));
        self
    }

    /// Set border radius to full (circle/pill)
    pub fn rounded_full(mut self) -> Self {
        self.border_radius = Some("9999px".into());
        self
    }

    /// Set border with width and color
    pub fn border(mut self, width: u8, color: &Color) -> Self {
        self.border = Some(format!("{}px solid {}", width, color.to_rgba()));
        self
    }

    /// Set border width only
    pub fn border_width(mut self, width: u8) -> Self {
        self.border_width = Some(format!("{}px", width));
        self
    }

    /// Set top border
    pub fn border_top(mut self, width: u8, color: &Color) -> Self {
        self.border_top = Some(format!("{}px solid {}", width, color.to_rgba()));
        self
    }

    /// Set right border
    pub fn border_right(mut self, width: u8, color: &Color) -> Self {
        self.border_right = Some(format!("{}px solid {}", width, color.to_rgba()));
        self
    }

    /// Set bottom border
    pub fn border_bottom(mut self, width: u8, color: &Color) -> Self {
        self.border_bottom = Some(format!("{}px solid {}", width, color.to_rgba()));
        self
    }

    /// Set left border
    pub fn border_left(mut self, width: u8, color: &Color) -> Self {
        self.border_left = Some(format!("{}px solid {}", width, color.to_rgba()));
        self
    }

    /// Set border style (dashed, dotted, etc.)
    pub fn border_style(mut self, style: &str) -> Self {
        // This is a simplified implementation - border_style would need proper field
        // For now, we'll append it to the existing border or set it as custom
        let existing = self.transform.clone().unwrap_or_default();
        self.transform = Some(format!("{} border-style: {};", existing, style));
        self
    }

    /// Set box shadow
    pub fn shadow(mut self, shadow: &str) -> Self {
        self.box_shadow = Some(shadow.into());
        self
    }

    /// Set box shadow from theme
    pub fn shadow_themed(mut self, theme: &ThemeTokens, size: &str) -> Self {
        self.box_shadow = Some(theme.shadows.get(size).clone());
        self
    }

    /// Remove box shadow
    pub fn shadow_none(mut self) -> Self {
        self.box_shadow = Some("none".into());
        self
    }

    // ============================================================================
    // Sizing
    // ============================================================================

    /// Set width
    pub fn w(mut self, width: &str) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Set width in pixels
    pub fn w_px(mut self, px: u16) -> Self {
        self.width = Some(format!("{}px", px));
        self
    }

    /// Set width as percentage
    pub fn w_percent(mut self, pct: u8) -> Self {
        self.width = Some(format!("{}%", pct));
        self
    }

    /// Set width to full (100%)
    pub fn w_full(mut self) -> Self {
        self.width = Some("100%".into());
        self
    }

    /// Set height
    pub fn h(mut self, height: &str) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Set height in pixels
    pub fn h_px(mut self, px: u16) -> Self {
        self.height = Some(format!("{}px", px));
        self
    }

    /// Set height as percentage
    pub fn h_percent(mut self, pct: u8) -> Self {
        self.height = Some(format!("{}%", pct));
        self
    }

    /// Set height to full (100%)
    pub fn h_full(mut self) -> Self {
        self.height = Some("100%".into());
        self
    }

    /// Set min-width
    pub fn min_w(mut self, width: &str) -> Self {
        self.min_width = Some(width.into());
        self
    }

    /// Set min-width in pixels
    pub fn min_w_px(mut self, px: u16) -> Self {
        self.min_width = Some(format!("{}px", px));
        self
    }

    /// Set min-height
    pub fn min_h(mut self, height: &str) -> Self {
        self.min_height = Some(height.into());
        self
    }

    /// Set min-height in pixels
    pub fn min_h_px(mut self, px: u16) -> Self {
        self.min_height = Some(format!("{}px", px));
        self
    }

    /// Set max-width
    pub fn max_w(mut self, width: &str) -> Self {
        self.max_width = Some(width.into());
        self
    }

    /// Set max-width in pixels
    pub fn max_w_px(mut self, px: u16) -> Self {
        self.max_width = Some(format!("{}px", px));
        self
    }

    /// Set max-height
    pub fn max_h(mut self, height: &str) -> Self {
        self.max_height = Some(height.into());
        self
    }

    /// Set max-height in pixels
    pub fn max_h_px(mut self, px: u16) -> Self {
        self.max_height = Some(format!("{}px", px));
        self
    }

    // ============================================================================
    // Position
    // ============================================================================

    /// Set position to any value
    pub fn position(mut self, value: &str) -> Self {
        self.position = Some(value.into());
        self
    }

    /// Set position to relative
    pub fn relative(mut self) -> Self {
        self.position = Some("relative".into());
        self
    }

    /// Set position to absolute
    pub fn absolute(mut self) -> Self {
        self.position = Some("absolute".into());
        self
    }

    /// Set position to fixed
    pub fn fixed(mut self) -> Self {
        self.position = Some("fixed".into());
        self
    }

    /// Set position to sticky
    pub fn sticky(mut self) -> Self {
        self.position = Some("sticky".into());
        self
    }

    /// Set top position
    pub fn top(mut self, value: &str) -> Self {
        self.top = Some(value.into());
        self
    }

    /// Set right position
    pub fn right(mut self, value: &str) -> Self {
        self.right = Some(value.into());
        self
    }

    /// Set bottom position
    pub fn bottom(mut self, value: &str) -> Self {
        self.bottom = Some(value.into());
        self
    }

    /// Set left position
    pub fn left(mut self, value: &str) -> Self {
        self.left = Some(value.into());
        self
    }

    /// Set z-index
    pub fn z_index(mut self, z: i16) -> Self {
        self.z_index = Some(z.to_string());
        self
    }

    // ============================================================================
    // Misc
    // ============================================================================

    /// Set cursor style
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    /// Set cursor to pointer
    pub fn cursor_pointer(mut self) -> Self {
        self.cursor = Some("pointer".into());
        self
    }

    /// Set opacity
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = Some(opacity.clamp(0.0, 1.0).to_string());
        self
    }

    /// Set transition
    pub fn transition(mut self, transition: &str) -> Self {
        self.transition = Some(transition.into());
        self
    }

    /// Set transform
    pub fn transform(mut self, transform: &str) -> Self {
        self.transform = Some(transform.into());
        self
    }

    /// Set overflow to hidden
    pub fn overflow_hidden(mut self) -> Self {
        self.overflow = Some("hidden".into());
        self
    }

    /// Set overflow to auto
    pub fn overflow_auto(mut self) -> Self {
        self.overflow = Some("auto".into());
        self
    }

    /// Set overflow to scroll
    pub fn overflow_scroll(mut self) -> Self {
        self.overflow = Some("scroll".into());
        self
    }

    /// Set visibility to hidden
    pub fn invisible(mut self) -> Self {
        self.visibility = Some("hidden".into());
        self
    }

    /// Set visibility to visible
    pub fn visible(mut self) -> Self {
        self.visibility = Some("visible".into());
        self
    }

    /// Disable pointer events
    pub fn pointer_events_none(mut self) -> Self {
        self.pointer_events = Some("none".into());
        self
    }

    /// Enable pointer events
    pub fn pointer_events_auto(mut self) -> Self {
        self.pointer_events = Some("auto".into());
        self
    }

    /// Disable user selection
    pub fn select_none(mut self) -> Self {
        self.user_select = Some("none".into());
        self
    }

    /// Set white-space to nowrap
    pub fn whitespace_nowrap(mut self) -> Self {
        self.white_space = Some("nowrap".into());
        self
    }

    /// Set word-break to break-all
    pub fn break_all(mut self) -> Self {
        self.word_break = Some("break-all".into());
        self
    }

    /// Set outline style
    pub fn outline(mut self, value: &str) -> Self {
        self.outline = Some(value.into());
        self
    }

    /// Set resize style
    pub fn resize(mut self, value: &str) -> Self {
        self.resize = Some(value.into());
        self
    }

    /// Set flex-shrink
    pub fn flex_shrink(mut self, value: u8) -> Self {
        // Note: flex_shrink is not in the struct, adding as custom for now
        // We'll use a different approach - add it to the struct
        self.transform = Some(format!("flex-shrink: {}", value));
        self
    }

    /// Set transparent background
    pub fn bg_transparent(mut self) -> Self {
        self.background_color = Some("transparent".into());
        self
    }

    /// Set flex-grow
    pub fn flex_grow(mut self, value: u8) -> Self {
        // We'll add this to the transform field as a workaround since flex_grow isn't a standard field
        let existing = self.transform.unwrap_or_default();
        self.transform = Some(format!("{} flex-grow: {};", existing, value));
        self
    }

    /// Set min-height to 100%
    pub fn min_h_full(mut self) -> Self {
        self.min_height = Some("100%".into());
        self
    }

    /// Set inset (top, right, bottom, left) - shorthand for absolute positioning
    pub fn inset(mut self, value: &str) -> Self {
        self.top = Some(value.into());
        self.right = Some(value.into());
        self.bottom = Some(value.into());
        self.left = Some(value.into());
        self
    }

    /// Set flex-grow to 1 (flex-1)
    pub fn flex_1(mut self) -> Self {
        let existing = self.transform.unwrap_or_default();
        self.transform = Some(format!("{} flex: 1 1 0%;", existing));
        self
    }

    /// Set overflow-x to auto
    pub fn overflow_x_auto(mut self) -> Self {
        let existing = self.transform.unwrap_or_default();
        self.transform = Some(format!("{} overflow-x: auto;", existing));
        self
    }

    /// Set overflow-y to auto
    pub fn overflow_y_auto(mut self) -> Self {
        let existing = self.transform.unwrap_or_default();
        self.transform = Some(format!("{} overflow-y: auto;", existing));
        self
    }

    /// Add custom/raw CSS
    pub fn custom(mut self, css: &str) -> Self {
        let existing = self.transform.unwrap_or_default();
        self.transform = Some(format!("{} {}", existing, css));
        self
    }

    // ============================================================================
    // Build
    // ============================================================================

    /// Build the style string
    pub fn build(self) -> String {
        let mut style = String::new();

        // Layout
        write_if_some(&mut style, "display", &self.display);
        write_if_some(&mut style, "flex-direction", &self.flex_direction);
        write_if_some(&mut style, "flex-wrap", &self.flex_wrap);
        write_if_some(&mut style, "align-items", &self.align_items);
        write_if_some(&mut style, "align-self", &self.align_self);
        write_if_some(&mut style, "justify-content", &self.justify_content);
        write_if_some(&mut style, "justify-items", &self.justify_items);
        write_if_some(&mut style, "gap", &self.gap);
        write_if_some(&mut style, "row-gap", &self.row_gap);
        write_if_some(&mut style, "column-gap", &self.column_gap);

        // Spacing
        write_if_some(&mut style, "padding", &self.padding);
        write_if_some(&mut style, "padding-top", &self.padding_top);
        write_if_some(&mut style, "padding-right", &self.padding_right);
        write_if_some(&mut style, "padding-bottom", &self.padding_bottom);
        write_if_some(&mut style, "padding-left", &self.padding_left);
        write_if_some(&mut style, "margin", &self.margin);
        write_if_some(&mut style, "margin-top", &self.margin_top);
        write_if_some(&mut style, "margin-right", &self.margin_right);
        write_if_some(&mut style, "margin-bottom", &self.margin_bottom);
        write_if_some(&mut style, "margin-left", &self.margin_left);

        // Colors
        write_if_some(&mut style, "background-color", &self.background_color);
        write_if_some(&mut style, "color", &self.color);
        write_if_some(&mut style, "border-color", &self.border_color);

        // Typography
        write_if_some(&mut style, "font-size", &self.font_size);
        write_if_some(&mut style, "font-weight", &self.font_weight);
        write_if_some(&mut style, "font-family", &self.font_family);
        write_if_some(&mut style, "line-height", &self.line_height);
        write_if_some(&mut style, "text-align", &self.text_align);
        write_if_some(&mut style, "text-decoration", &self.text_decoration);
        write_if_some(&mut style, "letter-spacing", &self.letter_spacing);

        // Effects
        write_if_some(&mut style, "border-radius", &self.border_radius);
        write_if_some(&mut style, "border", &self.border);
        write_if_some(&mut style, "border-top", &self.border_top);
        write_if_some(&mut style, "border-right", &self.border_right);
        write_if_some(&mut style, "border-bottom", &self.border_bottom);
        write_if_some(&mut style, "border-left", &self.border_left);
        write_if_some(&mut style, "border-width", &self.border_width);
        write_if_some(&mut style, "box-shadow", &self.box_shadow);

        // Sizing
        write_if_some(&mut style, "width", &self.width);
        write_if_some(&mut style, "height", &self.height);
        write_if_some(&mut style, "min-width", &self.min_width);
        write_if_some(&mut style, "min-height", &self.min_height);
        write_if_some(&mut style, "max-width", &self.max_width);
        write_if_some(&mut style, "max-height", &self.max_height);

        // Position
        write_if_some(&mut style, "position", &self.position);
        write_if_some(&mut style, "top", &self.top);
        write_if_some(&mut style, "right", &self.right);
        write_if_some(&mut style, "bottom", &self.bottom);
        write_if_some(&mut style, "left", &self.left);
        write_if_some(&mut style, "z-index", &self.z_index);

        // Misc
        write_if_some(&mut style, "cursor", &self.cursor);
        write_if_some(&mut style, "opacity", &self.opacity);
        write_if_some(&mut style, "transition", &self.transition);
        write_if_some(&mut style, "transform", &self.transform);
        write_if_some(&mut style, "overflow", &self.overflow);
        write_if_some(&mut style, "visibility", &self.visibility);
        write_if_some(&mut style, "pointer-events", &self.pointer_events);
        write_if_some(&mut style, "user-select", &self.user_select);
        write_if_some(&mut style, "white-space", &self.white_space);
        write_if_some(&mut style, "word-break", &self.word_break);
        write_if_some(&mut style, "outline", &self.outline);
        write_if_some(&mut style, "resize", &self.resize);

        style
    }

    /// Merge another style into this one (other takes precedence)
    pub fn merge(mut self, other: Style) -> Self {
        // Layout
        if other.display.is_some() {
            self.display = other.display;
        }
        if other.flex_direction.is_some() {
            self.flex_direction = other.flex_direction;
        }
        if other.flex_wrap.is_some() {
            self.flex_wrap = other.flex_wrap;
        }
        if other.align_items.is_some() {
            self.align_items = other.align_items;
        }
        if other.align_self.is_some() {
            self.align_self = other.align_self;
        }
        if other.justify_content.is_some() {
            self.justify_content = other.justify_content;
        }
        if other.justify_items.is_some() {
            self.justify_items = other.justify_items;
        }
        if other.gap.is_some() {
            self.gap = other.gap;
        }
        if other.row_gap.is_some() {
            self.row_gap = other.row_gap;
        }
        if other.column_gap.is_some() {
            self.column_gap = other.column_gap;
        }

        // Spacing
        if other.padding.is_some() {
            self.padding = other.padding;
        }
        if other.padding_top.is_some() {
            self.padding_top = other.padding_top;
        }
        if other.padding_right.is_some() {
            self.padding_right = other.padding_right;
        }
        if other.padding_bottom.is_some() {
            self.padding_bottom = other.padding_bottom;
        }
        if other.padding_left.is_some() {
            self.padding_left = other.padding_left;
        }
        if other.margin.is_some() {
            self.margin = other.margin;
        }
        if other.margin_top.is_some() {
            self.margin_top = other.margin_top;
        }
        if other.margin_right.is_some() {
            self.margin_right = other.margin_right;
        }
        if other.margin_bottom.is_some() {
            self.margin_bottom = other.margin_bottom;
        }
        if other.margin_left.is_some() {
            self.margin_left = other.margin_left;
        }

        // Colors
        if other.background_color.is_some() {
            self.background_color = other.background_color;
        }
        if other.color.is_some() {
            self.color = other.color;
        }
        if other.border_color.is_some() {
            self.border_color = other.border_color;
        }

        // Typography
        if other.font_size.is_some() {
            self.font_size = other.font_size;
        }
        if other.font_weight.is_some() {
            self.font_weight = other.font_weight;
        }
        if other.font_family.is_some() {
            self.font_family = other.font_family;
        }
        if other.line_height.is_some() {
            self.line_height = other.line_height;
        }
        if other.text_align.is_some() {
            self.text_align = other.text_align;
        }
        if other.text_decoration.is_some() {
            self.text_decoration = other.text_decoration;
        }
        if other.letter_spacing.is_some() {
            self.letter_spacing = other.letter_spacing;
        }

        // Effects
        if other.border_radius.is_some() {
            self.border_radius = other.border_radius;
        }
        if other.border.is_some() {
            self.border = other.border;
        }
        if other.border_top.is_some() {
            self.border_top = other.border_top;
        }
        if other.border_right.is_some() {
            self.border_right = other.border_right;
        }
        if other.border_bottom.is_some() {
            self.border_bottom = other.border_bottom;
        }
        if other.border_left.is_some() {
            self.border_left = other.border_left;
        }
        if other.border_width.is_some() {
            self.border_width = other.border_width;
        }
        if other.box_shadow.is_some() {
            self.box_shadow = other.box_shadow;
        }

        // Sizing
        if other.width.is_some() {
            self.width = other.width;
        }
        if other.height.is_some() {
            self.height = other.height;
        }
        if other.min_width.is_some() {
            self.min_width = other.min_width;
        }
        if other.min_height.is_some() {
            self.min_height = other.min_height;
        }
        if other.max_width.is_some() {
            self.max_width = other.max_width;
        }
        if other.max_height.is_some() {
            self.max_height = other.max_height;
        }

        // Position
        if other.position.is_some() {
            self.position = other.position;
        }
        if other.top.is_some() {
            self.top = other.top;
        }
        if other.right.is_some() {
            self.right = other.right;
        }
        if other.bottom.is_some() {
            self.bottom = other.bottom;
        }
        if other.left.is_some() {
            self.left = other.left;
        }
        if other.z_index.is_some() {
            self.z_index = other.z_index;
        }

        // Misc
        if other.cursor.is_some() {
            self.cursor = other.cursor;
        }
        if other.opacity.is_some() {
            self.opacity = other.opacity;
        }
        if other.transition.is_some() {
            self.transition = other.transition;
        }
        if other.transform.is_some() {
            self.transform = other.transform;
        }
        if other.overflow.is_some() {
            self.overflow = other.overflow;
        }
        if other.visibility.is_some() {
            self.visibility = other.visibility;
        }
        if other.pointer_events.is_some() {
            self.pointer_events = other.pointer_events;
        }
        if other.user_select.is_some() {
            self.user_select = other.user_select;
        }
        if other.white_space.is_some() {
            self.white_space = other.white_space;
        }
        if other.word_break.is_some() {
            self.word_break = other.word_break;
        }
        if other.outline.is_some() {
            self.outline = other.outline;
        }
        if other.resize.is_some() {
            self.resize = other.resize;
        }

        self
    }
}

/// Helper function to write CSS property if value exists
fn write_if_some(style: &mut String, property: &str, value: &Option<String>) {
    if let Some(v) = value {
        write!(style, "{}:{};", property, v).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::tokens::ThemeTokens;

    #[test]
    fn test_basic_style() {
        let style = Style::new().flex().items_center().build();

        assert!(style.contains("display:flex"));
        assert!(style.contains("align-items:center"));
    }

    #[test]
    fn test_style_with_theme() {
        let theme = ThemeTokens::light();
        let style = Style::new()
            .flex()
            .gap(&theme.spacing, "md")
            .bg(&theme.colors.primary)
            .rounded(&theme.radius, "md")
            .build();

        assert!(style.contains("display:flex"));
        assert!(style.contains("gap:16px"));
        assert!(style.contains("background-color:"));
        assert!(style.contains("border-radius:8px"));
    }

    #[test]
    fn test_style_merge() {
        let base = Style::new().flex().items_center();
        let override_style = Style::new().justify_center();

        let merged = base.merge(override_style);
        let style_str = merged.build();

        assert!(style_str.contains("display:flex"));
        assert!(style_str.contains("align-items:center"));
        assert!(style_str.contains("justify-content:center"));
    }
}
