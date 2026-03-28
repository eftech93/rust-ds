//! File Upload organism component
//!
//! Drag-and-drop file upload with progress and preview.
//! Supports multiple file types, file size limits, and displays file names.

use crate::theme::use_theme;
use dioxus::prelude::*;

/// Type of file being uploaded
#[derive(Clone, PartialEq, Debug)]
pub enum FileType {
    /// Image file (jpg, png, gif, etc.)
    Image,
    /// Document file (pdf, doc, docx, etc.)
    Document,
    /// Video file (mp4, mov, etc.)
    Video,
    /// Audio file (mp3, wav, etc.)
    Audio,
    /// Archive file (zip, rar, etc.)
    Archive,
    /// Unknown/other file type
    Other,
}

impl FileType {
    /// Detect file type from extension
    pub fn from_file_name(file_name: &str) -> Self {
        let ext = file_name
            .split('.')
            .last()
            .unwrap_or("")
            .to_lowercase();
        match ext.as_str() {
            "jpg" | "jpeg" | "png" | "gif" | "webp" | "svg" | "bmp" | "ico" | "tiff" | "tif"
                => FileType::Image,
            "pdf" | "doc" | "docx" | "txt" | "rtf" | "odt" | "xls" | "xlsx" | "ppt"
                | "pptx" | "csv" => FileType::Document,
            "mp4" | "mov" | "avi" | "mkv" | "flv" | "wmv" | "webm" | "m4v"
                => FileType::Video,
            "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a" | "wma"
                => FileType::Audio,
            "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz"
                => FileType::Archive,
            _ => FileType::Other,
        }
    }

    /// Get icon for file type
    pub fn icon(&self) -> &'static str {
        match self {
            FileType::Image => "🖼️",
            FileType::Document => "📄",
            FileType::Video => "🎬",
            FileType::Audio => "🎵",
            FileType::Archive => "📦",
            FileType::Other => "📎",
        }
    }

    /// Get color for file type
    pub fn color(&self) -> &'static str {
        match self {
            FileType::Image => "#3b82f6",    // blue
            FileType::Document => "#64748b", // slate
            FileType::Video => "#ef4444",    // red
            FileType::Audio => "#8b5cf6",    // purple
            FileType::Archive => "#f59e0b",  // amber
            FileType::Other => "#6b7280",    // gray
        }
    }
}

/// Uploaded file info
#[derive(Clone, PartialEq, Debug)]
pub struct UploadedFile {
    pub name: String,
    pub size: u64,
    pub file_type: String,
    pub data: Vec<u8>,
}

/// File upload properties
#[derive(Props, Clone, PartialEq)]
pub struct FileUploadProps {
    /// Accepted file types (MIME types or extensions, e.g., "image/*", ".pdf")
    #[props(default)]
    pub accept: Option<String>,
    /// Maximum file size in MB
    #[props(default)]
    pub max_size_mb: Option<f64>,
    /// Maximum number of files
    #[props(default = 1)]
    pub max_files: usize,
    /// Multiple files allowed
    #[props(default = false)]
    pub multiple: bool,
    /// Single file mode (alias for !multiple)
    #[props(default)]
    pub single: bool,
    /// Upload handler
    pub on_upload: EventHandler<Vec<UploadedFile>>,
    /// Change handler (called when files are selected)
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
    /// Show file list
    #[props(default = true)]
    pub show_file_list: bool,
}

/// File upload component
#[component]
pub fn FileUpload(props: FileUploadProps) -> Element {
    let theme = use_theme();
    let mut is_dragging = use_signal(|| false);
    let mut files = use_signal(|| Vec::<UploadedFile>::new());
    let mut validation_error = use_signal(|| None::<String>);

    // Determine if multiple files are allowed
    let allow_multiple = props.multiple && !props.single;

    let class_css = props
        .class
        .as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();

    let border_color = if props.error.is_some() || validation_error().is_some() {
        theme.tokens.read().colors.destructive.to_rgba()
    } else if is_dragging() {
        theme.tokens.read().colors.primary.to_rgba()
    } else {
        theme.tokens.read().colors.border.to_rgba()
    };

    let bg_color = if is_dragging() {
        format!(
            "{}15",
            theme
                .tokens
                .read()
                .colors
                .primary
                .to_rgba()
                .trim_start_matches('#')
        )
    } else {
        theme.tokens.read().colors.background.to_rgba()
    };

    // Validate file closure
    let _validate_file = {
        let accept = props.accept.clone();
        let max_size_mb = props.max_size_mb;
        move |name: &str, size: u64| -> Result<(), String> {
            // Check file type
            if let Some(ref accept) = accept {
                let is_valid = accept.split(',').any(|pattern| {
                    let pattern = pattern.trim();
                    if pattern == "*/*" {
                        return true;
                    }
                    if pattern.ends_with("/*") {
                        let prefix = pattern.trim_end_matches("/*");
                        let ft = FileType::from_file_name(name);
                        return match prefix {
                            "image" => matches!(ft, FileType::Image),
                            "video" => matches!(ft, FileType::Video),
                            "audio" => matches!(ft, FileType::Audio),
                            "application" => matches!(ft, FileType::Document | FileType::Archive),
                            _ => true,
                        };
                    }
                    name.to_lowercase().ends_with(&pattern.to_lowercase())
                });
                if !is_valid {
                    return Err(format!("Invalid file type. Accepted: {}", accept));
                }
            }

            // Check file size
            if let Some(max_mb) = max_size_mb {
                let max_bytes = (max_mb * 1024.0 * 1024.0) as u64;
                if size > max_bytes {
                    return Err(format!(
                        "File too large. Maximum size: {} MB",
                        max_mb
                    ));
                }
            }

            Ok(())
        }
    };

    // Handle file selection
    let handle_file_select = {
        let on_upload = props.on_upload.clone();
        let on_change = props.on_change.clone();
        let max_files = props.max_files;
        let allow_multiple = allow_multiple;
        let validate_file = _validate_file.clone();
        #[allow(unused_mut, unused_variables)]
        let mut files_signal = files.clone();
        #[allow(unused_variables)]
        move |e: Event<FormData>| {
            validation_error.set(None);

            // Access the raw HTML input element to get files
            #[cfg(all(feature = "web", target_arch = "wasm32"))]
            {
                use dioxus::web::WebEventExt;
                use wasm_bindgen::JsCast;
                use web_sys::HtmlInputElement;

                if let Some(target) = e.data().as_web_event().target() {
                    if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                        if let Some(file_list) = input.files() {
                            let mut new_files = Vec::new();
                            let file_count = file_list.length();
                            
                            for i in 0..file_count {
                                if let Some(file) = file_list.get(i) {
                                    // Check max files limit
                                    if !allow_multiple && i > 0 {
                                        break;
                                    }
                                    if files_signal().len() + new_files.len() >= max_files {
                                        validation_error.set(Some(format!("Maximum {} files allowed", max_files)));
                                        break;
                                    }
                                    
                                    let name = file.name();
                                    let size = file.size() as u64;
                                    let file_type = file.type_();
                                    
                                    // Validate file
                                    if let Err(err) = validate_file(&name, size) {
                                        validation_error.set(Some(err));
                                        continue;
                                    }
                                    
                                    new_files.push(UploadedFile {
                                        name,
                                        size,
                                        file_type,
                                        data: Vec::new(), // Data would be read asynchronously
                                    });
                                }
                            }
                            
                            // Update files signal
                            if !new_files.is_empty() {
                                files_signal.with_mut(|f| {
                                    if allow_multiple {
                                        f.extend(new_files.clone());
                                    } else {
                                        *f = new_files.clone();
                                    }
                                });
                                
                                // Call callbacks
                                let all_files = files_signal();
                                on_upload.call(all_files.clone());
                                if let Some(ref on_change_cb) = on_change {
                                    on_change_cb.call(all_files.clone());
                                }
                            }
                            
                            // Clear the input value to allow re-selecting the same file
                            input.set_value("");
                        }
                    }
                }
            }
            
            // Non-web platforms - placeholder
            #[cfg(not(all(feature = "web", target_arch = "wasm32")))]
            {
                let _ = (&on_upload, &on_change, &allow_multiple, &max_files, &validate_file);
            }
        }
    };

    // Remove a file
    let mut remove_file = {
        let on_upload = props.on_upload.clone();
        let on_change = props.on_change.clone();
        let mut files_signal = files.clone();
        move |index: usize| {
            files_signal.with_mut(|f| {
                if index < f.len() {
                    f.remove(index);
                }
            });
            
            // Call callbacks with updated files
            let all_files = files_signal();
            on_upload.call(all_files.clone());
            if let Some(ref on_change_cb) = on_change {
                on_change_cb.call(all_files.clone());
            }
        }
    };

    // Clear all files
    let mut clear_files = {
        let on_upload = props.on_upload.clone();
        let on_change = props.on_change.clone();
        move || {
            files.set(Vec::new());
            on_upload.call(Vec::new());
            if let Some(ref on_change_cb) = on_change {
                on_change_cb.call(Vec::new());
            }
        }
    };

    // Helper text with constraints info
    let default_label = if allow_multiple {
        "Upload files"
    } else {
        "Upload file"
    };
    let label_text = props.label.clone().unwrap_or_else(|| default_label.to_string());

    let helper_text = props.helper_text.clone().or_else(|| {
        let mut parts = Vec::new();
        if let Some(max_mb) = props.max_size_mb {
            parts.push(format!("Max {} MB", max_mb));
        }
        if allow_multiple {
            parts.push(format!("Max {} files", props.max_files));
        } else {
            parts.push("Single file only".to_string());
        }
        if props.accept.is_some() {
            parts.push("Specific types only".to_string());
        }
        if parts.is_empty() {
            None
        } else {
            Some(parts.join(" • "))
        }
    });

    // Get current file count for conditional rendering
    let file_count = files().len();
    let at_max_files = file_count >= props.max_files;

    rsx! {
        div {
            class: "file-upload{class_css}",
            style: "display: flex; flex-direction: column; gap: 12px;",

            // Drop zone (only show if not at max files)
            if !at_max_files {
                div {
                    class: "file-upload-dropzone",
                    style: "padding: 32px; border: 2px dashed {border_color}; border-radius: 12px; background: {bg_color}; text-align: center; transition: all 0.15s ease;",
                    ondragenter: move |e: Event<dioxus::html::DragData>| {
                        e.prevent_default();
                        if !props.disabled {
                            is_dragging.set(true);
                        }
                    },
                    ondragover: move |e: Event<dioxus::html::DragData>| {
                        e.prevent_default();
                    },
                    ondragleave: move |_| {
                        is_dragging.set(false);
                    },
                    ondrop: {
                        let on_upload = props.on_upload.clone();
                        let on_change = props.on_change.clone();
                        let max_files = props.max_files;
                        let allow_multiple = allow_multiple;
                        let validate_file = _validate_file.clone();
                        #[allow(unused_mut, unused_variables)]
                        let mut files_signal = files.clone();
                        move |e: Event<dioxus::html::DragData>| {
                            e.prevent_default();
                            is_dragging.set(false);
                            validation_error.set(None);
                            
                            #[cfg(all(feature = "web", target_arch = "wasm32"))]
                            {
                                use dioxus::web::WebEventExt;
                                use wasm_bindgen::JsCast;
                                use web_sys::DataTransfer;
                                
                                if let Some(data_transfer) = e.data().as_web_event().dyn_ref::<DataTransfer>() {
                                    if let Some(file_list) = data_transfer.files() {
                                        let mut new_files = Vec::new();
                                        let file_count = file_list.length();
                                        
                                        for i in 0..file_count {
                                            if let Some(file) = file_list.get(i) {
                                                if !allow_multiple && i > 0 {
                                                    break;
                                                }
                                                if files_signal().len() + new_files.len() >= max_files {
                                                    validation_error.set(Some(format!("Maximum {} files allowed", max_files)));
                                                    break;
                                                }
                                                
                                                let name = file.name();
                                                let size = file.size() as u64;
                                                let file_type = file.type_();
                                                
                                                if let Err(err) = validate_file(&name, size) {
                                                    validation_error.set(Some(err));
                                                    continue;
                                                }
                                                
                                                new_files.push(UploadedFile {
                                                    name,
                                                    size,
                                                    file_type,
                                                    data: Vec::new(),
                                                });
                                            }
                                        }
                                        
                                        if !new_files.is_empty() {
                                            files_signal.with_mut(|f| {
                                                if allow_multiple {
                                                    f.extend(new_files.clone());
                                                } else {
                                                    *f = new_files.clone();
                                                }
                                            });
                                            
                                            let all_files = files_signal();
                                            on_upload.call(all_files.clone());
                                            if let Some(ref on_change_cb) = on_change {
                                                on_change_cb.call(all_files.clone());
                                            }
                                        }
                                    }
                                }
                            }
                            
                            #[cfg(not(all(feature = "web", target_arch = "wasm32")))]
                            {
                                let _ = (&on_upload, &on_change, &allow_multiple, &max_files, &validate_file);
                            }
                        }
                    },

                    // Upload icon
                    div {
                        style: "font-size: 40px; margin-bottom: 12px;",
                        "📁"
                    }

                    // Label
                    p {
                        style: "margin: 0 0 8px 0; font-size: 16px; font-weight: 500; color: {theme.tokens.read().colors.foreground.to_rgba()};",
                        "{label_text}"
                    }

                    // Helper text with constraints
                    if let Some(helper) = helper_text {
                        p {
                            style: "margin: 0 0 16px 0; font-size: 13px; color: {theme.tokens.read().colors.muted.to_rgba()};",
                            "{helper}"
                        }
                    }

                    // File input
                    label {
                        class: "file-upload-button",
                        style: "display: inline-block; padding: 10px 20px; font-size: 14px; font-weight: 500; color: white; background: {theme.tokens.read().colors.primary.to_rgba()}; border-radius: 8px; cursor: pointer; transition: opacity 0.15s ease;",

                        if allow_multiple {
                            "Browse files"
                        } else {
                            "Browse file"
                        }

                        input {
                            r#type: "file",
                            style: "display: none;",
                            accept: props.accept.as_deref().unwrap_or("*/*"),
                            multiple: allow_multiple,
                            disabled: props.disabled || props.loading,
                            onchange: handle_file_select,
                        }
                    }
                }
            }

            // Error messages
            if let Some(error) = props.error.clone() {
                p {
                    class: "file-upload-error",
                    style: "margin: 0; font-size: 14px; color: {theme.tokens.read().colors.destructive.to_rgba()};",
                    "{error}"
                }
            }
            if let Some(error) = validation_error() {
                p {
                    class: "file-upload-validation-error",
                    style: "margin: 0; font-size: 14px; color: {theme.tokens.read().colors.destructive.to_rgba()};",
                    "{error}"
                }
            }

            // File list with clear button
            if props.show_file_list && !files().is_empty() {
                div {
                    class: "file-upload-list-container",
                    style: "margin-top: 8px;",

                    // Header with count and clear button
                    div {
                        style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;",

                        p {
                            style: "margin: 0; font-size: 14px; font-weight: 500; color: {theme.tokens.read().colors.foreground.to_rgba()};",
                            {
                                let count = files().len();
                                let file_word = if count > 1 { "files" } else { "file" };
                                format!("{} {} selected", count, file_word)
                            }
                        }

                        button {
                            r#type: "button",
                            style: "font-size: 12px; color: {theme.tokens.read().colors.muted.to_rgba()}; background: none; border: none; cursor: pointer; padding: 4px 8px;",
                            onclick: move |_| clear_files(),
                            "Clear all"
                        }
                    }

                    // File list
                    div {
                        class: "file-upload-list",
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        for (i, file) in files().iter().enumerate() {
                            FileUploadItem {
                                key: "{i}",
                                file: file.clone(),
                                on_remove: {
                                    move || remove_file(i)
                                },
                            }
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
}

/// File upload item component
#[component]
fn FileUploadItem(props: FileUploadItemProps) -> Element {
    let theme = use_theme();

    let file_type = FileType::from_file_name(&props.file.name);
    let file_icon = file_type.icon();
    let size_text = format_file_size(props.file.size);

    rsx! {
        div {
            class: "file-upload-item",
            style: "display: flex; align-items: center; gap: 12px; padding: 12px; border: 1px solid {theme.tokens.read().colors.border.to_rgba()}; border-radius: 8px; background: {theme.tokens.read().colors.background.to_rgba()};",

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
                    title: "{props.file.name}",
                    "{props.file.name}"
                }

                p {
                    class: "file-upload-item-size",
                    style: "margin: 4px 0 0 0; font-size: 12px; color: {theme.tokens.read().colors.muted.to_rgba()};",
                    "{size_text} • {props.file.file_type}"
                }
            }

            // Remove button
            button {
                r#type: "button",
                class: "file-upload-item-remove",
                style: "flex-shrink: 0; background: none; border: none; cursor: pointer; font-size: 18px; color: {theme.tokens.read().colors.muted.to_rgba()}; padding: 4px; transition: color 0.15s ease;",
                onclick: move |_| props.on_remove.call(()),
                "✕"
            }
        }
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
