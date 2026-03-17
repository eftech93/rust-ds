//! Step atom component
//!
//! Individual step indicator with number/icon and state support.

use dioxus::prelude::*;

use crate::atoms::{Icon, IconSize, IconColor};

/// Step state
#[derive(Clone, PartialEq, Default, Debug)]
pub enum StepState {
    /// Step not yet reached
    #[default]
    Pending,
    /// Current active step
    Active,
    /// Step completed
    Completed,
    /// Step has error
    Error,
}

impl StepState {
    /// Get the color for this state
    pub fn color(&self) -> &'static str {
        match self {
            StepState::Pending => "#94a3b8",
            StepState::Active => "#0f172a",
            StepState::Completed => "#22c55e",
            StepState::Error => "#ef4444",
        }
    }

    /// Get the background color for this state
    pub fn bg_color(&self) -> &'static str {
        match self {
            StepState::Pending => "#f1f5f9",
            StepState::Active => "#0f172a",
            StepState::Completed => "#22c55e",
            StepState::Error => "#fef2f2",
        }
    }

    /// Get the text color for this state
    pub fn text_color(&self) -> &'static str {
        match self {
            StepState::Pending => "#64748b",
            StepState::Active => "white",
            StepState::Completed => "white",
            StepState::Error => "#ef4444",
        }
    }
}

/// Step size
#[derive(Clone, PartialEq, Default, Debug)]
pub enum StepSize {
    /// Small step indicator
    Sm,
    /// Medium step indicator (default)
    #[default]
    Md,
    /// Large step indicator
    Lg,
}

impl StepSize {
    /// Get the size in pixels
    pub fn size_px(&self) -> u32 {
        match self {
            StepSize::Sm => 24,
            StepSize::Md => 32,
            StepSize::Lg => 40,
        }
    }

    /// Get font size
    pub fn font_size(&self) -> &'static str {
        match self {
            StepSize::Sm => "12px",
            StepSize::Md => "14px",
            StepSize::Lg => "16px",
        }
    }
}

/// Step indicator atom
#[derive(Props, Clone, PartialEq)]
pub struct StepIndicatorProps {
    /// Step number (1-indexed)
    #[props(default = 1)]
    pub step: u32,
    /// Step state
    #[props(default)]
    pub state: StepState,
    /// Step size
    #[props(default)]
    pub size: StepSize,
    /// Optional icon to replace number
    #[props(default)]
    pub icon: Option<String>,
    /// Custom background color (overrides state color)
    #[props(default)]
    pub bg_color: Option<String>,
    /// Custom text color
    #[props(default)]
    pub text_color: Option<String>,
    /// Optional click handler
    #[props(default)]
    pub on_click: Option<EventHandler<()>>,
    /// Optional aria label
    #[props(default)]
    pub aria_label: Option<String>,
}

/// Step indicator atom - shows the step number or icon in a circle
#[component]
pub fn StepIndicator(props: StepIndicatorProps) -> Element {
    let size = props.size.size_px();
    let bg = props.bg_color.clone().unwrap_or_else(|| props.state.bg_color().to_string());
    let color = props.text_color.clone().unwrap_or_else(|| props.state.text_color().to_string());
    let border = if props.state == StepState::Active {
        "2px solid #3b82f6"
    } else {
        "none"
    };
    
    let cursor = if props.on_click.is_some() { "pointer" } else { "default" };
    let clickable = props.on_click.clone();
    
    // Content based on state and icon
    let content: Element = if let Some(icon) = props.icon.clone() {
        rsx! {
            Icon {
                name: icon,
                size: match props.size {
                    StepSize::Sm => IconSize::Small,
                    StepSize::Md => IconSize::Medium,
                    StepSize::Lg => IconSize::Large,
                },
                color: if props.state == StepState::Completed {
                    IconColor::Success
                } else if props.state == StepState::Error {
                    IconColor::Destructive
                } else {
                    IconColor::Current
                },
            }
        }
    } else {
        match props.state {
            StepState::Completed => rsx! {
                Icon {
                    name: "check".to_string(),
                    size: match props.size {
                        StepSize::Sm => IconSize::Small,
                        StepSize::Md => IconSize::Medium,
                        StepSize::Lg => IconSize::Large,
                    },
                    color: IconColor::Success,
                }
            },
            StepState::Error => rsx! {
                Icon {
                    name: "alert-triangle".to_string(),
                    size: match props.size {
                        StepSize::Sm => IconSize::Small,
                        StepSize::Md => IconSize::Medium,
                        StepSize::Lg => IconSize::Large,
                    },
                    color: IconColor::Destructive,
                }
            },
            _ => rsx! { "{props.step}" },
        }
    };
    
    let font_size_val = props.size.font_size();
    
    rsx! {
        div {
            style: "
                width: {size}px; 
                height: {size}px; 
                border-radius: 50%; 
                background: {bg}; 
                color: {color}; 
                border: {border};
                display: flex; 
                align-items: center; 
                justify-content: center; 
                font-size: {font_size_val}; 
                font-weight: 600;
                flex-shrink: 0;
                cursor: {cursor};
                transition: all 200ms ease;
            ",
            aria_label: props.aria_label.clone().unwrap_or_else(|| format!("Step {}", props.step)),
            aria_current: if props.state == StepState::Active { Some("step") } else { None },
            onclick: move |_| {
                if let Some(handler) = clickable.clone() {
                    handler.call(());
                }
            },
            
            {content}
        }
    }
}

/// Step connector line between steps
#[derive(Props, Clone, PartialEq)]
pub struct StepConnectorProps {
    /// Connector orientation
    #[props(default = true)]
    pub horizontal: bool,
    /// Whether the connector represents completed progress
    #[props(default)]
    pub completed: bool,
    /// Connector color (overrides default)
    #[props(default)]
    pub color: Option<String>,
    /// Connector thickness
    #[props(default = "2px".to_string())]
    pub thickness: String,
}

/// Step connector atom - line connecting step indicators
#[component]
pub fn StepConnector(props: StepConnectorProps) -> Element {
    let color = props.color.clone().unwrap_or_else(|| {
        if props.completed { "#22c55e".to_string() } else { "#e2e8f0".to_string() }
    });
    let thickness_val = props.thickness.clone();
    
    if props.horizontal {
        rsx! {
            div {
                style: "
                    flex: 1;
                    height: {thickness_val};
                    background: {color};
                    min-width: 24px;
                    transition: background 200ms ease;
                ",
                role: "separator",
                aria_orientation: "horizontal",
            }
        }
    } else {
        rsx! {
            div {
                style: "
                    width: {thickness_val};
                    flex: 1;
                    background: {color};
                    min-height: 24px;
                    margin-left: 15px;
                    transition: background 200ms ease;
                ",
                role: "separator",
                aria_orientation: "vertical",
            }
        }
    }
}

/// Step label atom
#[derive(Props, Clone, PartialEq)]
pub struct StepLabelProps {
    /// Label text
    pub label: String,
    /// Optional description/subtitle
    #[props(default)]
    pub description: Option<String>,
    /// Step state (affects color)
    #[props(default)]
    pub state: StepState,
    /// Text size
    #[props(default)]
    pub size: StepSize,
}

/// Step label atom - displays step title and optional description
#[component]
pub fn StepLabel(props: StepLabelProps) -> Element {
    let label_color = if props.state == StepState::Active { "#0f172a" } else { "#64748b" };
    let font_size_val = match props.size {
        StepSize::Sm => "13px",
        StepSize::Md => "14px",
        StepSize::Lg => "16px",
    };
    let desc_size_val = match props.size {
        StepSize::Sm => "11px",
        StepSize::Md => "12px",
        StepSize::Lg => "13px",
    };
    let weight_val = if props.state == StepState::Active { "600" } else { "500" };
    
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 2px;",
            
            span {
                style: "
                    font-size: {font_size_val}; 
                    font-weight: {weight_val}; 
                    color: {label_color};
                    white-space: nowrap;
                ",
                "{props.label}"
            }
            
            if let Some(desc) = props.description.clone() {
                span {
                    style: "
                        font-size: {desc_size_val}; 
                        color: #94a3b8;
                        white-space: nowrap;
                    ",
                    "{desc}"
                }
            }
        }
    }
}
