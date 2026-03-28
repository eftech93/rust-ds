//! Kanban Board organism component
//!
//! A drag-and-drop style board with columns and cards for task management.

use crate::atoms::{Button, ButtonSize, ButtonVariant, Icon, IconColor, IconSize, Label, TextSize};
use crate::styles::Style;
use crate::theme::{use_style, use_theme};
use dioxus::prelude::*;

/// Kanban card data
#[derive(Clone, PartialEq, Debug)]
pub struct KanbanCard {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub assignee: Option<String>,
    pub due_date: Option<String>,
}

impl KanbanCard {
    /// Create a new kanban card
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: None,
            tags: Vec::new(),
            assignee: None,
            due_date: None,
        }
    }

    /// Set description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set tags
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// Set assignee (avatar URL or initials)
    pub fn with_assignee(mut self, assignee: impl Into<String>) -> Self {
        self.assignee = Some(assignee.into());
        self
    }

    /// Set due date
    pub fn with_due_date(mut self, due_date: impl Into<String>) -> Self {
        self.due_date = Some(due_date.into());
        self
    }
}

/// Kanban column data
#[derive(Clone, PartialEq, Debug)]
pub struct KanbanColumn {
    pub id: String,
    pub title: String,
    pub cards: Vec<KanbanCard>,
    pub color: Option<String>,
}

impl KanbanColumn {
    /// Create a new kanban column
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            cards: Vec::new(),
            color: None,
        }
    }

    /// Set cards
    pub fn with_cards(mut self, cards: Vec<KanbanCard>) -> Self {
        self.cards = cards;
        self
    }

    /// Set column color (accent color for header)
    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Add a card to the column
    pub fn add_card(&mut self, card: KanbanCard) {
        self.cards.push(card);
    }
}

/// Kanban board properties
#[derive(Props, Clone, PartialEq)]
pub struct KanbanProps {
    /// Columns with their cards
    pub columns: Vec<KanbanColumn>,
    /// Handler for column changes (new column order or content)
    #[props(default)]
    pub on_columns_change: Option<EventHandler<Vec<KanbanColumn>>>,
    /// Handler for card moves (card_id, column_id, new_index)
    #[props(default)]
    pub on_card_move: Option<EventHandler<(String, String, usize)>>,
    /// Handler for card clicks
    #[props(default)]
    pub on_card_click: Option<EventHandler<String>>,
    /// Allow adding new columns
    #[props(default = false)]
    pub allow_add_column: bool,
    /// Handler for add column button
    #[props(default)]
    pub on_add_column: Option<EventHandler<()>>,
    /// Handler for add card in a column (column_id)
    #[props(default)]
    pub on_add_card: Option<EventHandler<String>>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    /// Column width (default: 280px)
    #[props(default = "280px".to_string())]
    pub column_width: String,
    /// Board height (default: 100%)
    #[props(default = "100%".to_string())]
    pub height: String,
    /// Show card count in column headers
    #[props(default = true)]
    pub show_card_count: bool,
    /// Enable card hover effects
    #[props(default = true)]
    pub card_hover: bool,
}

/// Kanban board component
///
/// # Example
/// ```rust,ignore
/// use dioxus_ui_system::organisms::{Kanban, KanbanColumn, KanbanCard};
///
/// let columns = vec![
///     KanbanColumn::new("todo", "To Do")
///         .with_color("#ef4444")
///         .with_cards(vec![
///             KanbanCard::new("1", "Task 1")
///                 .with_description("Description here")
///                 .with_tags(vec!["urgent".to_string()])
///                 .with_assignee("JD"),
///         ]),
///     KanbanColumn::new("done", "Done"),
/// ];
///
/// rsx! {
///     Kanban {
///         columns: columns,
///         on_card_click: |card_id| println!("Clicked: {}", card_id),
///     }
/// }
/// ```
#[component]
pub fn Kanban(props: KanbanProps) -> Element {
    let _theme = use_theme();
    let height = props.height.clone();

    let board_style = use_style(move |t| {
        Style::new()
            .flex()
            .flex_row()
            .gap(&t.spacing, "md")
            .overflow_x_auto()
            .p(&t.spacing, "md")
            .h(&height)
            .build()
    });

    let final_style = if let Some(custom) = &props.style {
        format!("{} {}", board_style(), custom)
    } else {
        board_style()
    };

    let columns = props.columns.clone();
    let column_width = props.column_width.clone();

    rsx! {
        div {
            style: "{final_style}",

            for column in columns {
                KanbanColumnView {
                    key: "{column.id}",
                    column: column,
                    width: column_width.clone(),
                    on_card_click: props.on_card_click,
                    on_add_card: props.on_add_card,
                    show_card_count: props.show_card_count,
                    card_hover: props.card_hover,
                }
            }

            if props.allow_add_column {
                AddColumnButton {
                    on_add_column: props.on_add_column,
                }
            }
        }
    }
}

/// Individual kanban column component
#[derive(Props, Clone, PartialEq)]
pub struct KanbanColumnViewProps {
    pub column: KanbanColumn,
    pub width: String,
    pub on_card_click: Option<EventHandler<String>>,
    pub on_add_card: Option<EventHandler<String>>,
    pub show_card_count: bool,
    pub card_hover: bool,
}

#[component]
pub fn KanbanColumnView(props: KanbanColumnViewProps) -> Element {
    let theme = use_theme();
    let width = props.width.clone();

    let column_style = use_style(move |t| {
        Style::new()
            .flex()
            .flex_col()
            .rounded(&t.radius, "lg")
            .bg(&t.colors.muted)
            .min_w(&width)
            .max_w(&width)
            .h_full()
            .build()
    });

    let header_style = use_style(|t| {
        Style::new()
            .flex()
            .items_center()
            .justify_between()
            .p(&t.spacing, "md")
            .border_bottom(1, &t.colors.border)
            .build()
    });

    let title_style = use_style(|t| {
        Style::new()
            .flex()
            .items_center()
            .gap(&t.spacing, "sm")
            .font_weight(600)
            .text(&t.typography, "sm")
            .build()
    });

    let cards_container_style = use_style(|t| {
        Style::new()
            .flex()
            .flex_col()
            .gap(&t.spacing, "sm")
            .p(&t.spacing, "md")
            .flex_grow(1)
            .overflow_y_auto()
            .build()
    });

    let column = props.column.clone();
    let card_count = column.cards.len();
    let accent_color = column.color.clone();
    let column_id = column.id.clone();

    rsx! {
        div {
            style: "{column_style}",

            // Column Header
            div {
                style: "{header_style}",

                div {
                    style: "{title_style}",

                    if let Some(color) = accent_color {
                        div {
                            style: "width: 12px; height: 12px; border-radius: 50%; background: {color}; flex-shrink: 0;",
                        }
                    }

                    span {
                        "{column.title}"
                    }

                    if props.show_card_count {
                        span {
                            style: "background: {theme.tokens.read().colors.muted_foreground.to_rgba()}20; color: {theme.tokens.read().colors.muted_foreground.to_rgba()}; padding: 2px 8px; border-radius: 9999px; font-size: 12px; font-weight: 500;",
                            "{card_count}"
                        }
                    }
                }

                // Column actions menu placeholder
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    Icon {
                        name: "more-horizontal".to_string(),
                        size: IconSize::Small,
                        color: IconColor::Muted,
                    }
                }
            }

            // Cards Container
            div {
                style: "{cards_container_style}",

                for card in column.cards {
                    KanbanCardView {
                        key: "{card.id}",
                        card: card,
                        on_click: props.on_card_click.clone(),
                        hover: props.card_hover,
                    }
                }
            }

            // Add Card Button
            if let Some(on_add) = props.on_add_card.clone() {
                div {
                    style: "padding: 0 12px 12px 12px;",
                    Button {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::Sm,
                        full_width: true,
                        onclick: move |_| on_add.call(column_id.clone()),

                        Icon {
                            name: "plus".to_string(),
                            size: IconSize::Small,
                            color: IconColor::Muted,
                        }
                        "Add card"
                    }
                }
            }
        }
    }
}

/// Individual kanban card component
#[derive(Props, Clone, PartialEq)]
pub struct KanbanCardViewProps {
    pub card: KanbanCard,
    pub on_click: Option<EventHandler<String>>,
    pub hover: bool,
}

#[component]
pub fn KanbanCardView(props: KanbanCardViewProps) -> Element {
    let theme = use_theme();
    let mut is_hovered = use_signal(|| false);

    let card_style = use_style(move |t| {
        let base = Style::new()
            .bg(&t.colors.background)
            .rounded(&t.radius, "md")
            .border(1, &t.colors.border)
            .p(&t.spacing, "md")
            .cursor_pointer()
            .transition("all 150ms ease");

        if props.hover && is_hovered() {
            base.shadow("0 4px 12px rgba(0, 0, 0, 0.1)")
                .transform("translateY(-2px)")
        } else {
            base
        }
        .build()
    });

    let drag_handle_style = use_style(|t| {
        Style::new()
            .flex()
            .items_center()
            .justify_center()
            .cursor("grab")
            .text_color(&t.colors.muted_foreground)
            .build()
    });

    let tags_style = use_style(|t| {
        Style::new()
            .flex()
            .flex_wrap()
            .gap_px(4)
            .mt(&t.spacing, "sm")
            .build()
    });

    let footer_style = use_style(|t| {
        Style::new()
            .flex()
            .items_center()
            .justify_between()
            .mt(&t.spacing, "md")
            .pt(&t.spacing, "sm")
            .border_top(1, &t.colors.border)
            .build()
    });

    let card = props.card.clone();
    let card_id = card.id.clone();
    let onclick_handler = props.on_click.clone();

    rsx! {
        div {
            style: "{card_style}",
            onmouseenter: move |_| is_hovered.set(true),
            onmouseleave: move |_| is_hovered.set(false),
            onclick: move |_| {
                if let Some(handler) = &onclick_handler {
                    handler.call(card_id.clone());
                }
            },

            // Drag Handle & Title Row
            div {
                style: "display: flex; gap: 8px; align-items: flex-start;",

                // Drag handle (visual only)
                div {
                    style: "{drag_handle_style} flex-shrink: 0; padding-top: 2px;",
                    Icon {
                        name: "grip-vertical".to_string(),
                        size: IconSize::Small,
                        color: IconColor::Muted,
                    }
                }

                // Card content
                div {
                    style: "flex: 1; min-width: 0;",

                    Label {
                        size: TextSize::Small,
                        weight: crate::atoms::TextWeight::Medium,
                        "{card.title}"
                    }

                    if let Some(description) = card.description {
                        p {
                            style: "margin: 4px 0 0 0; color: {theme.tokens.read().colors.muted_foreground.to_rgba()}; font-size: 13px; line-height: 1.4; overflow: hidden; display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical;",
                            "{description}"
                        }
                    }
                }
            }

            // Tags
            if !card.tags.is_empty() {
                div {
                    style: "{tags_style}",

                    for tag in card.tags {
                        KanbanTag {
                            key: "{tag}",
                            label: tag,
                        }
                    }
                }
            }

            // Footer: Assignee & Due Date
            if card.assignee.is_some() || card.due_date.is_some() {
                div {
                    style: "{footer_style}",

                    div {
                        style: "display: flex; align-items: center; gap: 8px;",

                        if let Some(assignee) = card.assignee {
                            div {
                                style: "display: flex; align-items: center; gap: 4px;",

                                if assignee.starts_with("http") {
                                    // Avatar image
                                    img {
                                        src: "{assignee}",
                                        style: "width: 24px; height: 24px; border-radius: 50%; object-fit: cover;",
                                        alt: "Assignee",
                                    }
                                } else {
                                    // Initials avatar
                                    div {
                                        style: "width: 24px; height: 24px; border-radius: 50%; background: {theme.tokens.read().colors.primary.to_rgba()}; color: white; display: flex; align-items: center; justify-content: center; font-size: 11px; font-weight: 600;",
                                        "{assignee.chars().take(2).collect::<String>().to_uppercase()}"
                                    }
                                }
                            }
                        }
                    }

                    if let Some(due_date) = card.due_date {
                        div {
                            style: "display: flex; align-items: center; gap: 4px; color: {theme.tokens.read().colors.muted_foreground.to_rgba()}; font-size: 12px;",

                            Icon {
                                name: "calendar".to_string(),
                                size: IconSize::Small,
                                color: IconColor::Muted,
                            }
                            "{due_date}"
                        }
                    }
                }
            }
        }
    }
}

/// Simple tag for kanban cards
#[derive(Props, Clone, PartialEq)]
pub struct KanbanTagProps {
    pub label: String,
}

#[component]
pub fn KanbanTag(props: KanbanTagProps) -> Element {
    // Generate a consistent color based on the label
    let color_hash = props
        .label
        .bytes()
        .fold(0u32, |acc, b| acc.wrapping_add(b as u32));
    let hue = color_hash % 360;
    let bg_color = format!("hsl({}, 70%, 90%)", hue);
    let text_color = format!("hsl({}, 70%, 30%)", hue);

    rsx! {
        span {
            style: "display: inline-block; padding: 2px 8px; border-radius: 4px; font-size: 11px; font-weight: 500; background: {bg_color}; color: {text_color}; white-space: nowrap;",
            "{props.label}"
        }
    }
}

/// Add column button component
#[derive(Props, Clone, PartialEq)]
pub struct AddColumnButtonProps {
    pub on_add_column: Option<EventHandler<()>>,
}

#[component]
pub fn AddColumnButton(props: AddColumnButtonProps) -> Element {
    let theme = use_theme();
    let mut is_hovered = use_signal(|| false);

    let button_style = use_style(move |t| {
        let base = Style::new()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .rounded(&t.radius, "lg")
            .border(2, &t.colors.border)
            .min_w_px(280)
            .h_full()
            .cursor_pointer()
            .gap(&t.spacing, "sm")
            .transition("all 150ms ease");

        if is_hovered() {
            base.border_color(&t.colors.primary).bg(&t.colors.muted)
        } else {
            base.border_style("dashed")
        }
        .build()
    });

    let onclick_handler = props.on_add_column.clone();

    rsx! {
        div {
            style: "{button_style}",
            onmouseenter: move |_| is_hovered.set(true),
            onmouseleave: move |_| is_hovered.set(false),
            onclick: move |_| {
                if let Some(handler) = &onclick_handler {
                    handler.call(());
                }
            },

            Icon {
                name: "plus".to_string(),
                size: IconSize::Large,
                color: IconColor::Muted,
            }

            span {
                style: "color: {theme.tokens.read().colors.muted_foreground.to_rgba()}; font-size: 14px; font-weight: 500;",
                "Add column"
            }
        }
    }
}

/// Simple Kanban board without complex state management
///
/// A simplified version for basic use cases
#[derive(Props, Clone, PartialEq)]
pub struct SimpleKanbanProps {
    pub columns: Vec<KanbanColumn>,
    #[props(default)]
    pub on_card_click: Option<EventHandler<String>>,
    #[props(default = "100%".to_string())]
    pub height: String,
}

#[component]
pub fn SimpleKanban(props: SimpleKanbanProps) -> Element {
    rsx! {
        Kanban {
            columns: props.columns.clone(),
            on_card_click: props.on_card_click.clone(),
            height: props.height.clone(),
            show_card_count: true,
            card_hover: true,
        }
    }
}
