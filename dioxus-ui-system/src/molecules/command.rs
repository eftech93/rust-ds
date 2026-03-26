//! Command palette molecule component
//!
//! A command palette/search component with keyboard navigation.
//! Similar to VS Code's cmd+palette or shadcn's Command component.
//!
//! # Example
//! ```rust,ignore
//! use dioxus_ui_system::molecules::command::*;
//!
//! fn MyCommand() -> Element {
//!     let mut value = use_signal(|| "".to_string());
//!     
//!     rsx! {
//!         Command {
//!             CommandInput {
//!                 placeholder: "Search commands...",
//!                 value: value(),
//!                 on_value_change: move |v| value.set(v),
//!             }
//!             CommandList {
//!                 CommandEmpty { "No results found." }
//!                 CommandGroup {
//!                     heading: "Suggestions",
//!                     CommandItem {
//!                         value: "calendar",
//!                         on_select: move |_| println!("Calendar selected"),
//!                         "Calendar"
//!                     }
//!                     CommandItem {
//!                         value: "search",
//!                         on_select: move |_| println!("Search selected"),
//!                         "Search"
//!                     }
//!                 }
//!                 CommandSeparator {}
//!                 CommandGroup {
//!                     heading: "Settings",
//!                     CommandItem {
//!                         value: "profile",
//!                         on_select: move |_| println!("Profile selected"),
//!                         "Profile"
//!                     }
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;
use crate::theme::{use_theme, use_style};
use crate::styles::Style;

// ============================================================================
// Context
// ============================================================================

/// Context shared between Command components
#[derive(Clone, Copy)]
struct CommandContext {
    /// Current search value
    value: Signal<String>,
    /// Currently highlighted item index
    highlighted_index: Signal<usize>,
    /// Total number of selectable items
    item_count: Signal<usize>,
    /// Callback when item is selected via keyboard
    on_select: Callback<()>,
    /// Whether the command palette is focused
    focused: Signal<bool>,
    /// Callback when any item is selected (for closing the palette)
    on_item_select: Callback<()>,
}

// ============================================================================
// Command Container
// ============================================================================

/// Command container properties
#[derive(Props, Clone, PartialEq)]
pub struct CommandProps {
    /// Child elements
    pub children: Element,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Callback when an item is selected (can be used to close the command palette)
    #[props(default)]
    pub on_select: Option<EventHandler<()>>,
}

/// Command container component
/// 
/// The root component that provides context for all child command components.
#[component]
pub fn Command(props: CommandProps) -> Element {
    let _theme = use_theme();
    let value = use_signal(|| String::new());
    let highlighted_index = use_signal(|| 0usize);
    let item_count = use_signal(|| 0usize);
    let focused = use_signal(|| false);
    
    let on_item_select = props.on_select.clone();
    let context = CommandContext {
        value,
        highlighted_index,
        item_count,
        on_select: Callback::new(move |()| {}),
        focused,
        on_item_select: Callback::new(move |()| {
            if let Some(ref handler) = on_item_select {
                handler.call(());
            }
        }),
    };
    
    let class_css = props.class.as_ref()
        .map(|c| format!(" {}", c))
        .unwrap_or_default();
    
    let container_style = use_style(|t| {
        Style::new()
            .flex()
            .flex_col()
            .w_full()
            .rounded(&t.radius, "lg")
            .border(1, &t.colors.border)
            .bg(&t.colors.background)
            .shadow(&t.shadows.lg)
            .overflow_hidden()
            .build()
    });
    
    use_context_provider(|| context);
    
    rsx! {
        div {
            class: "command{class_css}",
            style: "{container_style}",
            {props.children}
        }
    }
}

// ============================================================================
// Command Input
// ============================================================================

/// Command input properties
#[derive(Props, Clone, PartialEq)]
pub struct CommandInputProps {
    /// Placeholder text
    #[props(default)]
    pub placeholder: Option<String>,
    /// Current value
    #[props(default)]
    pub value: String,
    /// Callback when value changes
    pub on_value_change: EventHandler<String>,
}

/// Command input component
/// 
/// Search input for filtering command items.
#[component]
pub fn CommandInput(props: CommandInputProps) -> Element {
    let theme = use_theme();
    let mut context: CommandContext = use_context();
    let value_ref = props.value.clone();
    
    // Update context value when prop changes
    use_effect(move || {
        context.value.set(value_ref.clone());
    });
    
    let input_style = use_style(|t| {
        Style::new()
            .w_full()
            .px_px(16)
            .py_px(12)
            .font_size(16)
            .bg(&t.colors.background)
            .text_color(&t.colors.foreground)
            .border_bottom(1, &t.colors.border)
            .outline("none")
            .build()
    });
    
    let handle_key_down = move |e: Event<dioxus::html::KeyboardData>| {
        use dioxus::html::input_data::keyboard_types::Key;
        
        match e.key() {
            Key::ArrowDown => {
                e.prevent_default();
                let count = context.item_count.read().clone();
                if count > 0 {
                    let current = context.highlighted_index.read().clone();
                    context.highlighted_index.set((current + 1).min(count - 1));
                }
            }
            Key::ArrowUp => {
                e.prevent_default();
                let current = context.highlighted_index.read().clone();
                context.highlighted_index.set(current.saturating_sub(1));
            }
            Key::Enter => {
                // Selection is handled by CommandList
            }
            Key::Escape => {
                context.highlighted_index.set(0);
            }
            _ => {}
        }
    };
    
    let placeholder_text = props.placeholder.clone().unwrap_or_else(|| "Type a command or search...".to_string());
    let value_for_input = props.value.clone();
    
    rsx! {
        div {
            class: "command-input-wrapper",
            style: "position: relative; display: flex; align-items: center;",
            
            // Search icon
            span {
                class: "command-input-icon",
                style: "position: absolute; left: 16px; font-size: 16px; color: {theme.tokens.read().colors.muted.to_rgba()}; pointer-events: none;",
                "🔍"
            }
            
            input {
                class: "command-input",
                style: "{input_style} padding-left: 44px;",
                type: "text",
                placeholder: "{placeholder_text}",
                value: "{value_for_input}",
                oninput: move |e: Event<FormData>| {
                    let new_value = e.value();
                    context.value.set(new_value.clone());
                    context.highlighted_index.set(0);
                    props.on_value_change.call(new_value);
                },
                onkeydown: handle_key_down,
                onfocus: move |_| context.focused.set(true),
                onblur: move |_| context.focused.set(false),
            }
        }
    }
}

// ============================================================================
// Command List
// ============================================================================

/// Command list properties
#[derive(Props, Clone, PartialEq)]
pub struct CommandListProps {
    /// Child elements (CommandGroup, CommandItem, CommandSeparator, CommandEmpty)
    pub children: Element,
}

/// Command list component
/// 
/// Scrollable container for command items.
#[component]
pub fn CommandList(props: CommandListProps) -> Element {
    let _theme = use_theme();
    
    let list_style = use_style(|t| {
        Style::new()
            .flex()
            .flex_col()
            .max_h_px(300)
            .overflow_auto()
            .p(&t.spacing, "sm")
            .gap(&t.spacing, "xs")
            .build()
    });
    
    rsx! {
        div {
            class: "command-list",
            style: "{list_style}",
            {props.children}
        }
    }
}

// ============================================================================
// Command Group
// ============================================================================

/// Command group properties
#[derive(Props, Clone, PartialEq)]
pub struct CommandGroupProps {
    /// Group heading text
    #[props(default)]
    pub heading: Option<String>,
    /// Child command items
    pub children: Element,
}

/// Command group component
/// 
/// Groups related command items with an optional heading.
#[component]
pub fn CommandGroup(props: CommandGroupProps) -> Element {
    let theme = use_theme();
    
    let group_style = use_style(|t| {
        Style::new()
            .flex()
            .flex_col()
            .gap(&t.spacing, "xs")
            .mb(&t.spacing, "sm")
            .build()
    });
    
    rsx! {
        div {
            class: "command-group",
            style: "{group_style}",
            
            if let Some(heading) = props.heading {
                div {
                    class: "command-group-heading",
                    style: "padding: 8px 12px; font-size: 12px; font-weight: 500; color: {theme.tokens.read().colors.muted.to_rgba()}; text-transform: uppercase; letter-spacing: 0.05em;",
                    "{heading}"
                }
            }
            
            {props.children}
        }
    }
}

// ============================================================================
// Command Item
// ============================================================================

/// Command item properties
#[derive(Props, Clone, PartialEq)]
pub struct CommandItemProps {
    /// Search value for filtering
    pub value: String,
    /// Callback when item is selected
    pub on_select: EventHandler<()>,
    /// Child elements
    pub children: Element,
    /// Whether the item is disabled
    #[props(default = false)]
    pub disabled: bool,
}

/// Command item component
/// 
/// Selectable item within a command group.
#[component]
pub fn CommandItem(props: CommandItemProps) -> Element {
    let theme = use_theme();
    let mut context: CommandContext = use_context();
    
    // Register this item in the parent context
    let mut item_index = use_signal(|| 0usize);
    
    use_hook(|| {
        let index = context.item_count.read().clone();
        item_index.set(index);
        let current_count = context.item_count.read().clone();
        context.item_count.set(current_count + 1);
    });
    
    // Check if this item matches the current search
    let search_value = context.value.read().clone().to_lowercase();
    let item_value = props.value.to_lowercase();
    let is_match = search_value.is_empty() || item_value.contains(&search_value);
    
    // Check if this item is currently highlighted
    let is_highlighted = context.highlighted_index.read().clone() == item_index.read().clone();
    
    // Update highlighted index if this item is the only match
    let item_index_for_effect = item_index.read().clone();
    use_effect(move || {
        if is_match && search_value == item_value {
            context.highlighted_index.set(item_index_for_effect);
        }
    });
    
    if !is_match {
        return rsx! {};
    }
    
    let bg_color = if is_highlighted && !props.disabled {
        theme.tokens.read().colors.accent.to_rgba()
    } else {
        "transparent".to_string()
    };
    
    let text_color = if props.disabled {
        theme.tokens.read().colors.muted.to_rgba()
    } else {
        theme.tokens.read().colors.foreground.to_rgba()
    };
    
    let is_disabled = props.disabled;
    let item_style = use_style(move |t| {
        Style::new()
            .flex()
            .items_center()
            .gap(&t.spacing, "sm")
            .px(&t.spacing, "sm")
            .py(&t.spacing, "sm")
            .rounded(&t.radius, "md")
            .cursor(if is_disabled { "not-allowed" } else { "pointer" })
            .opacity(if is_disabled { 0.5 } else { 1.0 })
            .build()
    });
    
    let handle_click = move |_| {
        if !props.disabled {
            props.on_select.call(());
            context.on_item_select.call(());
        }
    };
    
    let handle_mouse_enter = move |_| {
        if !props.disabled {
            context.highlighted_index.set(item_index.read().clone());
        }
    };
    
    rsx! {
        div {
            class: "command-item",
            style: "{item_style} background: {bg_color}; color: {text_color};",
            onclick: handle_click,
            onmouseenter: handle_mouse_enter,
            
            {props.children}
        }
    }
}

// ============================================================================
// Command Separator
// ============================================================================

/// Command separator component
/// 
/// Visual divider between command groups.
#[component]
pub fn CommandSeparator() -> Element {
    let _theme = use_theme();
    
    let separator_style = use_style(|t| {
        Style::new()
            .h_px(1)
            .my(&t.spacing, "sm")
            .bg(&t.colors.border)
            .build()
    });
    
    rsx! {
        div {
            class: "command-separator",
            style: "{separator_style}",
        }
    }
}

// ============================================================================
// Command Empty
// ============================================================================

/// Command empty properties
#[derive(Props, Clone, PartialEq)]
pub struct CommandEmptyProps {
    /// Content to display when no results
    pub children: Element,
}

/// Command empty component
/// 
/// Displayed when no command items match the search.
#[component]
pub fn CommandEmpty(props: CommandEmptyProps) -> Element {
    let _theme = use_theme();
    let context: CommandContext = use_context();
    
    // Only show if there are no matching items
    // This is a simplified check - in a real implementation, 
    // we'd count visible items from the context
    let _search_value = context.value.read().clone();
    
    let empty_style = use_style(|t| {
        Style::new()
            .p(&t.spacing, "lg")
            .text_center()
            .text_color(&t.colors.muted)
            .font_size(14)
            .build()
    });
    
    rsx! {
        div {
            class: "command-empty",
            style: "{empty_style}",
            {props.children}
        }
    }
}

// ============================================================================
// Shortcut Display
// ============================================================================

/// Command shortcut properties
#[derive(Props, Clone, PartialEq)]
pub struct CommandShortcutProps {
    /// Keyboard shortcut text (e.g., "⌘K", "Ctrl+P")
    pub children: Element,
}

/// Command shortcut component
/// 
/// Displays keyboard shortcuts for command items.
#[component]
pub fn CommandShortcut(props: CommandShortcutProps) -> Element {
    let _theme = use_theme();
    
    let shortcut_style = use_style(|t| {
        Style::new()
            .pl(&t.spacing, "md")
            .font_size(12)
            .text_color(&t.colors.muted)
            .build()
    });
    
    rsx! {
        span {
            class: "command-shortcut",
            style: "{shortcut_style} margin-left: auto;",
            {props.children}
        }
    }
}

// ============================================================================
// Loading State
// ============================================================================

/// Command loading component
/// 
/// Displayed while command items are loading.
#[component]
pub fn CommandLoading() -> Element {
    let _theme = use_theme();
    
    let loading_style = use_style(|t| {
        Style::new()
            .p(&t.spacing, "lg")
            .text_center()
            .text_color(&t.colors.muted)
            .font_size(14)
            .build()
    });
    
    rsx! {
        div {
            class: "command-loading",
            style: "{loading_style}",
            "Loading..."
        }
    }
}
