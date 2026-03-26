//! Rating atom component
//!
//! Star rating display and input.

use dioxus::prelude::*;
use crate::theme::use_theme;

/// Rating properties
#[derive(Props, Clone, PartialEq)]
pub struct RatingProps {
    /// Current rating value
    pub value: f32,
    /// Maximum rating (default: 5)
    #[props(default = 5)]
    pub max: u8,
    /// Allow half stars
    #[props(default = true)]
    pub allow_half: bool,
    /// Interactive (clickable)
    #[props(default = false)]
    pub interactive: bool,
    /// Size in pixels
    #[props(default = 20)]
    pub size: u16,
    /// Color for filled stars
    pub fill_color: Option<String>,
    /// Color for empty stars
    pub empty_color: Option<String>,
    /// Change handler (only used when interactive)
    #[props(default)]
    pub on_change: Option<EventHandler<f32>>,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Show rating value text
    #[props(default = false)]
    pub show_value: bool,
}

/// Rating component (star display/input)
#[component]
pub fn Rating(props: RatingProps) -> Element {
    let theme = use_theme();
    
    let fill_color = props.fill_color.unwrap_or_else(|| "#fbbf24".to_string()); // amber-400
    let empty_color = props.empty_color.unwrap_or_else(|| {
        theme.tokens.read().colors.muted.to_rgba()
    });
    
    let class_css = props.class.as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();
    
    let value = props.value.clamp(0.0, props.max as f32);
    
    rsx! {
        span {
            class: "rating{class_css}",
            style: "display: inline-flex; align-items: center; gap: 4px;",
            
            div {
                style: "display: inline-flex; gap: 2px;",
                
                for i in 1..=props.max {
                    {
                        let star_value = i as f32;
                        let is_filled = value >= star_value;
                        let is_half = props.allow_half && !is_filled && value >= star_value - 0.5;
                        let star_color = if is_filled || is_half { fill_color.clone() } else { empty_color.clone() };
                        
                        let cursor = if props.interactive { "cursor: pointer;" } else { "" };
                        
                        rsx! {
                            span {
                                key: "{i}",
                                style: "font-size: {props.size}px; color: {star_color}; {cursor} user-select: none; line-height: 1;",
                                onclick: {
                                    let on_change = props.on_change.clone();
                                    move |e: Event<dioxus::html::MouseData>| {
                                        if props.interactive {
                                            e.stop_propagation();
                                            if let Some(handler) = &on_change {
                                                handler.call(star_value);
                                            }
                                        }
                                    }
                                },
                                
                                if is_half {
                                    // Half star using linear gradient
                                    span {
                                        style: "background: linear-gradient(90deg, {fill_color} 50%, {empty_color} 50%); -webkit-background-clip: text; -webkit-text-fill-color: transparent; background-clip: text;",
                                        "★"
                                    }
                                } else {
                                    "★"
                                }
                            }
                        }
                    }
                }
            }
            
            if props.show_value {
                span {
                    style: format!("font-size: {}px; color: {}; margin-left: 8px;", props.size * 3 / 4, theme.tokens.read().colors.foreground.to_rgba()),
                    "{value:.1}"
                }
            }
        }
    }
}

/// Rating input properties (interactive rating selector)
#[derive(Props, Clone, PartialEq)]
pub struct RatingInputProps {
    /// Current rating value
    #[props(default = 0.0)]
    pub value: f32,
    /// Maximum rating
    #[props(default = 5)]
    pub max: u8,
    /// Allow half stars
    #[props(default = false)]
    pub allow_half: bool,
    /// Size in pixels
    #[props(default = 24)]
    pub size: u16,
    /// Change handler
    pub on_change: EventHandler<f32>,
    /// Label
    #[props(default)]
    pub label: Option<String>,
    /// Show clear button
    #[props(default = false)]
    pub clearable: bool,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Rating input component (interactive)
#[component]
pub fn RatingInput(props: RatingInputProps) -> Element {
    let mut hover_value = use_signal(|| None::<f32>);
    
    let display_value = hover_value().unwrap_or(props.value);
    
    let class_css = props.class.as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();
    
    rsx! {
        div {
            class: "rating-input{class_css}",
            style: "display: flex; flex-direction: column; gap: 8px;",
            
            if let Some(label) = props.label {
                label {
                    style: "font-size: 14px; font-weight: 500;",
                    "{label}"
                }
            }
            
            div {
                style: "display: flex; align-items: center; gap: 12px;",
                
                div {
                    style: "display: inline-flex; gap: 4px;",
                    
                    for i in 1..=props.max {
                        {
                            let star_value = i as f32;
                            let is_filled = display_value >= star_value;
                            let is_half = props.allow_half && !is_filled && display_value >= star_value - 0.5;
                            
                            let on_click = move |_| {
                                let new_value = if props.allow_half && display_value == star_value - 0.5 {
                                    star_value
                                } else if display_value == star_value {
                                    0.0 // Clear on second click
                                } else {
                                    star_value
                                };
                                props.on_change.call(new_value);
                                hover_value.set(None);
                            };
                            
                            let on_mouse_enter = move |_| {
                                hover_value.set(Some(star_value));
                            };
                            
                            rsx! {
                                button {
                                    key: "{i}",
                                    type: "button",
                                    style: "font-size: {props.size}px; background: none; border: none; cursor: pointer; padding: 2px; line-height: 1; transition: transform 0.15s ease;",
                                    onclick: on_click,
                                    onmouseenter: on_mouse_enter,
                                    
                                    if is_half {
                                        span {
                                            style: "color: #fbbf24;",
                                            "★"
                                        }
                                    } else if is_filled {
                                        span {
                                            style: "color: #fbbf24;",
                                            "★"
                                        }
                                    } else {
                                        span {
                                            style: "color: #d1d5db;",
                                            "★"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                if props.clearable && props.value > 0.0 {
                    button {
                        type: "button",
                        style: "font-size: 12px; color: #6b7280; background: none; border: none; cursor: pointer; text-decoration: underline;",
                        onclick: move |_| props.on_change.call(0.0),
                        "Clear"
                    }
                }
                
                span {
                    style: "font-size: 14px; color: #374151; min-width: 40px;",
                    "{display_value:.1}"
                }
            }
        }
    }
}

/// Review summary properties
#[derive(Props, Clone, PartialEq)]
pub struct ReviewSummaryProps {
    /// Average rating
    pub average: f32,
    /// Total number of reviews
    pub total: u32,
    /// Distribution of ratings (5 stars, 4 stars, etc.)
    #[props(default)]
    pub distribution: Option<Vec<u32>>,
    /// Maximum rating
    #[props(default = 5)]
    pub max: u8,
    /// Show write review button
    #[props(default = false)]
    pub show_write_button: bool,
    /// Write review handler
    #[props(default)]
    pub on_write_review: Option<EventHandler<()>>,
}

/// Review summary component
#[component]
pub fn ReviewSummary(props: ReviewSummaryProps) -> Element {
    let class_css = "";
    
    rsx! {
        div {
            class: "review-summary{class_css}",
            style: "display: flex; flex-direction: column; gap: 16px; padding: 20px; border: 1px solid #e5e7eb; border-radius: 12px; background: white;",
            
            div {
                style: "display: flex; align-items: center; gap: 16px;",
                
                div {
                    style: "text-align: center;",
                    
                    div {
                        style: "font-size: 48px; font-weight: 700; color: #111827; line-height: 1;",
                        "{props.average:.1}"
                    }
                    
                    Rating {
                        value: props.average,
                        max: props.max,
                        size: 16,
                        show_value: false,
                    }
                    
                    div {
                        style: "font-size: 14px; color: #6b7280; margin-top: 4px;",
                        "{props.total} reviews"
                    }
                }
                
                if let Some(distribution) = props.distribution {
                    div {
                        style: "flex: 1; display: flex; flex-direction: column; gap: 4px;",
                        
                        for (i, count) in distribution.iter().enumerate().rev() {
                            {
                                let stars = i + 1;
                                let percentage = if props.total > 0 {
                                    (*count as f32 / props.total as f32) * 100.0
                                } else {
                                    0.0
                                };
                                
                                rsx! {
                                    div {
                                        key: "{stars}",
                                        style: "display: flex; align-items: center; gap: 8px; font-size: 12px;",
                                        
                                        span {
                                            style: "width: 40px; color: #6b7280;",
                                            "{stars} star"
                                        }
                                        
                                        div {
                                            style: "flex: 1; height: 8px; background: #e5e7eb; border-radius: 4px; overflow: hidden;",
                                            
                                            div {
                                                style: "height: 100%; width: {percentage}%; background: #fbbf24; border-radius: 4px;",
                                            }
                                        }
                                        
                                        span {
                                            style: "width: 40px; text-align: right; color: #6b7280;",
                                            "{count}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            if props.show_write_button {
                button {
                    type: "button",
                    style: "padding: 10px 16px; background: #111827; color: white; border: none; border-radius: 8px; font-size: 14px; font-weight: 500; cursor: pointer;",
                    onclick: move |_| {
                        if let Some(handler) = &props.on_write_review {
                            handler.call(());
                        }
                    },
                    "Write a review"
                }
            }
        }
    }
}
