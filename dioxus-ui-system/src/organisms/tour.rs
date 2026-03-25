//! Tour organism component
//!
//! A product walkthrough/onboarding component for guiding users through features.

use dioxus::prelude::*;
use crate::theme::{use_theme, use_style};
use crate::styles::Style;
use crate::theme::tokens::Color;

/// Placement of the tour popover relative to the target
#[derive(Default, Clone, PartialEq, Debug)]
pub enum Placement {
    #[default]
    Top,
    Right,
    Bottom,
    Left,
    TopStart,
    TopEnd,
    RightStart,
    RightEnd,
    BottomStart,
    BottomEnd,
    LeftStart,
    LeftEnd,
}

/// Individual tour step
#[derive(Clone, PartialEq, Debug)]
pub struct TourStep {
    /// CSS selector for the target element
    pub target: String,
    /// Step title
    pub title: String,
    /// Step content/description
    pub content: String,
    /// Placement relative to target
    pub placement: Placement,
}

impl TourStep {
    /// Create a new tour step
    pub fn new(target: impl Into<String>, title: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            target: target.into(),
            title: title.into(),
            content: content.into(),
            placement: Placement::Bottom,
        }
    }
    
    /// Set the placement
    pub fn with_placement(mut self, placement: Placement) -> Self {
        self.placement = placement;
        self
    }
}

/// Tour component properties
#[derive(Props, Clone, PartialEq)]
pub struct TourProps {
    /// Tour steps
    pub steps: Vec<TourStep>,
    /// Whether the tour is open
    pub open: bool,
    /// Callback when open state changes
    pub on_open_change: EventHandler<bool>,
    /// Callback when tour is finished
    #[props(default)]
    pub on_finish: Option<EventHandler<()>>,
    /// Callback when step changes
    #[props(default)]
    pub on_step_change: Option<EventHandler<usize>>,
    /// Current step index (controlled)
    #[props(default)]
    pub current_step: Option<usize>,
    /// Mask/overlay color
    #[props(default)]
    pub mask_color: Option<String>,
    /// Show progress indicator
    #[props(default = true)]
    pub show_progress: bool,
    /// Show skip button
    #[props(default = true)]
    pub show_skip: bool,
    /// Allow clicking highlighted element
    #[props(default)]
    pub allow_interaction: bool,
}

/// Tour component for product walkthroughs
#[component]
pub fn Tour(props: TourProps) -> Element {
    let theme = use_theme();
    let mut internal_step = use_signal(|| 0);
    
    // Read theme values once
    let theme_colors = use_memo(move || {
        let t = theme.tokens.read();
        (
            t.colors.background.to_rgba(),
            t.colors.foreground.to_rgba(),
            t.colors.muted.to_rgba(),
            t.colors.primary.to_rgba(),
            t.colors.border.to_rgba(),
            t.radius.md.clone(),
            t.shadows.xl.clone(),
        )
    });
    
    let (bg_color, fg_color, muted_color, primary_color, border_color, radius_md, shadow_xl) = theme_colors();
    
    let current_step = if let Some(step) = props.current_step {
        step
    } else {
        internal_step()
    };
    
    let total_steps = props.steps.len();
    if total_steps == 0 || !props.open {
        return rsx! {};
    }
    
    let step = props.steps.get(current_step).cloned().unwrap_or_else(|| {
        props.steps.first().cloned().unwrap()
    });
    
    let mask_color = props.mask_color.clone().unwrap_or_else(|| {
        Color::new_rgba(0, 0, 0, 0.5).to_rgba()
    });
    
    let overlay_style = use_style(|_t| {
        Style::new()
            .fixed()
            .inset("0")
            .z_index(9998)
            .build()
    });
    
    let popover_style = use_style(|t| {
        Style::new()
            .absolute()
            .z_index(9999)
            .p(&t.spacing, "lg")
            .min_w_px(300)
            .max_w_px(400)
            .build()
    });
    
    let handle_next = move |_| {
        let next = (current_step + 1).min(total_steps - 1);
        if props.current_step.is_none() {
            internal_step.set(next);
        }
        if let Some(on_step_change) = props.on_step_change.clone() {
            on_step_change.call(next);
        }
        if next == total_steps - 1 && current_step == total_steps - 1 {
            if let Some(on_finish) = props.on_finish.clone() {
                on_finish.call(());
            }
            props.on_open_change.call(false);
        }
    };
    
    let handle_prev = move |_| {
        let prev = current_step.saturating_sub(1);
        if props.current_step.is_none() {
            internal_step.set(prev);
        }
        if let Some(on_step_change) = props.on_step_change.clone() {
            on_step_change.call(prev);
        }
    };
    
    let handle_skip = move |_| {
        props.on_open_change.call(false);
    };
    
    let is_first = current_step == 0;
    let is_last = current_step == total_steps - 1;
    
    rsx! {
        // Backdrop
        div {
            style: "{overlay_style} background: {mask_color};",
            onclick: handle_skip,
        }
        
        // Popover (positioned via CSS/data attributes)
        div {
            style: "{popover_style} background: {bg_color}; border-radius: {radius_md}; box-shadow: {shadow_xl};",
            "data-placement": "{step.placement:?}",
            
            // Header
            h3 {
                style: "margin: 0 0 8px 0; font-size: 16px; font-weight: 600; color: {fg_color};",
                "{step.title}"
            }
            
            // Content
            p {
                style: "margin: 0 0 16px 0; font-size: 14px; color: {muted_color}; line-height: 1.5;",
                "{step.content}"
            }
            
            // Footer
            div {
                style: "display: flex; justify-content: space-between; align-items: center; margin-top: 16px;",
                
                // Progress
                if props.show_progress {
                    span {
                        style: "font-size: 12px; color: {muted_color};",
                        "Step {current_step + 1} of {total_steps}"
                    }
                }
                
                // Buttons
                div {
                    style: "display: flex; gap: 8px;",
                    
                    if props.show_skip && !is_last {
                        button {
                            style: "padding: 6px 12px; font-size: 13px; background: transparent; border: none; color: {muted_color}; cursor: pointer;",
                            onclick: handle_skip,
                            "Skip"
                        }
                    }
                    
                    if !is_first {
                        button {
                            style: "padding: 6px 12px; font-size: 13px; background: {muted_color}; border: none; border-radius: {radius_md}; color: {fg_color}; cursor: pointer;",
                            onclick: handle_prev,
                            "Previous"
                        }
                    }
                    
                    button {
                        style: "padding: 6px 12px; font-size: 13px; background: {primary_color}; border: none; border-radius: {radius_md}; color: white; cursor: pointer;",
                        onclick: handle_next,
                        if is_last { "Finish" } else { "Next" }
                    }
                }
            }
            
            // Progress dots
            if props.show_progress {
                div {
                    style: "display: flex; justify-content: center; gap: 6px; margin-top: 16px;",
                    
                    for i in 0..total_steps {
                        {let dot_bg = if i == current_step { primary_color.clone() } else { border_color.clone() }; rsx! {
                            div {
                                style: "width: 8px; height: 8px; border-radius: 50%; background: {dot_bg};"
                            }
                        }}
                    }
                }
            }
        }
    }
}

/// Hook for controlling a tour
pub fn use_tour(steps: Vec<TourStep>) -> (Signal<bool>, Signal<usize>, TourController) {
    let is_open = use_signal(|| false);
    let current_step = use_signal(|| 0);
    
    let controller = TourController {
        is_open,
        current_step,
        steps,
    };
    
    (is_open, current_step, controller)
}

/// Tour controller for programmatic control
#[derive(Clone)]
pub struct TourController {
    is_open: Signal<bool>,
    current_step: Signal<usize>,
    steps: Vec<TourStep>,
}

impl TourController {
    /// Start the tour from the beginning
    pub fn start(&mut self) {
        self.current_step.set(0);
        self.is_open.set(true);
    }
    
    /// Stop/close the tour
    pub fn stop(&mut self) {
        self.is_open.set(false);
    }
    
    /// Go to the next step
    pub fn next(&mut self) {
        let current = *self.current_step.read();
        let max = self.steps.len().saturating_sub(1);
        self.current_step.set((current + 1).min(max));
    }
    
    /// Go to the previous step
    pub fn prev(&mut self) {
        let current = *self.current_step.read();
        self.current_step.set(current.saturating_sub(1));
    }
    
    /// Go to a specific step
    pub fn go_to(&mut self, step: usize) {
        let max = self.steps.len().saturating_sub(1);
        self.current_step.set(step.min(max));
    }
    
    /// Get current step index
    pub fn get_current_step(&self) -> usize {
        *self.current_step.read()
    }
    
    /// Check if tour is open
    pub fn get_is_open(&self) -> bool {
        *self.is_open.read()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tour_step_creation() {
        let step = TourStep::new("#header", "Welcome", "This is the header");
        assert_eq!(step.target, "#header");
        assert_eq!(step.title, "Welcome");
        assert_eq!(step.content, "This is the header");
        assert_eq!(step.placement, Placement::Bottom);
    }

    #[test]
    fn test_tour_step_with_placement() {
        let step = TourStep::new("#sidebar", "Nav", "Navigation menu")
            .with_placement(Placement::Right);
        assert_eq!(step.placement, Placement::Right);
    }
}
