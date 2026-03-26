//! Image Uploader organism component
//!
//! A file upload component for images with preview, drag-drop, and validation.

use dioxus::prelude::*;
use crate::theme::{use_theme, use_style};
use crate::styles::Style;

/// Upload status for an image
#[derive(Clone, PartialEq, Debug, Default)]
pub enum UploadStatus {
    /// File selected but not yet uploaded
    #[default]
    Pending,
    /// Currently uploading
    Uploading,
    /// Upload successful
    Success,
    /// Upload failed with error message
    Error(String),
}

/// Uploaded image information
#[derive(Clone, PartialEq, Debug)]
pub struct UploadedImage {
    /// Unique identifier for the image
    pub id: String,
    /// Original file name
    pub file_name: String,
    /// Preview URL (base64 or blob URL)
    pub preview_url: String,
    /// File size in bytes
    pub size: usize,
    /// Current upload status
    pub status: UploadStatus,
    /// Optional progress percentage (0-100)
    pub progress: Option<u8>,
}

impl UploadedImage {
    /// Create a new uploaded image
    pub fn new(id: impl Into<String>, file_name: impl Into<String>, preview_url: impl Into<String>, size: usize) -> Self {
        Self {
            id: id.into(),
            file_name: file_name.into(),
            preview_url: preview_url.into(),
            size,
            status: UploadStatus::Pending,
            progress: None,
        }
    }

    /// Set the upload status
    pub fn with_status(mut self, status: UploadStatus) -> Self {
        self.status = status;
        self
    }

    /// Set the upload progress
    pub fn with_progress(mut self, progress: u8) -> Self {
        self.progress = Some(progress);
        self
    }
}

/// Image uploader properties
#[derive(Props, Clone, PartialEq)]
pub struct ImageUploaderProps {
    /// Current value (list of uploaded images)
    #[props(default)]
    pub value: Vec<UploadedImage>,
    /// Callback when the image list changes
    pub on_change: EventHandler<Vec<UploadedImage>>,
    /// Maximum number of files allowed
    #[props(default)]
    pub max_files: Option<usize>,
    /// Maximum file size in MB (default: 5.0)
    #[props(default = 5.0)]
    pub max_size_mb: f32,
    /// Accepted MIME types (default: ["image/*"])
    #[props(default = vec!["image/*".to_string()])]
    pub accept: Vec<String>,
    /// Disabled state
    #[props(default = false)]
    pub disabled: bool,
    /// Allow multiple file selection
    #[props(default = true)]
    pub multiple: bool,
    /// Show image preview thumbnails
    #[props(default = true)]
    pub show_preview: bool,
    /// Optional upload URL for auto-upload
    #[props(default)]
    pub upload_url: Option<String>,
    /// Optional label text
    #[props(default)]
    pub label: Option<String>,
    /// Optional helper text
    #[props(default)]
    pub helper_text: Option<String>,
    /// Optional error message
    #[props(default)]
    pub error: Option<String>,
    /// Custom CSS class
    #[props(default)]
    pub class: Option<String>,
    /// Placeholder text for dropzone
    #[props(default = "Drag and drop images here, or click to browse".to_string())]
    pub placeholder: String,
}

/// Image uploader component
#[component]
pub fn ImageUploader(props: ImageUploaderProps) -> Element {
    let theme = use_theme();
    let mut is_dragging = use_signal(|| false);
    let mut images = use_signal(|| props.value.clone());
    let mut validation_error = use_signal(|| None::<String>);

    // Sync with external value changes
    use_effect(move || {
        images.set(props.value.clone());
    });

    let container_style = use_style(move |_t| {
        Style::new()
            .flex()
            .flex_col()
            .gap_px(12)
            .w_full()
            .build()
    });

    let max_size_bytes = (props.max_size_mb * 1024.0 * 1024.0) as usize;
    let accept_string = props.accept.join(",");
    let accept_for_validation = accept_string.clone();

    // Validate a file
    let validate_file = move |file_name: &str, size: usize| -> Result<(), String> {
        // Check file extension/MIME type
        let is_valid_type = props.accept.iter().any(|accept| {
            if accept == "image/*" {
                file_name.ends_with(".jpg")
                    || file_name.ends_with(".jpeg")
                    || file_name.ends_with(".png")
                    || file_name.ends_with(".gif")
                    || file_name.ends_with(".webp")
                    || file_name.ends_with(".svg")
                    || file_name.ends_with(".bmp")
            } else {
                file_name.to_lowercase().ends_with(&accept.replace("image/", "."))
            }
        });

        if !is_valid_type {
            return Err(format!("Invalid file type. Accepted: {}", accept_for_validation));
        }

        // Check file size
        if size > max_size_bytes {
            return Err(format!(
                "File too large. Maximum size: {} MB",
                props.max_size_mb
            ));
        }

        // Check max files
        if let Some(max) = props.max_files {
            if images().len() >= max {
                return Err(format!("Maximum {} files allowed", max));
            }
        }

        Ok(())
    };

    // Handle file selection
    let handle_files = {
        let on_change = props.on_change.clone();
        move |new_files: Vec<(String, String, usize)>| {
            validation_error.set(None);
            let mut current_images = images();
            let mut has_error = false;

            for (file_name, preview_url, size) in new_files {
                if let Err(e) = validate_file(&file_name, size) {
                    validation_error.set(Some(e));
                    has_error = true;
                    continue;
                }

                let id = format!("img_{}_{}", current_images.len(), now_millis());
                let image = UploadedImage::new(id, file_name, preview_url, size);
                current_images.push(image);

                // Check max files limit
                if let Some(max) = props.max_files {
                    if current_images.len() >= max {
                        break;
                    }
                }
            }

            if !has_error {
                images.set(current_images.clone());
                on_change.call(current_images);
            }
        }
    };

    // Remove an image
    let remove_image = {
        let on_change = props.on_change.clone();
        move |index: usize| {
            let mut current = images();
            if index < current.len() {
                current.remove(index);
                images.set(current.clone());
                on_change.call(current);
            }
        }
    };

    // Border color based on state
    let has_error = props.error.is_some();
    let border_color = use_style(move |t| {
        if has_error || validation_error().is_some() {
            t.colors.destructive.to_rgba()
        } else if is_dragging() {
            t.colors.primary.to_rgba()
        } else {
            t.colors.border.to_rgba()
        }
    });

    // Background color based on drag state
    let bg_color = use_style(move |t| {
        if is_dragging() {
            format!("{}20", t.colors.primary.to_rgba().trim_start_matches('#'))
        } else {
            t.colors.background.to_rgba()
        }
    });

    // Dropzone style
    let dropzone_style = use_style(move |t| {
        Style::new()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .p_px(48)
            .rounded(&t.radius, "lg")
            .border(2, &t.colors.border)
            .border_color(&t.colors.border)
            .cursor(if props.disabled { "not-allowed" } else { "pointer" })
            .opacity(if props.disabled { 0.5 } else { 1.0 })
            .transition("all 150ms ease")
            .build()
    });

    rsx! {
        div {
            class: "image-uploader {props.class.clone().unwrap_or_default()}",
            style: "{container_style}",

            // Label
            if let Some(label) = props.label.clone() {
                div {
                    style: "font-size: 14px; font-weight: 500; color: {theme.tokens.read().colors.foreground.to_rgba()}; margin-bottom: 4px;",
                    "{label}"
                }
            }

            // Drop zone
            div {
                class: "image-uploader-dropzone",
                style: "{dropzone_style} border-color: {border_color}; background: {bg_color}; border-style: dashed;",
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
                ondrop: move |e: Event<dioxus::html::DragData>| {
                    e.prevent_default();
                    is_dragging.set(false);
                    if props.disabled {
                        return;
                    }
                    // Note: Actual file reading would require platform-specific implementation
                    // This is a placeholder for the drop handler
                },

                // Upload icon
                div {
                    style: "font-size: 48px; margin-bottom: 16px;",
                    "🖼️"
                }

                // Placeholder text
                p {
                    style: "margin: 0 0 8px 0; font-size: 16px; font-weight: 500; color: {theme.tokens.read().colors.foreground.to_rgba()}; text-align: center;",
                    "{props.placeholder}"
                }

                // Helper text
                if let Some(helper) = props.helper_text.clone() {
                    p {
                        style: "margin: 0 0 16px 0; font-size: 14px; color: {theme.tokens.read().colors.muted.to_rgba()}; text-align: center;",
                        "{helper}"
                    }
                }

                // File info
                p {
                    style: "margin: 0; font-size: 12px; color: {theme.tokens.read().colors.muted.to_rgba()}; text-align: center;",
                    "Max size: {props.max_size_mb} MB"
                    if let Some(max) = props.max_files {
                        " • Max files: {max}"
                    }
                }

                // File input
                label {
                    class: "image-uploader-button",
                    style: "display: inline-block; margin-top: 16px; padding: 10px 20px; font-size: 14px; font-weight: 500; color: white; background: {theme.tokens.read().colors.primary.to_rgba()}; border-radius: 8px; cursor: pointer; transition: opacity 0.15s ease;",

                    "Browse files"

                    input {
                        r#type: "file",
                        style: "display: none;",
                        accept: "{accept_string}",
                        multiple: props.multiple,
                        disabled: props.disabled,
                        onchange: move |_e: Event<FormData>| {
                            // Note: File reading is platform-specific
                            // This would typically use web_sys or native file APIs
                        },
                    }
                }
            }

            // Error messages
            if let Some(error) = props.error.clone() {
                p {
                    class: "image-uploader-error",
                    style: "margin: 4px 0 0 0; font-size: 14px; color: {theme.tokens.read().colors.destructive.to_rgba()};",
                    "{error}"
                }
            }
            if let Some(error) = validation_error() {
                p {
                    class: "image-uploader-validation-error",
                    style: "margin: 4px 0 0 0; font-size: 14px; color: {theme.tokens.read().colors.destructive.to_rgba()};",
                    "{error}"
                }
            }

            // Image preview grid
            if props.show_preview && !images().is_empty() {
                div {
                    class: "image-uploader-preview-grid",
                    style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(120px, 1fr)); gap: 12px; margin-top: 16px;",

                    for (index, image) in images().iter().enumerate() {
                        ImagePreviewItem {
                            key: "{image.id}",
                            image: image.clone(),
                            index: index,
                            on_remove: {
                                let mut remove_image = remove_image.clone();
                                move || remove_image(index)
                            },
                        }
                    }
                }
            }

            // Image list (when preview is disabled)
            if !props.show_preview && !images().is_empty() {
                div {
                    class: "image-uploader-list",
                    style: "display: flex; flex-direction: column; gap: 8px; margin-top: 16px;",

                    for (index, image) in images().iter().enumerate() {
                        ImageListItem {
                            key: "{image.id}",
                            image: image.clone(),
                            on_remove: {
                                let mut remove_image = remove_image.clone();
                                move || remove_image(index)
                            },
                        }
                    }
                }
            }
        }
    }
}

/// Image preview item properties
#[derive(Props, Clone, PartialEq)]
pub struct ImagePreviewItemProps {
    /// The uploaded image
    pub image: UploadedImage,
    /// Index in the list
    pub index: usize,
    /// Remove callback
    pub on_remove: EventHandler<()>,
}

/// Image preview thumbnail component
#[component]
fn ImagePreviewItem(props: ImagePreviewItemProps) -> Element {
    let theme = use_theme();
    let image = props.image.clone();

    let container_style = use_style(move |t| {
        Style::new()
            .relative()
            .rounded(&t.radius, "md")
            .overflow_hidden()
            .border(1, &t.colors.border)
            .build()
    });

    let status_indicator = match &image.status {
        UploadStatus::Pending => ("⏳", theme.tokens.read().colors.warning.to_rgba()),
        UploadStatus::Uploading => ("📤", theme.tokens.read().colors.primary.to_rgba()),
        UploadStatus::Success => ("✓", theme.tokens.read().colors.success.to_rgba()),
        UploadStatus::Error(_) => ("✕", theme.tokens.read().colors.destructive.to_rgba()),
    };

    rsx! {
        div {
            class: "image-preview-item",
            style: "{container_style} aspect-ratio: 1;",

            // Image thumbnail
            img {
                src: "{image.preview_url}",
                alt: "{image.file_name}",
                style: "width: 100%; height: 100%; object-fit: cover;",
            }

            // Status overlay for uploading/pending
            if matches!(image.status, UploadStatus::Uploading | UploadStatus::Pending) {
                div {
                    style: "position: absolute; inset: 0; background: rgba(0, 0, 0, 0.5); display: flex; align-items: center; justify-content: center;",
                    "{status_indicator.0}"
                }
            }

            // Progress bar
            if let Some(progress) = image.progress {
                div {
                    style: "position: absolute; bottom: 0; left: 0; right: 0; height: 4px; background: {theme.tokens.read().colors.muted.to_rgba()};",
                    div {
                        style: "height: 100%; width: {progress}%; background: {theme.tokens.read().colors.primary.to_rgba()}; transition: width 0.3s ease;",
                    }
                }
            }

            // Status badge
            div {
                style: "position: absolute; top: 4px; right: 4px; width: 20px; height: 20px; border-radius: 50%; background: {status_indicator.1}; display: flex; align-items: center; justify-content: center; font-size: 12px; color: white;",
                "{status_indicator.0}"
            }

            // Remove button
            button {
                r#type: "button",
                class: "image-preview-remove",
                style: "position: absolute; top: 4px; left: 4px; width: 24px; height: 24px; border-radius: 50%; background: rgba(0, 0, 0, 0.5); border: none; cursor: pointer; display: flex; align-items: center; justify-content: center; font-size: 14px; color: white; transition: background 0.15s ease;",
                onclick: move |_| props.on_remove.call(()),
                "✕"
            }

            // File name tooltip on hover
            div {
                class: "image-preview-tooltip",
                style: "position: absolute; bottom: 0; left: 0; right: 0; padding: 8px; background: linear-gradient(to top, rgba(0,0,0,0.8), transparent); font-size: 11px; color: white; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; opacity: 0; transition: opacity 0.15s ease;",
                style: ":hover {{ opacity: 1; }}",
                "{image.file_name}"
            }
        }
    }
}

/// Image list item properties
#[derive(Props, Clone, PartialEq)]
pub struct ImageListItemProps {
    /// The uploaded image
    pub image: UploadedImage,
    /// Remove callback
    pub on_remove: EventHandler<()>,
}

/// Image list item component (compact view)
#[component]
fn ImageListItem(props: ImageListItemProps) -> Element {
    let theme = use_theme();
    let image = props.image.clone();

    let container_style = use_style(move |t| {
        Style::new()
            .flex()
            .items_center()
            .gap_px(12)
            .p_px(12)
            .rounded(&t.radius, "md")
            .border(1, &t.colors.border)
            .bg(&t.colors.background)
            .build()
    });

    let status_icon = match &image.status {
        UploadStatus::Pending => "⏳",
        UploadStatus::Uploading => "📤",
        UploadStatus::Success => "✓",
        UploadStatus::Error(_) => "✕",
    };

    let status_color = match &image.status {
        UploadStatus::Pending => theme.tokens.read().colors.warning.to_rgba(),
        UploadStatus::Uploading => theme.tokens.read().colors.primary.to_rgba(),
        UploadStatus::Success => theme.tokens.read().colors.success.to_rgba(),
        UploadStatus::Error(_) => theme.tokens.read().colors.destructive.to_rgba(),
    };

    let size_text = format_file_size(image.size);

    rsx! {
        div {
            class: "image-list-item",
            style: "{container_style}",

            // Status icon
            div {
                style: "flex-shrink: 0; width: 32px; height: 32px; border-radius: 50%; background: {status_color}20; display: flex; align-items: center; justify-content: center; font-size: 14px; color: {status_color};",
                "{status_icon}"
            }

            // File info
            div {
                style: "flex: 1; min-width: 0;",

                p {
                    style: "margin: 0; font-size: 14px; font-weight: 500; color: {theme.tokens.read().colors.foreground.to_rgba()}; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                    "{image.file_name}"
                }

                p {
                    style: "margin: 4px 0 0 0; font-size: 12px; color: {theme.tokens.read().colors.muted.to_rgba()};",
                    "{size_text}"
                }

                // Progress bar
                if let Some(progress) = image.progress {
                    div {
                        style: "margin-top: 8px; height: 4px; background: {theme.tokens.read().colors.muted.to_rgba()}; border-radius: 2px; overflow: hidden;",
                        div {
                            style: "height: 100%; width: {progress}%; background: {theme.tokens.read().colors.primary.to_rgba()}; border-radius: 2px; transition: width 0.3s ease;",
                        }
                    }
                }

                // Error message
                if let UploadStatus::Error(msg) = &image.status {
                    p {
                        style: "margin: 4px 0 0 0; font-size: 12px; color: {theme.tokens.read().colors.destructive.to_rgba()};",
                        "{msg}"
                    }
                }
            }

            // Remove button
            button {
                r#type: "button",
                class: "image-list-remove",
                style: "flex-shrink: 0; background: none; border: none; cursor: pointer; font-size: 18px; color: {theme.tokens.read().colors.muted.to_rgba()}; padding: 4px; transition: color 0.15s ease;",
                onclick: move |_| props.on_remove.call(()),
                "✕"
            }
        }
    }
}

/// Format file size to human readable string
fn format_file_size(size: usize) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.1} {}", size, UNITS[unit_index])
}

// Placeholder for js_sys when not targeting wasm
#[cfg(not(target_arch = "wasm32"))]
mod js_sys {
    pub struct Date;
    impl Date {
        pub fn now() -> f64 {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as f64
        }
    }
}

/// Get current timestamp in milliseconds (platform-agnostic)
fn now_millis() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
