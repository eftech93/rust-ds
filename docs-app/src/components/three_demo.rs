//! Dioxus Three - Web Demo
//!
//! Minimal version with cube selection workflow

use dioxus::prelude::*;
use dioxus_three::{ModelConfig, ModelFormat, ShaderPreset};
use dioxus_ui_system::prelude::*;
use dioxus_ui_system::atoms::{NumberInput};

/// The main ThreeView demo component - Cube selection workflow
#[component]
pub fn ThreeViewDemo() -> Element {
    // Scene state - start with one cube
    let mut models = use_signal(|| {
        vec![SceneModel {
            id: 0,
            name: "Cube 1".to_string(),
            config: ModelConfig::new("", ModelFormat::Cube).with_color("#ff6b6b"),
        }]
    });
    let next_id = use_signal(|| 1usize);
    let mut selected_model = use_signal(|| 0usize);

    // Camera state
    let mut cam_x = use_signal(|| 8.0f32);
    let mut cam_y = use_signal(|| 8.0f32);
    let mut cam_z = use_signal(|| 8.0f32);

    // Global options
    let mut auto_rotate = use_signal(|| true);
    let mut rot_speed = use_signal(|| 1.0f32);
    let mut show_grid = use_signal(|| true);
    let mut show_axes = use_signal(|| true);
    let mut shader = use_signal(|| ShaderPreset::None);

    rsx! {
        div {
            style: "display: flex; align-items: stretch; width: 100%; margin: 0; padding: 0; gap: 0;",
            
            // LEFT: Control Panel
            div {
                style: "width: 290px; flex-shrink: 0; border: 1px solid var(--border); border-radius: 4px; margin: 0; padding: 0 20px;",
                
                // Model Section
                div {
                    style: "border-bottom: 1px solid var(--border); margin: 0; padding: 8px 0;",
                    
                    div { style: "font-size: 11px; font-weight: 600; margin: 0 0 4px 0; padding: 0; color: var(--foreground, #111827);", "📦 Cubes" }
                    
                    // Add Cube button
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        full_width: true,
                        style: Some("padding: 4px 6px; font-size: 11px; margin: 0 0 8px 0;".to_string()),
                        onclick: move |_| {
                            let id = next_id();
                            let colors = vec!["#ff6b6b", "#4ecdc4", "#45b7d1", "#96ceb4", "#ffeaa7", "#dfe6e9", "#fd79a8", "#a29bfe"];
                            let color = colors[id % colors.len()];
                            let config = ModelConfig::new("", ModelFormat::Cube)
                                .with_color(color);
                            models.write().push(SceneModel {
                                id,
                                name: format!("Cube {}", id + 1),
                                config,
                            });
                            selected_model.set(models().len() - 1);
                        },
                        "➕ Add Cube"
                    }
                    
                    // Cube list
                    div {
                        style: "display: flex; flex-wrap: wrap; gap: 4px; margin: 0; padding: 0;",
                        
                        for (idx, model) in models.read().iter().enumerate() {
                            CubeListItem {
                                name: model.name.clone(),
                                color: model.config.color.clone(),
                                is_selected: idx == selected_model(),
                                on_click: move |_| selected_model.set(idx),
                            }
                        }
                    }
                }
                
                // Selected Model Controls
                SelectedModelControls { models, selected_idx: selected_model() }
                
                // Scene Options Section
                div {
                    style: "border-bottom: 1px solid var(--border); margin: 0; padding: 8px 0;",
                    
                    div { style: "font-size: 11px; font-weight: 600; margin: 0 0 4px 0; padding: 0; color: var(--foreground, #111827);", "🎬 Scene" }
                    
                    div {
                        style: "display: flex; margin: 0; padding: 0; gap: 8px;",
                        
                        div {
                            style: "display: flex; align-items: center; margin: 0; padding: 0;",
                            
                            Switch {
                                checked: auto_rotate(),
                                on_change: move |v| auto_rotate.set(v),
                            }
                            span { style: "font-size: 10px; margin: 0 0 0 4px; padding: 0; color: var(--foreground, #111827);", "Rotate" }
                        }
                        
                        div {
                            style: "display: flex; align-items: center; margin: 0; padding: 0;",
                            
                            Switch {
                                checked: show_grid(),
                                on_change: move |v| show_grid.set(v),
                            }
                            span { style: "font-size: 10px; margin: 0 0 0 4px; padding: 0; color: var(--foreground, #111827);", "Grid" }
                        }
                        
                        div {
                            style: "display: flex; align-items: center; margin: 0; padding: 0;",
                            
                            Switch {
                                checked: show_axes(),
                                on_change: move |v| show_axes.set(v),
                            }
                            span { style: "font-size: 10px; margin: 0 0 0 4px; padding: 0; color: var(--foreground, #111827);", "Axes" }
                        }
                    }
                    
                    if auto_rotate() {
                        div { style: "margin: 8px 0 0 0; padding: 0;",
                            Slider {
                                min: 0.0,
                                max: 5.0,
                                step: 0.1,
                                value: rot_speed() as f64,
                                on_change: EventHandler::new(move |v: f64| rot_speed.set(v as f32)),
                            }
                        }
                    }
                }
                
                // Camera Section
                div {
                    style: "border-bottom: 1px solid var(--border); margin: 0; padding: 8px 0;",
                    
                    div { style: "font-size: 11px; font-weight: 600; margin: 0 0 4px 0; padding: 0; color: var(--foreground, #111827);", "📷 Camera" }
                    
                    CameraSlider { label: "X", value: cam_x, min: -30.0, max: 30.0 }
                    CameraSlider { label: "Y", value: cam_y, min: -30.0, max: 30.0 }
                    CameraSlider { label: "Z", value: cam_z, min: -30.0, max: 30.0 }
                    
                    div {
                        style: "display: flex; margin: 8px 0 0 0; padding: 0; gap: 4px;",
                        
                        Button {
                            variant: ButtonVariant::Secondary,
                            size: ButtonSize::Sm,
                            style: Some("padding: 4px 8px; font-size: 11px; margin: 0; flex: 1;".to_string()),
                            onclick: move |_| { cam_x.set(0.0); cam_y.set(15.0); cam_z.set(0.01); },
                            "Top"
                        }
                        
                        Button {
                            variant: ButtonVariant::Secondary,
                            size: ButtonSize::Sm,
                            style: Some("padding: 4px 8px; font-size: 11px; margin: 0; flex: 1;".to_string()),
                            onclick: move |_| { cam_x.set(15.0); cam_y.set(0.0); cam_z.set(0.0); },
                            "Side"
                        }
                        
                        Button {
                            variant: ButtonVariant::Primary,
                            size: ButtonSize::Sm,
                            style: Some("padding: 4px 8px; font-size: 11px; margin: 0; flex: 1;".to_string()),
                            onclick: move |_| { cam_x.set(10.0); cam_y.set(10.0); cam_z.set(10.0); },
                            "Iso"
                        }
                    }
                }
                
                // Reset Button
                div {
                    style: "margin: 0; padding: 8px 0;",
                    Button {
                        variant: ButtonVariant::Destructive,
                        size: ButtonSize::Sm,
                        full_width: true,
                        style: Some("padding: 4px 6px; font-size: 11px; margin: 0;".to_string()),
                        onclick: move |_| {
                            models.set(vec![
                                SceneModel {
                                    id: 0,
                                    name: "Cube 1".to_string(),
                                    config: ModelConfig::new("", ModelFormat::Cube)
                                        .with_color("#ff6b6b"),
                                }
                            ]);
                            selected_model.set(0);
                            cam_x.set(8.0); cam_y.set(8.0); cam_z.set(8.0);
                            auto_rotate.set(true);
                            shader.set(ShaderPreset::None);
                        },
                        "🗑️ Reset Scene"
                    }
                }
            }
            
            // RIGHT: 3D Renderer
            div {
                style: "flex: 1; background: #000000; border-radius: 4px; margin: 0; padding: 0; min-height: 500px;",
                
                ThreeViewWrapper {
                    models: models,
                    cam_x: cam_x,
                    cam_y: cam_y,
                    cam_z: cam_z,
                    auto_rotate: auto_rotate,
                    rot_speed: rot_speed,
                    show_grid: show_grid,
                    show_axes: show_axes,
                    shader: shader,
                }
            }
        }
    }
}

/// Cube list item component
#[component]
fn CubeListItem(name: String, color: String, is_selected: bool, on_click: EventHandler<()>) -> Element {
    let border_color = if is_selected { "2px solid #3b82f6" } else { "2px solid transparent" };
    let bg_color = if is_selected { "rgba(59, 130, 246, 0.1)" } else { "rgba(0,0,0,0.05)" };
    
    rsx! {
        div {
            onclick: move |_| on_click.call(()),
            style: "display: flex; align-items: center; gap: 4px; padding: 4px 8px; border-radius: 4px; cursor: pointer; background: {bg_color}; border: {border_color}; font-size: 11px;",
            
            div {
                style: "width: 12px; height: 12px; border-radius: 2px; background: {color}; border: 1px solid rgba(0,0,0,0.2);"
            }
            span { style: "color: var(--foreground, #111827);", "{name}" }
        }
    }
}

/// Camera slider component
#[component]
fn CameraSlider(label: String, value: Signal<f32>, min: f32, max: f32) -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: center; margin: 0 0 4px 0; padding: 0; width: 100%;",
            span { style: "font-size: 10px; width: 14px; flex-shrink: 0; margin: 0; padding: 0; color: var(--foreground, #111827);", "{label}" }
            div {
                style: "flex: 1; margin: 0; padding: 0; min-width: 0;",
                Slider {
                    min: min as f64,
                    max: max as f64,
                    step: 0.1,
                    value: value() as f64,
                    on_change: EventHandler::new(move |v: f64| value.set(v as f32)),
                }
            }
            span { style: "font-size: 9px; width: 24px; text-align: right; flex-shrink: 0; margin: 0; padding: 0; color: var(--foreground, #111827);", "{value():.0}" }
        }
    }
}

/// Wrapper component that takes signals and subscribes to them
#[component]
fn ThreeViewWrapper(
    models: Signal<Vec<SceneModel>>,
    cam_x: Signal<f32>,
    cam_y: Signal<f32>,
    cam_z: Signal<f32>,
    auto_rotate: Signal<bool>,
    rot_speed: Signal<f32>,
    show_grid: Signal<bool>,
    show_axes: Signal<bool>,
    shader: Signal<ShaderPreset>,
) -> Element {
    let model_configs: Vec<ModelConfig> = models.read().iter().map(|m| m.config.clone()).collect();
    
    let cx = cam_x();
    let cy = cam_y();
    let cz = cam_z();
    let ar = auto_rotate();
    let rs = rot_speed();
    let sg = show_grid();
    let sa = show_axes();
    let _sh = shader();
    
    rsx! {
        dioxus_three::ThreeView {
            models: model_configs,
            cam_x: cx,
            cam_y: cy,
            cam_z: cz,
            auto_rotate: ar,
            rot_speed: rs,
            show_grid: sg,
            show_axes: sa,
            shader: _sh,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
struct SceneModel {
    id: usize,
    name: String,
    config: ModelConfig,
}

#[component]
fn SelectedModelControls(models: Signal<Vec<SceneModel>>, selected_idx: usize) -> Element {
    let model = models.read().get(selected_idx).cloned();

    let model_name = model.as_ref().map(|m| m.name.clone()).unwrap_or_default();
    let pos_x = model.as_ref().map(|m| m.config.pos_x).unwrap_or(0.0);
    let pos_y = model.as_ref().map(|m| m.config.pos_y).unwrap_or(0.0);
    let pos_z = model.as_ref().map(|m| m.config.pos_z).unwrap_or(0.0);
    let rot_x = model.as_ref().map(|m| m.config.rot_x).unwrap_or(0.0);
    let rot_y = model.as_ref().map(|m| m.config.rot_y).unwrap_or(0.0);
    let rot_z = model.as_ref().map(|m| m.config.rot_z).unwrap_or(0.0);
    let scale = model.as_ref().map(|m| m.config.scale).unwrap_or(1.0);
    let scale_formatted = format!("{:.2}", scale);
    let color = model
        .as_ref()
        .map(|m| m.config.color.clone())
        .unwrap_or_else(|| "#ff6b6b".to_string());

    rsx! {
        div {
            style: "border-bottom: 1px solid var(--border); margin: 0; padding: 8px 0;",
            
            div { style: "font-size: 11px; font-weight: 600; margin: 0 0 8px 0; padding: 0; color: var(--foreground, #111827);", "🔧 {model_name}" }
            
            // Position
            div {
                style: "margin: 0 0 8px 0; padding: 0;",
                div { style: "font-size: 10px; font-weight: 500; margin: 0 0 4px 0; color: var(--foreground, #111827);", "Position" }
                div {
                    style: "display: flex; gap: 8px;",
                    NumberInput {
                        value: pos_x as f64,
                        on_change: move |v| {
                            let mut m = models.write();
                            if let Some(model) = m.get_mut(selected_idx) {
                                model.config.pos_x = v as f32;
                            }
                        },
                        step: 0.1,
                        precision: Some(2),
                        show_buttons: false,
                        style: Some("width: 60px;".to_string()),
                    }
                    NumberInput {
                        value: pos_y as f64,
                        on_change: move |v| {
                            let mut m = models.write();
                            if let Some(model) = m.get_mut(selected_idx) {
                                model.config.pos_y = v as f32;
                            }
                        },
                        step: 0.1,
                        precision: Some(2),
                        show_buttons: false,
                        style: Some("width: 60px;".to_string()),
                    }
                    NumberInput {
                        value: pos_z as f64,
                        on_change: move |v| {
                            let mut m = models.write();
                            if let Some(model) = m.get_mut(selected_idx) {
                                model.config.pos_z = v as f32;
                            }
                        },
                        step: 0.1,
                        precision: Some(2),
                        show_buttons: false,
                        style: Some("width: 60px;".to_string()),
                    }
                }
            }
            
            // Rotation
            div {
                style: "margin: 0 0 8px 0; padding: 0;",
                div { style: "font-size: 10px; font-weight: 500; margin: 0 0 4px 0; color: var(--foreground, #111827);", "Rotation" }
                div {
                    style: "display: flex; gap: 8px;",
                    NumberInput {
                        value: rot_x as f64,
                        on_change: move |v| {
                            let mut m = models.write();
                            if let Some(model) = m.get_mut(selected_idx) {
                                model.config.rot_x = v as f32;
                            }
                        },
                        step: 1.0,
                        precision: Some(1),
                        show_buttons: false,
                        style: Some("width: 60px;".to_string()),
                    }
                    NumberInput {
                        value: rot_y as f64,
                        on_change: move |v| {
                            let mut m = models.write();
                            if let Some(model) = m.get_mut(selected_idx) {
                                model.config.rot_y = v as f32;
                            }
                        },
                        step: 1.0,
                        precision: Some(1),
                        show_buttons: false,
                        style: Some("width: 60px;".to_string()),
                    }
                    NumberInput {
                        value: rot_z as f64,
                        on_change: move |v| {
                            let mut m = models.write();
                            if let Some(model) = m.get_mut(selected_idx) {
                                model.config.rot_z = v as f32;
                            }
                        },
                        step: 1.0,
                        precision: Some(1),
                        show_buttons: false,
                        style: Some("width: 60px;".to_string()),
                    }
                }
            }
            
            // Scale
            div {
                style: "margin: 0 0 8px 0; padding: 0;",
                div { style: "font-size: 10px; font-weight: 500; margin: 0 0 4px 0; color: var(--foreground, #111827);", "Size (Scale)" }
                Slider {
                    min: 0.1,
                    max: 5.0,
                    step: 0.1,
                    value: scale as f64,
                    on_change: EventHandler::new(move |v: f64| {
                        let mut m = models.write();
                        if let Some(model) = m.get_mut(selected_idx) {
                            model.config.scale = v as f32;
                        }
                    }),
                }
                div { style: "font-size: 10px; margin: 4px 0 0 0; padding: 0; color: var(--foreground, #111827);", "{scale_formatted}"
                }
            }
            
            // Color
            div {
                style: "margin: 0; padding: 0;",
                div { style: "font-size: 10px; font-weight: 500; margin: 0 0 4px 0; color: var(--foreground, #111827);", "Color" }
                div {
                    style: "display: flex; align-items: center; margin: 0; padding: 0; gap: 8px;",
                    
                    input {
                        r#type: "color",
                        value: "{color}",
                        oninput: move |e| {
                            let c = e.value();
                            let mut m = models.write();
                            if let Some(model) = m.get_mut(selected_idx) {
                                model.config.color = c;
                            }
                        },
                        style: "width: 32px; height: 32px; border: none; border-radius: 4px; cursor: pointer; padding: 0; margin: 0;",
                    }
                    span { style: "font-size: 10px; margin: 0; padding: 0; color: var(--foreground, #111827);", "{color}" }
                }
            }
        }
    }
}

#[allow(dead_code)]
fn shader_to_string(shader: &ShaderPreset) -> String {
    match shader {
        ShaderPreset::Gradient => "gradient".to_string(),
        ShaderPreset::Water => "water".to_string(),
        ShaderPreset::Hologram => "hologram".to_string(),
        ShaderPreset::Toon => "toon".to_string(),
        ShaderPreset::Heatmap => "heatmap".to_string(),
        ShaderPreset::Custom(_) => "custom".to_string(),
        _ => "none".to_string(),
    }
}

#[allow(dead_code)]
fn string_to_shader(s: &str) -> ShaderPreset {
    match s {
        "gradient" => ShaderPreset::Gradient,
        "water" => ShaderPreset::Water,
        "hologram" => ShaderPreset::Hologram,
        "toon" => ShaderPreset::Toon,
        "heatmap" => ShaderPreset::Heatmap,
        _ => ShaderPreset::None,
    }
}
