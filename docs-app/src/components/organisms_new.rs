//! New Organism component documentation pages (Phase 1-4)

use dioxus::prelude::*;
use dioxus_ui_system::prelude::*;
use dioxus_ui_system::organisms::{Calendar, CalendarMode, DateRangePicker, DateRangePreset, default_presets, Carousel, CarouselOptions, CarouselContent, CarouselItem, CarouselPrevious, CarouselNext, Tree, TreeNodeData, Timeline, TimelinePosition, TimelineItem, TimelineDot, TimelineContent, Menubar, ResizablePanelGroup, ResizablePanel, ResizableHandle, Direction, Kanban, KanbanColumn, KanbanCard, ImageUploader};
use crate::docs_ui::{DocPage, Section, ExampleBox, CodeBlock, PropsTable};

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
                CodeBlock { code: r#"Calendar {
    mode: CalendarMode::Single,
    value: selected_date(),
    on_change: move |date| selected.set(date),
}"#.to_string() }
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
                            show_week_numbers: true,
                            on_change: move |_| {},
                        }
                    }
                }
            }
        }
    }
}

/// Date Range Picker documentation page
#[component]
pub fn DateRangePickerPage() -> Element {
    rsx! {
        DocPage {
            title: "Date Range Picker",
            description: "A date picker for selecting date ranges with presets.",
            
            Section { title: "Basic",
                ExampleBox {
                    Box { style: "max-width: 300px;",
                        DateRangePicker {
                            start_date: Some("2024-01-01".to_string()),
                            end_date: Some("2024-01-31".to_string()),
                            on_start_date_change: move |_| {},
                            on_end_date_change: move |_| {},
                        }
                    }
                }
                CodeBlock { code: r#"DateRangePicker {
    start_date: start(),
    end_date: end(),
    on_start_date_change: move |d| start.set(d),
    on_end_date_change: move |d| end.set(d),
}"#.to_string() }
            }
            
            Section { title: "With Presets",
                ExampleBox {
                    Box { style: "max-width: 300px;",
                        DateRangePicker {
                            presets: default_presets(),
                            on_start_date_change: move |_| {},
                            on_end_date_change: move |_| {},
                        }
                    }
                }
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
            description: "An image and content slider with navigation controls.",
            
            Section { title: "Basic",
                ExampleBox {
                    Carousel {
                        CarouselContent {
                            CarouselItem { index: 0,
                                Box { style: "height: 200px; background: #3b82f6; display: flex; align-items: center; justify-content: center; color: white; border-radius: 8px;", "Slide 1" }
                            }
                            CarouselItem { index: 1,
                                Box { style: "height: 200px; background: #22c55e; display: flex; align-items: center; justify-content: center; color: white; border-radius: 8px;", "Slide 2" }
                            }
                            CarouselItem { index: 2,
                                Box { style: "height: 200px; background: #f59e0b; display: flex; align-items: center; justify-content: center; color: white; border-radius: 8px;", "Slide 3" }
                            }
                        }
                        CarouselPrevious {}
                        CarouselNext {}
                    }
                }
                CodeBlock { code: r#"Carousel {
    CarouselContent {
        CarouselItem { "Slide 1" }
        CarouselItem { "Slide 2" }
        CarouselItem { "Slide 3" }
    }
    CarouselPrevious {}
    CarouselNext {}
}"#.to_string() }
            }
            
            Section { title: "With Dots",
                ExampleBox {
                    "Carousel with dot indicators"
                }
            }
        }
    }
}

/// Tree documentation page
#[component]
pub fn TreePage() -> Element {
    let tree_data = vec![
        TreeNodeData::new("1", "src")
            .with_child(TreeNodeData::new("1-1", "components")
                .with_child(TreeNodeData::new("1-1-1", "button.rs"))
                .with_child(TreeNodeData::new("1-1-2", "input.rs")))
            .with_child(TreeNodeData::new("1-2", "styles")
                .with_child(TreeNodeData::new("1-2-1", "theme.rs"))),
        TreeNodeData::new("2", "Cargo.toml"),
    ];
    
    rsx! {
        DocPage {
            title: "Tree",
            description: "A hierarchical tree view for displaying nested data.",
            
            Section { title: "Basic",
                ExampleBox {
                    Box { style: "max-width: 300px; border: 1px solid #e5e7eb; border-radius: 8px; padding: 16px;",
                        Tree {
                            data: tree_data.clone(),
                            on_select: move |_| {},
                            on_toggle_expand: move |_| {},
                        }
                    }
                }
                CodeBlock { code: r#"let data = vec![
    TreeNodeData::new("1", "src")
        .with_child(TreeNodeData::new("1-1", "components")),
    TreeNodeData::new("2", "Cargo.toml"),
];

Tree {
    data: data,
    on_select: move |id| println!("Selected: {}", id),
}"#.to_string() }
            }
            
            Section { title: "With Connector Lines",
                ExampleBox {
                    Box { style: "max-width: 300px; border: 1px solid #e5e7eb; border-radius: 8px; padding: 16px;",
                        Tree {
                            data: tree_data.clone(),
                            show_lines: true,
                            on_select: move |_| {},
                            on_toggle_expand: move |_| {},
                        }
                    }
                }
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
            description: "A vertical timeline for displaying chronological events.",
            
            Section { title: "Basic",
                ExampleBox {
                    Timeline {
                        TimelineItem {
                            TimelineDot {}
                            TimelineContent {
                                h4 { style: "margin: 0;", "Event 1" }
                                p { style: "margin: 4px 0 0 0; color: #6b7280;", "Description of event 1" }
                            }
                        }
                        TimelineItem {
                            TimelineDot {}
                            TimelineContent {
                                h4 { style: "margin: 0;", "Event 2" }
                                p { style: "margin: 4px 0 0 0; color: #6b7280;", "Description of event 2" }
                            }
                        }
                        TimelineItem {
                            TimelineDot {}
                            TimelineContent {
                                h4 { style: "margin: 0;", "Event 3" }
                                p { style: "margin: 4px 0 0 0; color: #6b7280;", "Description of event 3" }
                            }
                        }
                    }
                }
                CodeBlock { code: r#"Timeline {
    TimelineItem {
        TimelineDot {}
        TimelineContent { "Event 1" }
    }
    TimelineItem {
        TimelineDot {}
        TimelineContent { "Event 2" }
    }
}"#.to_string() }
            }
            
            Section { title: "Alternate Position",
                ExampleBox {
                    Timeline {
                        position: TimelinePosition::Alternate,
                        "Timeline with alternating left/right content"
                    }
                }
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
            description: "An application menu bar with nested dropdown menus.",
            
            Section { title: "Basic",
                ExampleBox {
                    Menubar {
                        "File | Edit | View menu structure"
                    }
                }
                CodeBlock { code: r#"Menubar {
    MenubarMenu {
        MenubarTrigger { label: "File" }
        MenubarContent {
            MenubarItem { onclick: move |_| {}, "New" }
            MenubarItem { onclick: move |_| {}, "Open" }
            MenubarSeparator {}
            MenubarItem { onclick: move |_| {}, "Exit" }
        }
    }
}"#.to_string() }
            }
        }
    }
}

/// Resizable Panels documentation page
#[component]
pub fn ResizablePage() -> Element {
    rsx! {
        DocPage {
            title: "Resizable Panels",
            description: "Split-pane layout with draggable resize handles.",
            
            Section { title: "Horizontal",
                ExampleBox {
                    Box { style: "height: 200px; border: 1px solid #e5e7eb; border-radius: 8px;",
                        ResizablePanelGroup {
                            direction: Direction::Horizontal,
                            ResizablePanel { default_size: Some(30.0), 
                                Box { style: "padding: 16px; background: #f3f4f6; height: 100%;", "Left Panel" }
                            }
                            ResizableHandle {}
                            ResizablePanel { default_size: Some(70.0),
                                Box { style: "padding: 16px; background: #e5e7eb; height: 100%;", "Right Panel" }
                            }
                        }
                    }
                }
                CodeBlock { code: r#"ResizablePanelGroup {
    direction: Direction::Horizontal,
    ResizablePanel { default_size: Some(30.0), "Left" }
    ResizableHandle {}
    ResizablePanel { default_size: Some(70.0), "Right" }
}"#.to_string() }
            }
            
            Section { title: "Vertical",
                ExampleBox {
                    Box { style: "height: 300px; border: 1px solid #e5e7eb; border-radius: 8px;",
                        ResizablePanelGroup {
                            direction: Direction::Vertical,
                            "Vertical split example"
                        }
                    }
                }
            }
        }
    }
}

/// Kanban documentation page
#[component]
pub fn KanbanPage() -> Element {
    let columns = vec![
        KanbanColumn {
            id: "todo".to_string(),
            title: "To Do".to_string(),
            color: Some("#ef4444".to_string()),
            cards: vec![
                KanbanCard {
                    id: "1".to_string(),
                    title: "Research competitors".to_string(),
                    description: Some("Analyze top 5 competitors".to_string()),
                    tags: vec!["research".to_string()],
                    assignee: Some("JD".to_string()),
                    due_date: Some("2024-03-20".to_string()),
                },
                KanbanCard {
                    id: "2".to_string(),
                    title: "Design mockups".to_string(),
                    description: None,
                    tags: vec!["design".to_string()],
                    assignee: Some("AS".to_string()),
                    due_date: None,
                },
            ],
        },
        KanbanColumn {
            id: "inprogress".to_string(),
            title: "In Progress".to_string(),
            color: Some("#f59e0b".to_string()),
            cards: vec![
                KanbanCard {
                    id: "3".to_string(),
                    title: "Implement auth".to_string(),
                    description: Some("JWT authentication".to_string()),
                    tags: vec!["backend".to_string(), "urgent".to_string()],
                    assignee: Some("MK".to_string()),
                    due_date: Some("2024-03-18".to_string()),
                },
            ],
        },
        KanbanColumn {
            id: "done".to_string(),
            title: "Done".to_string(),
            color: Some("#22c55e".to_string()),
            cards: vec![],
        },
    ];
    
    rsx! {
        DocPage {
            title: "Kanban Board",
            description: "A drag-and-drop task board with columns and cards.",
            
            Section { title: "Basic",
                ExampleBox {
                    Box { style: "height: 400px; overflow-x: auto;",
                        Kanban {
                            columns: columns,
                            on_columns_change: move |_| {},
                            show_card_count: true,
                        }
                    }
                }
                CodeBlock { code: r#"let columns = vec![
    KanbanColumn {
        id: "todo".to_string(),
        title: "To Do".to_string(),
        cards: vec![KanbanCard { ... }],
    },
];

Kanban {
    columns: columns,
    on_columns_change: move |cols| columns.set(cols),
}"#.to_string() }
            }
        }
    }
}

/// Image Uploader documentation page
#[component]
pub fn ImageUploaderPage() -> Element {
    rsx! {
        DocPage {
            title: "Image Uploader",
            description: "A file upload component for images with preview and validation.",
            
            Section { title: "Basic",
                ExampleBox {
                    ImageUploader {
                        value: vec![],
                        on_change: move |_| {},
                    }
                }
                CodeBlock { code: r#"ImageUploader {
    value: images(),
    on_change: move |imgs| images.set(imgs),
    max_files: Some(5),
    max_size_mb: 5.0,
    accept: vec!["image/*".to_string()],
}"#.to_string() }
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
        }
    }
}
