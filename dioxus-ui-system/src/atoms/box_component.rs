//! Box atom component
//!
//! A foundational layout primitive that provides consistent spacing,
//! borders, backgrounds, and flexbox utilities. Similar to a div but
//! with theme-aware styling built-in.

use dioxus::prelude::*;
use crate::theme::use_style;
use crate::styles::Style;
use crate::theme::tokens::Color;

/// Box display type
#[derive(Default, Clone, PartialEq, Debug)]
pub enum BoxDisplay {
    /// Block display (default)
    #[default]
    Block,
    /// Flex display
    Flex,
    /// Inline flex display
    InlineFlex,
    /// Inline block display
    InlineBlock,
    /// Grid display
    Grid,
    /// Inline display
    Inline,
    /// Hidden/none
    None,
}

impl BoxDisplay {
    fn as_str(&self) -> &'static str {
        match self {
            BoxDisplay::Block => "block",
            BoxDisplay::Flex => "flex",
            BoxDisplay::InlineFlex => "inline-flex",
            BoxDisplay::InlineBlock => "inline-block",
            BoxDisplay::Grid => "grid",
            BoxDisplay::Inline => "inline",
            BoxDisplay::None => "none",
        }
    }
}

/// Flex direction
#[derive(Default, Clone, PartialEq, Debug)]
pub enum FlexDirection {
    /// Row direction (horizontal)
    #[default]
    Row,
    /// Column direction (vertical)
    Column,
    /// Row reverse
    RowReverse,
    /// Column reverse
    ColumnReverse,
}

impl FlexDirection {
    fn as_str(&self) -> &'static str {
        match self {
            FlexDirection::Row => "row",
            FlexDirection::Column => "column",
            FlexDirection::RowReverse => "row-reverse",
            FlexDirection::ColumnReverse => "column-reverse",
        }
    }
}

/// Flex wrap
#[derive(Default, Clone, PartialEq, Debug)]
pub enum FlexWrap {
    /// No wrap (default)
    #[default]
    NoWrap,
    /// Wrap to next line
    Wrap,
    /// Wrap reverse
    WrapReverse,
}

impl FlexWrap {
    fn as_str(&self) -> &'static str {
        match self {
            FlexWrap::NoWrap => "nowrap",
            FlexWrap::Wrap => "wrap",
            FlexWrap::WrapReverse => "wrap-reverse",
        }
    }
}

/// Justify content alignment
#[derive(Default, Clone, PartialEq, Debug)]
pub enum JustifyContent {
    /// Start alignment (default)
    #[default]
    Start,
    /// End alignment
    End,
    /// Center alignment
    Center,
    /// Space between
    SpaceBetween,
    /// Space around
    SpaceAround,
    /// Space evenly
    SpaceEvenly,
}

impl JustifyContent {
    fn as_str(&self) -> &'static str {
        match self {
            JustifyContent::Start => "flex-start",
            JustifyContent::End => "flex-end",
            JustifyContent::Center => "center",
            JustifyContent::SpaceBetween => "space-between",
            JustifyContent::SpaceAround => "space-around",
            JustifyContent::SpaceEvenly => "space-evenly",
        }
    }
}

/// Align items
#[derive(Default, Clone, PartialEq, Debug)]
pub enum AlignItems {
    /// Stretch (default)
    #[default]
    Stretch,
    /// Start alignment
    Start,
    /// End alignment
    End,
    /// Center alignment
    Center,
    /// Baseline alignment
    Baseline,
}

impl AlignItems {
    fn as_str(&self) -> &'static str {
        match self {
            AlignItems::Stretch => "stretch",
            AlignItems::Start => "flex-start",
            AlignItems::End => "flex-end",
            AlignItems::Center => "center",
            AlignItems::Baseline => "baseline",
        }
    }
}

/// Spacing scale sizes
#[derive(Default, Clone, PartialEq, Debug)]
pub enum SpacingSize {
    /// No spacing
    None,
    /// Extra small (4px)
    Xs,
    /// Small (8px)
    #[default]
    Sm,
    /// Medium (16px)
    Md,
    /// Large (24px)
    Lg,
    /// Extra large (32px)
    Xl,
    /// Extra extra large (48px)
    Xxl,
}

impl SpacingSize {
    fn as_str(&self) -> &'static str {
        match self {
            SpacingSize::None => "none",
            SpacingSize::Xs => "xs",
            SpacingSize::Sm => "sm",
            SpacingSize::Md => "md",
            SpacingSize::Lg => "lg",
            SpacingSize::Xl => "xl",
            SpacingSize::Xxl => "xxl",
        }
    }
}

/// Border radius sizes
#[derive(Default, Clone, PartialEq, Debug)]
pub enum RadiusSize {
    /// No radius
    None,
    /// Small radius (4px)
    Sm,
    /// Medium radius (8px)
    #[default]
    Md,
    /// Large radius (12px)
    Lg,
    /// Extra large radius (16px)
    Xl,
    /// Full radius (circle/pill)
    Full,
}

impl RadiusSize {
    fn as_str(&self) -> &'static str {
        match self {
            RadiusSize::None => "none",
            RadiusSize::Sm => "sm",
            RadiusSize::Md => "md",
            RadiusSize::Lg => "lg",
            RadiusSize::Xl => "xl",
            RadiusSize::Full => "full",
        }
    }
}

/// Shadow sizes
#[derive(Default, Clone, PartialEq, Debug)]
pub enum ShadowSize {
    /// No shadow
    None,
    /// Small shadow
    Sm,
    /// Medium shadow (default)
    #[default]
    Md,
    /// Large shadow
    Lg,
    /// Extra large shadow
    Xl,
    /// Inner shadow
    Inner,
}

impl ShadowSize {
    fn as_str(&self) -> &'static str {
        match self {
            ShadowSize::None => "none",
            ShadowSize::Sm => "sm",
            ShadowSize::Md => "md",
            ShadowSize::Lg => "lg",
            ShadowSize::Xl => "xl",
            ShadowSize::Inner => "inner",
        }
    }
}

/// Background color options
#[derive(Clone, PartialEq, Debug)]
pub enum BackgroundColor {
    /// Primary color
    Primary,
    /// Secondary color
    Secondary,
    /// Background color
    Background,
    /// Foreground color (text color as bg)
    Foreground,
    /// Muted color
    Muted,
    /// Accent color
    Accent,
    /// Card color
    Card,
    /// Popover color
    Popover,
    /// Destructive color
    Destructive,
    /// Success color
    Success,
    /// Warning color
    Warning,
    /// Transparent
    Transparent,
    /// Custom color
    Custom(Color),
}

impl Default for BackgroundColor {
    fn default() -> Self {
        BackgroundColor::Transparent
    }
}

/// Border width
#[derive(Default, Clone, PartialEq, Debug)]
pub enum BorderWidth {
    /// No border
    #[default]
    None,
    /// Thin border (1px)
    Thin,
    /// Medium border (2px)
    Medium,
    /// Thick border (4px)
    Thick,
}

impl BorderWidth {
    fn as_px(&self) -> u8 {
        match self {
            BorderWidth::None => 0,
            BorderWidth::Thin => 1,
            BorderWidth::Medium => 2,
            BorderWidth::Thick => 4,
        }
    }
}

/// Overflow behavior
#[derive(Default, Clone, PartialEq, Debug)]
pub enum Overflow {
    /// Visible (default)
    #[default]
    Visible,
    /// Hidden
    Hidden,
    /// Scroll
    Scroll,
    /// Auto
    Auto,
}

impl Overflow {
    fn as_str(&self) -> &'static str {
        match self {
            Overflow::Visible => "visible",
            Overflow::Hidden => "hidden",
            Overflow::Scroll => "scroll",
            Overflow::Auto => "auto",
        }
    }
}

/// Position type
#[derive(Default, Clone, PartialEq, Debug)]
pub enum Position {
    /// Static (default)
    #[default]
    Static,
    /// Relative
    Relative,
    /// Absolute
    Absolute,
    /// Fixed
    Fixed,
    /// Sticky
    Sticky,
}

impl Position {
    fn as_str(&self) -> &'static str {
        match self {
            Position::Static => "static",
            Position::Relative => "relative",
            Position::Absolute => "absolute",
            Position::Fixed => "fixed",
            Position::Sticky => "sticky",
        }
    }
}

/// Box component properties
#[derive(Props, Clone, PartialEq)]
pub struct BoxProps {
    /// Box content
    pub children: Element,
    /// Display type
    #[props(default)]
    pub display: BoxDisplay,
    /// Flex direction (when display is flex)
    #[props(default)]
    pub flex_direction: FlexDirection,
    /// Flex wrap
    #[props(default)]
    pub flex_wrap: FlexWrap,
    /// Justify content
    #[props(default)]
    pub justify_content: JustifyContent,
    /// Align items
    #[props(default)]
    pub align_items: AlignItems,
    /// Padding (all sides)
    #[props(default)]
    pub padding: SpacingSize,
    /// Padding X (horizontal)
    #[props(default)]
    pub px: Option<SpacingSize>,
    /// Padding Y (vertical)
    #[props(default)]
    pub py: Option<SpacingSize>,
    /// Padding top
    #[props(default)]
    pub pt: Option<SpacingSize>,
    /// Padding right
    #[props(default)]
    pub pr: Option<SpacingSize>,
    /// Padding bottom
    #[props(default)]
    pub pb: Option<SpacingSize>,
    /// Padding left
    #[props(default)]
    pub pl: Option<SpacingSize>,
    /// Margin (all sides)
    #[props(default)]
    pub margin: SpacingSize,
    /// Margin X (horizontal)
    #[props(default)]
    pub mx: Option<SpacingSize>,
    /// Margin Y (vertical)
    #[props(default)]
    pub my: Option<SpacingSize>,
    /// Margin top
    #[props(default)]
    pub mt: Option<SpacingSize>,
    /// Margin right
    #[props(default)]
    pub mr: Option<SpacingSize>,
    /// Margin bottom
    #[props(default)]
    pub mb: Option<SpacingSize>,
    /// Margin left
    #[props(default)]
    pub ml: Option<SpacingSize>,
    /// Gap between children (for flex/grid)
    #[props(default)]
    pub gap: SpacingSize,
    /// Background color
    #[props(default)]
    pub background: BackgroundColor,
    /// Border radius
    #[props(default)]
    pub border_radius: RadiusSize,
    /// Border width
    #[props(default)]
    pub border: BorderWidth,
    /// Border color (uses border color from theme by default)
    #[props(default)]
    pub border_color: Option<BackgroundColor>,
    /// Box shadow
    #[props(default)]
    pub shadow: ShadowSize,
    /// Width (e.g., "100%", "200px", "auto")
    #[props(default)]
    pub width: Option<String>,
    /// Height (e.g., "100%", "200px", "auto")
    #[props(default)]
    pub height: Option<String>,
    /// Minimum width
    #[props(default)]
    pub min_width: Option<String>,
    /// Minimum height
    #[props(default)]
    pub min_height: Option<String>,
    /// Maximum width
    #[props(default)]
    pub max_width: Option<String>,
    /// Maximum height
    #[props(default)]
    pub max_height: Option<String>,
    /// Overflow behavior
    #[props(default)]
    pub overflow: Overflow,
    /// Position type
    #[props(default)]
    pub position: Position,
    /// Top position (when position is not static)
    #[props(default)]
    pub top: Option<String>,
    /// Right position
    #[props(default)]
    pub right: Option<String>,
    /// Bottom position
    #[props(default)]
    pub bottom: Option<String>,
    /// Left position
    #[props(default)]
    pub left: Option<String>,
    /// Z-index
    #[props(default)]
    pub z_index: Option<i16>,
    /// Opacity (0.0 - 1.0)
    #[props(default)]
    pub opacity: Option<f32>,
    /// Cursor style
    #[props(default)]
    pub cursor: Option<String>,
    /// Click handler
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    /// Mouse enter handler
    #[props(default)]
    pub onmouseenter: Option<EventHandler<MouseEvent>>,
    /// Mouse leave handler
    #[props(default)]
    pub onmouseleave: Option<EventHandler<MouseEvent>>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
    /// Element ID
    #[props(default)]
    pub id: Option<String>,
}

/// Box atom component
///
/// A foundational layout primitive for building consistent UIs.
///
/// # Example
/// ```rust,ignore
/// use dioxus::prelude::*;
/// use dioxus_ui_system::atoms::{Box, BoxDisplay, SpacingSize, BackgroundColor};
///
/// rsx! {
///     Box {
///         display: BoxDisplay::Flex,
///         padding: SpacingSize::Md,
///         background: BackgroundColor::Card,
///         border_radius: RadiusSize::Md,
///         "Content goes here"
///     }
/// }
/// ```
#[component]
pub fn Box(props: BoxProps) -> Element {
    let display = props.display.clone();
    let flex_direction = props.flex_direction.clone();
    let flex_wrap = props.flex_wrap.clone();
    let justify_content = props.justify_content.clone();
    let align_items = props.align_items.clone();
    let padding = props.padding.clone();
    let px = props.px.clone();
    let py = props.py.clone();
    let pt = props.pt.clone();
    let pr = props.pr.clone();
    let pb = props.pb.clone();
    let pl = props.pl.clone();
    let margin = props.margin.clone();
    let mx = props.mx.clone();
    let my = props.my.clone();
    let mt = props.mt.clone();
    let mr = props.mr.clone();
    let mb = props.mb.clone();
    let ml = props.ml.clone();
    let gap = props.gap.clone();
    let background = props.background.clone();
    let border_radius = props.border_radius.clone();
    let border = props.border.clone();
    let border_color = props.border_color.clone();
    let shadow = props.shadow.clone();
    let overflow = props.overflow.clone();
    let position = props.position.clone();
    
    let style = use_style(move |t| {
        let mut style = Style::new();
        
        // Display
        style = match display {
            BoxDisplay::Block => style.block(),
            BoxDisplay::Flex => style.flex(),
            BoxDisplay::InlineFlex => style.inline_flex(),
            BoxDisplay::InlineBlock => style.inline_block(),
            BoxDisplay::Grid => style.grid(),
            BoxDisplay::Inline => style,
            BoxDisplay::None => style.hidden(),
        };
        
        // Flex properties (only apply if flex display)
        if display == BoxDisplay::Flex || display == BoxDisplay::InlineFlex {
            style = Style {
                flex_direction: Some(flex_direction.as_str().into()),
                ..style
            };
            style = Style {
                flex_wrap: Some(flex_wrap.as_str().into()),
                ..style
            };
            style = Style {
                justify_content: Some(justify_content.as_str().into()),
                ..style
            };
            style = Style {
                align_items: Some(align_items.as_str().into()),
                ..style
            };
        }
        
        // Gap
        if gap != SpacingSize::None && (display == BoxDisplay::Flex || display == BoxDisplay::InlineFlex || display == BoxDisplay::Grid) {
            style = style.gap(&t.spacing, gap.as_str());
        }
        
        // Padding
        if padding != SpacingSize::None {
            style = style.p(&t.spacing, padding.as_str());
        }
        if let Some(px_size) = &px {
            if *px_size != SpacingSize::None {
                style = style.px(&t.spacing, px_size.as_str());
            }
        }
        if let Some(py_size) = &py {
            if *py_size != SpacingSize::None {
                style = style.py(&t.spacing, py_size.as_str());
            }
        }
        if let Some(pt_size) = &pt {
            if *pt_size != SpacingSize::None {
                style = style.pt(&t.spacing, pt_size.as_str());
            }
        }
        if let Some(pr_size) = &pr {
            if *pr_size != SpacingSize::None {
                style = style.pr(&t.spacing, pr_size.as_str());
            }
        }
        if let Some(pb_size) = &pb {
            if *pb_size != SpacingSize::None {
                style = style.pb(&t.spacing, pb_size.as_str());
            }
        }
        if let Some(pl_size) = &pl {
            if *pl_size != SpacingSize::None {
                style = style.pl(&t.spacing, pl_size.as_str());
            }
        }
        
        // Margin
        if margin != SpacingSize::None {
            style = style.m(&t.spacing, margin.as_str());
        }
        if let Some(mx_size) = &mx {
            if *mx_size != SpacingSize::None {
                style = style.mx(&t.spacing, mx_size.as_str());
            }
        }
        if let Some(my_size) = &my {
            if *my_size != SpacingSize::None {
                style = style.my(&t.spacing, my_size.as_str());
            }
        }
        if let Some(mt_size) = &mt {
            if *mt_size != SpacingSize::None {
                style = style.mt(&t.spacing, mt_size.as_str());
            }
        }
        if let Some(mr_size) = &mr {
            if *mr_size != SpacingSize::None {
                style = style.mr(&t.spacing, mr_size.as_str());
            }
        }
        if let Some(mb_size) = &mb {
            if *mb_size != SpacingSize::None {
                style = style.mb(&t.spacing, mb_size.as_str());
            }
        }
        if let Some(ml_size) = &ml {
            if *ml_size != SpacingSize::None {
                style = style.ml(&t.spacing, ml_size.as_str());
            }
        }
        
        // Background color
        let bg_color = match &background {
            BackgroundColor::Primary => t.colors.primary.clone(),
            BackgroundColor::Secondary => t.colors.secondary.clone(),
            BackgroundColor::Background => t.colors.background.clone(),
            BackgroundColor::Foreground => t.colors.foreground.clone(),
            BackgroundColor::Muted => t.colors.muted.clone(),
            BackgroundColor::Accent => t.colors.accent.clone(),
            BackgroundColor::Card => t.colors.card.clone(),
            BackgroundColor::Popover => t.colors.popover.clone(),
            BackgroundColor::Destructive => t.colors.destructive.clone(),
            BackgroundColor::Success => t.colors.success.clone(),
            BackgroundColor::Warning => t.colors.warning.clone(),
            BackgroundColor::Transparent => Color::new_rgba(0, 0, 0, 0.0),
            BackgroundColor::Custom(c) => c.clone(),
        };
        style = style.bg(&bg_color);
        
        // Border radius
        style = style.rounded(&t.radius, border_radius.as_str());
        
        // Border
        if border != BorderWidth::None {
            let border_c = match &border_color {
                Some(BackgroundColor::Primary) => t.colors.primary.clone(),
                Some(BackgroundColor::Secondary) => t.colors.secondary.clone(),
                Some(BackgroundColor::Background) => t.colors.background.clone(),
                Some(BackgroundColor::Foreground) => t.colors.foreground.clone(),
                Some(BackgroundColor::Muted) => t.colors.muted.clone(),
                Some(BackgroundColor::Accent) => t.colors.accent.clone(),
                Some(BackgroundColor::Card) => t.colors.card.clone(),
                Some(BackgroundColor::Popover) => t.colors.popover.clone(),
                Some(BackgroundColor::Destructive) => t.colors.destructive.clone(),
                Some(BackgroundColor::Success) => t.colors.success.clone(),
                Some(BackgroundColor::Warning) => t.colors.warning.clone(),
                Some(BackgroundColor::Transparent) => Color::new_rgba(0, 0, 0, 0.0),
                Some(BackgroundColor::Custom(c)) => c.clone(),
                None => t.colors.border.clone(),
            };
            style = style.border(border.as_px(), &border_c);
        }
        
        // Shadow
        if shadow != ShadowSize::None {
            style = style.shadow_themed(&t, shadow.as_str());
        }
        
        // Overflow
        style = Style {
            overflow: Some(overflow.as_str().into()),
            ..style
        };
        
        // Position
        style = Style {
            position: Some(position.as_str().into()),
            ..style
        };
        
        // Opacity
        if let Some(op) = props.opacity {
            style = style.opacity(op.clamp(0.0, 1.0));
        }
        
        // Cursor
        if let Some(c) = &props.cursor {
            style = style.cursor(c);
        }
        
        style.build()
    });
    
    // Build additional styles string
    let mut additional_styles = String::new();
    
    // Width
    if let Some(w) = &props.width {
        additional_styles.push_str(&format!("width: {}; ", w));
    }
    // Height
    if let Some(h) = &props.height {
        additional_styles.push_str(&format!("height: {}; ", h));
    }
    // Min width
    if let Some(mw) = &props.min_width {
        additional_styles.push_str(&format!("min-width: {}; ", mw));
    }
    // Min height
    if let Some(mh) = &props.min_height {
        additional_styles.push_str(&format!("min-height: {}; ", mh));
    }
    // Max width
    if let Some(mw) = &props.max_width {
        additional_styles.push_str(&format!("max-width: {}; ", mw));
    }
    // Max height
    if let Some(mh) = &props.max_height {
        additional_styles.push_str(&format!("max-height: {}; ", mh));
    }
    // Position offsets
    if let Some(top) = &props.top {
        additional_styles.push_str(&format!("top: {}; ", top));
    }
    if let Some(right) = &props.right {
        additional_styles.push_str(&format!("right: {}; ", right));
    }
    if let Some(bottom) = &props.bottom {
        additional_styles.push_str(&format!("bottom: {}; ", bottom));
    }
    if let Some(left) = &props.left {
        additional_styles.push_str(&format!("left: {}; ", left));
    }
    // Z-index
    if let Some(z) = props.z_index {
        additional_styles.push_str(&format!("z-index: {}; ", z));
    }
    
    // Combine styles
    let final_style = if let Some(custom) = &props.style {
        format!("{} {}{}", style(), additional_styles, custom)
    } else {
        format!("{} {}", style(), additional_styles)
    };
    
    let class = props.class.clone().unwrap_or_default();
    let id = props.id.clone().unwrap_or_default();
    
    rsx! {
        div {
            style: "{final_style}",
            class: "{class}",
            id: "{id}",
            onclick: move |e| {
                if let Some(handler) = &props.onclick {
                    handler.call(e);
                }
            },
            onmouseenter: move |e| {
                if let Some(handler) = &props.onmouseenter {
                    handler.call(e);
                }
            },
            onmouseleave: move |e| {
                if let Some(handler) = &props.onmouseleave {
                    handler.call(e);
                }
            },
            {props.children}
        }
    }
}

/// VStack component - Vertical stack layout
///
/// Convenience wrapper around Box with flex column layout.
#[component]
pub fn VStack(
    children: Element,
    #[props(default)]
    gap: SpacingSize,
    #[props(default)]
    padding: SpacingSize,
    #[props(default)]
    align: AlignItems,
    #[props(default)]
    justify: JustifyContent,
    #[props(default)]
    background: BackgroundColor,
    #[props(default)]
    width: Option<String>,
    #[props(default)]
    height: Option<String>,
    #[props(default)]
    style: Option<String>,
    #[props(default)]
    class: Option<String>,
) -> Element {
    rsx! {
        Box {
            display: BoxDisplay::Flex,
            flex_direction: FlexDirection::Column,
            align_items: align,
            justify_content: justify,
            gap: gap,
            padding: padding,
            background: background,
            width: width,
            height: height,
            style: style,
            class: class,
            {children}
        }
    }
}

/// HStack component - Horizontal stack layout
///
/// Convenience wrapper around Box with flex row layout.
#[component]
pub fn HStack(
    children: Element,
    #[props(default)]
    gap: SpacingSize,
    #[props(default)]
    padding: SpacingSize,
    #[props(default)]
    align: AlignItems,
    #[props(default)]
    justify: JustifyContent,
    #[props(default)]
    background: BackgroundColor,
    #[props(default)]
    width: Option<String>,
    #[props(default)]
    height: Option<String>,
    #[props(default)]
    style: Option<String>,
    #[props(default)]
    class: Option<String>,
) -> Element {
    rsx! {
        Box {
            display: BoxDisplay::Flex,
            flex_direction: FlexDirection::Row,
            align_items: align,
            justify_content: justify,
            gap: gap,
            padding: padding,
            background: background,
            width: width,
            height: height,
            style: style,
            class: class,
            {children}
        }
    }
}

/// Center component - Center content both vertically and horizontally
///
/// Convenience wrapper around Box with center alignment.
#[component]
pub fn Center(
    children: Element,
    #[props(default)]
    padding: SpacingSize,
    #[props(default)]
    background: BackgroundColor,
    #[props(default)]
    width: Option<String>,
    #[props(default)]
    height: Option<String>,
    #[props(default)]
    style: Option<String>,
    #[props(default)]
    class: Option<String>,
) -> Element {
    rsx! {
        Box {
            display: BoxDisplay::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            padding: padding,
            background: background,
            width: width,
            height: height,
            style: style,
            class: class,
            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_display_variants() {
        assert_eq!(BoxDisplay::Flex.as_str(), "flex");
        assert_eq!(BoxDisplay::Block.as_str(), "block");
        assert_eq!(BoxDisplay::None.as_str(), "none");
    }

    #[test]
    fn test_flex_direction_variants() {
        assert_eq!(FlexDirection::Row.as_str(), "row");
        assert_eq!(FlexDirection::Column.as_str(), "column");
    }

    #[test]
    fn test_justify_content_variants() {
        assert_eq!(JustifyContent::Center.as_str(), "center");
        assert_eq!(JustifyContent::SpaceBetween.as_str(), "space-between");
    }

    #[test]
    fn test_align_items_variants() {
        assert_eq!(AlignItems::Center.as_str(), "center");
        assert_eq!(AlignItems::Stretch.as_str(), "stretch");
    }

    #[test]
    fn test_spacing_size_to_str() {
        assert_eq!(SpacingSize::Md.as_str(), "md");
        assert_eq!(SpacingSize::Lg.as_str(), "lg");
    }

    #[test]
    fn test_border_width_to_px() {
        assert_eq!(BorderWidth::None.as_px(), 0);
        assert_eq!(BorderWidth::Thin.as_px(), 1);
        assert_eq!(BorderWidth::Medium.as_px(), 2);
        assert_eq!(BorderWidth::Thick.as_px(), 4);
    }
}
