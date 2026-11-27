//! Carousel organism component
//!
//! An image/content slider with navigation controls, touch swipe support,
//! and autoplay functionality.

use dioxus::prelude::*;
use std::time::Duration;

use crate::styles::Style;
use crate::theme::{use_style, use_theme};

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
    /// Element id
    #[props(default)]
    pub id: Option<String>,
    /// Custom CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Accessible name of the carousel region (`aria-label`, default "Carousel")
    #[props(default)]
    pub aria_label: Option<String>,
}

/// Carousel container component
///
/// Provides context for child components and manages the carousel state.
///
/// # Accessibility
///
/// Follows the WAI-ARIA carousel pattern: the container is a focusable
/// `role="region"` with `aria-roledescription="carousel"` and an `aria-label`
/// (see `aria_label`), slides are `role="group"` with
/// `aria-roledescription="slide"` and `aria-label="n of m"`, and the
/// previous/next buttons have `aria-label`s. While focus is inside the
/// carousel, Left/Right (Up/Down when vertical) move to the previous/next
/// slide and Home/End jump to the first/last one. `is_paused` in the context
/// becomes `true` while the carousel has focus (and while hovered when
/// `pause_on_hover` is set), so autoplay implementations built on the context
/// should honour it.
#[component]
pub fn Carousel(props: CarouselProps) -> Element {
    let opts = props.opts.clone();
    let orientation = props.orientation.clone();

    let mut current_index = use_signal(|| opts.start_index);
    let total_items = use_signal(|| 0usize);
    let mut is_paused = use_signal(|| false);
    let mut is_hovered = use_signal(|| false);
    let mut has_focus = use_signal(|| false);

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

    let container_style =
        use_style(move |_t| Style::new().relative().w_full().overflow_hidden().build());

    let pause_on_hover = opts.pause_on_hover;
    let looping = opts.r#loop;

    let handle_mouse_enter = move |_| {
        is_hovered.set(true);
        is_paused.set(should_pause(true, has_focus(), pause_on_hover));
    };

    let handle_mouse_leave = move |_| {
        is_hovered.set(false);
        is_paused.set(should_pause(false, has_focus(), pause_on_hover));
    };

    let handle_focus_in = move |_| {
        has_focus.set(true);
        is_paused.set(should_pause(is_hovered(), true, pause_on_hover));
    };

    let handle_focus_out = move |_| {
        has_focus.set(false);
        is_paused.set(should_pause(is_hovered(), false, pause_on_hover));
    };

    let key_orientation = orientation;
    let handle_keydown = move |e: KeyboardEvent| {
        let target = carousel_key_target(
            &e.key(),
            current_index(),
            total_items(),
            &key_orientation,
            looping,
        );
        if let Some(target) = target {
            e.prevent_default();
            go_to.call(target);
        }
    };

    use_context_provider(|| context);

    let label = props
        .aria_label
        .clone()
        .unwrap_or_else(|| "Carousel".to_string());

    rsx! {
        div {
            id: "{props.id.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            role: "region",
            aria_roledescription: "carousel",
            "aria-label": "{label}",
            tabindex: "0",
            style: "{container_style} {props.style.clone().unwrap_or_default()}",
            onmouseenter: handle_mouse_enter,
            onmouseleave: handle_mouse_leave,
            onfocusin: handle_focus_in,
            onfocusout: handle_focus_out,
            onkeydown: handle_keydown,
            {props.children}
        }
    }
}

/// Whether autoplay should be paused given hover/focus state.
fn should_pause(hovered: bool, focused: bool, pause_on_hover: bool) -> bool {
    focused || (hovered && pause_on_hover)
}

/// Slide index to show after pressing `key`, or `None` if the key is not
/// handled (or would move past an end of a non-looping carousel).
fn carousel_key_target(
    key: &Key,
    current: usize,
    total: usize,
    orientation: &Orientation,
    looping: bool,
) -> Option<usize> {
    if total == 0 {
        return None;
    }
    let last = total - 1;
    let current = current.min(last);
    let (prev, next) = match orientation {
        Orientation::Horizontal => (Key::ArrowLeft, Key::ArrowRight),
        Orientation::Vertical => (Key::ArrowUp, Key::ArrowDown),
    };
    match key {
        k if *k == next => {
            if current < last {
                Some(current + 1)
            } else {
                looping.then_some(0)
            }
        }
        k if *k == prev => {
            if current > 0 {
                Some(current - 1)
            } else {
                looping.then_some(last)
            }
        }
        Key::Home => Some(0),
        Key::End => Some(last),
        _ => None,
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
    /// Element id
    #[props(default)]
    pub id: Option<String>,
    /// Custom CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Carousel content wrapper
///
/// Wraps the slides and handles the scrolling/transform.
#[component]
pub fn CarouselContent(props: CarouselContentProps) -> Element {
    let carousel = use_carousel();

    // Get current index for transform calculation
    let current_idx = carousel.as_ref().map_or(0, |ctx| *ctx.current_index.read());

    // Announce slide changes unless autoplay is actively rotating them.
    let aria_live = match &carousel {
        Some(ctx) if ctx.options.autoplay_ms.is_some() && !*ctx.is_paused.read() => "off",
        _ => "polite",
    };

    // Count children and update total_items
    // This is done by rendering and letting CarouselItem update the count
    let content_style = use_style(move |_t| {
        Style::new()
            .flex()
            .transition("transform 500ms ease-in-out")
            .transform(&format!("translateX(-{}%)", current_idx * 100))
            .build()
    });

    rsx! {
        div {
            id: "{props.id.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            style: "{content_style} {props.style.clone().unwrap_or_default()}",
            "aria-live": "{aria_live}",
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
    /// Total number of slides, for the "n of m" label. Defaults to the count
    /// tracked by the parent [`Carousel`].
    #[props(default)]
    pub total: Option<usize>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Element id
    #[props(default)]
    pub id: Option<String>,
    /// Custom CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Individual carousel slide/item
#[component]
pub fn CarouselItem(props: CarouselItemProps) -> Element {
    let _theme = use_theme();
    let carousel = use_carousel();
    let index = props.index;

    // Update total_items count when this item is mounted
    let total_items_signal = carousel.as_ref().map(|ctx| ctx.total_items);
    use_effect(move || {
        if let Some(mut total_items) = total_items_signal {
            total_items.with_mut(|count| {
                if index >= *count {
                    *count = index + 1;
                }
            });
        }
    });

    let is_active = carousel
        .as_ref()
        .is_some_and(|ctx| *ctx.current_index.read() == props.index);

    let total = props
        .total
        .or_else(|| carousel.as_ref().map(|ctx| *ctx.total_items.read()))
        .unwrap_or(0)
        .max(index + 1);

    let item_style = use_style(move |_t| Style::new().min_w("100%").w_full().h_full().build());

    rsx! {
        div {
            id: "{props.id.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            role: "group",
            aria_roledescription: "slide",
            "aria-label": "{index + 1} of {total}",
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
    /// Element id
    #[props(default)]
    pub id: Option<String>,
}

/// Previous slide button
#[component]
pub fn CarouselPrevious(props: CarouselPreviousProps) -> Element {
    let carousel = use_carousel();

    let can_go_prev = carousel.as_ref().is_some_and(|ctx| *ctx.can_go_prev.read());
    let go_prev = carousel.as_ref().map(|ctx| ctx.go_prev);

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
            .cursor(if can_go_prev {
                "pointer"
            } else {
                "not-allowed"
            })
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
            id: "{props.id.clone().unwrap_or_default()}",
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
    /// Element id
    #[props(default)]
    pub id: Option<String>,
}

/// Next slide button
#[component]
pub fn CarouselNext(props: CarouselNextProps) -> Element {
    let carousel = use_carousel();

    let can_go_next = carousel.as_ref().is_some_and(|ctx| *ctx.can_go_next.read());
    let go_next = carousel.as_ref().map(|ctx| ctx.go_next);

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
            .cursor(if can_go_next {
                "pointer"
            } else {
                "not-allowed"
            })
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
            id: "{props.id.clone().unwrap_or_default()}",
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
#[allow(dead_code)]
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
    /// Element id
    #[props(default)]
    pub id: Option<String>,
    /// Custom CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Pagination dots indicator
#[component]
pub fn CarouselDots(props: CarouselDotsProps) -> Element {
    let carousel = use_carousel();

    let count = props
        .count
        .unwrap_or_else(|| carousel.as_ref().map_or(0, |ctx| *ctx.total_items.read()));

    let current_index = carousel.as_ref().map_or(0, |ctx| *ctx.current_index.read());
    let go_to = carousel.as_ref().map(|ctx| ctx.go_to);

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
            id: "{props.id.clone().unwrap_or_default()}",
            class: "{props.class.clone().unwrap_or_default()}",
            role: "tablist",
            aria_label: "Slide navigation",
            style: "{container_style} {props.style.clone().unwrap_or_default()}",

            for index in 0..count {
                CarouselDot {
                    index: index,
                    is_active: index == current_index,
                    go_to: go_to,
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
    let go_to = props.go_to;

    let dot_style = use_style(move |t| {
        Style::new()
            .w_px(if is_active { 24 } else { 8 })
            .h_px(8)
            .rounded_full()
            .bg(if is_active {
                &t.colors.primary
            } else {
                &t.colors.muted
            })
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
    /// Element id
    #[props(default)]
    pub id: Option<String>,
    /// Custom CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Accessible name of the carousel region (`aria-label`, default "Carousel")
    #[props(default)]
    pub aria_label: Option<String>,
}

/// Simple carousel with built-in controls
///
/// See [`Carousel`] for roles and keyboard support.
#[component]
pub fn SimpleCarousel(props: SimpleCarouselProps) -> Element {
    let items_len = props.items.len();

    rsx! {
        Carousel {
            id: props.id.clone(),
            class: props.class.clone(),
            style: props.style.clone(),
            opts: props.opts.clone(),
            orientation: props.orientation.clone(),
            on_index_change: props.on_index_change,
            aria_label: props.aria_label.clone(),

            CarouselContent {
                for (index, item) in props.items.iter().enumerate() {
                    CarouselItem {
                        key: "{index}",
                        index: index,
                        total: items_len,
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
    /// Element id
    #[props(default)]
    pub id: Option<String>,
    /// Custom CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Accessible name of the carousel region (`aria-label`, default "Carousel")
    #[props(default)]
    pub aria_label: Option<String>,
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
            id: props.id.clone(),
            class: props.class.clone(),
            opts: props.opts,
            orientation: props.orientation,
            on_index_change: props.on_index_change,
            style: props.style,
            aria_label: props.aria_label,

            {props.children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(app: fn() -> Element) -> String {
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
        dioxus_ssr::render(&dom)
    }

    #[test]
    fn simple_carousel_has_carousel_semantics() {
        let html = render(|| {
            rsx! {
                SimpleCarousel {
                    aria_label: "Featured",
                    items: vec![rsx! { "One" }, rsx! { "Two" }, rsx! { "Three" }],
                }
            }
        });
        assert!(html.contains("role=\"region\""), "{html}");
        assert!(html.contains("aria-roledescription=\"carousel\""), "{html}");
        assert!(html.contains("aria-label=\"Featured\""), "{html}");
        assert!(html.contains("tabindex=\"0\""), "{html}");
        assert_eq!(
            html.matches("aria-roledescription=\"slide\"").count(),
            3,
            "{html}"
        );
        assert!(html.contains("aria-label=\"1 of 3\""), "{html}");
        assert!(html.contains("aria-label=\"3 of 3\""), "{html}");
        assert!(html.contains("aria-label=\"Previous slide\""), "{html}");
        assert!(html.contains("aria-label=\"Next slide\""), "{html}");
        assert!(html.contains("aria-live=\"polite\""), "{html}");
    }

    #[test]
    fn carousel_defaults_label() {
        let html = render(|| rsx! { Carousel { CarouselContent { "x" } } });
        assert!(html.contains("aria-label=\"Carousel\""), "{html}");
    }

    #[test]
    fn carousel_keys_respect_orientation_and_loop() {
        let h = Orientation::Horizontal;
        assert_eq!(
            carousel_key_target(&Key::ArrowRight, 0, 3, &h, false),
            Some(1)
        );
        assert_eq!(
            carousel_key_target(&Key::ArrowLeft, 1, 3, &h, false),
            Some(0)
        );
        assert_eq!(carousel_key_target(&Key::ArrowRight, 2, 3, &h, false), None);
        assert_eq!(carousel_key_target(&Key::ArrowLeft, 0, 3, &h, false), None);
        assert_eq!(
            carousel_key_target(&Key::ArrowRight, 2, 3, &h, true),
            Some(0)
        );
        assert_eq!(
            carousel_key_target(&Key::ArrowLeft, 0, 3, &h, true),
            Some(2)
        );
        assert_eq!(carousel_key_target(&Key::Home, 2, 3, &h, false), Some(0));
        assert_eq!(carousel_key_target(&Key::End, 0, 3, &h, false), Some(2));
        assert_eq!(carousel_key_target(&Key::ArrowDown, 0, 3, &h, true), None);
        assert_eq!(
            carousel_key_target(&Key::ArrowDown, 0, 3, &Orientation::Vertical, true),
            Some(1)
        );
        assert_eq!(carousel_key_target(&Key::ArrowRight, 0, 0, &h, true), None);
    }

    #[test]
    fn pauses_on_focus_and_optionally_on_hover() {
        assert!(should_pause(false, true, false));
        assert!(should_pause(true, false, true));
        assert!(!should_pause(true, false, false));
        assert!(!should_pause(false, false, true));
    }
}
