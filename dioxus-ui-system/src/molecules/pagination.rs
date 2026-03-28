//! Pagination molecule component
//!
//! Page navigation for lists and tables.

use crate::theme::use_theme;
use dioxus::prelude::*;

/// Pagination size
#[derive(Default, Clone, PartialEq, Debug)]
pub enum PaginationSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Pagination properties
#[derive(Props, Clone, PartialEq)]
pub struct PaginationProps {
    /// Total number of pages
    pub total_pages: u32,
    /// Current page (1-indexed)
    #[props(default = 1)]
    pub current_page: u32,
    /// Page change handler
    pub on_change: EventHandler<u32>,
    /// Number of sibling pages to show
    #[props(default = 1)]
    pub sibling_count: u32,
    /// Show first/last buttons
    #[props(default = true)]
    pub show_first_last: bool,
    /// Show previous/next buttons
    #[props(default = true)]
    pub show_prev_next: bool,
    /// Simple mode (prev/next only)
    #[props(default = false)]
    pub simple: bool,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Pagination component
#[component]
pub fn Pagination(props: PaginationProps) -> Element {
    let theme = use_theme();

    let class_css = props
        .class
        .as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();

    let current = props.current_page.max(1).min(props.total_pages);
    let bg_color = theme.tokens.read().colors.primary.to_rgba();
    let muted_color = theme.tokens.read().colors.muted.to_rgba();
    let text_color = theme.tokens.read().colors.foreground.to_rgba();
    let border_color = theme.tokens.read().colors.border.to_rgba();

    // Generate page numbers to display
    let pages = generate_pages(current, props.total_pages, props.sibling_count);
    let total_pages = props.total_pages;

    rsx! {
        nav {
            class: "pagination{class_css}",
            style: "display: flex; align-items: center; gap: 4px;",
            aria_label: "Pagination",

            if props.simple {
                // Simple mode: just prev/next
                button {
                    type: "button",
                    class: "pagination-prev",
                    style: format!("padding: 8px 12px; font-size: 14px; background: white; color: {text_color}; border: 1px solid {border_color}; border-radius: 6px; cursor: pointer; opacity: {};", if current == 1 { "0.5" } else { "1" }),
                    disabled: current == 1,
                    onclick: move |_| {
                        if current > 1 {
                            props.on_change.call(current - 1);
                        }
                    },
                    "← Previous"
                }

                span {
                    class: "pagination-info",
                    style: "padding: 0 12px; font-size: 14px; color: {muted_color};",
                    "Page {current} of {total_pages}"
                }

                button {
                    type: "button",
                    class: "pagination-next",
                    style: format!("padding: 8px 12px; font-size: 14px; background: white; color: {text_color}; border: 1px solid {border_color}; border-radius: 6px; cursor: pointer; opacity: {};", if current == props.total_pages { "0.5" } else { "1" }),
                    disabled: current == props.total_pages,
                    onclick: move |_| {
                        if current < props.total_pages {
                            props.on_change.call(current + 1);
                        }
                    },
                    "Next →"
                }
            } else {
                // Full pagination

                // First page button
                if props.show_first_last && props.total_pages > 1 {
                    button {
                        type: "button",
                        class: "pagination-first",
                        style: format!("padding: 8px 12px; font-size: 14px; background: white; color: {text_color}; border: 1px solid {border_color}; border-radius: 6px; cursor: pointer; opacity: {}; min-width: 36px;", if current == 1 { "0.5" } else { "1" }),
                        disabled: current == 1,
                        onclick: move |_| props.on_change.call(1),
                        "«"
                    }
                }

                // Previous button
                if props.show_prev_next {
                    button {
                        type: "button",
                        class: "pagination-prev",
                        style: format!("padding: 8px 12px; font-size: 14px; background: white; color: {text_color}; border: 1px solid {border_color}; border-radius: 6px; cursor: pointer; opacity: {}; min-width: 36px;", if current == 1 { "0.5" } else { "1" }),
                        opacity: if current == 1 { "0.5" } else { "1" },
                        disabled: current == 1,
                        onclick: move |_| {
                            if current > 1 {
                                props.on_change.call(current - 1);
                            }
                        },
                        "‹"
                    }
                }

                // Page numbers
                for (idx, item) in pages.iter().enumerate() {
                    {
                        let idx = idx;
                        match item {
                            PageItem::Number(page) => {
                                let is_active = *page == current;
                                let btn_bg = if is_active { bg_color.clone() } else { "white".to_string() };
                                let btn_color = if is_active { "white".to_string() } else { text_color.clone() };
                                let btn_border = if is_active { bg_color.clone() } else { border_color.clone() };
                                let page_num = *page;

                                rsx! {
                                    button {
                                        key: "page-{page_num}",
                                        type: "button",
                                        class: if is_active { "pagination-active" } else { "" },
                                        style: "padding: 8px 12px; font-size: 14px; background: {btn_bg}; color: {btn_color}; border: 1px solid {btn_border}; border-radius: 6px; cursor: pointer; min-width: 36px; transition: all 0.15s ease;",
                                        aria_current: if is_active { "page" } else { "" },
                                        onclick: move |_| props.on_change.call(page_num),
                                        "{page_num}"
                                    }
                                }
                            }
                            PageItem::Ellipsis => {
                                rsx! {
                                    span {
                                        key: "ellipsis-{idx}",
                                        class: "pagination-ellipsis",
                                        style: "padding: 8px 12px; font-size: 14px; color: {muted_color};",
                                        "…"
                                    }
                                }
                            }
                        }
                    }
                }

                // Next button
                if props.show_prev_next {
                    button {
                        type: "button",
                        class: "pagination-next",
                        style: format!("padding: 8px 12px; font-size: 14px; background: white; color: {text_color}; border: 1px solid {border_color}; border-radius: 6px; cursor: pointer; opacity: {}; min-width: 36px;", if current == props.total_pages { "0.5" } else { "1" }),
                        disabled: current == props.total_pages,
                        onclick: move |_| {
                            if current < props.total_pages {
                                props.on_change.call(current + 1);
                            }
                        },
                        "›"
                    }
                }

                // Last page button
                if props.show_first_last && props.total_pages > 1 {
                    button {
                        type: "button",
                        class: "pagination-last",
                        style: format!("padding: 8px 12px; font-size: 14px; background: white; color: {text_color}; border: 1px solid {border_color}; border-radius: 6px; cursor: pointer; opacity: {}; min-width: 36px;", if current == props.total_pages { "0.5" } else { "1" }),
                        disabled: current == props.total_pages,
                        onclick: move |_| props.on_change.call(props.total_pages),
                        "»"
                    }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
enum PageItem {
    Number(u32),
    Ellipsis,
}

fn generate_pages(current: u32, total: u32, sibling_count: u32) -> Vec<PageItem> {
    let mut pages = Vec::new();

    if total <= 7 {
        // Show all pages
        for i in 1..=total {
            pages.push(PageItem::Number(i));
        }
    } else {
        // Complex case with ellipsis
        let left_sibling = current.saturating_sub(sibling_count).max(1);
        let right_sibling = (current + sibling_count).min(total);

        let show_left_ellipsis = left_sibling > 2;
        let show_right_ellipsis = right_sibling < total - 1;

        // Always show first page
        pages.push(PageItem::Number(1));

        if show_left_ellipsis {
            pages.push(PageItem::Ellipsis);
        } else if left_sibling > 1 {
            pages.push(PageItem::Number(2));
        }

        // Middle pages
        for i in left_sibling..=right_sibling {
            if i > 1 && i < total {
                pages.push(PageItem::Number(i));
            }
        }

        if show_right_ellipsis {
            pages.push(PageItem::Ellipsis);
        } else if right_sibling < total {
            pages.push(PageItem::Number(total - 1));
        }

        // Always show last page
        if total > 1 {
            pages.push(PageItem::Number(total));
        }
    }

    pages
}

/// Page size selector properties
#[derive(Props, Clone, PartialEq)]
pub struct PageSizeSelectorProps {
    /// Available page sizes
    #[props(default = vec![10, 25, 50, 100])]
    pub options: Vec<u32>,
    /// Current page size
    pub value: u32,
    /// Change handler
    pub on_change: EventHandler<u32>,
    /// Label
    #[props(default)]
    pub label: Option<String>,
    /// Per page suffix
    #[props(default)]
    pub suffix: Option<String>,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Page size selector component
#[component]
pub fn PageSizeSelector(props: PageSizeSelectorProps) -> Element {
    let class_css = props
        .class
        .as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();

    rsx! {
        div {
            class: "page-size-selector{class_css}",
            style: "display: flex; align-items: center; gap: 8px; font-size: 14px;",

            if let Some(label) = props.label.clone() {
                span {
                    "{label}"
                }
            }

            select {
                class: "page-size-select",
                style: "padding: 6px 10px; border: 1px solid #e2e8f0; border-radius: 6px; font-size: 14px; background: white; cursor: pointer;",
                onchange: move |e: Event<FormData>| {
                    if let Ok(val) = e.value().parse::<u32>() {
                        props.on_change.call(val);
                    }
                },

                for size in props.options.iter() {
                    option {
                        key: "{size}",
                        value: "{size}",
                        selected: *size == props.value,
                        "{size}"
                    }
                }
            }

            if let Some(suffix) = props.suffix.clone() {
                span {
                    "{suffix}"
                }
            }
        }
    }
}

/// Pagination info component
#[derive(Props, Clone, PartialEq)]
pub struct PaginationInfoProps {
    /// Current page
    pub current_page: u32,
    /// Page size
    pub page_size: u32,
    /// Total items
    pub total_items: u32,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Pagination info (showing X-Y of Z results)
#[component]
pub fn PaginationInfo(props: PaginationInfoProps) -> Element {
    let theme = use_theme();

    let class_css = props
        .class
        .as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();

    let start = ((props.current_page - 1) * props.page_size) + 1;
    let end = (props.current_page * props.page_size).min(props.total_items);
    let total_items = props.total_items;
    let muted_color = theme.tokens.read().colors.muted.to_rgba();

    rsx! {
        div {
            class: "pagination-info{class_css}",
            style: "font-size: 14px; color: {muted_color};",
            "Showing {start}–{end} of {total_items} results"
        }
    }
}
