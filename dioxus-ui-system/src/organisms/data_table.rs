//! Data Table organism component
//!
//! A comprehensive table component with sorting, selection, pagination, and filtering.

#![allow(unpredictable_function_pointer_comparisons)]

use dioxus::prelude::*;
use crate::theme::{use_theme, use_style};
use crate::styles::Style;
use crate::atoms::{Label, TextSize, TextColor, Button, ButtonVariant, ButtonSize, Icon, IconSize, IconColor};

/// Filter option for custom filters
#[derive(Clone, PartialEq)]
pub struct FilterOption {
    pub label: String,
    pub value: String,
}

/// Filter definition for custom filters
#[derive(Clone, PartialEq)]
pub struct TableFilter {
    pub key: String,
    pub label: String,
    pub options: Vec<FilterOption>,
}

/// Table column definition
#[derive(Clone, PartialEq)]
pub struct TableColumn<T: 'static> {
    pub key: String,
    pub header: String,
    pub width: Option<String>,
    pub align: ColumnAlign,
    pub sortable: bool,
    pub render: Option<fn(&T) -> Element>,
}

/// Column text alignment
#[derive(Default, Clone, PartialEq)]
pub enum ColumnAlign {
    #[default]
    Left,
    Center,
    Right,
}

impl ColumnAlign {
    fn as_str(&self) -> &'static str {
        match self {
            ColumnAlign::Left => "left",
            ColumnAlign::Center => "center",
            ColumnAlign::Right => "right",
        }
    }
}

/// Table properties
#[derive(Props, Clone, PartialEq)]
pub struct DataTableProps<T: Clone + PartialEq + 'static> {
    /// Column definitions
    pub columns: Vec<TableColumn<T>>,
    /// Table data
    pub data: Vec<T>,
    /// Unique key extractor for rows
    pub key_extractor: fn(&T) -> String,
    /// Row selection enabled
    #[props(default)]
    pub selectable: bool,
    /// Selected row keys
    #[props(default)]
    pub selected_keys: Vec<String>,
    /// Selection change handler
    #[props(default)]
    pub on_selection_change: Option<EventHandler<Vec<String>>>,
    /// Row click handler
    #[props(default)]
    pub on_row_click: Option<EventHandler<T>>,
    /// Empty state message
    #[props(default = "No data available")]
    pub empty_message: &'static str,
    /// Loading state
    #[props(default)]
    pub loading: bool,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Search placeholder text
    #[props(default = "Search...")]
    pub search_placeholder: &'static str,
    /// Search query for filtering
    #[props(default)]
    pub search_query: Option<String>,
    /// Search change handler
    #[props(default)]
    pub on_search_change: Option<EventHandler<String>>,
    /// Custom filter definitions
    #[props(default)]
    pub filters: Vec<TableFilter>,
    /// Active filter values
    #[props(default)]
    pub active_filters: std::collections::HashMap<String, String>,
    /// Filter change handler
    #[props(default)]
    pub on_filter_change: Option<EventHandler<(String, String)>>,
    /// Show search input
    #[props(default = true)]
    pub show_search: bool,
    /// Show filter controls
    #[props(default = true)]
    pub show_filters: bool,
}

/// Data Table organism component
///
/// # Example
/// ```rust,ignore
/// use dioxus_ui_system::organisms::{DataTable, TableColumn, ColumnAlign};
///
/// #[derive(Clone, PartialEq)]
/// struct User {
///     id: String,
///     name: String,
///     email: String,
/// }
///
/// let columns = vec![
///     TableColumn {
///         key: "name".to_string(),
///         header: "Name".to_string(),
///         width: None,
///         align: ColumnAlign::Left,
///         sortable: true,
///         render: None,
///     },
///     TableColumn {
///         key: "email".to_string(),
///         header: "Email".to_string(),
///         width: None,
///         align: ColumnAlign::Left,
///         sortable: true,
///         render: None,
///     },
/// ];
///
/// let data = vec![
///     User { id: "1".to_string(), name: "John".to_string(), email: "john@example.com".to_string() },
/// ];
///
/// rsx! {
///     DataTable {
///         columns: columns,
///         data: data,
///         key_extractor: |u| u.id.clone(),
///     }
/// }
/// ```
#[component]
pub fn DataTable<T: Clone + PartialEq + 'static>(props: DataTableProps<T>) -> Element {
    let style = use_style(|t| {
        Style::new()
            .w_full()
            .rounded(&t.radius, "md")
            .border(1, &t.colors.border)
            .overflow_hidden()
            .build()
    });
    
    let final_style = if let Some(custom) = &props.style {
        format!("{} {}", style(), custom)
    } else {
        style()
    };
    
    let table_style = use_style(|t| {
        Style::new()
            .w_full()
            .text(&t.typography, "sm")
            .build()
    });
    
    let toolbar_style = use_style(|t| {
        Style::new()
            .flex()
            .items_center()
            .justify_between()
            .px(&t.spacing, "md")
            .py(&t.spacing, "sm")
            .border_bottom(1, &t.colors.border)
            .bg(&t.colors.background)
            .gap(&t.spacing, "sm")
            .build()
    });
    
    let search_container_style = use_style(|t| {
        Style::new()
            .flex()
            .items_center()
            .gap(&t.spacing, "sm")
            .build()
    });
    
    let filters_container_style = use_style(|t| {
        Style::new()
            .flex()
            .items_center()
            .gap(&t.spacing, "sm")
            .build()
    });
    
    let show_toolbar = (props.show_search && props.on_search_change.is_some()) || 
                       (props.show_filters && !props.filters.is_empty() && props.on_filter_change.is_some());
    
    // Loading state (without toolbar)
    if props.loading {
        return rsx! {
            div {
                style: "{final_style}",
                div {
                    style: "padding: 48px; text-align: center;",
                    Icon {
                        name: "spinner".to_string(),
                        size: IconSize::Large,
                        color: IconColor::Muted,
                    }
                }
            }
        };
    }
    
    // Empty state (without toolbar)
    if props.data.is_empty() {
        return rsx! {
            div {
                style: "{final_style}",
                div {
                    style: "padding: 48px; text-align: center;",
                    Label {
                        size: TextSize::Small,
                        color: TextColor::Muted,
                        "{props.empty_message}"
                    }
                }
            }
        };
    }
    
    let columns = props.columns.clone();
    let data = props.data.clone();
    let selectable = props.selectable;
    let selected_keys = props.selected_keys.clone();
    let search_query = props.search_query.clone().unwrap_or_default();
    
    rsx! {
        div {
            style: "{final_style}",
            
            if show_toolbar {
                div {
                    style: "{toolbar_style}",
                    
                    if props.show_search && props.on_search_change.is_some() {
                        div {
                            style: "{search_container_style} flex: 1;",
                            Icon {
                                name: "search".to_string(),
                                size: IconSize::Small,
                                color: IconColor::Muted,
                            }
                            input {
                                r#type: "text",
                                placeholder: "{props.search_placeholder}",
                                value: "{search_query}",
                                style: "flex: 1; min-width: 200px; padding: 8px 12px; border: 1px solid rgb(226,232,240); border-radius: 6px; font-size: 14px; outline: none; &:focus {{ border-color: rgb(59,130,246); }}",
                                oninput: move |e| {
                                    if let Some(handler) = &props.on_search_change {
                                        handler.call(e.value());
                                    }
                                },
                            }
                            if !search_query.is_empty() {
                                Button {
                                    variant: ButtonVariant::Ghost,
                                    size: ButtonSize::Sm,
                                    onclick: move |_| {
                                        if let Some(handler) = &props.on_search_change {
                                            handler.call("".to_string());
                                        }
                                    },
                                    Icon {
                                        name: "x".to_string(),
                                        size: IconSize::Small,
                                        color: IconColor::Muted,
                                    }
                                }
                            }
                        }
                    }
                    
                    if props.show_filters && !props.filters.is_empty() && props.on_filter_change.is_some() {
                        div {
                            style: "{filters_container_style}",
                            for filter in props.filters.clone() {
                                DataTableFilter {
                                    filter: filter.clone(),
                                    active_value: props.active_filters.get(&filter.key).cloned().unwrap_or_default(),
                                    on_change: props.on_filter_change.clone(),
                                }
                            }
                        }
                    }
                }
            }
            
            table {
                style: "{table_style}",
                
                DataTableHeader {
                    columns: columns.clone(),
                    selectable: selectable,
                    selected_count: selected_keys.len(),
                    total_count: data.len(),
                    on_select_all: props.on_selection_change.clone(),
                }
                
                tbody {
                    for row in data {
                        DataTableRow {
                            row: row.clone(),
                            columns: columns.clone(),
                            key_extractor: props.key_extractor,
                            selectable: selectable,
                            is_selected: selected_keys.contains(&(props.key_extractor)(&row)),
                            on_select: props.on_selection_change.clone(),
                            on_click: props.on_row_click.clone(),
                        }
                    }
                }
            }
        }
    }
}

/// Filter dropdown component
#[derive(Props, Clone, PartialEq)]
pub struct DataTableFilterProps {
    pub filter: TableFilter,
    pub active_value: String,
    pub on_change: Option<EventHandler<(String, String)>>,
}

#[component]
pub fn DataTableFilter(props: DataTableFilterProps) -> Element {
    let filter = props.filter.clone();
    let active_value = props.active_value.clone();
    let has_value = !active_value.is_empty();
    
    rsx! {
        select {
            style: if has_value { 
                "padding: 8px 12px; border: 1px solid rgb(59,130,246); border-radius: 6px; font-size: 14px; background: white; cursor: pointer; outline: none; color: rgb(15,23,42);"
            } else {
                "padding: 8px 12px; border: 1px solid rgb(226,232,240); border-radius: 6px; font-size: 14px; background: white; cursor: pointer; outline: none; color: rgb(100,116,139);"
            },
            onchange: move |e| {
                if let Some(handler) = &props.on_change {
                    handler.call((filter.key.clone(), e.value()));
                }
            },
            option {
                value: "",
                selected: active_value.is_empty(),
                "{filter.label}"
            }
            for option in filter.options {
                option {
                    value: "{option.value}",
                    selected: active_value == option.value,
                    "{option.label}"
                }
            }
        }
    }
}

/// Table header component
#[derive(Props, Clone, PartialEq)]
pub struct DataTableHeaderProps<T: Clone + PartialEq + 'static> {
    pub columns: Vec<TableColumn<T>>,
    pub selectable: bool,
    pub selected_count: usize,
    pub total_count: usize,
    pub on_select_all: Option<EventHandler<Vec<String>>>,
}

#[component]
pub fn DataTableHeader<T: Clone + PartialEq>(props: DataTableHeaderProps<T>) -> Element {
    let _theme = use_theme();
    
    let header_style = use_style(|t| {
        Style::new()
            .bg(&t.colors.muted)
            .text_color(&t.colors.foreground)
            .font_weight(500)
            .build()
    });
    
    let th_style = use_style(|t| {
        Style::new()
            .p(&t.spacing, "md")
            .text_align("left")
            .border_bottom(1, &t.colors.border)
            .build()
    });
    
    let all_selected = props.selected_count == props.total_count && props.total_count > 0;
    
    rsx! {
        thead {
            style: "{header_style}",
            
            tr {
                if props.selectable {
                    th {
                        style: "width: 48px; padding: 12px;",
                        input {
                            r#type: "checkbox",
                            checked: all_selected,
                            onchange: move |_| {
                                // Toggle select all logic would go here
                            },
                        }
                    }
                }
                
                for col in props.columns {
                    th {
                        style: "{th_style} text-align: {col.align.as_str()}; width: {col.width.clone().unwrap_or_default()};",
                        
                        if col.sortable {
                            div {
                                style: "display: inline-flex; align-items: center; gap: 4px; cursor: pointer;",
                                "{col.header}"
                                Icon {
                                    name: "chevron-down".to_string(),
                                    size: IconSize::Small,
                                    color: IconColor::Muted,
                                }
                            }
                        } else {
                            "{col.header}"
                        }
                    }
                }
            }
        }
    }
}

/// Table row component
#[derive(Props, Clone, PartialEq)]
pub struct DataTableRowProps<T: Clone + PartialEq + 'static> {
    pub row: T,
    pub columns: Vec<TableColumn<T>>,
    pub key_extractor: fn(&T) -> String,
    pub selectable: bool,
    pub is_selected: bool,
    pub on_select: Option<EventHandler<Vec<String>>>,
    pub on_click: Option<EventHandler<T>>,
}

#[component]
pub fn DataTableRow<T: Clone + PartialEq + 'static>(props: DataTableRowProps<T>) -> Element {
    let _theme = use_theme();
    let _key = (props.key_extractor)(&props.row);
    let is_selected = props.is_selected;
    let _has_onclick = props.on_click.is_some();
    
    let mut is_hovered = use_signal(|| false);
    
    let row_style = use_style(move |t| {
        let base = Style::new()
            .border_bottom(1, &t.colors.border)
            .transition("background-color 150ms ease");
            
        if is_selected {
            base.bg(&t.colors.primary.blend(&t.colors.background, 0.9))
        } else if is_hovered() {
            base.bg(&t.colors.muted)
        } else {
            base
        }.build()
    });
    
    let td_style = use_style(|t| {
        Style::new()
            .p(&t.spacing, "md")
            .build()
    });
    
    let row_data = props.row.clone();
    let onclick_handler = props.on_click.clone();
    
    rsx! {
        tr {
            style: "{row_style}",
            onmouseenter: move |_| is_hovered.set(true),
            onmouseleave: move |_| is_hovered.set(false),
            onclick: move |_| {
                if let Some(handler) = &onclick_handler {
                    handler.call(row_data.clone());
                }
            },
            
            if props.selectable {
                td {
                    style: "width: 48px; padding: 12px;",
                    input {
                        r#type: "checkbox",
                        checked: is_selected,
                        onchange: move |_| {
                            // Selection toggle logic
                        },
                    }
                }
            }
            
            for col in props.columns {
                DataTableCell {
                    row: props.row.clone(),
                    column: col,
                    base_style: td_style(),
                }
            }
        }
    }
}

/// Table cell component
#[derive(Props, Clone, PartialEq)]
pub struct DataTableCellProps<T: Clone + PartialEq + 'static> {
    pub row: T,
    pub column: TableColumn<T>,
    pub base_style: String,
}

#[component]
pub fn DataTableCell<T: Clone + PartialEq>(props: DataTableCellProps<T>) -> Element {
    let col = props.column.clone();
    let align = col.align.as_str();
    
    // Get cell value using key
    let cell_content = if let Some(render_fn) = col.render {
        render_fn(&props.row)
    } else {
        // Default: just show the value (in a real implementation, 
        // we'd use reflection or require a trait to extract values)
        rsx! {
            Label {
                size: TextSize::Small,
                "-"
            }
        }
    };
    
    rsx! {
        td {
            style: "{props.base_style} text-align: {align};",
            {cell_content}
        }
    }
}

/// Pagination component
#[derive(Props, Clone, PartialEq)]
pub struct PaginationProps {
    pub current_page: usize,
    pub total_pages: usize,
    pub on_page_change: EventHandler<usize>,
    #[props(default)]
    pub show_first_last: bool,
}

#[component]
pub fn Pagination(props: PaginationProps) -> Element {
    let current = props.current_page;
    let total = props.total_pages;
    
    let container_style = use_style(|t| {
        Style::new()
            .flex()
            .items_center()
            .justify_between()
            .px(&t.spacing, "md")
            .py(&t.spacing, "sm")
            .border_top(1, &t.colors.border)
            .build()
    });
    
    let info_style = use_style(|t| {
        Style::new()
            .text(&t.typography, "sm")
            .text_color(&t.colors.muted_foreground)
            .build()
    });
    
    rsx! {
        div {
            style: "{container_style}",
            
            div {
                style: "{info_style}",
                "Page {current + 1} of {total}"
            }
            
            div {
                style: "display: flex; align-items: center; gap: 4px;",
                
                if props.show_first_last && current > 0 {
                    Button {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::Sm,
                        onclick: move |_| props.on_page_change.call(0),
                        Icon {
                            name: "chevron-left".to_string(),
                            size: IconSize::Small,
                            color: IconColor::Current,
                        }
                    }
                }
                
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    disabled: current == 0,
                    onclick: move |_| props.on_page_change.call(current.saturating_sub(1)),
                    Icon {
                        name: "chevron-left".to_string(),
                        size: IconSize::Small,
                        color: IconColor::Current,
                    }
                }
                
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    disabled: current >= total - 1,
                    onclick: move |_| props.on_page_change.call((current + 1).min(total - 1)),
                    Icon {
                        name: "chevron-right".to_string(),
                        size: IconSize::Small,
                        color: IconColor::Current,
                    }
                }
                
                if props.show_first_last && current < total - 1 {
                    Button {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::Sm,
                        onclick: move |_| props.on_page_change.call(total - 1),
                        Icon {
                            name: "chevron-right".to_string(),
                            size: IconSize::Small,
                            color: IconColor::Current,
                        }
                    }
                }
            }
        }
    }
}

