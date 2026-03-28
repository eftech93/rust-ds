//! Advanced Card organism components
//!
//! Provides 10+ card variations with different layouts, actions, and content types.

use dioxus::prelude::*;

use crate::atoms::{
    Button, ButtonSize, ButtonVariant, Heading, HeadingLevel, Icon, IconColor, IconSize, Label,
    TextSize,
};
use crate::molecules::{
    Badge, BadgeVariant, Card, CardContent, CardFooter, CardFooterJustify, CardHeader, CardVariant,
};

/// ============================================================================
/// 1. Single Action Card
/// ============================================================================
/// Card with a single primary action button
#[derive(Props, Clone, PartialEq)]
pub struct ActionCardProps {
    /// Card title
    pub title: String,
    /// Card description
    pub description: String,
    /// Action button text
    pub action_label: String,
    /// Action button callback
    pub on_action: EventHandler<()>,
    /// Optional icon name
    #[props(default)]
    pub icon: Option<String>,
    /// Card variant
    #[props(default)]
    pub variant: CardVariant,
    /// Optional badge
    #[props(default)]
    pub badge: Option<String>,
}

#[component]
pub fn ActionCard(props: ActionCardProps) -> Element {
    rsx! {
        Card {
            variant: props.variant,
            full_width: true,

            CardContent {
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",

                    if let Some(icon) = props.icon.clone() {
                        div {
                            style: "width: 48px; height: 48px; background: #f1f5f9; border-radius: 12px; display: flex; align-items: center; justify-content: center;",

                            Icon {
                                name: icon,
                                size: IconSize::Large,
                                color: IconColor::Primary,
                            }
                        }
                    }

                    if let Some(badge) = props.badge.clone() {
                        Badge {
                            variant: BadgeVariant::Secondary,
                            "{badge}"
                        }
                    }

                    Heading {
                        level: HeadingLevel::H4,
                        "{props.title}"
                    }

                    p {
                        style: "margin: 0; color: #64748b; font-size: 14px; line-height: 1.5;",
                        "{props.description}"
                    }

                    Button {
                        variant: ButtonVariant::Primary,
                        full_width: true,
                        onclick: move |_| props.on_action.call(()),
                        "{props.action_label}"
                    }
                }
            }
        }
    }
}

/// ============================================================================
/// 2. Dual Action Card
/// ============================================================================
/// Card with two action buttons (primary and secondary)
#[derive(Props, Clone, PartialEq)]
pub struct DualActionCardProps {
    /// Card title
    pub title: String,
    /// Card description
    pub description: String,
    /// Primary action label
    pub primary_label: String,
    /// Secondary action label
    pub secondary_label: String,
    /// Primary action callback
    pub on_primary: EventHandler<()>,
    /// Secondary action callback
    pub on_secondary: EventHandler<()>,
    /// Optional icon
    #[props(default)]
    pub icon: Option<String>,
}

#[component]
pub fn DualActionCard(props: DualActionCardProps) -> Element {
    rsx! {
        Card {
            variant: CardVariant::Default,
            full_width: true,

            CardContent {
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",

                    if let Some(icon) = props.icon.clone() {
                        div {
                            style: "width: 48px; height: 48px; background: #f1f5f9; border-radius: 12px; display: flex; align-items: center; justify-content: center;",

                            Icon {
                                name: icon,
                                size: IconSize::Large,
                                color: IconColor::Primary,
                            }
                        }
                    }

                    Heading {
                        level: HeadingLevel::H4,
                        "{props.title}"
                    }

                    p {
                        style: "margin: 0; color: #64748b; font-size: 14px; line-height: 1.5;",
                        "{props.description}"
                    }
                }
            }

            CardFooter {
                justify: CardFooterJustify::Between,

                Button {
                    variant: ButtonVariant::Ghost,
                    onclick: move |_| props.on_secondary.call(()),
                    "{props.secondary_label}"
                }

                Button {
                    variant: ButtonVariant::Primary,
                    onclick: move |_| props.on_primary.call(()),
                    "{props.primary_label}"
                }
            }
        }
    }
}

/// ============================================================================
/// 3. Image Card
/// ============================================================================
/// Card with an image header
#[derive(Props, Clone, PartialEq)]
pub struct ImageCardProps {
    /// Image URL
    pub image_url: String,
    /// Image alt text
    #[props(default)]
    pub image_alt: String,
    /// Card title
    pub title: String,
    /// Card description
    pub description: String,
    /// Optional action
    #[props(default)]
    pub action_label: Option<String>,
    /// Action callback
    #[props(default)]
    pub on_action: Option<EventHandler<()>>,
    /// Image aspect ratio (default: 16/9)
    #[props(default = "16/9".to_string())]
    pub aspect_ratio: String,
}

#[component]
pub fn ImageCard(props: ImageCardProps) -> Element {
    rsx! {
        Card {
            variant: CardVariant::Default,
            full_width: true,
            padding: Some("0".to_string()),

            // Image
            div {
                style: "width: 100%; aspect-ratio: {props.aspect_ratio}; overflow: hidden;",

                img {
                    src: "{props.image_url}",
                    alt: "{props.image_alt}",
                    style: "width: 100%; height: 100%; object-fit: cover;",
                }
            }

            CardContent {
                div {
                    style: "display: flex; flex-direction: column; gap: 8px; padding: 4px 0;",

                    Heading {
                        level: HeadingLevel::H4,
                        "{props.title}"
                    }

                    p {
                        style: "margin: 0; color: #64748b; font-size: 14px; line-height: 1.5;",
                        "{props.description}"
                    }

                    if let Some(label) = props.action_label.clone() {
                        if let Some(handler) = props.on_action.clone() {
                            Button {
                                variant: ButtonVariant::Ghost,
                                onclick: move |_| handler.call(()),
                                "{label} →"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// ============================================================================
/// 4. Image Card with Actions
/// ============================================================================
/// Card with image and action buttons overlay
#[derive(Props, Clone, PartialEq)]
pub struct ImageActionCardProps {
    /// Image URL
    pub image_url: String,
    /// Card title
    pub title: String,
    /// Card description
    pub description: String,
    /// Primary action
    pub primary_label: String,
    /// Secondary action
    pub secondary_label: String,
    /// Primary callback
    pub on_primary: EventHandler<()>,
    /// Secondary callback
    pub on_secondary: EventHandler<()>,
    /// Optional badge on image
    #[props(default)]
    pub badge: Option<String>,
}

#[component]
pub fn ImageActionCard(props: ImageActionCardProps) -> Element {
    rsx! {
        Card {
            variant: CardVariant::Default,
            full_width: true,
            padding: Some("0".to_string()),

            // Image container with badge
            div {
                style: "position: relative; width: 100%; aspect-ratio: 16/9; overflow: hidden;",

                img {
                    src: "{props.image_url}",
                    alt: "{props.title}",
                    style: "width: 100%; height: 100%; object-fit: cover;",
                }

                if let Some(badge) = props.badge.clone() {
                    div {
                        style: "position: absolute; top: 12px; left: 12px;",

                        Badge {
                            variant: BadgeVariant::Success,
                            "{badge}"
                        }
                    }
                }
            }

            CardContent {
                div {
                    style: "display: flex; flex-direction: column; gap: 12px; padding: 4px 0;",

                    Heading {
                        level: HeadingLevel::H4,
                        "{props.title}"
                    }

                    p {
                        style: "margin: 0; color: #64748b; font-size: 14px; line-height: 1.5;",
                        "{props.description}"
                    }
                }
            }

            CardFooter {
                justify: CardFooterJustify::Between,

                Button {
                    variant: ButtonVariant::Secondary,
                    onclick: move |_| props.on_secondary.call(()),
                    "{props.secondary_label}"
                }

                Button {
                    variant: ButtonVariant::Primary,
                    onclick: move |_| props.on_primary.call(()),
                    "{props.primary_label}"
                }
            }
        }
    }
}

/// ============================================================================
/// 5. Profile Card
/// ============================================================================
/// Card showing user/profile information
#[derive(Props, Clone, PartialEq)]
pub struct ProfileCardProps {
    /// Avatar URL
    #[props(default)]
    pub avatar_url: Option<String>,
    /// User name
    pub name: String,
    /// User role/title
    #[props(default)]
    pub role: Option<String>,
    /// User description
    #[props(default)]
    pub description: Option<String>,
    /// Follow/Connect button label
    #[props(default = "Connect".to_string())]
    pub action_label: String,
    /// Action callback
    #[props(default)]
    pub on_action: Option<EventHandler<()>>,
    /// Show social stats
    #[props(default)]
    pub stats: Vec<(String, String)>,
}

#[component]
pub fn ProfileCard(props: ProfileCardProps) -> Element {
    let initials: String = props
        .name
        .split_whitespace()
        .filter_map(|s| s.chars().next())
        .collect::<String>()
        .to_uppercase()
        .chars()
        .take(2)
        .collect();

    rsx! {
        Card {
            variant: CardVariant::Default,
            full_width: true,

            CardContent {
                div {
                    style: "display: flex; flex-direction: column; align-items: center; gap: 16px; text-align: center;",

                    // Avatar
                    if let Some(url) = props.avatar_url.clone() {
                        img {
                            src: "{url}",
                            alt: "{props.name}",
                            style: "width: 80px; height: 80px; border-radius: 50%; object-fit: cover;",
                        }
                    } else {
                        div {
                            style: "width: 80px; height: 80px; border-radius: 50%; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); display: flex; align-items: center; justify-content: center; color: white; font-weight: 600; font-size: 24px;",
                            "{initials}"
                        }
                    }

                    // Info
                    div {
                        Heading {
                            level: HeadingLevel::H4,
                            "{props.name}"
                        }

                        if let Some(role) = props.role.clone() {
                            Label {
                                size: TextSize::Small,
                                color: crate::atoms::TextColor::Muted,
                                "{role}"
                            }
                        }
                    }

                    if let Some(desc) = props.description.clone() {
                        p {
                            style: "margin: 0; color: #64748b; font-size: 14px; line-height: 1.5;",
                            "{desc}"
                        }
                    }

                    // Stats
                    if !props.stats.is_empty() {
                        div {
                            style: "display: flex; gap: 24px; justify-content: center; width: 100%; padding-top: 8px; border-top: 1px solid #e2e8f0;",

                            for (label, value) in props.stats.clone() {
                                div {
                                    style: "text-align: center;",

                                    div {
                                        style: "font-size: 18px; font-weight: 700; color: #0f172a;",
                                        "{value}"
                                    }

                                    Label {
                                        size: TextSize::ExtraSmall,
                                        color: crate::atoms::TextColor::Muted,
                                        "{label}"
                                    }
                                }
                            }
                        }
                    }

                    // Action
                    if let Some(handler) = props.on_action.clone() {
                        Button {
                            variant: ButtonVariant::Primary,
                            full_width: true,
                            onclick: move |_| handler.call(()),
                            "{props.action_label}"
                        }
                    }
                }
            }
        }
    }
}

/// ============================================================================
/// 6. Pricing Card
/// ============================================================================
/// Pricing plan card
#[derive(Props, Clone, PartialEq)]
pub struct PricingCardProps {
    /// Plan name
    pub plan: String,
    /// Price (e.g., "$29")
    pub price: String,
    /// Billing period (e.g., "/month")
    #[props(default = "/month".to_string())]
    pub period: String,
    /// Plan description
    #[props(default)]
    pub description: Option<String>,
    /// List of features
    pub features: Vec<String>,
    /// CTA button label
    #[props(default = "Get Started".to_string())]
    pub cta_label: String,
    /// CTA callback
    pub on_cta: EventHandler<()>,
    /// Is this the recommended plan
    #[props(default)]
    pub recommended: bool,
}

#[component]
pub fn PricingCard(props: PricingCardProps) -> Element {
    let action_element: Option<Element> = if props.recommended {
        Some(rsx! {
            Badge {
                variant: BadgeVariant::Success,
                "Recommended"
            }
        })
    } else {
        None
    };

    rsx! {
        Card {
            variant: if props.recommended { CardVariant::Elevated } else { CardVariant::Default },
            full_width: true,

            CardHeader {
                title: props.plan.clone(),
                action: action_element
            }

            CardContent {
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",

                    // Price
                    div {
                        style: "text-align: center;",

                        span {
                            style: "font-size: 36px; font-weight: 800; color: #0f172a;",
                            "{props.price}"
                        }

                        span {
                            style: "color: #64748b; font-size: 14px;",
                            "{props.period}"
                        }
                    }

                    if let Some(desc) = props.description.clone() {
                        p {
                            style: "margin: 0; color: #64748b; font-size: 14px; text-align: center;",
                            "{desc}"
                        }
                    }

                    // Features
                    ul {
                        style: "margin: 0; padding: 0; list-style: none; display: flex; flex-direction: column; gap: 8px;",

                        for feature in props.features.clone() {
                            li {
                                style: "display: flex; align-items: center; gap: 8px; font-size: 14px;",

                                Icon {
                                    name: "check".to_string(),
                                    size: IconSize::Small,
                                    color: IconColor::Success,
                                }

                                "{feature}"
                            }
                        }
                    }
                }
            }

            CardFooter {
                Button {
                    variant: if props.recommended { ButtonVariant::Primary } else { ButtonVariant::Secondary },
                    full_width: true,
                    onclick: move |_| props.on_cta.call(()),
                    "{props.cta_label}"
                }
            }
        }
    }
}

/// ============================================================================
/// 7. Horizontal Card
/// ============================================================================
/// Card with horizontal layout (image left, content right)
#[derive(Props, Clone, PartialEq)]
pub struct HorizontalCardProps {
    /// Image URL
    pub image_url: String,
    /// Card title
    pub title: String,
    /// Card description
    pub description: String,
    /// Optional action
    #[props(default)]
    pub action_label: Option<String>,
    /// Action callback
    #[props(default)]
    pub on_action: Option<EventHandler<()>>,
}

#[component]
pub fn HorizontalCard(props: HorizontalCardProps) -> Element {
    rsx! {
        Card {
            variant: CardVariant::Default,
            full_width: true,
            padding: Some("0".to_string()),

            div {
                style: "display: flex; flex-direction: row;",

                // Image (left side)
                div {
                    style: "width: 120px; min-height: 120px; flex-shrink: 0;",

                    img {
                        src: "{props.image_url}",
                        alt: "{props.title}",
                        style: "width: 100%; height: 100%; object-fit: cover; border-radius: 12px 0 0 12px;",
                    }
                }

                // Content (right side)
                div {
                    style: "flex: 1; padding: 16px; display: flex; flex-direction: column; justify-content: center;",

                    Heading {
                        level: HeadingLevel::H4,
                        "{props.title}"
                    }

                    p {
                        style: "margin: 4px 0 0 0; color: #64748b; font-size: 13px; line-height: 1.4;",
                        "{props.description}"
                    }

                    if let Some(label) = props.action_label.clone() {
                        if let Some(handler) = props.on_action.clone() {
                            div {
                                style: "margin-top: 12px;",

                                Button {
                                    variant: ButtonVariant::Ghost,
                                    size: ButtonSize::Sm,
                                    onclick: move |_| handler.call(()),
                                    "{label} →"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// ============================================================================
/// 8. Notification Card
/// ============================================================================
/// Card for notifications/alerts with dismiss action
#[derive(Props, Clone, PartialEq)]
pub struct NotificationCardProps {
    /// Notification title
    pub title: String,
    /// Notification message
    pub message: String,
    /// Notification type
    #[props(default)]
    pub notification_type: NotificationType,
    /// Timestamp
    #[props(default)]
    pub timestamp: Option<String>,
    /// Dismiss callback
    #[props(default)]
    pub on_dismiss: Option<EventHandler<()>>,
    /// Icon name (auto-selected based on type if not provided)
    #[props(default)]
    pub icon: Option<String>,
}

/// Notification type variants
#[derive(Clone, PartialEq, Default)]
pub enum NotificationType {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

#[component]
pub fn NotificationCard(props: NotificationCardProps) -> Element {
    let (icon, icon_color, border_color) = match props.notification_type {
        NotificationType::Info => ("info", IconColor::Primary, "#3b82f6"),
        NotificationType::Success => ("check-circle", IconColor::Success, "#22c55e"),
        NotificationType::Warning => ("alert-triangle", IconColor::Warning, "#eab308"),
        NotificationType::Error => ("x-circle", IconColor::Destructive, "#ef4444"),
    };

    let icon_name = props.icon.clone().unwrap_or_else(|| icon.to_string());

    rsx! {
        Card {
            variant: CardVariant::Default,
            full_width: true,
            style: Some(format!("border-left: 4px solid {};", border_color)),

            CardContent {
                div {
                    style: "display: flex; gap: 12px;",

                    // Icon
                    div {
                        style: "flex-shrink: 0; padding-top: 2px;",

                        Icon {
                            name: icon_name,
                            size: IconSize::Medium,
                            color: icon_color,
                        }
                    }

                    // Content
                    div {
                        style: "flex: 1; min-width: 0;",

                        div {
                            style: "display: flex; justify-content: space-between; align-items: flex-start; gap: 8px;",

                            Heading {
                                level: HeadingLevel::H4,
                                "{props.title}"
                            }

                            if let Some(handler) = props.on_dismiss.clone() {
                                button {
                                    style: "background: none; border: none; cursor: pointer; padding: 4px; color: #94a3b8;",
                                    onclick: move |_| handler.call(()),
                                    "✕"
                                }
                            }
                        }

                        p {
                            style: "margin: 4px 0 0 0; color: #64748b; font-size: 14px; line-height: 1.5;",
                            "{props.message}"
                        }

                        if let Some(time) = props.timestamp.clone() {
                            Label {
                                size: TextSize::ExtraSmall,
                                color: crate::atoms::TextColor::Muted,
                                "{time}"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// ============================================================================
/// 9. Stat Card
/// ============================================================================
/// Card for displaying statistics/metrics
#[derive(Props, Clone, PartialEq)]
pub struct StatCardProps {
    /// Stat label
    pub label: String,
    /// Stat value
    pub value: String,
    /// Change indicator (e.g., "+12%")
    #[props(default)]
    pub change: Option<String>,
    /// Is the change positive
    #[props(default)]
    pub change_positive: Option<bool>,
    /// Icon name
    #[props(default)]
    pub icon: Option<String>,
    /// Icon background color
    #[props(default = "#f1f5f9".to_string())]
    pub icon_bg: String,
}

#[component]
pub fn StatCard(props: StatCardProps) -> Element {
    let is_positive = props.change_positive.unwrap_or(true);
    let change_color = if is_positive { "#22c55e" } else { "#ef4444" };

    rsx! {
        Card {
            variant: CardVariant::Default,
            full_width: true,

            CardContent {
                div {
                    style: "display: flex; align-items: flex-start; justify-content: space-between;",

                    // Text content
                    div {
                        Label {
                            size: TextSize::Small,
                            color: crate::atoms::TextColor::Muted,
                            "{props.label}"
                        }

                        div {
                            style: "display: flex; align-items: baseline; gap: 8px; margin-top: 4px;",

                            span {
                                style: "font-size: 28px; font-weight: 700; color: #0f172a;",
                                "{props.value}"
                            }

                            if let Some(change) = props.change.clone() {
                                span {
                                    style: "font-size: 13px; font-weight: 600; color: {change_color};",
                                    "{change}"
                                }
                            }
                        }
                    }

                    // Icon
                    if let Some(icon) = props.icon.clone() {
                        div {
                            style: "width: 40px; height: 40px; background: {props.icon_bg}; border-radius: 10px; display: flex; align-items: center; justify-content: center;",

                            Icon {
                                name: icon,
                                size: IconSize::Medium,
                                color: IconColor::Primary,
                            }
                        }
                    }
                }
            }
        }
    }
}

/// ============================================================================
/// 10. Expandable Card
/// ============================================================================
/// Card that can expand/collapse to show more content
#[derive(Props, Clone, PartialEq)]
pub struct ExpandableCardProps {
    /// Card title
    pub title: String,
    /// Preview content (always visible)
    pub preview: Element,
    /// Expanded content (shown when expanded)
    pub expanded_content: Element,
    /// Initial expanded state
    #[props(default)]
    pub default_expanded: bool,
}

#[component]
pub fn ExpandableCard(props: ExpandableCardProps) -> Element {
    let mut is_expanded = use_signal(|| props.default_expanded);

    rsx! {
        Card {
            variant: CardVariant::Default,
            full_width: true,

            CardHeader {
                title: props.title.clone(),

                action: rsx! {
                    button {
                        style: "background: none; border: none; cursor: pointer; padding: 4px; transition: transform 200ms;",
                        style: if is_expanded() { "transform: rotate(180deg);" } else { "" },
                        onclick: move |_| is_expanded.toggle(),

                        Icon {
                            name: "chevron-down".to_string(),
                            size: IconSize::Medium,
                            color: IconColor::Muted,
                        }
                    }
                }
            }

            CardContent {
                {props.preview}

                if is_expanded() {
                    div {
                        style: "margin-top: 16px; padding-top: 16px; border-top: 1px solid #e2e8f0; animation: fadeIn 200ms ease;",
                        {props.expanded_content}
                    }
                }
            }
        }
    }
}

/// ============================================================================
/// 11. Media Card (Bonus)
/// ============================================================================
/// Card optimized for media content with overlay controls
#[derive(Props, Clone, PartialEq)]
pub struct MediaCardProps {
    /// Media URL (image or video thumbnail)
    pub media_url: String,
    /// Media type indicator
    #[props(default)]
    pub media_type: MediaType,
    /// Card title
    pub title: String,
    /// Creator/Author
    #[props(default)]
    pub creator: Option<String>,
    /// Duration (for video/audio)
    #[props(default)]
    pub duration: Option<String>,
    /// Like callback
    #[props(default)]
    pub on_like: Option<EventHandler<()>>,
    /// Share callback
    #[props(default)]
    pub on_share: Option<EventHandler<()>>,
    /// Play callback
    #[props(default)]
    pub on_play: Option<EventHandler<()>>,
}

/// Media type
#[derive(Clone, PartialEq, Default)]
pub enum MediaType {
    #[default]
    Image,
    Video,
    Audio,
}

#[component]
pub fn MediaCard(props: MediaCardProps) -> Element {
    let overlay_icon = match props.media_type {
        MediaType::Image => None,
        MediaType::Video => Some("play-circle"),
        MediaType::Audio => Some("play-circle"),
    };

    rsx! {
        Card {
            variant: CardVariant::Default,
            full_width: true,
            padding: Some("0".to_string()),

            // Media container
            div {
                style: "position: relative; width: 100%; aspect-ratio: 16/9; overflow: hidden;",

                img {
                    src: "{props.media_url}",
                    alt: "{props.title}",
                    style: "width: 100%; height: 100%; object-fit: cover;",
                }

                // Play overlay for video/audio
                if let Some(icon) = overlay_icon {
                    if let Some(handler) = props.on_play.clone() {
                        div {
                            style: "position: absolute; inset: 0; display: flex; align-items: center; justify-content: center; background: rgba(0,0,0,0.3); cursor: pointer;",
                            onclick: move |_| handler.call(()),

                            div {
                                style: "width: 60px; height: 60px; background: white; border-radius: 50%; display: flex; align-items: center; justify-content: center;",

                                Icon {
                                    name: icon.to_string(),
                                    size: IconSize::Large,
                                    color: IconColor::Primary,
                                }
                            }
                        }
                    }
                }

                // Duration badge
                if let Some(duration) = props.duration.clone() {
                    div {
                        style: "position: absolute; bottom: 8px; right: 8px; background: rgba(0,0,0,0.7); color: white; padding: 2px 8px; border-radius: 4px; font-size: 12px;",
                        "{duration}"
                    }
                }
            }

            // Content
            CardContent {
                div {
                    style: "display: flex; justify-content: space-between; align-items: flex-start; gap: 8px;",

                    div {
                        style: "flex: 1; min-width: 0;",

                        Heading {
                            level: HeadingLevel::H4,
                            "{props.title}"
                        }

                        if let Some(creator) = props.creator.clone() {
                            Label {
                                size: TextSize::Small,
                                color: crate::atoms::TextColor::Muted,
                                "{creator}"
                            }
                        }
                    }

                    // Action buttons
                    div {
                        style: "display: flex; gap: 8px;",

                        if let Some(handler) = props.on_like.clone() {
                            button {
                                style: "background: none; border: none; cursor: pointer; padding: 4px;",
                                onclick: move |_| handler.call(()),

                                Icon {
                                    name: "heart".to_string(),
                                    size: IconSize::Medium,
                                    color: IconColor::Muted,
                                }
                            }
                        }

                        if let Some(handler) = props.on_share.clone() {
                            button {
                                style: "background: none; border: none; cursor: pointer; padding: 4px;",
                                onclick: move |_| handler.call(()),

                                Icon {
                                    name: "share".to_string(),
                                    size: IconSize::Medium,
                                    color: IconColor::Muted,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
