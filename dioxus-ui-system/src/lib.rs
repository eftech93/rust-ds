//! Dioxus UI System
//!
//! A pure Rust design system for Dioxus with Atomic Design principles.
//! 
//! ## Features
//!
//! - **Atomic Design Architecture**: Components organized as Atoms, Molecules, and Organisms
//! - **Type-Safe Theming**: Comprehensive theme system with light/dark/brand modes
//! - **Pure Rust Styling**: No CSS files - all styles generated in Rust
//! - **Tailwind-like API**: Fluent style builder for rapid UI development
//! - **Multi-Platform**: Works on Web (WASM), Desktop, and Mobile
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use dioxus_ui_system::prelude::*;
//!
//! fn App() -> Element {
//!     rsx! {
//!         ThemeProvider {
//!             Card {
//!                 CardHeader { title: "Welcome" }
//!                 CardContent {
//!                     Button {
//!                         variant: ButtonVariant::Primary,
//!                         "Click me"
//!                     }
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```

pub mod theme;
pub mod styles;
pub mod atoms;
pub mod molecules;
pub mod organisms;

/// Prelude module for convenient imports
pub mod prelude {
    //! Convenient re-exports for common types
    
    // Theme
    pub use crate::theme::{
        ThemeTokens, ThemeMode, ColorScale, Color, 
        SpacingScale, RadiusScale, TypographyScale, Typography,
        ThemeContext, ThemeProvider, use_theme, use_style, ThemeToggle, ThemeSelector,
    };
    
    // Styles
    pub use crate::styles::Style;
    pub use crate::{cx, style_if};
    
    // Atoms
    pub use crate::atoms::{
        Button, ButtonProps, ButtonVariant, ButtonSize, ButtonType, IconButton,
        Input, InputProps, InputType,
        Label, LabelProps, TextSize, TextWeight, TextColor, LabelElement, Heading, HeadingLevel, MutedText,
        Icon, IconProps, IconSize, IconColor, IconBtn as IconButton2,
        Checkbox, CheckboxProps,
        Radio, RadioProps, RadioGroup, RadioGroupProps, RadioDirection,
        Switch, SwitchProps, SwitchSize,
        Select, SelectProps, SelectOption, MultiSelect, MultiSelectProps,
        TextArea, TextAreaProps, AutoResizeTextArea, AutoResizeTextAreaProps,
        Box, BoxProps, BoxDisplay, FlexDirection, FlexWrap, JustifyContent, AlignItems,
        SpacingSize, RadiusSize, ShadowSize, BackgroundColor, BorderWidth, Overflow, Position,
        VStack, HStack, Center,
    };
    
    // Molecules
    pub use crate::molecules::{
        InputGroup,
        Card, CardProps, CardVariant, CardHeader, CardHeaderProps, 
        CardContent, CardContentProps, CardFooter, CardFooterProps, CardFooterJustify,
        Badge, BadgeProps, BadgeVariant, BadgeSize, StatusBadge, StatusBadgeProps, StatusType,
        Alert, AlertProps, AlertVariant,
        Avatar, AvatarProps, AvatarSize, AvatarGroup, AvatarGroupProps,
        Breadcrumb, BreadcrumbProps, BreadcrumbItem,
        Dialog, DialogProps, DialogFooter, DialogFooterProps, DialogFooterAlign, AlertDialog, AlertDialogProps,
        DropdownMenu, DropdownMenuProps, DropdownMenuItem, DropdownAlign, DropdownMenuSeparator, DropdownMenuLabel, DropdownMenuLabelProps,
        Popover, PopoverProps, PopoverPlacement, PopoverHeader, PopoverHeaderProps, PopoverFooter, PopoverFooterProps,
        Separator, SeparatorProps, SeparatorOrientation,
        Skeleton, SkeletonProps, SkeletonCircle, SkeletonCircleProps, SkeletonText, SkeletonTextProps, SkeletonCard, SkeletonCardProps,
        Tooltip, TooltipProps, TooltipPlacement, SimpleTooltip, SimpleTooltipProps,
    };
    
    // Organisms
    pub use crate::organisms::{
        Header, HeaderProps, NavItem, HeaderNavLink, MobileMenuToggle,
        UserMenu, UserMenuProps, UserMenuItem,
        DataTable, DataTableProps, TableColumn, ColumnAlign, Pagination, PaginationProps,
        Tabs, TabsProps, TabItem, TabsVariant, TabPanel, TabPanelProps, VerticalTabs, VerticalTabsProps,
        Accordion, AccordionProps, AccordionItem, AccordionItem2, AccordionItem2Props,
        Layout, LayoutProps, LayoutType, LayoutNavItem,
    };
    
    // Charts
    pub use crate::organisms::charts::{
        BarChart, BarChartProps, BarChartVariant,
        LineChart, LineChartProps, LineChartVariant,
        PieChart, PieChartProps, PieChartVariant, DonutChart, GaugeChart,
        Sparkline, SparklineProps, SparklineVariant, TrendIndicator,
        ChartDataPoint, ChartSeries, ChartMargin, ChartAxis, 
        LegendPosition, ChartTooltip, ChartAnimation, AnimationEasing,
        calculate_nice_ticks, format_compact_number, format_currency, format_percentage,
        palettes, utils,
    };
}

// Re-export at crate root for convenience
pub use theme::*;
pub use styles::*;
