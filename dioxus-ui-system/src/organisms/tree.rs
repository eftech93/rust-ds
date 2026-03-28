//! Tree organism component
//!
//! A hierarchical tree view for displaying nested data with expand/collapse,
//! selection, and optional connector lines.

use crate::styles::Style;
use crate::theme::{use_style, use_theme};
use dioxus::prelude::*;

/// Tree node data structure for recursive tree representation
#[derive(Clone, PartialEq)]
pub struct TreeNodeData {
    /// Unique identifier for the node
    pub id: String,
    /// Display label for the node
    pub label: String,
    /// Optional icon name for the node
    pub icon: Option<String>,
    /// Child nodes (empty if leaf node)
    pub children: Vec<TreeNodeData>,
    /// Whether the node is disabled
    pub disabled: bool,
}

impl TreeNodeData {
    /// Create a new tree node
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            children: Vec::new(),
            disabled: false,
        }
    }

    /// Set the icon for the node
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Add a child node
    pub fn with_child(mut self, child: TreeNodeData) -> Self {
        self.children.push(child);
        self
    }

    /// Set multiple children at once
    pub fn with_children(mut self, children: Vec<TreeNodeData>) -> Self {
        self.children = children;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Check if this is a leaf node (no children)
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

/// Tree component properties
#[derive(Props, Clone, PartialEq)]
pub struct TreeProps {
    /// Tree node data
    pub data: Vec<TreeNodeData>,
    /// Currently selected node ID
    #[props(default)]
    pub selected_id: Option<String>,
    /// Callback when a node is selected
    #[props(default)]
    pub on_select: Option<EventHandler<String>>,
    /// List of expanded node IDs
    #[props(default)]
    pub expanded_ids: Vec<String>,
    /// Callback when a node's expand/collapse state changes
    #[props(default)]
    pub on_toggle_expand: Option<EventHandler<String>>,
    /// Whether to show connector lines between nodes
    #[props(default)]
    pub show_lines: bool,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
}

/// Tree organism component
///
/// A hierarchical tree view for displaying nested data.
///
/// # Example
/// ```rust,ignore
/// use dioxus_ui_system::organisms::{Tree, TreeNodeData};
///
/// fn App() -> Element {
///     let data = vec![
///         TreeNodeData::new("1", "Documents")
///             .with_icon("folder")
///             .with_child(
///                 TreeNodeData::new("1.1", "Work")
///                     .with_icon("folder")
///                     .with_child(TreeNodeData::new("1.1.1", "report.pdf").with_icon("file"))
///             ),
///     ];
///     
///     let expanded = vec!["1".to_string()];
///     
///     rsx! {
///         Tree {
///             data: data,
///             expanded_ids: expanded,
///             show_lines: true,
///         }
///     }
/// }
/// ```
#[component]
pub fn Tree(props: TreeProps) -> Element {
    let _theme = use_theme();

    let container_style = use_style(|t| {
        Style::new()
            .w_full()
            .flex()
            .flex_col()
            .text(&t.typography, "sm")
            .text_color(&t.colors.foreground)
            .build()
    });

    rsx! {
        div {
            style: "{container_style} {props.style.clone().unwrap_or_default()}",
            role: "tree",
            aria_multiselectable: "false",

            for node in &props.data {
                TreeNode {
                    key: "{node.id}",
                    node: node.clone(),
                    depth: 0,
                    selected_id: props.selected_id.clone(),
                    on_select: props.on_select.clone(),
                    expanded_ids: props.expanded_ids.clone(),
                    on_toggle_expand: props.on_toggle_expand.clone(),
                    show_lines: props.show_lines,
                }
            }
        }
    }
}

/// Internal properties for TreeNode component
#[derive(Props, Clone, PartialEq)]
struct TreeNodeProps {
    /// Node data
    node: TreeNodeData,
    /// Current depth level (for indentation)
    depth: usize,
    /// Currently selected node ID
    selected_id: Option<String>,
    /// Callback when node is selected
    on_select: Option<EventHandler<String>>,
    /// List of expanded node IDs
    expanded_ids: Vec<String>,
    /// Callback when expand/collapse is toggled
    on_toggle_expand: Option<EventHandler<String>>,
    /// Whether to show connector lines
    show_lines: bool,
}

/// Individual tree node component
#[component]
fn TreeNode(props: TreeNodeProps) -> Element {
    let _theme = use_theme();
    let mut is_hovered = use_signal(|| false);

    let node_id = props.node.id.clone();
    let is_expanded = props.expanded_ids.contains(&node_id);
    let is_selected = props.selected_id.as_ref() == Some(&node_id);
    let is_disabled = props.node.disabled;
    let has_children = !props.node.is_leaf();
    let depth = props.depth;

    // Indentation per level (in pixels)
    const INDENT_SIZE: usize = 20;

    let node_row_style = use_style(move |t| {
        let indent = depth * INDENT_SIZE;
        let mut base = Style::new()
            .w_full()
            .flex()
            .items_center()
            .gap_px(4)
            .py(&t.spacing, "xs")
            .px(&t.spacing, "sm")
            .rounded(&t.radius, "md")
            .cursor(if is_disabled {
                "not-allowed"
            } else {
                "pointer"
            })
            .transition("all 150ms ease")
            .opacity(if is_disabled { 0.5 } else { 1.0 });

        // Apply indentation using inline style for margin-left
        base.margin_left = Some(format!("{}px", indent));

        // Selection styling
        let base = if is_selected {
            base.bg(&t.colors.primary)
                .text_color(&t.colors.primary_foreground)
        } else if is_hovered() && !is_disabled {
            base.bg(&t.colors.muted).text_color(&t.colors.foreground)
        } else {
            base.bg_transparent().text_color(&t.colors.foreground)
        };

        base.build()
    });

    let children_container_style = use_style(|_| Style::new().w_full().flex().flex_col().build());

    let handle_select = {
        let node_id = node_id.clone();
        let on_select = props.on_select.clone();
        move |_| {
            if is_disabled {
                return;
            }
            if let Some(on_select) = &on_select {
                on_select.call(node_id.clone());
            }
        }
    };

    let handle_toggle = {
        let node_id = node_id.clone();
        let on_toggle = props.on_toggle_expand.clone();
        move |e: Event<MouseData>| {
            e.stop_propagation();
            if is_disabled {
                return;
            }
            if let Some(on_toggle) = &on_toggle {
                on_toggle.call(node_id.clone());
            }
        }
    };

    // Chevron rotation based on expanded state
    let chevron_rotation = if is_expanded { 90.0 } else { 0.0 };

    rsx! {
        div {
            role: "treeitem",
            aria_expanded: if has_children { Some(is_expanded.to_string()) } else { None },
            aria_selected: is_selected.to_string(),
            aria_disabled: is_disabled.to_string(),

            // Node row
            div {
                style: "{node_row_style}",
                onclick: handle_select,
                onmouseenter: move |_| if !is_disabled { is_hovered.set(true) },
                onmouseleave: move |_| is_hovered.set(false),

                // Expand/collapse chevron (only for nodes with children)
                if has_children {
                    button {
                        style: "display: inline-flex; align-items: center; justify-content: center; width: 16px; height: 16px; padding: 0; border: none; background: transparent; cursor: pointer; flex-shrink: 0;",
                        type: "button",
                        aria_label: if is_expanded { "Collapse" } else { "Expand" },
                        onclick: handle_toggle,

                        span {
                            style: "display: inline-flex; transform: rotate({chevron_rotation}deg); transition: transform 200ms ease;",
                            TreeChevron {}
                        }
                    }
                } else {
                    // Spacer for leaf nodes to align with expandable nodes
                    span { style: "width: 16px; flex-shrink: 0;" }
                }

                // Connector line (optional)
                if props.show_lines && depth > 0 {
                    TreeConnector { is_last: false }
                }

                // Node icon (if provided)
                if let Some(icon_name) = &props.node.icon {
                    span {
                        style: "display: inline-flex; flex-shrink: 0;",
                        TreeIcon { name: icon_name.clone() }
                    }
                }

                // Node label
                span {
                    style: "user-select: none; flex: 1;",
                    "{props.node.label}"
                }
            }

            // Children container
            if is_expanded && has_children {
                div {
                    style: "{children_container_style}",
                    role: "group",

                    for child in &props.node.children {
                        TreeNode {
                            key: "{child.id}",
                            node: child.clone(),
                            depth: depth + 1,
                            selected_id: props.selected_id.clone(),
                            on_select: props.on_select.clone(),
                            expanded_ids: props.expanded_ids.clone(),
                            on_toggle_expand: props.on_toggle_expand.clone(),
                            show_lines: props.show_lines,
                        }
                    }
                }
            }
        }
    }
}

/// Chevron icon for expand/collapse
#[component]
fn TreeChevron() -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            style: "width: 14px; height: 14px;",
            polyline { points: "9 18 15 12 9 6" }
        }
    }
}

/// Connector line component for tree visualization
#[component]
fn TreeConnector(is_last: bool) -> Element {
    let _theme = use_theme();
    let line_style = use_style(|t| Style::new().w_px(8).h_px(1).bg(&t.colors.border).build());

    rsx! {
        span { style: "{line_style}" }
    }
}

/// Tree icon component for node icons
#[derive(Props, Clone, PartialEq)]
struct TreeIconProps {
    /// Icon name or SVG path
    name: String,
}

#[component]
fn TreeIcon(props: TreeIconProps) -> Element {
    let svg_content = get_tree_icon_svg(&props.name);

    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            style: "width: 16px; height: 16px;",
            dangerous_inner_html: "{svg_content}",
        }
    }
}

/// Get SVG path data for preset tree icons
fn get_tree_icon_svg(name: &str) -> String {
    match name {
        "folder" => r#"<path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>"#,
        "folder-open" => r#"<path d="M6 17h12l2-9H8l-2 9z"/><path d="M2 17h20"/><path d="M2 8h20"/>"#,
        "file" => r#"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"/><polyline points="14 2 14 8 20 8"/>"#,
        "file-text" => r#"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/><line x1="10" y1="9" x2="8" y2="9"/>"#,
        "document" => r#"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"/><polyline points="14 2 14 8 20 8"/>"#,
        "image" => r#"<rect x="3" y="3" width="18" height="18" rx="2" ry="2"/><circle cx="8.5" cy="8.5" r="1.5"/><polyline points="21 15 16 10 5 21"/>"#,
        "video" => r#"<rect x="2" y="2" width="20" height="20" rx="2.18" ry="2.18"/><line x1="7" y1="2" x2="7" y2="22"/><line x1="17" y1="2" x2="17" y2="22"/><line x1="2" y1="12" x2="22" y2="12"/><line x1="2" y1="7" x2="7" y2="7"/><line x1="2" y1="17" x2="7" y2="17"/><line x1="17" y1="17" x2="22" y2="17"/><line x1="17" y1="7" x2="22" y2="7"/>"#,
        "music" => r#"<path d="M9 18V5l12-2v13"/><circle cx="6" cy="18" r="3"/><circle cx="18" cy="16" r="3"/>"#,
        "code" => r#"<polyline points="16 18 22 12 16 6"/><polyline points="8 6 2 12 8 18"/>"#,
        "database" => r#"<ellipse cx="12" cy="5" rx="9" ry="3"/><path d="M21 12c0 1.66-4 3-9 3s-9-1.34-9-3"/><path d="M3 5v14c0 1.66 4 3 9 3s9-1.34 9-3V5"/>"#,
        "box" => r#"<path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/><polyline points="3.27 6.96 12 12.01 20.73 6.96"/><line x1="12" y1="22.08" x2="12" y2="12"/>"#,
        "bookmark" => r#"<path d="m19 21-7-5-7 5V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16z"/>"#,
        "tag" => r#"<path d="M2 12V2h10l9 9-9 9-9-9z"/><circle cx="7" cy="7" r="2"/>"#,
        "star" => r#"<polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/>"#,
        "heart" => r#"<path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"/>"#,
        "user" => r#"<path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/>"#,
        "users" => r#"<path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M23 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/>"#,
        "home" => r#"<path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><polyline points="9 22 9 12 15 12 15 22"/>"#,
        "settings" => r#"<circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>"#,
        // Default: return as raw SVG path
        _ => name,
    }.to_string()
}

/// Tree node builder component with fluent API
#[derive(Props, Clone, PartialEq)]
pub struct TreeNodeBuilderProps {
    /// Initial node data
    #[props(default)]
    pub initial_data: Vec<TreeNodeData>,
    /// Callback when tree data changes
    #[props(default)]
    pub on_change: Option<EventHandler<Vec<TreeNodeData>>>,
}

/// Stateful tree component with built-in state management
///
/// This component manages its own state for selection and expansion,
/// making it easier to use when you don't need external control.
#[component]
pub fn TreeWithState(props: TreeProps) -> Element {
    // Use signals for internal state
    let internal_selected = use_signal(|| None::<String>);
    let internal_expanded = use_signal(|| Vec::<String>::new());

    // Determine which state to use - external (props) or internal (signals)
    let selected_id = props.selected_id.clone().or_else(|| internal_selected());

    let expanded_ids = if props.expanded_ids.is_empty() {
        internal_expanded()
    } else {
        props.expanded_ids.clone()
    };

    // Create event handlers that update internal state if no external handler provided
    let on_select: EventHandler<String> = if let Some(handler) = props.on_select.clone() {
        handler
    } else {
        let mut selected = internal_selected.clone();
        EventHandler::new(move |id: String| {
            selected.set(Some(id));
        })
    };

    let on_toggle_expand: EventHandler<String> =
        if let Some(handler) = props.on_toggle_expand.clone() {
            handler
        } else {
            let mut expanded = internal_expanded.clone();
            EventHandler::new(move |id: String| {
                expanded.with_mut(|exp| {
                    if exp.contains(&id) {
                        exp.retain(|x| x != &id);
                    } else {
                        exp.push(id);
                    }
                });
            })
        };

    rsx! {
        Tree {
            data: props.data.clone(),
            selected_id: selected_id,
            on_select: on_select,
            expanded_ids: expanded_ids,
            on_toggle_expand: on_toggle_expand,
            show_lines: props.show_lines,
            style: props.style.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_node_data_builder() {
        let node = TreeNodeData::new("1", "Root")
            .with_icon("folder")
            .with_child(TreeNodeData::new("1.1", "Child").with_icon("file"))
            .disabled(false);

        assert_eq!(node.id, "1");
        assert_eq!(node.label, "Root");
        assert_eq!(node.icon, Some("folder".to_string()));
        assert_eq!(node.children.len(), 1);
        assert!(!node.disabled);
    }

    #[test]
    fn test_tree_node_is_leaf() {
        let leaf = TreeNodeData::new("1", "Leaf");
        assert!(leaf.is_leaf());

        let parent = TreeNodeData::new("2", "Parent").with_child(TreeNodeData::new("2.1", "Child"));
        assert!(!parent.is_leaf());
    }
}
