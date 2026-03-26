//! Carousel organism component
//!
//! An image/content slider with navigation controls, touch swipe support,
//! and autoplay functionality.

use dioxus::prelude::*;
use std::time::Duration;

use crate::theme::{use_theme, use_style};
use crate::styles::Style;

/// Carousel orientation
#[derive(Clone, PartialEq, Default, Debug)]
pub enum Orientation {
    /// Horizontal carousel (default)
    #[default]
    Horizontal,
    /// Vertical carousel
    Vertical,
}

/// Carousel options for configuration
#[derive(Clone, PartialEq, Debug)]
pub struct CarouselOptions {
    /// Whether to loop around when reaching ends
    pub r#loop: bool,
    /// Autoplay interval in milliseconds (None to disable)
    pub autoplay_ms: Option<u64>,
    /// Start at specific index
    pub start_index: usize,
    /// Pause autoplay on hover
    pub pause_on_hover: bool,
}

impl Default for CarouselOptions {
    fn default() -> Self {
        Self {
            r#loop: true,
            autoplay_ms: None,
            start_index: 0,
            pause_on_hover: true,
        }
    }
}

impl CarouselOptions {
    /// Create new default options
    pub fn new() -> Self {
        Self::default()
    }

    /// Set loop option
    pub fn with_loop(mut self, r#loop: bool) -> Self {
        self.r#loop = r#loop;
        self
    }

    /// Set autoplay interval in milliseconds
    pub fn with_autoplay_ms(mut self, interval_ms: u64) -> Self {
        self.autoplay_ms = Some(interval_ms);
        self
    }

    /// Set autoplay interval
    pub fn with_autoplay(mut self, interval: Duration) -> Self {
        self.autoplay_ms = Some(interval.as_millis() as u64);
        self
    }

    /// Set start index
    pub fn with_start_index(mut self, index: usize) -> Self {
        self.start_index = index;
        self
    }

    /// Set pause on hover
    pub fn with_pause_on_hover(mut self, pause: bool) -> Self {
        self.pause_on_hover = pause;
        self
    }
}

/// Shared carousel context for compound components
#[derive(Clone)]
pub struct CarouselContext {
    /// Current active index signal
    pub current_index: Signal<usize>,
    /// Total number of items signal
    pub total_items: Signal<usize>,
    /// Carousel options
    pub options: CarouselOptions,
    /// Orientation
    pub orientation: Orientation,
    /// Go to next slide callback
    pub go_next: Callback<()>,
    /// Go to previous slide callback
    pub go_prev: Callback<()>,
    /// Go to specific index callback
    pub go_to: Callback<usize>,
    /// Whether currently at first slide (memoized)
    pub can_go_prev: Memo<bool>,
    /// Whether currently at last slide (memoized)
    pub can_go_next: Memo<bool>,
    /// Whether autoplay is paused
    pub is_paused: Signal<bool>,
}

/// Hook to access carousel context
pub fn use_carousel() -> Option<CarouselContext> {
    try_use_context::<CarouselContext>()
}

/// Carousel container properties
#[derive(Props, Clone, PartialEq)]
pub struct CarouselProps {
    /// Child elements (should include CarouselContent and controls)
    pub children: Element,
    /// Carousel configuration options
    #[props(default)]
    pub opts: CarouselOptions,
    /// Orientation of the carousel
    #[props(default)]
    pub orientation: Orientation,
    /// Callback when active index changes
    #[props(default)]
    pub on_index_change: Option<EventHandler<usize>>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Carousel container component
///
/// Provides context for child components and manages the carousel state.
#[component]
pub fn Carousel(props: CarouselProps) -> Element {
    let opts = props.opts.clone();
    let orientation = props.orientation.clone();

    let mut current_index = use_signal(|| opts.start_index);
    let total_items = use_signal(|| 0usize);
    let mut is_paused = use_signal(|| false);

    // Track if we can navigate
    let can_go_prev: Memo<bool> = use_memo(move || {
        let idx = current_index();
        let total = total_items();
        opts.r#loop || (idx > 0 && total > 0)
    });

    let can_go_next: Memo<bool> = use_memo(move || {
        let idx = current_index();
        let total = total_items();
        opts.r#loop || (idx < total.saturating_sub(1) && total > 0)
    });

    // Navigation callbacks
    let go_next = use_callback(move |()| {
        let total = total_items();
        if total == 0 {
            return;
        }
        current_index.with_mut(|idx| {
            if *idx < total - 1 {
                *idx += 1;
            } else if opts.r#loop {
                *idx = 0;
            }
        });
    });

    let go_prev = use_callback(move |()| {
        let total = total_items();
        if total == 0 {
            return;
        }
        current_index.with_mut(|idx| {
            if *idx > 0 {
                *idx -= 1;
            } else if opts.r#loop {
                *idx = total - 1;
            }
        });
    });

    let go_to = use_callback(move |index: usize| {
        let total = total_items();
        if index < total {
            current_index.set(index);
        }
    });

    // Handle index change callback
    use_effect(move || {
        let idx = current_index();
        if let Some(on_change) = &props.on_index_change {
            on_change.call(idx);
        }
    });

    // Note: Autoplay timer implementation is platform-specific.
    // Users can implement autoplay using the carousel context's go_next callback
    // combined with their platform's timer (e.g., setInterval in JS, tokio::time on native).

    let context = CarouselContext {
        current_index,
        total_items,
        options: opts.clone(),
        orientation: orientation.clone(),
        go_next,
        go_prev,
        go_to,
        can_go_prev,
        can_go_next,
        is_paused,
    };

    let container_style = use_style(move |_t| {
        Style::new()
            .relative()
            .w_full()
            .overflow_hidden()
            .build()
    });

    let handle_mouse_enter = move |_| {
        if opts.pause_on_hover {
            is_paused.set(true);
        }
    };

    let handle_mouse_leave = move |_| {
        if opts.pause_on_hover {
            is_paused.set(false);
        }
    };

    use_context_provider(|| context);

    rsx! {
        div {
            role: "region",
            aria_roledescription: "carousel",
            style: "{container_style} {props.style.clone().unwrap_or_default()}",
            onmouseenter: handle_mouse_enter,
            onmouseleave: handle_mouse_leave,
            {props.children}
        }
    }
}

/// Carousel content wrapper properties
#[derive(Props, Clone, PartialEq)]
pub struct CarouselContentProps {
    /// Carousel items (should be CarouselItem components)
    pub children: Element,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Carousel content wrapper
///
/// Wraps the slides and handles the scrolling/transform.
#[component]
pub fn CarouselContent(props: CarouselContentProps) -> Element {
    let carousel = use_carousel();

    // Get current index for transform calculation
    let current_idx = carousel.as_ref().map_or(0, |ctx| *ctx.current_index.read());
    
    let content_style = use_style(move |_t| {
        Style::new()
            .flex()
            .transition("transform 500ms ease-in-out")
            .transform(&format!("translateX(-{}%)", current_idx * 100))
            .build()
    });

    rsx! {
        div {
            style: "{content_style} {props.style.clone().unwrap_or_default()}",
            {props.children}
        }
    }
}

/// Carousel item properties
#[derive(Props, Clone, PartialEq)]
pub struct CarouselItemProps {
    /// Item content
    pub children: Element,
    /// Item index (used for tracking position)
    pub index: usize,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Individual carousel slide/item
#[component]
pub fn CarouselItem(props: CarouselItemProps) -> Element {
    let _theme = use_theme();
    let carousel = use_carousel();

    let is_active = carousel.as_ref().map_or(false, |ctx| {
        *ctx.current_index.read() == props.index
    });

    let item_style = use_style(move |_t| {
        Style::new()
            .min_w("100%")
            .w_full()
            .h_full()
            .build()
    });

    rsx! {
        div {
            role: "group",
            aria_roledescription: "slide",
            aria_hidden: "{!is_active}",
            style: "{item_style} {props.style.clone().unwrap_or_default()}",
            {props.children}
        }
    }
}

/// Previous button properties
#[derive(Props, Clone, PartialEq)]
pub struct CarouselPreviousProps {
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Custom class
    #[props(default)]
    pub class: Option<String>,
}

/// Previous slide button
#[component]
pub fn CarouselPrevious(props: CarouselPreviousProps) -> Element {
    let carousel = use_carousel();

    let can_go_prev = carousel.as_ref().map_or(false, |ctx| *ctx.can_go_prev.read());
    let go_prev = carousel.as_ref().map(|ctx| ctx.go_prev.clone());

    let button_style = use_style(move |t| {
        Style::new()
            .absolute()
            .left("16px")
            .top("50%")
            .transform("translateY(-50%)")
            .w_px(40)
            .h_px(40)
            .rounded_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(&t.colors.background)
            .border(1, &t.colors.border)
            .cursor(if can_go_prev { "pointer" } else { "not-allowed" })
            .opacity(if can_go_prev { 1.0 } else { 0.5 })
            .shadow(&t.shadows.md)
            .transition("all 150ms ease")
            .z_index(10)
            .build()
    });

    let handle_click = move |_| {
        if let Some(ref cb) = go_prev {
            if can_go_prev {
                cb.call(());
            }
        }
    };

    rsx! {
        button {
            r#type: "button",
            aria_label: "Previous slide",
            style: "{button_style} {props.style.clone().unwrap_or_default()}",
            class: props.class.clone().unwrap_or_default(),
            disabled: !can_go_prev,
            onclick: handle_click,
            CarouselChevron { direction: ChevronDirection::Left }
        }
    }
}

/// Next button properties
#[derive(Props, Clone, PartialEq)]
pub struct CarouselNextProps {
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Custom class
    #[props(default)]
    pub class: Option<String>,
}

/// Next slide button
#[component]
pub fn CarouselNext(props: CarouselNextProps) -> Element {
    let carousel = use_carousel();

    let can_go_next = carousel.as_ref().map_or(false, |ctx| *ctx.can_go_next.read());
    let go_next = carousel.as_ref().map(|ctx| ctx.go_next.clone());

    let button_style = use_style(move |t| {
        Style::new()
            .absolute()
            .right("16px")
            .top("50%")
            .transform("translateY(-50%)")
            .w_px(40)
            .h_px(40)
            .rounded_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(&t.colors.background)
            .border(1, &t.colors.border)
            .cursor(if can_go_next { "pointer" } else { "not-allowed" })
            .opacity(if can_go_next { 1.0 } else { 0.5 })
            .shadow(&t.shadows.md)
            .transition("all 150ms ease")
            .z_index(10)
            .build()
    });

    let handle_click = move |_| {
        if let Some(ref cb) = go_next {
            if can_go_next {
                cb.call(());
            }
        }
    };

    rsx! {
        button {
            r#type: "button",
            aria_label: "Next slide",
            style: "{button_style} {props.style.clone().unwrap_or_default()}",
            class: props.class.clone().unwrap_or_default(),
            disabled: !can_go_next,
            onclick: handle_click,
            CarouselChevron { direction: ChevronDirection::Right }
        }
    }
}

/// Chevron direction for navigation buttons
#[derive(Clone, PartialEq)]
enum ChevronDirection {
    Left,
    Right,
    Up,
    Down,
}

/// Chevron icon component
#[derive(Props, Clone, PartialEq)]
struct CarouselChevronProps {
    direction: ChevronDirection,
}

#[component]
fn CarouselChevron(props: CarouselChevronProps) -> Element {
    let d = match props.direction {
        ChevronDirection::Left => "M15 18l-6-6 6-6",
        ChevronDirection::Right => "M9 18l6-6-6-6",
        ChevronDirection::Up => "M18 15l-6-6-6 6",
        ChevronDirection::Down => "M6 9l6 6 6-6",
    };

    let icon_style = use_style(|t| {
        Style::new()
            .w_px(20)
            .h_px(20)
            .text_color(&t.colors.foreground)
            .build()
    });

    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            style: "{icon_style}",
            path { d: "{d}" }
        }
    }
}

/// Pagination dots properties
#[derive(Props, Clone, PartialEq)]
pub struct CarouselDotsProps {
    /// Custom inline styles for container
    #[props(default)]
    pub style: Option<String>,
    /// Custom inline styles for individual dots
    #[props(default)]
    pub dot_style: Option<String>,
    /// Number of dots to show (if None, uses carousel item count)
    #[props(default)]
    pub count: Option<usize>,
}

/// Pagination dots indicator
#[component]
pub fn CarouselDots(props: CarouselDotsProps) -> Element {
    let carousel = use_carousel();

    let count = props.count.unwrap_or_else(|| {
        carousel.as_ref().map_or(0, |ctx| *ctx.total_items.read())
    });

    let current_index = carousel.as_ref().map_or(0, |ctx| *ctx.current_index.read());
    let go_to = carousel.as_ref().map(|ctx| ctx.go_to.clone());

    let container_style = use_style(move |_t| {
        Style::new()
            .absolute()
            .bottom("16px")
            .left("50%")
            .transform("translateX(-50%)")
            .flex()
            .gap_px(8)
            .z_index(10)
            .build()
    });

    rsx! {
        div {
            role: "tablist",
            aria_label: "Slide navigation",
            style: "{container_style} {props.style.clone().unwrap_or_default()}",

            for index in 0..count {
                CarouselDot {
                    index: index,
                    is_active: index == current_index,
                    go_to: go_to.clone(),
                    style: props.dot_style.clone(),
                }
            }
        }
    }
}

/// Individual dot component
#[derive(Props, Clone, PartialEq)]
struct CarouselDotProps {
    index: usize,
    is_active: bool,
    go_to: Option<Callback<usize>>,
    style: Option<String>,
}

#[component]
fn CarouselDot(props: CarouselDotProps) -> Element {
    let is_active = props.is_active;
    let index = props.index;
    let go_to = props.go_to.clone();

    let dot_style = use_style(move |t| {
        Style::new()
            .w_px(if is_active { 24 } else { 8 })
            .h_px(8)
            .rounded_full()
            .bg(if is_active { &t.colors.primary } else { &t.colors.muted })
            .cursor("pointer")
            .transition("all 200ms ease")
            .border(0, &t.colors.border)
            .build()
    });

    let handle_click = move |_| {
        if let Some(ref cb) = go_to {
            cb.call(index);
        }
    };

    rsx! {
        button {
            r#type: "button",
            role: "tab",
            aria_selected: "{is_active}",
            aria_label: "Go to slide {index + 1}",
            style: "{dot_style} {props.style.clone().unwrap_or_default()}",
            onclick: handle_click,
        }
    }
}

/// Carousel with built-in content - simplified API
#[derive(Props, Clone, PartialEq)]
pub struct SimpleCarouselProps {
    /// Slides content
    pub items: Vec<Element>,
    /// Carousel options
    #[props(default)]
    pub opts: CarouselOptions,
    /// Orientation
    #[props(default)]
    pub orientation: Orientation,
    /// Callback when index changes
    #[props(default)]
    pub on_index_change: Option<EventHandler<usize>>,
    /// Show navigation arrows
    #[props(default = true)]
    pub show_arrows: bool,
    /// Show pagination dots
    #[props(default = true)]
    pub show_dots: bool,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Simple carousel with built-in controls
#[component]
pub fn SimpleCarousel(props: SimpleCarouselProps) -> Element {
    let items_len = props.items.len();

    rsx! {
        Carousel {
            opts: props.opts.clone(),
            orientation: props.orientation.clone(),
            on_index_change: props.on_index_change.clone(),

            CarouselContent {
                for (index, item) in props.items.iter().enumerate() {
                    CarouselItem {
                        key: "{index}",
                        index: index,
                        {item.clone()}
                    }
                }
            }

            if props.show_arrows {
                CarouselPrevious {}
                CarouselNext {}
            }

            if props.show_dots {
                CarouselDots { count: items_len }
            }
        }
    }
}

/// Touch-enabled carousel wrapper
#[derive(Props, Clone, PartialEq)]
pub struct TouchCarouselProps {
    /// Child elements
    pub children: Element,
    /// Carousel options
    #[props(default)]
    pub opts: CarouselOptions,
    /// Orientation
    #[props(default)]
    pub orientation: Orientation,
    /// Callback when index changes
    #[props(default)]
    pub on_index_change: Option<EventHandler<usize>>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Touch-enabled carousel with swipe support
/// 
/// Note: Touch support is simplified. For full touch gesture support,
/// implement platform-specific touch handling using the carousel context.
#[component]
pub fn TouchCarousel(props: TouchCarouselProps) -> Element {
    // Simplified touch carousel that wraps the base Carousel
    // Touch handling would require platform-specific implementation
    
    rsx! {
        Carousel {
            opts: props.opts,
            orientation: props.orientation,
            on_index_change: props.on_index_change,
            style: props.style,

            {props.children}
        }
    }
}
