//! New Organism component documentation pages (Phase 1-4)

use crate::components::three_demo::ThreeViewDemo;
use crate::docs_ui::{CodeBlock, DocPage, ExampleBox, PropsTable, Section};
use dioxus::prelude::*;
use dioxus_ui_system::organisms::{Calendar, CalendarMode, ImageUploader, Menubar};
use dioxus_ui_system::prelude::*;

/// ThreeView documentation page - dioxus-three 3D viewer component
#[component]
pub fn ThreeViewPage() -> Element {
    rsx! {
        DocPage {
            title: "ThreeView (dioxus-three)",
            description: "A Three.js 3D viewer component for Dioxus applications. Supports Desktop (WebView), Web (WASM), and Mobile platforms with multiple 3D formats and custom shaders.",

            ThreeViewInteractiveExample {}

            ThreeViewDocumentation {}
        }
    }
}

/// Interactive ThreeView Example - uses the working demo from dioxus-three
#[component]
fn ThreeViewInteractiveExample() -> Element {
    rsx! {
        Section { title: "Interactive Example",
            p { "This example demonstrates a ThreeView scene with interactive controls. Try adjusting the camera, shaders, and model properties:" }

            ThreeViewDemo {}
        }
    }
}

/// Main documentation content
#[component]
fn ThreeViewDocumentation() -> Element {
    // Using multiple # in raw strings to avoid conflicts with code containing "
    let basic_usage_code = r##"[dependencies]
dioxus-three = "0.0.3"
dioxus = { version = "0.6", features = ["desktop"] }"##;

    let web_usage_code = r##"[dependencies]
dioxus-three = "0.0.3"
dioxus = { version = "0.6", features = ["web"] }"##;

    let desktop_launch_code = r##"// Desktop (Windows/macOS/Linux)
dioxus_desktop::launch(app);"##;

    let web_launch_code = r##"// Web (WASM)
// Build with: dx build --platform web
// Or serve with: dx serve --platform web
dioxus_web::launch(app);"##;

    let mobile_launch_code = r##"// Mobile (iOS/Android)
// Build with: dx build --platform [android|ios]
dioxus_mobile::launch(app, vec![], Config::default());"##;

    let api_props_code = r##"ThreeViewProps {
    // Model (legacy single-model mode)
    model_url: Option<String>,
    format: ModelFormat,
    pos_x/y/z: f32,
    rot_x/y/z: f32,
    scale: f32,
    color: String,
    
    // Multi-model mode
    models: Vec<ModelConfig>,
    
    // Camera
    cam_x/y/z: f32,
    target_x/y/z: f32,
    
    // Display options
    auto_center: bool,
    auto_scale: bool,
    auto_rotate: bool,
    rot_speed: f32,
    show_grid: bool,
    show_axes: bool,
    background: String,
    shadows: bool,
    wireframe: bool,
    
    // Shader
    shader: ShaderPreset,
    
    // Styling
    class: String,
}"##;

    let model_config_code = r##"ModelConfig {
    url: String,
    format: ModelFormat,
    pos_x: f32,
    pos_y: f32,
    pos_z: f32,
    rot_x: f32,
    rot_y: f32,
    rot_z: f32,
    scale: f32,
    color: String,
}

// Builder methods:
ModelConfig::new(url, format)
    .with_position(x, y, z)
    .with_rotation(x, y, z)
    .with_scale(scale)
    .with_color(color)"##;

    let _cargo_dep_code = r##"[dependencies]
dioxus-three = "0.0.3""##;

    let cargo_git_code = r##"[dependencies]
dioxus-three = { git = "https://github.com/eftech93/dioxus-three" }"##;

    rsx! {
        DocPage {
            title: "ThreeView (dioxus-three)",
            description: "A Three.js 3D viewer component for Dioxus applications. Supports Desktop (WebView), Web (WASM), and Mobile platforms with multiple 3D formats and custom shaders.",

            Section { title: "Overview",
                p { "Dioxus Three is a multi-platform Three.js component that provides:" }
                ul {
                    li { "🖥️ Multi-Platform: Desktop (Windows/macOS/Linux), Web (WASM), Mobile (iOS/Android)" }
                    li { "📁 Multiple 3D model format support: OBJ, FBX, GLTF, GLB, STL, PLY, DAE" }
                    li { "📦 Multi-Model support: Load and display multiple models simultaneously" }
                    li { "🎨 Built-in default cube for quick testing" }
                    li { "📷 Camera controls with position and target settings" }
                    li { "🔄 Auto-rotation with adjustable speed" }
                    li { "📐 Grid and axes helpers" }
                    li { "🔲 Shadow support and wireframe mode" }
                    li { "🌊 Custom GLSL shader support with built-in presets" }
                }

            }

            Section { title: "Platform-Specific Usage",
                p { "Dioxus Three supports three different platforms. Here's how to launch your app on each:" }

                h4 { style: "margin-top: 16px;", "Desktop" }
                CodeBlock { code: desktop_launch_code.to_string() }

                h4 { style: "margin-top: 16px;", "Web (WASM)" }
                CodeBlock { code: web_launch_code.to_string() }

                h4 { style: "margin-top: 16px;", "Mobile" }
                CodeBlock { code: mobile_launch_code.to_string() }
            }

            Section { title: "Installation",
                p { "For Desktop applications, add to your Cargo.toml:" }
                CodeBlock { code: basic_usage_code.to_string() }

                p { style: "margin-top: 16px;", "For Web applications:" }
                CodeBlock { code: web_usage_code.to_string() }

                p { style: "margin-top: 16px;", "Or use the latest from git:" }
                CodeBlock { code: cargo_git_code.to_string() }
            }

            Section { title: "Supported Formats",
                p { "The following 3D formats are supported on all platforms:" }
                table { style: "width: 100%; border-collapse: collapse; margin-top: 16px;",
                    thead {
                        tr {
                            th { style: "text-align: left; padding: 8px; border-bottom: 2px solid #e2e8f0;", "Format" }
                            th { style: "text-align: left; padding: 8px; border-bottom: 2px solid #e2e8f0;", "Extension" }
                            th { style: "text-align: left; padding: 8px; border-bottom: 2px solid #e2e8f0;", "Description" }
                        }
                    }
                    tbody {
                        tr {
                            td { style: "padding: 8px; border-bottom: 1px solid #e2e8f0;", code { "ModelFormat::Obj" } }
                            td { style: "padding: 8px; border-bottom: 1px solid #e2e8f0;", ".obj" }
                            td { style: "padding: 8px; border-bottom: 1px solid #e2e8f0;", "Wavefront OBJ format" }
                        }
                        tr {
                            td { style: "padding: 8px; border-bottom: 1px solid #e2e8f0;", code { "ModelFormat::Fbx" } }
                            td { style: "padding: 8px; border-bottom: 1px solid #e2e8f0;", ".fbx" }
                            td { style: "padding: 8px; border-bottom: 1px solid #e2e8f0;", "Autodesk FBX format" }
                        }
                        tr {
                            td { style: "padding: 8px; border-bottom: 1px solid #e2e8f0;", code { "ModelFormat::Gltf" } }
                            td { style: "padding: 8px; border-bottom: 1px solid #e2e8f0;", ".gltf" }
                            td { style: "padding: 8px; border-bottom: 1px solid #e2e8f0;", "glTF 2.0 JSON format" }
                        }
                        tr {
                            td { style: "padding: 8px; border-bottom: 1px solid #e2e8f0;", code { "ModelFormat::Glb" } }
                            td { style: "padding: 8px; border-bottom: 1px solid #e2e8f0;", ".glb" }
                            td { style: "padding: 8px; border-bottom: 1px solid #e2e8f0;", "glTF 2.0 binary format" }
                        }
                        tr {
                            td { style: "padding: 8px; border-bottom: 1px solid #e2e8f0;", code { "ModelFormat::Stl" } }
                            td { style: "padding: 8px; border-bottom: 1px solid #e2e8f0;", ".stl" }
                            td { style: "padding: 8px; border-bottom: 1px solid #e2e8f0;", "STL stereolithography format" }
                        }
                        tr {
                            td { style: "padding: 8px; border-bottom: 1px solid #e2e8f0;", code { "ModelFormat::Ply" } }
                            td { style: "padding: 8px; border-bottom: 1px solid #e2e8f0;", ".ply" }
                            td { style: "padding: 8px; border-bottom: 1px solid #e2e8f0;", "Stanford PLY format" }
                        }
                        tr {
                            td { style: "padding: 8px; border-bottom: 1px solid #e2e8f0;", code { "ModelFormat::Dae" } }
                            td { style: "padding: 8px; border-bottom: 1px solid #e2e8f0;", ".dae" }
                            td { style: "padding: 8px; border-bottom: 1px solid #e2e8f0;", "Collada DAE format" }
                        }
                        tr {
                            td { style: "padding: 8px; border-bottom: 1px solid #e2e8f0;", code { "ModelFormat::Cube" } }
                            td { style: "padding: 8px; border-bottom: 1px solid #e2e8f0;", "N/A" }
                            td { style: "padding: 8px; border-bottom: 1px solid #e2e8f0;", "Default cube (no file needed)" }
                        }
                    }
                }
            }

            Section { title: "API Reference",
                p { "Key types and their fields:" }

                h4 { style: "margin-top: 16px;", "ThreeView Props" }
                CodeBlock { code: api_props_code.to_string() }

                h4 { style: "margin-top: 16px;", "ModelConfig" }
                CodeBlock { code: model_config_code.to_string() }

                h4 { style: "margin-top: 16px;", "Shader Presets" }
                p { "Available built-in shader effects:" }
                ul {
                    li { code { "ShaderPreset::None" } " - Standard PBR material (default)" }
                    li { code { "ShaderPreset::Gradient" } " - Animated RGB gradient effect" }
                    li { code { "ShaderPreset::Water" } " - Water/wave animation" }
                    li { code { "ShaderPreset::Hologram" } " - Hologram/cyberpunk effect" }
                    li { code { "ShaderPreset::Toon" } " - Toon/cel shading" }
                    li { code { "ShaderPreset::Heatmap" } " - Heat map visualization" }
                    li { code { "ShaderPreset::Custom(config)" } " - Custom GLSL shaders" }
                }
            }

            Section { title: "Running Examples",
                p { "The dioxus-three repository includes examples for all platforms:" }

                h4 { style: "margin-top: 16px;", "Desktop Demo" }
                CodeBlock { code: "cd examples/demo\ncargo run".to_string() }

                h4 { style: "margin-top: 16px;", "Web Demo (WASM)" }
                CodeBlock { code: "cd examples/web-demo\ndx serve --platform web".to_string() }

                h4 { style: "margin-top: 16px;", "Mobile Demo" }
                CodeBlock { code: "cd examples/mobile-demo\n# Android:\ndx build --platform android\n# iOS (macOS only):\ndx build --platform ios".to_string() }
            }
        }
    }
}

/// Calendar documentation page
#[component]
pub fn CalendarPage() -> Element {
    rsx! {
        DocPage {
            title: "Calendar",
            description: "A full calendar view for date selection and display.",

            Section { title: "Single Selection",
                ExampleBox {
                    Box { style: "max-width: 350px;",
                        Calendar {
                            mode: CalendarMode::Single,
                            value: Some("2024-03-15".to_string()),
                            on_change: move |_| {},
                        }
                    }
                }
                CodeBlock { code: "Calendar usage example".to_string() }
            }

            Section { title: "Range Selection",
                ExampleBox {
                    Box { style: "max-width: 350px;",
                        Calendar {
                            mode: CalendarMode::Range,
                            on_change: move |_| {},
                        }
                    }
                }
            }

            Section { title: "With Week Numbers",
                ExampleBox {
                    Box { style: "max-width: 380px;",
                        Calendar {
                            mode: CalendarMode::Single,
                            show_week_numbers: true,
                            on_change: move |_| {},
                        }
                    }
                }
            }

            Section { title: "Props",
                h4 { "CalendarProps" }
                PropsTable { props: vec![
                    ("value", "Option", "Selected date (YYYY-MM-DD format) for Single mode For Multiple mode, this is not used - use on_change to track multiple dates For Range mode, this represents the start date"),
                    ("on_change", "EventHandler<Option>", "Change handler - called when selection changes For Single mode: returns the selected date or None if cleared For Range mode: returns the start date when selecting, or end date format when complete"),
                    ("month", "Option", "Currently displayed month (YYYY-MM format) - for controlled mode"),
                    ("on_month_change", "EventHandler", "Callback when the displayed month changes"),
                    ("disabled_dates", "Vec", "List of disabled dates (YYYY-MM-DD format)"),
                    ("min_date", "Option", "Minimum selectable date (YYYY-MM-DD format)"),
                    ("max_date", "Option", "Maximum selectable date (YYYY-MM-DD format)"),
                    ("show_week_numbers", "bool", "Show week numbers on the left side"),
                    ("mode", "CalendarMode", "Selection mode"),
                    ("style", "Option", "Custom inline styles"),
                    ("id", "Option", "Element id"),
                    ("class", "Option", "Custom CSS classes")
                ] }
            }
        }
    }
}

/// DateRangePicker documentation page
#[component]
pub fn DateRangePickerPage() -> Element {
    let mut start_date = use_signal(|| None::<String>);
    let mut end_date = use_signal(|| None::<String>);

    rsx! {
        DocPage {
            title: "Date Range Picker",
            description: "Select a range of dates with preset options.",

            Section { title: "Basic",
                ExampleBox {
                    Box { style: "max-width: 320px;",
                        DateRangePicker {
                            start_date: start_date(),
                            end_date: end_date(),
                            on_start_date_change: move |v| start_date.set(v),
                            on_end_date_change: move |v| end_date.set(v),
                            on_range_change: move |(s, e)| {
                                start_date.set(s);
                                end_date.set(e);
                            },
                        }
                    }
                }
                CodeBlock { code: r#"let mut start = use_signal(|| None::<String>);
let mut end = use_signal(|| None::<String>);

DateRangePicker {
    start_date: start(),
    end_date: end(),
    on_start_date_change: move |v| start.set(v),
    on_end_date_change: move |v| end.set(v),
    on_range_change: move |(s, e)| {
        start.set(s);
        end.set(e);
    },
}"#.to_string() }
            }

            Section { title: "With Presets",
                ExampleBox {
                    Box { style: "max-width: 320px;",
                        DateRangePicker {
                            start_date: start_date(),
                            end_date: end_date(),
                            on_start_date_change: move |v| start_date.set(v),
                            on_end_date_change: move |v| end_date.set(v),
                            presets: default_presets(),
                        }
                    }
                }
                CodeBlock { code: r#"DateRangePicker {
    start_date: start(),
    end_date: end(),
    presets: default_presets(),
    // ...
}"#.to_string() }
            }

            Section { title: "Props",
                h4 { "DateRangePickerProps" }
                PropsTable { props: vec![
                    ("start_date", "Option", "Selected start date (YYYY-MM-DD format)"),
                    ("end_date", "Option", "Selected end date (YYYY-MM-DD format)"),
                    ("on_start_date_change", "Option<EventHandler<Option>>", "Start date change handler"),
                    ("on_end_date_change", "Option<EventHandler<Option>>", "End date change handler"),
                    ("on_range_change", "Option<EventHandler<(Option, Option)>>", "Range change handler (called when both dates are selected)"),
                    ("min_date", "Option", "Minimum selectable date (YYYY-MM-DD)"),
                    ("max_date", "Option", "Maximum selectable date (YYYY-MM-DD)"),
                    ("placeholder", "String", "Placeholder text"),
                    ("disabled", "bool", "Whether the picker is disabled"),
                    ("presets", "Vec", "Preset date ranges"),
                    ("class", "Option", "Additional CSS classes"),
                    ("id", "Option", "Element id"),
                    ("style", "Option", "Custom inline styles")
                ] }
                h4 { "DateRangePickerDropdownProps" }
                PropsTable { props: vec![
                    ("start_date", "Option", ""),
                    ("end_date", "Option", ""),
                    ("min_date", "Option", ""),
                    ("max_date", "Option", ""),
                    ("presets", "Vec", ""),
                    ("selecting_start", "bool", ""),
                    ("on_select", "EventHandler", ""),
                    ("on_preset_select", "EventHandler", ""),
                    ("on_close", "EventHandler", ""),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "DateRangePresetsProps" }
                PropsTable { props: vec![
                    ("presets", "Vec", ""),
                    ("on_preset_select", "EventHandler", ""),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "PresetButtonProps" }
                PropsTable { props: vec![
                    ("preset", "DateRangePreset", ""),
                    ("on_preset_select", "EventHandler", ""),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
            }
        }
    }
}

/// Carousel documentation page
#[component]
pub fn CarouselPage() -> Element {
    rsx! {
        DocPage {
            title: "Carousel",
            description: "Image and content slider with navigation controls.",

            Section { title: "Image Carousel",
                ExampleBox {
                    Box { style: "max-width: 600px;",
                        Carousel {
                            style: "height: 200px;".to_string(),
                            CarouselContent {
                                CarouselItem { index: 0,
                                    Box { style: "height: 100%; background: #3b82f6; border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-size: 24px;",
                                        "Slide 1"
                                    }
                                }
                                CarouselItem { index: 1,
                                    Box { style: "height: 100%; background: #ef4444; border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-size: 24px;",
                                        "Slide 2"
                                    }
                                }
                                CarouselItem { index: 2,
                                    Box { style: "height: 100%; background: #22c55e; border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-size: 24px;",
                                        "Slide 3"
                                    }
                                }
                                CarouselItem { index: 3,
                                    Box { style: "height: 100%; background: #f59e0b; border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-size: 24px;",
                                        "Slide 4"
                                    }
                                }
                            }
                            CarouselPrevious {}
                            CarouselNext {}
                            CarouselDots {}
                        }
                    }
                }
                CodeBlock { code: r#"Carousel {
    style: "height: 200px;".to_string(),
    CarouselContent {
        CarouselItem { index: 0,
            Box { style: "height: 100%; background: #3b82f6;", "Slide 1" }
        }
        CarouselItem { index: 1,
            Box { style: "height: 100%; background: #ef4444;", "Slide 2" }
        }
        CarouselItem { index: 2,
            Box { style: "height: 100%; background: #22c55e;", "Slide 3" }
        }
    }
    CarouselPrevious {}
    CarouselNext {}
    CarouselDots {}
}"#.to_string() }
            }

            Section { title: "With Dots",
                p { "Carousel with dot indicators" }
                ExampleBox {
                    Box { style: "max-width: 600px;",
                        Carousel {
                            style: "height: 160px;".to_string(),
                            CarouselContent {
                                CarouselItem { index: 0,
                                    Box { style: "height: 100%; background: #8b5cf6; border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-size: 20px;",
                                        "Dots Example 1"
                                    }
                                }
                                CarouselItem { index: 1,
                                    Box { style: "height: 100%; background: #ec4899; border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-size: 20px;",
                                        "Dots Example 2"
                                    }
                                }
                            }
                            CarouselDots {}
                        }
                    }
                }
            }

            Section { title: "Props",
                h4 { "CarouselProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Child elements (should include CarouselContent and controls)"),
                    ("opts", "CarouselOptions", "Carousel configuration options"),
                    ("orientation", "Orientation", "Orientation of the carousel"),
                    ("on_index_change", "Option<EventHandler>", "Callback when active index changes"),
                    ("style", "Option", "Custom inline styles"),
                    ("id", "Option", "Element id"),
                    ("class", "Option", "Custom CSS classes")
                ] }
                h4 { "CarouselContentProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Carousel items (should be CarouselItem components)"),
                    ("style", "Option", "Custom inline styles"),
                    ("id", "Option", "Element id"),
                    ("class", "Option", "Custom CSS classes")
                ] }
                h4 { "CarouselItemProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Item content"),
                    ("index", "usize", "Item index (used for tracking position)"),
                    ("style", "Option", "Custom inline styles"),
                    ("id", "Option", "Element id"),
                    ("class", "Option", "Custom CSS classes")
                ] }
                h4 { "CarouselPreviousProps" }
                PropsTable { props: vec![
                    ("style", "Option", "Custom inline styles"),
                    ("class", "Option", "Custom class"),
                    ("id", "Option", "Element id")
                ] }
                h4 { "CarouselNextProps" }
                PropsTable { props: vec![
                    ("style", "Option", "Custom inline styles"),
                    ("class", "Option", "Custom class"),
                    ("id", "Option", "Element id")
                ] }
                h4 { "CarouselDotsProps" }
                PropsTable { props: vec![
                    ("style", "Option", "Custom inline styles for container"),
                    ("dot_style", "Option", "Custom inline styles for individual dots"),
                    ("count", "Option", "Number of dots to show (if None, uses carousel item count)"),
                    ("id", "Option", "Element id"),
                    ("class", "Option", "Custom CSS classes")
                ] }
                h4 { "SimpleCarouselProps" }
                PropsTable { props: vec![
                    ("items", "Vec", "Slides content"),
                    ("opts", "CarouselOptions", "Carousel options"),
                    ("orientation", "Orientation", "Orientation"),
                    ("on_index_change", "Option<EventHandler>", "Callback when index changes"),
                    ("show_arrows", "bool", "Show navigation arrows"),
                    ("show_dots", "bool", "Show pagination dots"),
                    ("style", "Option", "Custom inline styles"),
                    ("id", "Option", "Element id"),
                    ("class", "Option", "Custom CSS classes")
                ] }
                h4 { "TouchCarouselProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Child elements"),
                    ("opts", "CarouselOptions", "Carousel options"),
                    ("orientation", "Orientation", "Orientation"),
                    ("on_index_change", "Option<EventHandler>", "Callback when index changes"),
                    ("style", "Option", "Custom inline styles"),
                    ("id", "Option", "Element id"),
                    ("class", "Option", "Custom CSS classes")
                ] }
            }
        }
    }
}

/// Tree documentation page
#[component]
pub fn TreePage() -> Element {
    let mut selected = use_signal(|| None::<String>);
    let mut expanded = use_signal(|| vec!["documents".to_string(), "images".to_string()]);

    let data = vec![
        TreeNodeData::new("documents", "Documents")
            .with_icon("folder")
            .with_children(vec![
                TreeNodeData::new("work", "Work")
                    .with_icon("folder")
                    .with_children(vec![
                        TreeNodeData::new("report", "report.pdf").with_icon("file"),
                        TreeNodeData::new("budget", "budget.xlsx").with_icon("file"),
                    ]),
                TreeNodeData::new("personal", "Personal")
                    .with_icon("folder")
                    .with_children(vec![
                        TreeNodeData::new("resume", "resume.docx").with_icon("file")
                    ]),
            ]),
        TreeNodeData::new("images", "Images")
            .with_icon("folder")
            .with_children(vec![
                TreeNodeData::new("vacation", "vacation.png").with_icon("image"),
                TreeNodeData::new("logo", "logo.svg").with_icon("image"),
            ]),
        TreeNodeData::new("music", "Music")
            .with_icon("folder")
            .with_children(vec![
                TreeNodeData::new("song", "song.mp3").with_icon("music")
            ]),
    ];

    rsx! {
        DocPage {
            title: "Tree",
            description: "Hierarchical tree view for displaying nested data.",

            Section { title: "Basic Tree",
                ExampleBox {
                    Box { style: "max-width: 300px; border: 1px solid #e2e8f0; border-radius: 8px; padding: 8px;",
                        Tree {
                            data: data.clone(),
                            selected_id: selected(),
                            on_select: move |id| selected.set(Some(id)),
                            expanded_ids: expanded(),
                            on_toggle_expand: move |id| {
                                expanded.with_mut(|exp| {
                                    if exp.contains(&id) {
                                        exp.retain(|x| x != &id);
                                    } else {
                                        exp.push(id);
                                    }
                                });
                            },
                            show_lines: true,
                        }
                    }
                }
                CodeBlock { code: r#"let mut selected = use_signal(|| None::<String>);
let mut expanded = use_signal(|| vec!["documents".to_string()]);

let data = vec![
    TreeNodeData::new("documents", "Documents")
        .with_icon("folder")
        .with_children(vec![
            TreeNodeData::new("work", "Work")
                .with_icon("folder")
                .with_child(TreeNodeData::new("report", "report.pdf").with_icon("file")),
        ]),
];

Tree {
    data: data,
    selected_id: selected(),
    on_select: move |id| selected.set(Some(id)),
    expanded_ids: expanded(),
    on_toggle_expand: move |id| { /* toggle logic */ },
    show_lines: true,
}"#.to_string() }
            }

            Section { title: "Props",
                h4 { "TreeProps" }
                PropsTable { props: vec![
                    ("data", "Vec", "Tree node data"),
                    ("selected_id", "Option", "Currently selected node ID"),
                    ("on_select", "Option<EventHandler>", "Callback when a node is selected"),
                    ("expanded_ids", "Vec", "List of expanded node IDs"),
                    ("on_toggle_expand", "Option<EventHandler>", "Callback when a node's expand/collapse state changes"),
                    ("show_lines", "bool", "Whether to show connector lines between nodes"),
                    ("style", "Option", "Custom inline styles"),
                    ("id", "Option", ""),
                    ("class", "Option", "")
                ] }
                h4 { "TreeNodeBuilderProps" }
                PropsTable { props: vec![
                    ("initial_data", "Vec", "Initial node data"),
                    ("on_change", "Option<EventHandler<Vec>>", "Callback when tree data changes"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
            }
        }
    }
}

/// Timeline documentation page
#[component]
pub fn TimelinePage() -> Element {
    rsx! {
        DocPage {
            title: "Timeline",
            description: "Vertical timeline for displaying chronological events.",

            Section { title: "Basic Timeline",
                ExampleBox {
                    Box { style: "max-width: 500px;",
                        Timeline {
                            TimelineItem {
                                icon: Some("rocket".to_string()),
                                TimelineContent {
                                    title: Some("Project Started".to_string()),
                                    subtitle: Some("Jan 15, 2024".to_string()),
                                    "Initial setup and planning phase completed."
                                }
                            }
                            TimelineItem {
                                icon: Some("check".to_string()),
                                dot_color: Some(Color::new(34, 197, 94)),
                                TimelineContent {
                                    title: Some("MVP Released".to_string()),
                                    subtitle: Some("Mar 10, 2024".to_string()),
                                    "First version shipped to early adopters."
                                }
                            }
                            TimelineItem {
                                icon: Some("users".to_string()),
                                dot_color: Some(Color::new(59, 130, 246)),
                                TimelineContent {
                                    title: Some("1000 Users".to_string()),
                                    subtitle: Some("Jun 22, 2024".to_string()),
                                    "Reached one thousand active users milestone."
                                }
                            }
                            TimelineItem {
                                last: true,
                                icon: Some("star".to_string()),
                                dot_color: Some(Color::new(245, 158, 11)),
                                TimelineContent {
                                    title: Some("v2.0 Launch".to_string()),
                                    subtitle: Some("Dec 1, 2024".to_string()),
                                    "Major update with new features and redesign."
                                }
                            }
                        }
                    }
                }
                CodeBlock { code: r#"Timeline {
    TimelineItem {
        icon: Some("rocket".to_string()),
        TimelineContent {
            title: Some("Project Started".to_string()),
            subtitle: Some("Jan 15, 2024".to_string()),
            "Initial setup..."
        }
    }
    TimelineItem {
        icon: Some("check".to_string()),
        dot_color: Some(Color::new(34, 197, 94)),
        TimelineContent {
            title: Some("MVP Released".to_string()),
            "First version shipped."
        }
    }
    TimelineItem {
        last: true,
        icon: Some("star".to_string()),
        dot_color: Some(Color::new(245, 158, 11)),
        TimelineContent {
            title: Some("v2.0 Launch".to_string()),
            "Major update."
        }
    }
}"#.to_string() }
            }

            Section { title: "Props",
                h4 { "TimelineProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Timeline items as children"),
                    ("position", "TimelinePosition", "Position of timeline content"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "TimelineItemProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Content for this timeline item"),
                    ("dot", "Option", "Custom dot element (replaces default dot)"),
                    ("dot_color", "Option", "Custom dot color"),
                    ("icon", "Option", "Icon name to display inside dot"),
                    ("last", "bool", "Whether this is the last item (no connector line after)"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "TimelineSeparatorProps" }
                PropsTable { props: vec![
                    ("dot", "Option", "Custom dot element"),
                    ("dot_color", "Option", "Custom dot color"),
                    ("icon", "Option", "Icon name for dot"),
                    ("last", "bool", "Whether this is the last item"),
                    ("line_color", "Option", "Line color"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "TimelineDotProps" }
                PropsTable { props: vec![
                    ("dot", "Option", "Custom dot element (replaces default)"),
                    ("color", "Option", "Dot background color"),
                    ("icon", "Option", "Icon name to display inside dot"),
                    ("size", "TimelineDotSize", "Dot size"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "TimelineContentProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Content children"),
                    ("title", "Option", "Optional title"),
                    ("subtitle", "Option", "Optional subtitle/timestamp"),
                    ("align", "Option", "Content alignment"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "TimelineOppositeContentProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Content children"),
                    ("align", "Option", "Text alignment"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "SimpleTimelineProps" }
                PropsTable { props: vec![
                    ("events", "Vec", "List of timeline events"),
                    ("position", "TimelinePosition", "Timeline position"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
            }
        }
    }
}

/// Menubar documentation page
#[component]
pub fn MenubarPage() -> Element {
    rsx! {
        DocPage {
            title: "Menubar",
            description: "Application menu bar with nested dropdown menus.",

            Section { title: "Basic Menubar",
                ExampleBox {
                    Menubar {
                        MenubarMenu {
                            MenubarTrigger { label: "File".to_string() }
                            MenubarContent {
                                MenubarItem { on_click: move |_| {}, "New" }
                                MenubarItem { on_click: move |_| {}, "Open" }
                                MenubarSeparator {}
                                MenubarSub {
                                    MenubarSubTrigger { "Recent Files" }
                                    MenubarSubContent {
                                        MenubarItem { on_click: move |_| {}, "document.txt" }
                                        MenubarItem { on_click: move |_| {}, "notes.md" }
                                        MenubarItem { on_click: move |_| {}, "project.rs" }
                                    }
                                }
                                MenubarSeparator {}
                                MenubarItem { on_click: move |_| {}, "Exit" }
                            }
                        }
                        MenubarMenu {
                            MenubarTrigger { label: "Edit".to_string() }
                            MenubarContent {
                                MenubarItem { on_click: move |_| {}, shortcut: Some("Ctrl+X".to_string()), "Cut" }
                                MenubarItem { on_click: move |_| {}, shortcut: Some("Ctrl+C".to_string()), "Copy" }
                                MenubarItem { on_click: move |_| {}, shortcut: Some("Ctrl+V".to_string()), "Paste" }
                                MenubarSeparator {}
                                MenubarItem { on_click: move |_| {}, "Preferences" }
                            }
                        }
                        MenubarMenu {
                            MenubarTrigger { label: "View".to_string() }
                            MenubarContent {
                                MenubarItem { on_click: move |_| {}, "Zoom In" }
                                MenubarItem { on_click: move |_| {}, "Zoom Out" }
                                MenubarSeparator {}
                                MenubarSub {
                                    MenubarSubTrigger { "Appearance" }
                                    MenubarSubContent {
                                        MenubarItem { on_click: move |_| {}, "Light Mode" }
                                        MenubarItem { on_click: move |_| {}, "Dark Mode" }
                                    }
                                }
                            }
                        }
                    }
                }
                CodeBlock { code: r#"Menubar {
    MenubarMenu {
        MenubarTrigger { label: "File".to_string() }
        MenubarContent {
            MenubarItem { on_click: move |_| {}, "New" }
            MenubarSub {
                MenubarSubTrigger { "Recent Files" }
                MenubarSubContent {
                    MenubarItem { on_click: move |_| {}, "document.txt" }
                }
            }
        }
    }
    MenubarMenu {
        MenubarTrigger { label: "Edit".to_string() }
        MenubarContent {
            MenubarItem { on_click: move |_| {}, "Cut", shortcut: Some("Ctrl+X".to_string()) }
        }
    }
}"#.to_string() }
            }

            Section { title: "Props",
                h4 { "MenubarProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Child elements (MenubarMenu components)"),
                    ("loop_navigation", "bool", "Whether to loop navigation (cycle through menus), default: true"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "MenubarMenuProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Child elements (MenubarTrigger and MenubarContent)"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "MenubarTriggerProps" }
                PropsTable { props: vec![
                    ("label", "String", "The label text for this menu"),
                    ("disabled", "bool", "Whether this trigger is disabled"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "MenubarContentProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Menu items and content"),
                    ("align", "MenubarAlign", "Optional alignment"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "MenubarItemProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Item content"),
                    ("on_click", "EventHandler", "Callback when item is clicked"),
                    ("disabled", "bool", "Whether the item is disabled"),
                    ("shortcut", "Option", "Optional keyboard shortcut display"),
                    ("icon", "Option", "Optional icon element"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "MenubarSubProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Child elements (MenubarSubTrigger and MenubarSubContent)"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "MenubarSubTriggerProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Trigger content"),
                    ("disabled", "bool", "Whether the item is disabled"),
                    ("shortcut", "Option", "Optional keyboard shortcut display"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "MenubarSubContentProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Submenu content"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "MenubarLabelProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Label text content"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "MenubarCheckboxItemProps" }
                PropsTable { props: vec![
                    ("children", "Element", "Item label"),
                    ("checked", "bool", "Whether the checkbox is checked"),
                    ("on_checked_change", "EventHandler", "Callback when checked state changes"),
                    ("disabled", "bool", "Whether the item is disabled"),
                    ("shortcut", "Option", "Optional keyboard shortcut display"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
            }
        }
    }
}

/// Resizable panels documentation page
#[component]
pub fn ResizablePage() -> Element {
    rsx! {
        DocPage {
            title: "Resizable Panels",
            description: "Split-pane layout with draggable resize handles.",

            Section { title: "Horizontal Split",
                ExampleBox {
                    Box { style: "height: 300px; border: 1px solid #e2e8f0; border-radius: 8px;",
                        ResizablePanelGroup {
                            direction: Direction::Horizontal,
                            ResizablePanel { default_size: Some(50.0),
                                Box { style: "padding: 16px; background: #f8fafc; height: 100%;",
                                    h4 { "Panel A" }
                                    p { "This panel takes up 50% of the space by default." }
                                }
                            }
                            ResizableHandle {}
                            ResizablePanel { default_size: Some(50.0),
                                Box { style: "padding: 16px; background: #f1f5f9; height: 100%;",
                                    h4 { "Panel B" }
                                    p { "Drag the handle between panels to resize." }
                                }
                            }
                        }
                    }
                }
                CodeBlock { code: r#"ResizablePanelGroup {
    direction: Direction::Horizontal,
    ResizablePanel { default_size: Some(50.0),
        Box { "Panel A" }
    }
    ResizableHandle {}
    ResizablePanel { default_size: Some(50.0),
        Box { "Panel B" }
    }
}"#.to_string() }
            }

            Section { title: "Props",
                h4 { "ResizablePanelGroupProps" }
                PropsTable { props: vec![
                    ("direction", "Direction", "Direction of the panel group"),
                    ("children", "Element", "Child elements (ResizablePanel and ResizableHandle components)"),
                    ("class", "Option", "Optional CSS class"),
                    ("id", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "ResizablePanelProps" }
                PropsTable { props: vec![
                    ("default_size", "Option", "Default size as percentage (0-100)"),
                    ("min_size", "Option", "Minimum size as percentage"),
                    ("max_size", "Option", "Maximum size as percentage"),
                    ("children", "Element", "Panel content"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "ResizableHandleProps" }
                PropsTable { props: vec![
                    ("disabled", "bool", "Whether the handle is disabled"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
            }
        }
    }
}

/// Kanban board documentation page
#[component]
pub fn KanbanPage() -> Element {
    let columns = vec![
        KanbanColumn::new("todo", "To Do")
            .with_color("#ef4444")
            .with_cards(vec![
                KanbanCard::new("1", "Design system research")
                    .with_description("Research existing design systems and patterns")
                    .with_tags(vec!["research".to_string(), "design".to_string()])
                    .with_assignee("JD")
                    .with_due_date("Apr 20"),
                KanbanCard::new("2", "Setup project repo")
                    .with_description("Initialize Git repository and CI/CD")
                    .with_tags(vec!["devops".to_string()])
                    .with_assignee("AL"),
            ]),
        KanbanColumn::new("in_progress", "In Progress")
            .with_color("#f59e0b")
            .with_cards(vec![KanbanCard::new("3", "Build component library")
                .with_description("Create base atom components")
                .with_tags(vec!["frontend".to_string()])
                .with_assignee("JD")
                .with_due_date("Apr 25")]),
        KanbanColumn::new("done", "Done")
            .with_color("#22c55e")
            .with_cards(vec![KanbanCard::new("4", "Project kickoff")
                .with_description("Initial team meeting and planning")
                .with_tags(vec!["planning".to_string()])
                .with_assignee("AL")
                .with_due_date("Apr 10")]),
    ];

    rsx! {
        DocPage {
            title: "Kanban Board",
            description: "Drag-and-drop task board with customizable columns.",

            Section { title: "Basic Kanban",
                ExampleBox {
                    Box { style: "height: 400px; overflow-x: auto;",
                        Kanban {
                            columns: columns.clone(),
                            on_card_click: move |id| println!("Clicked card: {}", id),
                        }
                    }
                }
                CodeBlock { code: r##"let columns = vec![
    KanbanColumn::new("todo", "To Do")
        .with_color("#ef4444")
        .with_cards(vec![
            KanbanCard::new("1", "Task 1")
                .with_description("Description")
                .with_tags(vec!["tag".to_string()])
                .with_assignee("JD")
                .with_due_date("Apr 20"),
        ]),
    KanbanColumn::new("in_progress", "In Progress")
        .with_color("#f59e0b"),
    KanbanColumn::new("done", "Done")
        .with_color("#22c55e"),
];

Kanban {
    columns: columns,
    on_card_click: move |id| println!("Clicked: {}", id),
}"##.to_string() }
            }

            Section { title: "Props",
                h4 { "KanbanProps" }
                PropsTable { props: vec![
                    ("columns", "Vec", "Columns with their cards"),
                    ("on_columns_change", "Option<EventHandler<Vec>>", "Handler for column changes (new column order or content)"),
                    ("on_card_move", "Option<EventHandler>", "Handler for card moves (card_id, column_id, new_index)"),
                    ("on_card_click", "Option<EventHandler>", "Handler for card clicks"),
                    ("allow_add_column", "bool", "Allow adding new columns"),
                    ("on_add_column", "Option<EventHandler>", "Handler for add column button"),
                    ("on_add_card", "Option<EventHandler>", "Handler for add card in a column (column_id)"),
                    ("style", "Option", "Custom inline styles"),
                    ("id", "Option", "Element id"),
                    ("class", "Option", "Custom CSS classes"),
                    ("column_width", "String", "Column width (default: 280px)"),
                    ("height", "String", "Board height (default: 100%)"),
                    ("show_card_count", "bool", "Show card count in column headers"),
                    ("card_hover", "bool", "Enable card hover effects")
                ] }
                h4 { "KanbanColumnViewProps" }
                PropsTable { props: vec![
                    ("column", "KanbanColumn", ""),
                    ("width", "String", ""),
                    ("on_card_click", "Option<EventHandler>", ""),
                    ("on_add_card", "Option<EventHandler>", ""),
                    ("show_card_count", "bool", ""),
                    ("card_hover", "bool", ""),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "KanbanCardViewProps" }
                PropsTable { props: vec![
                    ("card", "KanbanCard", ""),
                    ("on_click", "Option<EventHandler>", ""),
                    ("hover", "bool", ""),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "KanbanTagProps" }
                PropsTable { props: vec![
                    ("label", "String", ""),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "AddColumnButtonProps" }
                PropsTable { props: vec![
                    ("on_add_column", "Option<EventHandler>", ""),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "SimpleKanbanProps" }
                PropsTable { props: vec![
                    ("columns", "Vec", ""),
                    ("on_card_click", "Option<EventHandler>", ""),
                    ("height", "String", ""),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
            }
        }
    }
}

/// ImageUploader documentation page
#[component]
pub fn ImageUploaderPage() -> Element {
    rsx! {
        DocPage {
            title: "Image Uploader",
            description: "Image upload component with drag-and-drop and preview.",

            Section { title: "Basic Upload",
                ExampleBox {
                    ImageUploader {
                        value: vec![],
                        on_change: move |_| {},
                        max_files: Some(5),
                        max_size_mb: 5.0,
                        accept: vec!["image/*".to_string()],
                    }
                }
                CodeBlock { code: "ImageUploader usage".to_string() }
            }

            Section { title: "With Preview",
                ExampleBox {
                    ImageUploader {
                        value: vec![],
                        on_change: move |_| {},
                        show_preview: true,
                    }
                }
            }

            Section { title: "Single File",
                ExampleBox {
                    ImageUploader {
                        value: vec![],
                        on_change: move |_| {},
                        multiple: false,
                    }
                }
            }

            Section { title: "Props",
                h4 { "ImageUploaderProps" }
                PropsTable { props: vec![
                    ("value", "Vec", "Current value (list of uploaded images)"),
                    ("on_change", "EventHandler<Vec>", "Callback when the image list changes"),
                    ("max_files", "Option", "Maximum number of files allowed"),
                    ("max_size_mb", "f32", "Maximum file size in MB (default: 5.0)"),
                    ("accept", "Vec", "Accepted MIME types (default: [\"image/*\"])"),
                    ("disabled", "bool", "Disabled state"),
                    ("multiple", "bool", "Allow multiple file selection"),
                    ("show_preview", "bool", "Show image preview thumbnails"),
                    ("upload_url", "Option", "Optional upload URL for auto-upload"),
                    ("label", "Option", "Optional label text"),
                    ("helper_text", "Option", "Optional helper text"),
                    ("error", "Option", "Optional error message"),
                    ("class", "Option", "Custom CSS class"),
                    ("id", "Option", "Element id"),
                    ("style", "Option", "Custom inline styles"),
                    ("placeholder", "String", "Placeholder text for dropzone")
                ] }
                h4 { "ImagePreviewItemProps" }
                PropsTable { props: vec![
                    ("image", "UploadedImage", "The uploaded image"),
                    ("index", "usize", "Index in the list"),
                    ("on_remove", "EventHandler", "Remove callback"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "ImageListItemProps" }
                PropsTable { props: vec![
                    ("image", "UploadedImage", "The uploaded image"),
                    ("on_remove", "EventHandler", "Remove callback"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
            }
        }
    }
}

/// RichText documentation page
#[component]
pub fn RichTextPage() -> Element {
    let mut content = use_signal(|| "<p>Hello <b>world</b>!</p>".to_string());

    rsx! {
        DocPage {
            title: "Rich Text Editor",
            description: "A WYSIWYG text editor with formatting toolbar supporting bold, italic, headings, lists, and more.",

            Section { title: "Simple Rich Text",
                ExampleBox {
                    Box { style: "max-width: 600px;",
                        SimpleRichText {
                            value: content(),
                            placeholder: Some("Start typing...".to_string()),
                            on_change: Some(EventHandler::new(move |v: String| content.set(v))),
                        }
                    }
                }
                CodeBlock { code: r#"SimpleRichText {
    value: content(),
    placeholder: Some("Start typing...".to_string()),
    on_change: Some(EventHandler::new(move |v| content.set(v))),
}"#.to_string() }
            }

            Section { title: "Full Featured",
                ExampleBox {
                    Box { style: "max-width: 600px;",
                        FullRichText {
                            value: "<h2>Title</h2><p>This is a paragraph with <b>bold</b> and <i>italic</i> text.</p>".to_string(),
                            placeholder: Some("Enter rich content...".to_string()),
                            on_change: Some(EventHandler::new(move |_| {})),
                        }
                    }
                }
            }

            Section { title: "Disabled",
                ExampleBox {
                    Box { style: "max-width: 600px;",
                        SimpleRichText {
                            value: "<p>This editor is disabled.</p>".to_string(),
                            disabled: true,
                        }
                    }
                }
            }

            Section { title: "Props",
                h4 { "RichTextEditorProps" }
                PropsTable { props: vec![
                    ("value", "String", "Current HTML content"),
                    ("on_change", "Option<EventHandler>", "Change handler - called when content changes"),
                    ("placeholder", "Option", "Placeholder text when editor is empty"),
                    ("disabled", "bool", "Disabled state"),
                    ("min_height", "&'static str", "Minimum height of editor (default: \"200px\")"),
                    ("max_height", "Option", "Maximum height of editor"),
                    ("features", "RichTextFeatures", "Feature flags to enable/disable formatting options"),
                    ("class", "Option", "Additional CSS classes"),
                    ("id", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "RichTextToolbarProps" }
                PropsTable { props: vec![
                    ("features", "RichTextFeatures", "Feature flags"),
                    ("disabled", "bool", "Disabled state"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "ToolbarButtonProps" }
                PropsTable { props: vec![
                    ("title", "&'static str", "Button title (tooltip)"),
                    ("icon", "&'static str", "Button icon/text"),
                    ("command", "&'static str", "execCommand command name"),
                    ("value", "Option", "Optional command value"),
                    ("style", "String", "Base style"),
                    ("hover_style", "String", "Hover style"),
                    ("id", "Option", ""),
                    ("class", "Option", "")
                ] }
                h4 { "RichTextContentProps" }
                PropsTable { props: vec![
                    ("value", "String", "Current value"),
                    ("placeholder", "Option", "Placeholder text"),
                    ("disabled", "bool", "Disabled state"),
                    ("min_height", "&'static str", "Minimum height"),
                    ("max_height", "Option", "Maximum height"),
                    ("is_focused", "bool", "Whether the editor is focused"),
                    ("on_input", "Option<EventHandler>", "Input handler"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "SimpleRichTextProps" }
                PropsTable { props: vec![
                    ("value", "String", "Current HTML content"),
                    ("on_change", "Option<EventHandler>", "Change handler"),
                    ("placeholder", "Option", "Placeholder text"),
                    ("disabled", "bool", "Disabled state"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "MinimalRichTextProps" }
                PropsTable { props: vec![
                    ("value", "String", "Current HTML content"),
                    ("on_change", "Option<EventHandler>", "Change handler"),
                    ("placeholder", "Option", "Placeholder text"),
                    ("disabled", "bool", "Disabled state"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
                h4 { "FullRichTextProps" }
                PropsTable { props: vec![
                    ("value", "String", "Current HTML content"),
                    ("on_change", "Option<EventHandler>", "Change handler"),
                    ("placeholder", "Option", "Placeholder text"),
                    ("disabled", "bool", "Disabled state"),
                    ("min_height", "&'static str", "Minimum height (default: \"300px\")"),
                    ("max_height", "Option", "Maximum height"),
                    ("id", "Option", ""),
                    ("class", "Option", ""),
                    ("style", "Option", "")
                ] }
            }
        }
    }
}

/// Tour documentation page
#[component]
pub fn TourPage() -> Element {
    let mut tour_open = use_signal(|| false);

    rsx! {
        DocPage {
            title: "Tour",
            description: "A guided product walkthrough component with step-by-step highlights.",

            Section { title: "Basic Tour",
                ExampleBox {
                    Box { style: "display: flex; flex-direction: column; gap: 16px;",
                        Box {
                            id: "tour-step-1",
                            style: "padding: 16px; border: 2px dashed #cbd5e1; border-radius: 8px;",
                            "Step 1 target: Welcome area"
                        }
                        Box {
                            id: "tour-step-2",
                            style: "padding: 16px; border: 2px dashed #cbd5e1; border-radius: 8px;",
                            "Step 2 target: Features section"
                        }
                        Box {
                            id: "tour-step-3",
                            style: "padding: 16px; border: 2px dashed #cbd5e1; border-radius: 8px;",
                            "Step 3 target: Get started"
                        }
                        Button {
                            onclick: move |_| tour_open.set(true),
                            "Start Tour"
                        }
                    }
                }
                CodeBlock { code: r##"let mut tour_open = use_signal(|| false);

Tour {
    steps: vec![
        TourStep {
            target: "#welcome".to_string(),
            title: "Welcome".to_string(),
            content: "This is your dashboard.".to_string(),
            placement: Placement::Bottom,
        },
        TourStep {
            target: "#features".to_string(),
            title: "Features".to_string(),
            content: "Explore key features here.".to_string(),
            placement: Placement::Right,
        },
    ],
    open: tour_open(),
    on_open_change: move |v| tour_open.set(v),
}"##.to_string() }
            }

            Section { title: "Props",
                PropsTable { props: vec![
                    ("steps", "Vec<TourStep>", "Tour steps with target, title, content, placement"),
                    ("open", "bool", "Whether the tour is open"),
                    ("on_open_change", "EventHandler<bool>", "Callback when tour open state changes"),
                    ("on_finish", "Option<EventHandler>", "Callback when tour completes"),
                    ("on_step_change", "Option<EventHandler<usize>>", "Callback when step changes"),
                    ("mask", "bool", "Show backdrop mask (default: true)"),
                    ("show_dots", "bool", "Show step dots (default: true)"),
                    ("show_progress", "bool", "Show step counter (default: true)"),
                    ("allow_interaction", "bool", "Allow interacting with highlighted element (default: false)"),
                    ("class", "Option", "Additional CSS classes"),
                    ("id", "Option", "Element id"),
                    ("style", "Option", "Custom inline styles")
                ] }
            }
        }
    }
}
