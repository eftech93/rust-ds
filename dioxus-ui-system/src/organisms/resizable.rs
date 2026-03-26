//! Resizable organism component
//!
//! A split-pane component with draggable resize handles.
//! Supports both horizontal and vertical resizing with mouse and touch input.

use dioxus::prelude::*;
use crate::theme::{use_theme, use_style};
use crate::styles::Style;

/// Direction of the resizable panel group
#[derive(Clone, Copy, PartialEq, Default)]
pub enum Direction {
    /// Horizontal layout (panels side by side)
    #[default]
    Horizontal,
    /// Vertical layout (panels stacked)
    Vertical,
}

impl Direction {
    /// Get the cursor style for this direction
    fn cursor(self) -> &'static str {
        match self {
            Direction::Horizontal => "col-resize",
            Direction::Vertical => "row-resize",
        }
    }
}

/// Panel configuration
#[derive(Clone, PartialEq)]
struct PanelConfig {
    default_size: Option<f32>,
    min_size: Option<f32>,
    max_size: Option<f32>,
}

impl Default for PanelConfig {
    fn default() -> Self {
        Self {
            default_size: None,
            min_size: None,
            max_size: None,
        }
    }
}

/// Resizable state context
#[derive(Clone, Copy)]
struct ResizableContext {
    direction: Signal<Direction>,
    panel_configs: Signal<Vec<PanelConfig>>,
    panel_sizes: Signal<Vec<f32>>,
    dragging: Signal<bool>,
    active_handle: Signal<Option<usize>>,
    drag_start_pos: Signal<f32>,
    drag_start_sizes: Signal<Vec<f32>>,
    panel_count: Signal<usize>,
}

impl ResizableContext {
    fn new(direction: Direction) -> Self {
        Self {
            direction: use_signal(|| direction),
            panel_configs: use_signal(Vec::new),
            panel_sizes: use_signal(Vec::new),
            dragging: use_signal(|| false),
            active_handle: use_signal(|| None),
            drag_start_pos: use_signal(|| 0.0),
            drag_start_sizes: use_signal(Vec::new),
            panel_count: use_signal(|| 0),
        }
    }
}

/// Calculate initial sizes for all panels
fn calculate_sizes(configs: &[PanelConfig]) -> Vec<f32> {
    let panel_count = configs.len();
    
    if panel_count == 0 {
        return Vec::new();
    }

    let mut sizes = vec![0.0; panel_count];
    let mut assigned = 0.0;
    let mut unassigned_indices = Vec::new();

    for (i, config) in configs.iter().enumerate() {
        if let Some(default) = config.default_size {
            sizes[i] = default;
            assigned += default;
        } else {
            unassigned_indices.push(i);
        }
    }

    if !unassigned_indices.is_empty() {
        let remaining = (100.0 - assigned).max(10.0);
        let per_panel = remaining / unassigned_indices.len() as f32;
        for &i in &unassigned_indices {
            sizes[i] = per_panel;
        }
    }

    let total: f32 = sizes.iter().sum();
    if total > 0.0 {
        for size in &mut sizes {
            *size = (*size / total) * 100.0;
        }
    }

    sizes
}

/// Resizable panel group properties
#[derive(Props, Clone, PartialEq)]
pub struct ResizablePanelGroupProps {
    /// Direction of the panel group
    #[props(default)]
    pub direction: Direction,
    /// Child elements (ResizablePanel and ResizableHandle components)
    pub children: Element,
    /// Optional CSS class
    #[props(default)]
    pub class: Option<String>,
}

/// Container component for resizable panels
#[component]
pub fn ResizablePanelGroup(props: ResizablePanelGroupProps) -> Element {
    let _theme = use_theme();
    let direction = props.direction;
    
    let ctx = ResizableContext::new(direction);
    use_context_provider(|| ctx);

    let container_style = use_style(move |_t| {
        let base = Style::new()
            .w_full()
            .h_full()
            .overflow_hidden()
            .select_none();

        match direction {
            Direction::Horizontal => base.flex().flex_row(),
            Direction::Vertical => base.flex().flex_col(),
        }
        .build()
    });

    let class = props.class.unwrap_or_default();

    rsx! {
        div {
            class: "resizable-panel-group {class}",
            style: "{container_style}",
            {props.children}
        }
    }
}

/// Resizable panel properties
#[derive(Props, Clone, PartialEq)]
pub struct ResizablePanelProps {
    /// Default size as percentage (0-100)
    #[props(default)]
    pub default_size: Option<f32>,
    /// Minimum size as percentage
    #[props(default)]
    pub min_size: Option<f32>,
    /// Maximum size as percentage
    #[props(default)]
    pub max_size: Option<f32>,
    /// Panel content
    pub children: Element,
}

/// Individual resizable panel component
#[component]
pub fn ResizablePanel(props: ResizablePanelProps) -> Element {
    let mut ctx = use_context::<ResizableContext>();
    let mut panel_index = use_signal(|| None::<usize>);
    
    use_hook({
        let config = PanelConfig {
            default_size: props.default_size,
            min_size: props.min_size,
            max_size: props.max_size,
        };
        move || {
            let idx = *ctx.panel_count.read();
            panel_index.set(Some(idx));
            ctx.panel_count.set(idx + 1);
            
            ctx.panel_configs.with_mut(|configs| {
                configs.push(config);
            });
            
            let new_sizes = calculate_sizes(&ctx.panel_configs.read());
            ctx.panel_sizes.set(new_sizes);
        }
    });

    let size = use_memo(move || {
        let sizes = ctx.panel_sizes.read();
        let idx = panel_index.read().unwrap_or(0);
        sizes.get(idx).copied().unwrap_or(100.0)
    });

    let is_dragging = use_memo(move || *ctx.dragging.read());
    let direction = use_memo(move || *ctx.direction.read());

    rsx! {
        div {
            class: "resizable-panel",
            style: format!(
                "overflow:auto;{};{};{}",
                match *direction.read() {
                    Direction::Horizontal => format!("width:{}%", size.read()),
                    Direction::Vertical => "width:100%".to_string(),
                },
                match *direction.read() {
                    Direction::Horizontal => "height:100%",
                    Direction::Vertical => &format!("height:{}%", size.read()),
                },
                if *is_dragging.read() { "pointer-events:none;" } else { "" }
            ),
            {props.children}
        }
    }
}

/// Resizable handle properties
#[derive(Props, Clone, PartialEq)]
pub struct ResizableHandleProps {
    /// Whether the handle is disabled
    #[props(default = false)]
    pub disabled: bool,
}

/// Draggable resize handle component
#[component]
pub fn ResizableHandle(props: ResizableHandleProps) -> Element {
    let theme = use_theme();
    let mut ctx = use_context::<ResizableContext>();
    let mut handle_index = use_signal(|| None::<usize>);
    let mut is_hovered = use_signal(|| false);

    use_hook({
        let ctx = ctx.clone();
        move || {
            let current_count = *ctx.panel_count.read();
            if current_count > 0 {
                handle_index.set(Some(current_count - 1));
            }
        }
    });

    let is_dragging = use_memo(move || {
        let active = *ctx.active_handle.read();
        let my_idx = *handle_index.read();
        *ctx.dragging.read() && active == my_idx
    });

    let direction = *ctx.direction.read();
    let disabled = props.disabled;

    let handle_style = use_memo({
        let theme = theme.clone();
        move || {
            let t = theme.tokens.read();
            
            let base = Style::new()
                .flex_shrink(0)
                .cursor(if disabled { "not-allowed" } else { direction.cursor() })
                .transition("background-color 150ms ease")
                .flex()
                .items_center()
                .justify_center()
                .z_index(10);

            let size_style = match direction {
                Direction::Horizontal => base.w_px(8).h_full(),
                Direction::Vertical => base.h_px(8).w_full(),
            };

            let bg_color = if disabled {
                "transparent".to_string()
            } else if *is_dragging.read() {
                t.colors.primary.to_rgba()
            } else if *is_hovered.read() {
                t.colors.border.to_rgba()
            } else {
                "transparent".to_string()
            };

            size_style.bg_hex(&bg_color).build()
        }
    });

    let indicator_style = use_memo({
        let theme = theme.clone();
        move || {
            let t = theme.tokens.read();
            let hovered = *is_hovered.read();
            let is_drag = *is_dragging.read();
            
            let base = Style::new()
                .rounded(&t.radius, "full")
                .transition("all 150ms ease");

            let styled = if is_drag {
                base.bg(&t.colors.primary).shadow(&t.shadows.md)
            } else if hovered {
                base.bg(&t.colors.primary)
            } else {
                base.bg(&t.colors.border)
            };

            match direction {
                Direction::Horizontal => {
                    if hovered || is_drag {
                        styled.w_px(4).h_px(48).build()
                    } else {
                        styled.w_px(2).h_px(32).build()
                    }
                }
                Direction::Vertical => {
                    if hovered || is_drag {
                        styled.h_px(4).w_px(48).build()
                    } else {
                        styled.h_px(2).w_px(32).build()
                    }
                }
            }
        }
    });

    let mut panel_sizes = ctx.panel_sizes;
    let update_sizes_on_drag = {
        let ctx = ctx.clone();
        move |current_pos: f32| {
            let Some(handle_idx) = *ctx.active_handle.read() else {
                return;
            };

            let configs = ctx.panel_configs.read();
            let start_sizes = ctx.drag_start_sizes.read();
            
            if start_sizes.len() < 2 || handle_idx >= start_sizes.len() - 1 {
                return;
            }

            let left_idx = handle_idx;
            let right_idx = handle_idx + 1;

            let left_size = start_sizes[left_idx];
            let right_size = start_sizes[right_idx];
            let start_pos_val = *ctx.drag_start_pos.read();

            let delta_pct = ((current_pos - start_pos_val) / 500.0) * 100.0;

            let left_min = configs[left_idx].min_size.unwrap_or(5.0);
            let left_max = configs[left_idx].max_size.unwrap_or(95.0);
            let right_min = configs[right_idx].min_size.unwrap_or(5.0);
            let right_max = configs[right_idx].max_size.unwrap_or(95.0);

            let new_left = (left_size + delta_pct).clamp(left_min, left_max);
            let actual_delta = new_left - left_size;
            let new_right = (right_size - actual_delta).clamp(right_min, right_max);

            let mut new_sizes = (*start_sizes).clone();
            new_sizes[left_idx] = new_left;
            new_sizes[right_idx] = new_right;

            panel_sizes.set(new_sizes);
        }
    };

    rsx! {
        div {
            class: "resizable-handle",
            style: "{handle_style} {indicator_style}",
            onmousedown: move |evt: MouseEvent| {
                if disabled {
                    return;
                }
                
                if let Some(idx) = *handle_index.read() {
                    let coords = evt.data().client_coordinates();
                    let pos = match direction {
                        Direction::Horizontal => coords.x as f32,
                        Direction::Vertical => coords.y as f32,
                    };
                    ctx.active_handle.set(Some(idx));
                    ctx.dragging.set(true);
                    ctx.drag_start_pos.set(pos);
                    ctx.drag_start_sizes.set(ctx.panel_sizes.read().clone());
                }
            },
            onmousemove: {
                let mut update = update_sizes_on_drag.clone();
                move |evt: MouseEvent| {
                    if !*ctx.dragging.read() {
                        return;
                    }
                    
                    let coords = evt.data().client_coordinates();
                    let pos = match direction {
                        Direction::Horizontal => coords.x as f32,
                        Direction::Vertical => coords.y as f32,
                    };
                    update(pos);
                }
            },
            onmouseup: move |_| {
                ctx.dragging.set(false);
                ctx.active_handle.set(None);
            },
            onmouseenter: move |_| if !disabled { is_hovered.set(true) },
            onmouseleave: move |_| {
                is_hovered.set(false);
                ctx.dragging.set(false);
                ctx.active_handle.set(None);
            },
            ontouchstart: move |evt: TouchEvent| {
                if disabled {
                    return;
                }
                
                if let Some(touch) = evt.touches().first() {
                    if let Some(idx) = *handle_index.read() {
                        let coords = touch.client_coordinates();
                        let pos = match direction {
                            Direction::Horizontal => coords.x as f32,
                            Direction::Vertical => coords.y as f32,
                        };
                        ctx.active_handle.set(Some(idx));
                        ctx.dragging.set(true);
                        ctx.drag_start_pos.set(pos);
                        ctx.drag_start_sizes.set(ctx.panel_sizes.read().clone());
                    }
                }
            },
            ontouchmove: {
                let mut update = update_sizes_on_drag.clone();
                move |evt: TouchEvent| {
                    if !*ctx.dragging.read() {
                        return;
                    }
                    
                    if let Some(touch) = evt.touches().first() {
                        let coords = touch.client_coordinates();
                        let pos = match direction {
                            Direction::Horizontal => coords.x as f32,
                            Direction::Vertical => coords.y as f32,
                        };
                        update(pos);
                    }
                }
            },
            ontouchend: move |_| {
                ctx.dragging.set(false);
                ctx.active_handle.set(None);
            },
            ontouchcancel: move |_| {
                ctx.dragging.set(false);
                ctx.active_handle.set(None);
            },
        }
    }
}

/// Hook to access resizable panel sizes
pub fn use_resizable_panel_sizes() -> Signal<Vec<f32>> {
    let ctx = use_context::<ResizableContext>();
    ctx.panel_sizes
}

/// Hook to set a specific panel size programmatically
pub fn use_set_resizable_panel_size() -> impl FnMut(usize, f32) {
    let ctx = use_context::<ResizableContext>();
    let mut panel_sizes = ctx.panel_sizes;
    let panel_configs = ctx.panel_configs;
    move |index: usize, size: f32| {
        let mut sizes = panel_sizes.write();
        if index < sizes.len() {
            let configs = panel_configs.read();
            let min = configs[index].min_size.unwrap_or(0.0);
            let max = configs[index].max_size.unwrap_or(100.0);
            sizes[index] = size.clamp(min, max);
        }
    }
}
