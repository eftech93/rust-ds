//! Tabs organism component
//!
//! A set of layered sections of content—known as tab panels—that are displayed one at a time.
//!
//! # Accessibility
//!
//! Follows the WAI-ARIA tabs pattern: the tab strip is a `role="tablist"`, each
//! button is a `role="tab"` with `aria-selected`, and [`TabPanel`] is a
//! `role="tabpanel"` linked back to the active tab with `aria-labelledby`. Only
//! the active tab is in the Tab order (roving tabindex); arrow keys, `Home` and
//! `End` move focus and activate the tab, skipping disabled tabs.

use crate::a11y::{focus_element, next_enabled_index, use_unique_id, Orientation};
use crate::styles::Style;
use crate::theme::{use_style, use_theme};
use dioxus::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Index of the tab that should be reachable with Tab (roving tabindex): the
/// active tab if it is enabled, otherwise the first enabled tab.
fn focusable_tab_index(tabs: &[TabItem], active_tab: &str) -> Option<usize> {
    tabs.iter()
        .position(|t| t.id == active_tab && !t.disabled)
        .or_else(|| tabs.iter().position(|t| !t.disabled))
}

/// DOM id of a tab button (whitespace replaced so it is a valid IDREF).
fn tab_element_id(base_id: &str, tab_id: &str) -> String {
    let tab_id: String = tab_id
        .chars()
        .map(|c| if c.is_whitespace() { '-' } else { c })
        .collect();
    format!("{base_id}-tab-{tab_id}")
}

/// Shared keyboard handling for [`Tabs`] and [`VerticalTabs`] (automatic
/// activation: moving focus also selects the tab).
fn handle_tab_keydown(
    e: &KeyboardEvent,
    current: usize,
    tabs: &[TabItem],
    orientation: Orientation,
    base_id: &str,
    on_change: Option<EventHandler<String>>,
) {
    let disabled: Vec<bool> = tabs.iter().map(|t| t.disabled).collect();
    if let Some(next) = next_enabled_index(&e.key(), current, &disabled, orientation) {
        e.prevent_default();
        if next != current {
            if let Some(handler) = on_change {
                handler.call(tabs[next].id.clone());
            }
            focus_element(&tab_element_id(base_id, &tabs[next].id));
        }
    }
}

/// Links a [`TabPanel`] rendered inside [`Tabs`] to the active tab.
#[derive(Clone)]
struct TabsA11yContext {
    /// `(panel id, active tab element id)`, refreshed by `Tabs` on every render.
    state: Rc<RefCell<(String, Option<String>)>>,
    /// Custom id given to the `TabPanel`, so the active tab's `aria-controls` matches it.
    panel_override: Signal<Option<String>>,
}

/// Tab item definition
#[derive(Clone, PartialEq)]
pub struct TabItem {
    /// Tab ID
    pub id: String,
    /// Tab label
    pub label: String,
    /// Tab icon (optional)
    pub icon: Option<String>,
    /// Whether tab is disabled
    pub disabled: bool,
}

impl TabItem {
    /// Create a new tab item
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            disabled: false,
        }
    }

    /// Add an icon
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Tabs properties
#[derive(Props, Clone, PartialEq)]
pub struct TabsProps {
    /// Tab items
    pub tabs: Vec<TabItem>,
    /// Currently active tab ID
    pub active_tab: String,
    /// Callback when tab changes
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,
    /// Tab content (rendered based on active tab)
    pub children: Element,
    /// Tabs variant
    #[props(default)]
    pub variant: TabsVariant,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    #[props(default)]
    pub id: Option<String>,
    #[props(default)]
    pub class: Option<String>,
}

/// Tabs variant
#[derive(Default, Clone, PartialEq)]
pub enum TabsVariant {
    /// Default underline style
    #[default]
    Default,
    /// Enclosed/pill style
    Enclosed,
    /// Soft background style
    Soft,
}

/// Tabs organism component
///
/// Render a [`TabPanel`] as the child to show the active tab's content.
///
/// # Accessibility
///
/// - The tab strip has `role="tablist"`; tabs have `role="tab"`, `aria-selected`,
///   a stable `id` and (for the active tab) `aria-controls` pointing to the panel.
/// - A child [`TabPanel`] gets `role="tabpanel"`, `aria-labelledby` (active tab)
///   and `tabindex="0"`.
/// - Roving tabindex: only the active tab is in the Tab order.
/// - `ArrowLeft`/`ArrowRight` (wrapping), `Home` and `End` move focus and activate
///   the tab via `on_change`; disabled tabs are skipped.
/// - Element ids derive from the `id` prop when set, otherwise a generated id.
#[component]
pub fn Tabs(props: TabsProps) -> Element {
    let _theme = use_theme();
    let variant = props.variant.clone();

    let tabs_container_style = use_style(move |t| match variant {
        TabsVariant::Default => Style::new()
            .w_full()
            .border_bottom(1, &t.colors.border)
            .flex()
            .gap(&t.spacing, "md")
            .build(),
        TabsVariant::Enclosed => Style::new()
            .w_full()
            .bg(&t.colors.muted)
            .rounded(&t.radius, "md")
            .p(&t.spacing, "xs")
            .flex()
            .gap(&t.spacing, "xs")
            .build(),
        TabsVariant::Soft => Style::new().w_full().flex().gap(&t.spacing, "sm").build(),
    });

    let id = props.id.clone().unwrap_or_default();
    let class = props.class.clone().unwrap_or_default();

    let generated_id = use_unique_id("tabs");
    let base_id = props.id.clone().unwrap_or(generated_id);
    let default_panel_id = format!("{base_id}-panel");
    let ctx = use_context_provider(|| TabsA11yContext {
        state: Rc::new(RefCell::new((default_panel_id.clone(), None))),
        panel_override: Signal::new(None),
    });

    let tabs = props.tabs.clone();
    let focusable = focusable_tab_index(&tabs, &props.active_tab);
    let active_index = tabs
        .iter()
        .position(|t| t.id == props.active_tab && !t.disabled);
    let panel_id = ctx.panel_override.cloned().unwrap_or(default_panel_id);
    *ctx.state.borrow_mut() = (
        panel_id.clone(),
        active_index.map(|i| tab_element_id(&base_id, &tabs[i].id)),
    );

    rsx! {
        div {
            id: "{id}",
            class: "{class}",
            style: "{tabs_container_style} {props.style.clone().unwrap_or_default()}",
            role: "tablist",
            aria_orientation: "horizontal",

            for (index, tab) in tabs.iter().cloned().enumerate() {
                TabButton {
                    element_id: tab_element_id(&base_id, &tab.id),
                    controls: (Some(index) == active_index).then(|| panel_id.clone()),
                    focusable: Some(index) == focusable,
                    index: index,
                    siblings: tabs.clone(),
                    base_id: base_id.clone(),
                    is_active: props.active_tab == tab.id,
                    variant: props.variant.clone(),
                    on_click: props.on_change,
                    tab: tab.clone(),
                }
            }
        }
        {props.children}
    }
}

#[derive(Props, Clone, PartialEq)]
struct TabButtonProps {
    tab: TabItem,
    is_active: bool,
    variant: TabsVariant,
    #[props(default)]
    on_click: Option<EventHandler<String>>,
    /// DOM id of the button
    element_id: String,
    /// Id of the panel this tab controls (active tab only)
    #[props(default)]
    controls: Option<String>,
    /// Whether this tab is the roving-tabindex stop
    focusable: bool,
    /// Position of this tab in `siblings`
    index: usize,
    /// All tabs of the list (for keyboard navigation)
    siblings: Vec<TabItem>,
    /// Base id used to derive tab element ids
    base_id: String,
}

#[component]
fn TabButton(props: TabButtonProps) -> Element {
    let theme = use_theme();
    let mut is_hovered = use_signal(|| false);

    let is_active = props.is_active;
    let is_disabled = props.tab.disabled;
    let variant = props.variant.clone();

    // Computed every render (not memoized) so `is_active` changes restyle the tab.
    let button_style = {
        let tokens = theme.tokens.read();
        let t = &*tokens;
        let base = Style::new()
            .inline_flex()
            .items_center()
            .gap(&t.spacing, "xs")
            .px(&t.spacing, "md")
            .py(&t.spacing, "sm")
            .text(&t.typography, "sm")
            .font_weight(500)
            .cursor(if is_disabled {
                "not-allowed"
            } else {
                "pointer"
            })
            .transition("all 150ms ease")
            .border(0, &t.colors.border)
            .bg_transparent()
            .outline("none");

        match variant {
            TabsVariant::Default => {
                let styled = if is_active {
                    base.text_color(&t.colors.foreground)
                        .border_bottom(2, &t.colors.primary)
                        .mb_px(-1)
                } else {
                    base.text_color(&t.colors.muted_foreground)
                };

                if is_hovered() && !is_disabled && !is_active {
                    styled.text_color(&t.colors.foreground)
                } else {
                    styled
                }
                .build()
            }
            TabsVariant::Enclosed => {
                if is_active {
                    base.bg(&t.colors.background)
                        .text_color(&t.colors.foreground)
                        .shadow(&t.shadows.sm)
                        .rounded(&t.radius, "sm")
                        .build()
                } else if is_hovered() && !is_disabled {
                    base.text_color(&t.colors.foreground).build()
                } else {
                    base.text_color(&t.colors.muted_foreground).build()
                }
            }
            TabsVariant::Soft => {
                if is_active {
                    base.bg(&t.colors.secondary)
                        .text_color(&t.colors.secondary_foreground)
                        .rounded(&t.radius, "md")
                        .build()
                } else if is_hovered() && !is_disabled {
                    base.bg(&t.colors.muted)
                        .text_color(&t.colors.foreground)
                        .rounded(&t.radius, "md")
                        .build()
                } else {
                    base.text_color(&t.colors.muted_foreground).build()
                }
            }
        }
    };

    let handle_click = move |_| {
        if !is_disabled {
            if let Some(h) = &props.on_click {
                h.call(props.tab.id.clone());
            }
        }
    };

    let has_icon = props.tab.icon.is_some();

    let tabindex = if props.focusable { "0" } else { "-1" };
    let selected = if is_active { "true" } else { "false" };
    let index = props.index;
    let siblings = props.siblings.clone();
    let base_id = props.base_id.clone();
    let on_change = props.on_click;

    rsx! {
        button {
            id: "{props.element_id}",
            r#type: "button",
            role: "tab",
            aria_selected: selected,
            aria_controls: props.controls.clone(),
            tabindex: tabindex,
            style: "{button_style}",
            disabled: is_disabled,
            onclick: handle_click,
            onkeydown: move |e: KeyboardEvent| {
                handle_tab_keydown(
                    &e,
                    index,
                    &siblings,
                    Orientation::Horizontal,
                    &base_id,
                    on_change,
                )
            },
            onmouseenter: move |_| if !is_disabled { is_hovered.set(true) },
            onmouseleave: move |_| is_hovered.set(false),

            if has_icon {
                TabIcon { name: props.tab.icon.clone().unwrap() }
            }

            "{props.tab.label}"
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct TabIconProps {
    name: String,
}

#[component]
fn TabIcon(props: TabIconProps) -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            style: "width: 16px; height: 16px;",

            match props.name.as_str() {
                "settings" => rsx! {
                    path { d: "M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" }
                    circle { cx: "12", cy: "12", r: "3" }
                },
                _ => rsx! {
                    circle { cx: "12", cy: "12", r: "10" }
                },
            }
        }
    }
}

/// Tab panel component for tab content
#[derive(Props, Clone, PartialEq)]
pub struct TabPanelProps {
    /// Panel content
    pub children: Element,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    #[props(default)]
    pub id: Option<String>,
    #[props(default)]
    pub class: Option<String>,
}

///
/// # Accessibility
///
/// Renders `role="tabpanel"` with `tabindex="0"`. Inside [`Tabs`] it also gets
/// `aria-labelledby` pointing to the active tab and an id that the active tab's
/// `aria-controls` references (a custom `id` prop is honoured).
#[component]
pub fn TabPanel(props: TabPanelProps) -> Element {
    let _theme = use_theme();

    let panel_style = use_style(|t| Style::new().w_full().pt(&t.spacing, "md").build());

    let ctx = try_use_context::<TabsA11yContext>();
    let custom_id = props.id.clone();
    {
        let ctx = ctx.clone();
        let custom_id = custom_id.clone();
        use_effect(move || {
            if let (Some(ctx), Some(id)) = (&ctx, &custom_id) {
                let mut panel_override = ctx.panel_override;
                if panel_override.peek().as_deref() != Some(id.as_str()) {
                    panel_override.set(Some(id.clone()));
                }
            }
        });
    }

    let (id, labelled_by) = match &ctx {
        Some(ctx) => {
            let (panel_id, labelled_by) = ctx.state.borrow().clone();
            (custom_id.unwrap_or(panel_id), labelled_by)
        }
        None => (custom_id.unwrap_or_default(), None),
    };
    let class = props.class.clone().unwrap_or_default();

    rsx! {
        div {
            id: "{id}",
            class: "{class}",
            role: "tabpanel",
            aria_labelledby: labelled_by,
            tabindex: "0",
            style: "{panel_style} {props.style.clone().unwrap_or_default()}",
            {props.children}
        }
    }
}

/// Vertical tabs variant
#[derive(Props, Clone, PartialEq)]
pub struct VerticalTabsProps {
    /// Tab items
    pub tabs: Vec<TabItem>,
    /// Currently active tab ID
    pub active_tab: String,
    /// Callback when tab changes
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,
    /// Custom inline styles
    #[props(default)]
    pub style: Option<String>,
    #[props(default)]
    pub id: Option<String>,
    #[props(default)]
    pub class: Option<String>,
}

///
/// # Accessibility
///
/// The tab column has `role="tablist"` and `aria-orientation="vertical"`; tabs
/// have `role="tab"`, `aria-selected` and stable ids. Roving tabindex keeps only
/// the active tab in the Tab order; `ArrowUp`/`ArrowDown` (wrapping), `Home` and
/// `End` move focus and activate the tab via `on_change`, skipping disabled tabs.
#[component]
pub fn VerticalTabs(props: VerticalTabsProps) -> Element {
    let _theme = use_theme();

    let container_style = use_style(|t| Style::new().flex().gap(&t.spacing, "lg").build());

    let sidebar_style = use_style(|t| {
        Style::new()
            .w_px(200)
            .flex()
            .flex_col()
            .gap(&t.spacing, "xs")
            .build()
    });

    let id = props.id.clone().unwrap_or_default();
    let class = props.class.clone().unwrap_or_default();

    let generated_id = use_unique_id("vtabs");
    let base_id = props.id.clone().unwrap_or(generated_id);
    let tabs = props.tabs.clone();
    let focusable = focusable_tab_index(&tabs, &props.active_tab);

    rsx! {
        div {
            id: "{id}",
            class: "{class}",
            style: "{container_style} {props.style.clone().unwrap_or_default()}",

            div {
                style: "{sidebar_style}",
                role: "tablist",
                aria_orientation: "vertical",

                for (index, tab) in tabs.iter().cloned().enumerate() {
                    VerticalTabButton {
                        element_id: tab_element_id(&base_id, &tab.id),
                        focusable: Some(index) == focusable,
                        index: index,
                        siblings: tabs.clone(),
                        base_id: base_id.clone(),
                        is_active: props.active_tab == tab.id,
                        on_click: props.on_change,
                        tab: tab.clone(),
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct VerticalTabButtonProps {
    tab: TabItem,
    is_active: bool,
    #[props(default)]
    on_click: Option<EventHandler<String>>,
    /// DOM id of the button
    element_id: String,
    /// Whether this tab is the roving-tabindex stop
    focusable: bool,
    /// Position of this tab in `siblings`
    index: usize,
    /// All tabs of the list (for keyboard navigation)
    siblings: Vec<TabItem>,
    /// Base id used to derive tab element ids
    base_id: String,
}

#[component]
fn VerticalTabButton(props: VerticalTabButtonProps) -> Element {
    let theme = use_theme();
    let mut is_hovered = use_signal(|| false);

    let is_active = props.is_active;
    let is_disabled = props.tab.disabled;

    // Computed every render (not memoized) so `is_active` changes restyle the tab.
    let button_style = {
        let tokens = theme.tokens.read();
        let t = &*tokens;
        let base = Style::new()
            .w_full()
            .flex()
            .items_center()
            .gap(&t.spacing, "sm")
            .px(&t.spacing, "md")
            .py(&t.spacing, "sm")
            .rounded(&t.radius, "md")
            .text(&t.typography, "sm")
            .font_weight(500)
            .cursor(if is_disabled {
                "not-allowed"
            } else {
                "pointer"
            })
            .transition("all 150ms ease")
            .border(0, &t.colors.border)
            .bg_transparent()
            .text_align_left();

        if is_active {
            base.bg(&t.colors.secondary)
                .text_color(&t.colors.secondary_foreground)
        } else if is_hovered() && !is_disabled {
            base.bg(&t.colors.muted).text_color(&t.colors.foreground)
        } else {
            base.text_color(&t.colors.muted_foreground)
        }
        .build()
    };

    let handle_click = move |_| {
        if !is_disabled {
            if let Some(h) = &props.on_click {
                h.call(props.tab.id.clone());
            }
        }
    };

    let tabindex = if props.focusable { "0" } else { "-1" };
    let selected = if is_active { "true" } else { "false" };
    let index = props.index;
    let siblings = props.siblings.clone();
    let base_id = props.base_id.clone();
    let on_change = props.on_click;

    rsx! {
        button {
            id: "{props.element_id}",
            r#type: "button",
            role: "tab",
            aria_selected: selected,
            tabindex: tabindex,
            style: "{button_style}",
            disabled: is_disabled,
            onclick: handle_click,
            onkeydown: move |e: KeyboardEvent| {
                handle_tab_keydown(
                    &e,
                    index,
                    &siblings,
                    Orientation::Vertical,
                    &base_id,
                    on_change,
                )
            },
            onmouseenter: move |_| if !is_disabled { is_hovered.set(true) },
            onmouseleave: move |_| is_hovered.set(false),
            "{props.tab.label}"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(app: fn() -> Element) -> String {
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
        dioxus_ssr::render(&dom)
    }

    fn sample_tabs() -> Vec<TabItem> {
        vec![
            TabItem::new("a", "A"),
            TabItem::new("b", "B"),
            TabItem::new("c", "C").disabled(true),
        ]
    }

    #[test]
    fn tabs_render_tablist_tab_and_panel_semantics() {
        let html = render(|| {
            rsx! {
                Tabs {
                    id: "t",
                    tabs: sample_tabs(),
                    active_tab: "b",
                    TabPanel { "content" }
                }
            }
        });
        assert!(html.contains("role=\"tablist\""), "{html}");
        assert_eq!(html.matches("role=\"tab\"").count(), 3, "{html}");
        assert_eq!(html.matches("aria-selected=\"true\"").count(), 1, "{html}");
        assert_eq!(html.matches("aria-selected=\"false\"").count(), 2, "{html}");
        assert!(html.contains("id=\"t-tab-b\""), "{html}");
        assert_eq!(
            html.matches("aria-controls=\"t-panel\"").count(),
            1,
            "{html}"
        );
        // Roving tabindex: active tab + the panel are tabbable, other tabs are not.
        assert_eq!(html.matches("tabindex=\"0\"").count(), 2, "{html}");
        assert_eq!(html.matches("tabindex=\"-1\"").count(), 2, "{html}");
        assert!(html.contains("role=\"tabpanel\""), "{html}");
        assert!(html.contains("id=\"t-panel\""), "{html}");
        assert!(html.contains("aria-labelledby=\"t-tab-b\""), "{html}");
        assert!(html.contains("content"), "{html}");
    }

    #[test]
    fn vertical_tabs_declare_orientation() {
        let html = render(|| {
            rsx! { VerticalTabs { id: "v", tabs: sample_tabs(), active_tab: "a" } }
        });
        assert!(html.contains("role=\"tablist\""), "{html}");
        assert!(html.contains("aria-orientation=\"vertical\""), "{html}");
        assert_eq!(html.matches("role=\"tab\"").count(), 3, "{html}");
        assert!(html.contains("id=\"v-tab-a\""), "{html}");
        assert_eq!(html.matches("tabindex=\"0\"").count(), 1, "{html}");
    }

    #[test]
    fn standalone_panel_is_focusable_without_labelledby() {
        let html = render(|| rsx! { TabPanel { "x" } });
        assert!(html.contains("role=\"tabpanel\""), "{html}");
        assert!(html.contains("tabindex=\"0\""), "{html}");
        assert!(!html.contains("aria-labelledby"), "{html}");
    }

    #[test]
    fn focusable_tab_falls_back_to_first_enabled() {
        let tabs = sample_tabs();
        assert_eq!(focusable_tab_index(&tabs, "b"), Some(1));
        assert_eq!(focusable_tab_index(&tabs, "c"), Some(0));
        assert_eq!(focusable_tab_index(&tabs, "missing"), Some(0));
    }

    #[test]
    fn tab_ids_are_valid_idrefs() {
        assert_eq!(tab_element_id("t", "my tab"), "t-tab-my-tab");
    }
}
