//! File Upload organism component
//!
//! Drag-and-drop file upload with progress and preview.

use dioxus::prelude::*;
use crate::theme::use_theme;

/// File upload properties
#[derive(Props, Clone, PartialEq)]
pub struct FileUploadProps {
    /// Accepted file types
    #[props(default)]
    pub accept: Option<String>,
    /// Maximum file size in bytes
    #[props(default)]
    pub max_size: Option<u64>,
    /// Maximum number of files
    #[props(default = 1)]
    pub max_files: usize,
    /// Multiple files allowed
    #[props(default = false)]
    pub multiple: bool,
    /// Upload handler
    pub on_upload: EventHandler<Vec<UploadedFile>>,
    /// Change handler
    #[props(default)]
    pub on_change: Option<EventHandler<Vec<UploadedFile>>>,
    /// Label
    #[props(default)]
    pub label: Option<String>,
    /// Helper text
    #[props(default)]
    pub helper_text: Option<String>,
    /// Error message
    #[props(default)]
    pub error: Option<String>,
    /// Loading state
    #[props(default = false)]
    pub loading: bool,
    /// Disabled state
    #[props(default = false)]
    pub disabled: bool,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Uploaded file info
#[derive(Clone, PartialEq, Debug)]
pub struct UploadedFile {
    pub name: String,
    pub size: u64,
    pub file_type: String,
    pub data: Vec<u8>,
}

/// File upload component
#[component]
pub fn FileUpload(props: FileUploadProps) -> Element {
    let theme = use_theme();
    let mut is_dragging = use_signal(|| false);
    let mut files = use_signal(|| Vec::<UploadedFile>::new());
    
    let class_css = props.class.as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();
    
    let border_color = if props.error.is_some() {
        theme.tokens.read().colors.destructive.to_rgba()
    } else if is_dragging() {
        theme.tokens.read().colors.primary.to_rgba()
    } else {
        theme.tokens.read().colors.border.to_rgba()
    };
    
    let bg_color = if is_dragging() {
        format!("{}15", theme.tokens.read().colors.primary.to_rgba().trim_start_matches('#'))
    } else {
        theme.tokens.read().colors.background.to_rgba()
    };
    
    let _handle_files = {
        let on_upload = props.on_upload.clone();
        let on_change = props.on_change.clone();
        let max_files = props.max_files;
        let max_size = props.max_size;
        move |new_files: Vec<UploadedFile>| {
            let valid_files: Vec<_> = new_files.into_iter()
                .filter(|f| {
                    if let Some(max) = max_size {
                        f.size <= max
                    } else {
                        true
                    }
                })
                .take(max_files)
                .collect();
            
            files.set(valid_files.clone());
            
            if let Some(handler) = &on_change {
                handler.call(valid_files.clone());
            }
            
            on_upload.call(valid_files);
        }
    };
    
    rsx! {
        div {
            class: "file-upload{class_css}",
            style: "display: flex; flex-direction: column; gap: 12px;",
            
            // Drop zone
            div {
                class: "file-upload-dropzone",
                style: "padding: 48px; border: 2px dashed {border_color}; border-radius: 12px; background: {bg_color}; text-align: center; transition: all 0.15s ease;",
                ondragenter: move |e: Event<dioxus::html::DragData>| {
                    e.prevent_default();
                    is_dragging.set(true);
                },
                ondragover: move |e: Event<dioxus::html::DragData>| {
                    e.prevent_default();
                },
                ondragleave: move |_| {
                    is_dragging.set(false);
                },
                ondrop: move |e: Event<dioxus::html::DragData>| {
                    e.prevent_default();
                    is_dragging.set(false);
                    // Handle dropped files (simplified)
                },
                
                // Upload icon
                div {
                    style: "font-size: 48px; margin-bottom: 16px;",
                    "📁"
                }
                
                // Label
                if let Some(label) = props.label.clone() {
                    p {
                        style: "margin: 0 0 8px 0; font-size: 16px; font-weight: 500; color: {theme.tokens.read().colors.foreground.to_rgba()};",
                        "{label}"
                    }
                }
                
                // Helper text
                if let Some(helper) = props.helper_text {
                    p {
                        style: "margin: 0 0 16px 0; font-size: 14px; color: {theme.tokens.read().colors.muted.to_rgba()};",
                        "{helper}"
                    }
                }
                
                // File input
                label {
                    class: "file-upload-button",
                    style: "display: inline-block; padding: 10px 20px; font-size: 14px; font-weight: 500; color: white; background: {theme.tokens.read().colors.primary.to_rgba()}; border-radius: 8px; cursor: pointer; transition: opacity 0.15s ease;",
                    
                    "Browse files"
                    
                    input {
                        type: "file",
                        style: "display: none;",
                        accept: props.accept.as_deref().unwrap_or(""),
                        multiple: props.multiple,
                        disabled: props.disabled || props.loading,
                        onchange: move |_e: Event<FormData>| {
                            // Handle file selection (simplified)
                            let _files: Vec<UploadedFile> = Vec::new();
                            // In a real implementation, read file data
                        },
                    }
                }
            }
            
            // Error message
            if let Some(error) = props.error {
                p {
                    class: "file-upload-error",
                    style: "margin: 0; font-size: 14px; color: {theme.tokens.read().colors.destructive.to_rgba()};",
                    "{error}"
                }
            }
            
            // File list
            if !files().is_empty() {
                div {
                    class: "file-upload-list",
                    style: "display: flex; flex-direction: column; gap: 8px;",
                    
                    for (i, file) in files().iter().enumerate() {
                        FileUploadItem {
                            key: "{i}",
                            file: file.clone(),
                            on_remove: {
                                let mut files = files.clone();
                                move || {
                                    files.with_mut(|f| f.remove(i));
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}

/// File upload item properties
#[derive(Props, Clone, PartialEq)]
pub struct FileUploadItemProps {
    pub file: UploadedFile,
    pub on_remove: EventHandler<()>,
    #[props(default)]
    pub progress: Option<u8>,
}

/// File upload item component
#[component]
fn FileUploadItem(props: FileUploadItemProps) -> Element {
    let theme = use_theme();
    
    let file_icon = get_file_icon(&props.file.file_type);
    let size_text = format_file_size(props.file.size);
    
    rsx! {
        div {
            class: "file-upload-item",
            style: "display: flex; align-items: center; gap: 12px; padding: 12px; border: 1px solid {theme.tokens.read().colors.border.to_rgba()}; border-radius: 8px; background: white;",
            
            // Icon
            div {
                class: "file-upload-item-icon",
                style: "flex-shrink: 0; width: 40px; height: 40px; border-radius: 8px; background: {theme.tokens.read().colors.muted.to_rgba()}; display: flex; align-items: center; justify-content: center; font-size: 20px;",
                "{file_icon}"
            }
            
            // Info
            div {
                class: "file-upload-item-info",
                style: "flex: 1; min-width: 0;",
                
                p {
                    class: "file-upload-item-name",
                    style: "margin: 0; font-size: 14px; font-weight: 500; color: {theme.tokens.read().colors.foreground.to_rgba()}; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                    "{props.file.name}"
                }
                
                p {
                    class: "file-upload-item-size",
                    style: "margin: 4px 0 0 0; font-size: 12px; color: {theme.tokens.read().colors.muted.to_rgba()};",
                    "{size_text}"
                }
                
                // Progress bar
                if let Some(progress) = props.progress {
                    div {
                        class: "file-upload-item-progress",
                        style: "margin-top: 8px; height: 4px; background: {theme.tokens.read().colors.muted.to_rgba()}; border-radius: 2px; overflow: hidden;",
                        
                        div {
                            style: "height: 100%; width: {progress}%; background: {theme.tokens.read().colors.primary.to_rgba()}; border-radius: 2px; transition: width 0.3s ease;",
                        }
                    }
                }
            }
            
            // Remove button
            button {
                type: "button",
                class: "file-upload-item-remove",
                style: "flex-shrink: 0; background: none; border: none; cursor: pointer; font-size: 18px; color: #9ca3af; padding: 4px;",
                onclick: move |_| props.on_remove.call(()),
                "✕"
            }
        }
    }
}

fn get_file_icon(file_type: &str) -> &'static str {
    if file_type.starts_with("image/") {
        "🖼️"
    } else if file_type.starts_with("video/") {
        "🎬"
    } else if file_type.starts_with("audio/") {
        "🎵"
    } else if file_type.contains("pdf") {
        "📄"
    } else if file_type.contains("zip") || file_type.contains("rar") || file_type.contains("7z") {
        "📦"
    } else {
        "📎"
    }
}

fn format_file_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    format!("{:.1} {}", size, UNITS[unit_index])
}
