//! Stepper organism component
//!
//! Complete stepper with header, content areas, and navigation.

use dioxus::prelude::*;

use crate::atoms::{StepState, StepSize, Heading, HeadingLevel};
use crate::molecules::{
    StepItem, HorizontalStepper, VerticalStepper, StepperActions,
    StepperActionsAlign, StepItemComponent,
    Card, CardVariant,
};

/// Stepper variant
#[derive(Clone, PartialEq, Default, Debug)]
pub enum StepperVariant {
    /// Horizontal stepper (default)
    #[default]
    Horizontal,
    /// Vertical stepper
    Vertical,
    /// Compact stepper (smaller, no labels)
    Compact,
}

/// Stepper organism props
#[derive(Props, Clone, PartialEq)]
pub struct StepperProps {
    /// List of steps
    pub steps: Vec<StepItem>,
    /// Current active step index
    pub active_step: usize,
    /// Stepper variant
    #[props(default)]
    pub variant: StepperVariant,
    /// Step size
    #[props(default)]
    pub size: StepSize,
    /// Optional title
    #[props(default)]
    pub title: Option<String>,
    /// Optional description
    #[props(default)]
    pub description: Option<String>,
    /// Step change callback
    #[props(default)]
    pub on_step_change: Option<EventHandler<usize>>,
    /// Show step numbers
    #[props(default = true)]
    pub show_numbers: bool,
    /// Allow clicking on completed steps
    #[props(default = true)]
    pub allow_back_navigation: bool,
    /// Show content area
    #[props(default = true)]
    pub show_content: bool,
    /// Content elements for each step
    #[props(default)]
    pub children: Element,
}

/// Stepper organism - complete stepper component
#[component]
pub fn Stepper(props: StepperProps) -> Element {
    // Update step states based on active step
    let steps: Vec<StepItem> = props.steps.iter().enumerate().map(|(i, step)| {
        let mut updated = step.clone();
        updated.state = if i < props.active_step {
            StepState::Completed
        } else if i == props.active_step {
            StepState::Active
        } else {
            StepState::Pending
        };
        updated
    }).collect();
    
    rsx! {
        Card {
            variant: CardVariant::Default,
            full_width: true,
            
            div {
                style: "display: flex; flex-direction: column; gap: 24px;",
                
                // Header
                if props.title.is_some() || props.description.is_some() {
                    div {
                        if let Some(title) = props.title.clone() {
                            Heading {
                                level: HeadingLevel::H3,
                                "{title}"
                            }
                        }
                        
                        if let Some(desc) = props.description.clone() {
                            p {
                                style: "margin: 4px 0 0 0; color: #64748b; font-size: 14px;",
                                "{desc}"
                            }
                        }
                    }
                }
                
                // Stepper header
                match props.variant {
                    StepperVariant::Horizontal => rsx! {
                        HorizontalStepper {
                            steps: steps.clone(),
                            active_step: props.active_step,
                            size: props.size.clone(),
                            on_step_click: if props.allow_back_navigation {
                                props.on_step_change.clone()
                            } else {
                                None
                            },
                        }
                    },
                    StepperVariant::Vertical => rsx! {
                        VerticalStepper {
                            steps: steps.clone(),
                            active_step: props.active_step,
                            size: props.size.clone(),
                            on_step_click: if props.allow_back_navigation {
                                props.on_step_change.clone()
                            } else {
                                None
                            },
                        }
                    },
                    StepperVariant::Compact => rsx! {
                        CompactStepper {
                            steps: steps.clone(),
                            active_step: props.active_step,
                            size: StepSize::Sm,
                            on_step_click: props.on_step_change.clone(),
                        }
                    },
                }
                
                // Content area
                if props.show_content {
                    div {
                        style: "min-height: 100px;",
                        
                        {props.children}
                    }
                }
            }
        }
    }
}

/// Compact stepper - minimal horizontal stepper
#[derive(Props, Clone, PartialEq)]
pub struct CompactStepperProps {
    /// List of steps
    pub steps: Vec<StepItem>,
    /// Current active step
    pub active_step: usize,
    /// Step size
    #[props(default = StepSize::Sm)]
    pub size: StepSize,
    /// Click handler
    #[props(default)]
    pub on_step_click: Option<EventHandler<usize>>,
}

#[component]
pub fn CompactStepper(props: CompactStepperProps) -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: center; justify-content: center; gap: 8px;",
            
            for (index, step) in props.steps.iter().enumerate() {
                div {
                    key: "{index}",
                    style: "display: flex; align-items: center; gap: 8px;",
                    
                    StepItemComponent {
                        index: index,
                        step: step.clone(),
                        size: props.size.clone(),
                        show_connector: false,
                        connector_completed: false,
                        horizontal: true,
                        on_click: props.on_step_click.clone(),
                    }
                    
                    if index < props.steps.len() - 1 {
                        div {
                            style: if index < props.active_step { "width: 16px; height: 2px; background: #22c55e;" } else { "width: 16px; height: 2px; background: #e2e8f0;" },
                        }
                    }
                }
            }
        }
    }
}

/// Wizard stepper - full-featured wizard with validation
#[derive(Props, Clone, PartialEq)]
pub struct WizardProps {
    /// List of steps
    pub steps: Vec<WizardStep>,
    /// Current active step
    pub active_step: usize,
    /// Step change callback
    pub on_step_change: EventHandler<usize>,
    /// Finish callback
    pub on_finish: EventHandler<()>,
    /// Cancel callback
    #[props(default)]
    pub on_cancel: Option<EventHandler<()>>,
    /// Wizard title
    #[props(default)]
    pub title: Option<String>,
    /// Show progress summary
    #[props(default = true)]
    pub show_progress: bool,
    /// Enable validation
    #[props(default = true)]
    pub validate: bool,
    /// Step content
    pub children: Element,
}

/// Wizard step with validation
#[derive(Clone, PartialEq)]
pub struct WizardStep {
    /// Step label
    pub label: String,
    /// Step description
    pub description: Option<String>,
    /// Validation function - returns true if step is valid
    pub is_valid: bool,
    /// Step content
    pub content: Option<Element>,
}

impl WizardStep {
    /// Create new wizard step
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            description: None,
            is_valid: true,
            content: None,
        }
    }

    /// Add description
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set validation state
    pub fn valid(mut self, valid: bool) -> Self {
        self.is_valid = valid;
        self
    }
}

/// Wizard organism - full-featured multi-step wizard
#[component]
pub fn Wizard(props: WizardProps) -> Element {
    let total_steps = props.steps.len();
    let current = props.active_step;
    
    // Convert wizard steps to step items
    let step_items: Vec<StepItem> = props.steps.iter().enumerate().map(|(i, step)| {
        let state = if i < current {
            StepState::Completed
        } else if i == current {
            if step.is_valid {
                StepState::Active
            } else {
                StepState::Error
            }
        } else {
            StepState::Pending
        };
        
        StepItem {
            label: step.label.clone(),
            description: step.description.clone(),
            icon: None,
            state,
            disabled: false,
            error: if !step.is_valid && i == current {
                Some("Please complete required fields".to_string())
            } else {
                None
            },
        }
    }).collect();
    
    let can_go_next = props.steps.get(current).map(|s| s.is_valid).unwrap_or(true);
    let can_finish = current == total_steps - 1 && can_go_next;
    
    let progress = ((current + 1) as f32 / total_steps as f32 * 100.0) as u32;
    
    rsx! {
        Card {
            variant: CardVariant::Elevated,
            full_width: true,
            
            div {
                style: "display: flex; flex-direction: column; gap: 24px;",
                
                // Header with progress
                if let Some(title) = props.title.clone() {
                    div {
                        style: "display: flex; justify-content: space-between; align-items: center;",
                        
                        Heading {
                            level: HeadingLevel::H3,
                            "{title}"
                        }
                        
                        if props.show_progress {
                            span {
                                style: "font-size: 14px; color: #64748b;",
                                "Step {current + 1} of {total_steps}"
                            }
                        }
                    }
                }
                
                // Progress bar
                if props.show_progress {
                    div {
                        style: "width: 100%; height: 4px; background: #e2e8f0; border-radius: 2px; overflow: hidden;",
                        
                        div {
                            style: "height: 100%; width: {progress}%; background: #22c55e; transition: width 300ms ease;",
                        }
                    }
                }
                
                // Stepper
                HorizontalStepper {
                    steps: step_items,
                    active_step: current,
                    size: StepSize::Md,
                    on_step_click: Some(EventHandler::new(move |step: usize| {
                        // Only allow going back or to completed steps
                        if step <= current {
                            props.on_step_change.call(step);
                        }
                    })),
                }
                
                // Content
                div {
                    style: "min-height: 150px; padding: 16px 0;",
                    
                    {props.children}
                }
                
                // Actions
                StepperActions {
                    current_step: current,
                    total_steps: total_steps,
                    on_back: if current > 0 {
                        Some(EventHandler::new(move |_| {
                            if current > 0 {
                                props.on_step_change.call(current - 1);
                            }
                        }))
                    } else {
                        None
                    },
                    on_next: if !can_finish {
                        Some(EventHandler::new(move |_| {
                            if current < total_steps - 1 {
                                props.on_step_change.call(current + 1);
                            }
                        }))
                    } else {
                        None
                    },
                    on_finish: if can_finish {
                        Some(EventHandler::new(move |_| {
                            props.on_finish.call(());
                        }))
                    } else {
                        None
                    },
                    on_skip: None,
                    disable_next: !can_go_next,
                    align: StepperActionsAlign::SpaceBetween,
                }
            }
        }
    }
}

/// Step summary component - shows overview of all steps
#[derive(Props, Clone, PartialEq)]
pub struct StepSummaryProps {
    /// Steps data
    pub steps: Vec<StepSummaryItem>,
    /// Show edit buttons
    #[props(default)]
    pub editable: bool,
    /// Edit callback
    #[props(default)]
    pub on_edit: Option<EventHandler<usize>>,
}

/// Step summary item
#[derive(Clone, PartialEq)]
pub struct StepSummaryItem {
    /// Step label
    pub label: String,
    /// Step value/content
    pub value: String,
    /// Is step complete
    pub completed: bool,
}

impl StepSummaryItem {
    /// Create new summary item
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            completed: true,
        }
    }
}

/// Step summary - review all steps before final submission
#[component]
pub fn StepSummary(props: StepSummaryProps) -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 16px;",
            
            for (index, item) in props.steps.iter().enumerate() {
                div {
                    key: "{index}",
                    style: "
                        display: flex; 
                        justify-content: space-between; 
                        align-items: flex-start;
                        padding: 16px;
                        background: #f8fafc;
                        border-radius: 8px;
                        border: 1px solid #e2e8f0;
                    ",
                    
                    div {
                        style: "display: flex; flex-direction: column; gap: 4px;",
                        
                        span {
                            style: "font-size: 12px; font-weight: 600; color: #64748b; text-transform: uppercase;",
                            "{item.label}"
                        }
                        
                        span {
                            style: "font-size: 14px; color: #0f172a;",
                            "{item.value}"
                        }
                    }
                    
                    if props.editable {
                        if let Some(on_edit) = props.on_edit.clone() {
                            crate::atoms::Button {
                                variant: crate::atoms::ButtonVariant::Ghost,
                                size: crate::atoms::ButtonSize::Sm,
                                onclick: move |_| on_edit.call(index),
                                "Edit"
                            }
                        }
                    }
                }
            }
        }
    }
}
