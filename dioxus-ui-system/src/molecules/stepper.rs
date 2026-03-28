//! Stepper molecule components
//!
//! Provides step items and horizontal/vertical stepper layouts.

use dioxus::prelude::*;

use crate::atoms::{
    AlignItems, Box, HStack, JustifyContent, SpacingSize, StepConnector, StepIndicator, StepLabel,
    StepSize, StepState, VStack,
};

/// Step item data structure
#[derive(Clone, PartialEq, Debug)]
pub struct StepItem {
    /// Step label
    pub label: String,
    /// Optional step description
    pub description: Option<String>,
    /// Optional icon name
    pub icon: Option<String>,
    /// Step state
    pub state: StepState,
    /// Whether step is disabled
    pub disabled: bool,
    /// Optional error message
    pub error: Option<String>,
}

impl StepItem {
    /// Create a new step item
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            description: None,
            icon: None,
            state: StepState::Pending,
            disabled: false,
            error: None,
        }
    }

    /// Add description to step
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Add icon to step
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set step state
    pub fn with_state(mut self, state: StepState) -> Self {
        self.state = state;
        self
    }

    /// Mark step as disabled
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    /// Add error to step
    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self.state = StepState::Error;
        self
    }
}

/// Step item molecule - combines indicator and label
#[derive(Props, Clone, PartialEq)]
pub struct StepItemProps {
    /// Step index (0-based)
    pub index: usize,
    /// Step data
    pub step: StepItem,
    /// Step size
    #[props(default)]
    pub size: StepSize,
    /// Whether to show connector (not for last step)
    #[props(default = true)]
    pub show_connector: bool,
    /// Connector is completed
    #[props(default)]
    pub connector_completed: bool,
    /// Horizontal layout
    #[props(default = true)]
    pub horizontal: bool,
    /// Click handler
    #[props(default)]
    pub on_click: Option<EventHandler<usize>>,
}

/// Step item molecule - displays a single step with indicator, label, and optional connector
#[component]
pub fn StepItemComponent(props: StepItemProps) -> Element {
    let step = &props.step;
    let clickable = !step.disabled && props.on_click.is_some();
    let on_click = props.on_click.clone();
    let index = props.index;

    let indicator = rsx! {
        StepIndicator {
            step: (props.index + 1) as u32,
            state: step.state.clone(),
            size: props.size.clone(),
            icon: step.icon.clone(),
            on_click: if clickable {
                Some(EventHandler::new(move |_| {
                    if let Some(handler) = on_click.clone() {
                        handler.call(index);
                    }
                }))
            } else {
                None
            },
        }
    };

    let label = rsx! {
        StepLabel {
            label: step.label.clone(),
            description: step.description.clone(),
            state: step.state.clone(),
            size: props.size.clone(),
        }
    };

    let connector = if props.show_connector {
        Some(rsx! {
            StepConnector {
                horizontal: props.horizontal,
                completed: props.connector_completed,
            }
        })
    } else {
        None
    };

    let cursor_val = if clickable { "pointer" } else { "default" };
    let opacity_val = if step.disabled { "0.5" } else { "1" };

    if props.horizontal {
        rsx! {
            div {
                style: "display: flex; align-items: center; flex: 1;",

                // Step content (indicator + label stacked)
                div {
                    style: "display: flex; flex-direction: column; align-items: center; gap: 8px; cursor: {cursor_val}; opacity: {opacity_val};",

                    {indicator}
                    {label}
                }

                // Connector
                {connector}
            }
        }
    } else {
        rsx! {
            div {
                style: "display: flex; flex-direction: column;",

                // Step content row (indicator + label side by side)
                div {
                    style: "display: flex; align-items: flex-start; gap: 12px; cursor: {cursor_val}; opacity: {opacity_val};",

                    // Indicator column
                    div {
                        style: "display: flex; flex-direction: column; align-items: center;",

                        {indicator}
                        {connector}
                    }

                    // Label column
                    Box {
                        style: "padding-top: 6px;",
                        {label}
                    }
                }
            }
        }
    }
}

/// Horizontal stepper molecule
#[derive(Props, Clone, PartialEq)]
pub struct HorizontalStepperProps {
    /// List of steps
    pub steps: Vec<StepItem>,
    /// Current active step index (0-based)
    pub active_step: usize,
    /// Step size
    #[props(default)]
    pub size: StepSize,
    /// Optional click handler for steps
    #[props(default)]
    pub on_step_click: Option<EventHandler<usize>>,
    /// Allow clicking completed steps
    #[props(default = true)]
    pub allow_click_completed: bool,
}

/// Horizontal stepper - steps arranged horizontally
#[component]
pub fn HorizontalStepper(props: HorizontalStepperProps) -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: flex-start; width: 100%;",
            role: "tablist",
            aria_label: Some("Progress steps"),

            for (index, step) in props.steps.iter().enumerate() {
                StepItemComponent {
                    key: "{index}",
                    index: index,
                    step: step.clone(),
                    size: props.size.clone(),
                    show_connector: index < props.steps.len() - 1,
                    connector_completed: index < props.active_step,
                    horizontal: true,
                    on_click: props.on_step_click.clone(),
                }
            }
        }
    }
}

/// Vertical stepper molecule
#[derive(Props, Clone, PartialEq)]
pub struct VerticalStepperProps {
    /// List of steps
    pub steps: Vec<StepItem>,
    /// Current active step index (0-based)
    pub active_step: usize,
    /// Step size
    #[props(default)]
    pub size: StepSize,
    /// Optional click handler for steps
    #[props(default)]
    pub on_step_click: Option<EventHandler<usize>>,
}

/// Vertical stepper - steps arranged vertically
#[component]
pub fn VerticalStepper(props: VerticalStepperProps) -> Element {
    rsx! {
        VStack {
            gap: SpacingSize::None,
            align: AlignItems::Stretch,

            div {
                style: "display: flex; flex-direction: column; gap: 0;",
                role: "tablist",
                aria_label: Some("Progress steps"),
                aria_orientation: "vertical",

                for (index, step) in props.steps.iter().enumerate() {
                    StepItemComponent {
                        key: "{index}",
                        index: index,
                        step: step.clone(),
                        size: props.size.clone(),
                        show_connector: index < props.steps.len() - 1,
                        connector_completed: index < props.active_step,
                        horizontal: false,
                        on_click: props.on_step_click.clone(),
                    }
                }
            }
        }
    }
}

/// Step content panel for showing step content
#[derive(Props, Clone, PartialEq)]
pub struct StepContentProps {
    /// Current active step
    pub active_step: usize,
    /// Step index this content belongs to
    pub step_index: usize,
    /// Content to display
    pub children: Element,
}

/// Step content panel - shows content only when step is active
#[component]
pub fn StepContent(props: StepContentProps) -> Element {
    let is_active = props.active_step == props.step_index;

    rsx! {
        if is_active {
            div {
                style: "animation: fadeIn 200ms ease;",
                role: "tabpanel",
                id: "step-content-{props.step_index}",
                aria_labelledby: "step-{props.step_index}",

                {props.children}
            }
        }
    }
}

/// Stepper actions (navigation buttons)
#[derive(Props, Clone, PartialEq)]
pub struct StepperActionsProps {
    /// Current step
    pub current_step: usize,
    /// Total steps
    pub total_steps: usize,
    /// Back button handler
    #[props(default)]
    pub on_back: Option<EventHandler<()>>,
    /// Next button handler
    #[props(default)]
    pub on_next: Option<EventHandler<()>>,
    /// Finish button handler
    #[props(default)]
    pub on_finish: Option<EventHandler<()>>,
    /// Skip button handler
    #[props(default)]
    pub on_skip: Option<EventHandler<()>>,
    /// Custom finish button label
    #[props(default = "Finish".to_string())]
    pub finish_label: String,
    /// Custom next button label
    #[props(default = "Next".to_string())]
    pub next_label: String,
    /// Custom back button label
    #[props(default = "Back".to_string())]
    pub back_label: String,
    /// Disable back button
    #[props(default)]
    pub disable_back: bool,
    /// Disable next button
    #[props(default)]
    pub disable_next: bool,
    /// Show skip button
    #[props(default)]
    pub show_skip: bool,
    /// Alignment
    #[props(default = StepperActionsAlign::End)]
    pub align: StepperActionsAlign,
}

/// Stepper actions alignment
#[derive(Clone, PartialEq, Default)]
pub enum StepperActionsAlign {
    #[default]
    End,
    Center,
    SpaceBetween,
}

/// Stepper actions - navigation buttons for stepper
#[component]
pub fn StepperActions(props: StepperActionsProps) -> Element {
    let is_first = props.current_step == 0;
    let is_last = props.current_step >= props.total_steps - 1;

    let justify_content = match props.align {
        StepperActionsAlign::End => JustifyContent::End,
        StepperActionsAlign::Center => JustifyContent::Center,
        StepperActionsAlign::SpaceBetween => JustifyContent::SpaceBetween,
    };

    rsx! {
        HStack {
            style: "margin-top: 24px; padding-top: 24px; border-top: 1px solid #e2e8f0;",
            justify: justify_content,
            align: AlignItems::Center,
            gap: SpacingSize::Md,

            // Back button (or spacer for alignment)
            if !is_first {
                if let Some(on_back) = props.on_back.clone() {
                    crate::atoms::Button {
                        variant: crate::atoms::ButtonVariant::Secondary,
                        disabled: props.disable_back,
                        onclick: move |_| on_back.call(()),
                        "{props.back_label}"
                    }
                }
            } else if props.align == StepperActionsAlign::SpaceBetween {
                Box {}
            }

            // Skip button (optional)
            if props.show_skip && !is_last {
                if let Some(on_skip) = props.on_skip.clone() {
                    crate::atoms::Button {
                        variant: crate::atoms::ButtonVariant::Ghost,
                        onclick: move |_| on_skip.call(()),
                        "Skip"
                    }
                }
            }

            // Next/Finish button
            if is_last {
                if let Some(on_finish) = props.on_finish.clone() {
                    crate::atoms::Button {
                        variant: crate::atoms::ButtonVariant::Primary,
                        disabled: props.disable_next,
                        onclick: move |_| on_finish.call(()),
                        "{props.finish_label}"
                    }
                }
            } else {
                if let Some(on_next) = props.on_next.clone() {
                    crate::atoms::Button {
                        variant: crate::atoms::ButtonVariant::Primary,
                        disabled: props.disable_next,
                        onclick: move |_| on_next.call(()),
                        "{props.next_label}"
                    }
                }
            }
        }
    }
}
