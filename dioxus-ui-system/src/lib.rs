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

pub mod atoms;
pub mod config;
pub mod molecules;
pub mod organisms;
pub mod styles;
pub mod theme;

/// Prelude module for convenient imports
pub mod prelude {
    //! Convenient re-exports for common types

    // Config
    pub use crate::config::{
        global_config, set_global_config, ComponentConfig, Config, ConfigBuilder,
    };

    // Theme
    pub use crate::theme::{
        use_style, use_theme, Color, ColorScale, RadiusScale, SpacingScale, ThemeContext,
        ThemeMode, ThemeProvider, ThemeSelector, ThemeToggle, ThemeTokens, Typography,
        TypographyScale,
    };

    // Styles
    pub use crate::styles::Style;
    pub use crate::{cx, style_if};

    // Atoms
    pub use crate::atoms::{
        AlignItems,
        AutoResizeTextArea,
        AutoResizeTextAreaProps,
        BackgroundColor,
        BorderWidth,
        Box,
        BoxDisplay,
        BoxProps,
        Button,
        ButtonProps,
        ButtonSize,
        ButtonType,
        ButtonVariant,
        Center,
        Checkbox,
        CheckboxProps,
        DatePicker,
        DatePickerProps,
        // New atoms
        Divider,
        DividerOrientation,
        DividerProps,
        DividerVariant,
        FlexDirection,
        FlexWrap,
        HStack,
        Heading,
        HeadingLevel,
        Icon,
        IconBtn as IconButton2,
        IconButton,
        IconColor,
        IconProps,
        IconSize,
        Input,
        InputProps,
        InputTag,
        InputTagProps,
        InputType,
        JustifyContent,
        Label,
        LabelElement,
        LabelProps,
        MultiSelect,
        MultiSelectProps,
        MutedText,
        Overflow,
        Position,
        Progress,
        ProgressProps,
        ProgressSize,
        ProgressVariant,
        Radio,
        RadioDirection,
        RadioGroup,
        RadioGroupProps,
        RadioProps,
        RadiusSize,
        Rating,
        RatingProps,
        Select,
        SelectOption,
        SelectProps,
        ShadowSize,
        Skeleton,
        SkeletonAnimation,
        SkeletonCard as AtomSkeletonCard,
        SkeletonCardProps as AtomSkeletonCardProps,
        SkeletonProps,
        SkeletonShape,
        SkeletonText as AtomSkeletonText,
        SkeletonTextProps as AtomSkeletonTextProps,
        Slider,
        SliderMark,
        SliderProps,
        SpacingSize,
        Spinner,
        SpinnerProps,
        SpinnerSize,
        SpinnerVariant,
        Switch,
        SwitchProps,
        SwitchSize,
        Tag,
        TagData,
        TagGroup,
        TagProps,
        TagSize,
        TagVariant,
        TextArea,
        TextAreaProps,
        TextColor,
        TextSize,
        TextWeight,
        VStack,
    };

    // Molecules
    pub use crate::molecules::{
        use_toast,
        ActionListItem,
        ActionListItemProps,
        Alert,
        AlertDialog,
        AlertDialogProps,
        AlertProps,
        AlertVariant,
        Avatar,
        AvatarGroup,
        AvatarGroupProps,
        AvatarProps,
        AvatarSize,
        Badge,
        BadgeProps,
        BadgeSize,
        BadgeVariant,
        Breadcrumb,
        BreadcrumbItem,
        BreadcrumbProps,
        Card,
        CardContent,
        CardContentProps,
        CardFooter,
        CardFooterJustify,
        CardFooterProps,
        CardHeader,
        CardHeaderProps,
        CardProps,
        CardVariant,
        Combobox,
        ComboboxOption,
        ComboboxProps,
        // Command palette
        Command,
        CommandEmpty,
        CommandEmptyProps,
        CommandGroup,
        CommandGroupProps,
        CommandInput,
        CommandInputProps,
        CommandItem,
        CommandItemProps,
        CommandList,
        CommandListProps,
        CommandLoading,
        CommandProps,
        CommandSeparator,
        CommandShortcut,
        CommandShortcutProps,
        Comment,
        CommentProps,
        // Context Menu
        ContextMenu,
        ContextMenuCheckboxItem,
        ContextMenuCheckboxItemProps,
        ContextMenuContent,
        ContextMenuContentProps,
        ContextMenuItem,
        ContextMenuItemProps,
        ContextMenuLabel,
        ContextMenuLabelProps,
        ContextMenuProps,
        ContextMenuSeparator,
        ContextMenuSub,
        ContextMenuSubProps,
        ContextMenuSubTrigger,
        ContextMenuSubTriggerProps,
        ContextMenuTrigger,
        ContextMenuTriggerProps,
        Dialog,
        DialogFooter,
        DialogFooterAlign,
        DialogFooterProps,
        DialogProps,
        DropdownAlign,
        DropdownMenu,
        DropdownMenuItem,
        DropdownMenuLabel,
        DropdownMenuLabelProps,
        DropdownMenuProps,
        DropdownMenuSeparator,
        ExpandableListItem,
        ExpandableListItemProps,
        InputGroup,
        ListGroup,
        ListGroupProps,
        ListItem,
        ListItemProps,
        ListItemVariant,
        MediaContent,
        MediaContentProps,
        MediaObject,
        MediaObjectProps,
        MultiCombobox,
        MultiComboboxProps,
        // OTP Input
        OtpInput,
        OtpInputProps,
        PageSizeSelector,
        PageSizeSelectorProps,
        Pagination,
        PaginationInfo,
        PaginationInfoProps,
        PaginationProps,
        PaginationSize,
        Popover,
        PopoverFooter,
        PopoverFooterProps,
        PopoverHeader,
        PopoverHeaderProps,
        PopoverPlacement,
        PopoverProps,
        Separator,
        SeparatorOrientation,
        SeparatorProps,
        // Sheet
        Sheet,
        SheetFooter,
        SheetFooterAlign,
        SheetFooterProps,
        SheetProps,
        SheetSide,
        SimpleTooltip,
        SimpleTooltipProps,
        SkeletonCard,
        SkeletonCardProps,
        SkeletonCircle,
        SkeletonCircleProps,
        SkeletonMolecule,
        SkeletonMoleculeProps,
        SkeletonText,
        SkeletonTextProps,
        StatusBadge,
        StatusBadgeProps,
        StatusType,
        TimeInput,
        TimeInputProps,
        // Time Picker
        TimePicker,
        TimePickerProps,
        // New molecules
        Toast,
        ToastManager,
        ToastProps,
        ToastProvider,
        ToastProviderProps,
        ToastVariant,
        Tooltip,
        TooltipPlacement,
        TooltipProps,
    };

    // Organisms
    pub use crate::organisms::{
        Accordion, AccordionItem, AccordionItem2, AccordionItem2Props, AccordionProps, Calendar,
        CalendarMode, CalendarProps, ColumnAlign, DataTable, DataTableProps, FilterOption,
        FullRichText, FullRichTextProps, Header, HeaderNavLink, HeaderProps, Layout, LayoutNavItem,
        LayoutProps, LayoutType, MinimalRichText, MinimalRichTextProps, MobileMenuToggle, NavItem,
        RichTextEditor, RichTextEditorProps, RichTextFeatures, SimpleRichText, SimpleRichTextProps,
        TabItem, TabPanel, TabPanelProps, TableColumn, TableFilter, TablePagination,
        TablePaginationProps, Tabs, TabsProps, TabsVariant, UserMenu, UserMenuItem, UserMenuProps,
        VerticalTabs, VerticalTabsProps,
    };

    // Charts
    pub use crate::organisms::charts::{
        calculate_nice_ticks, format_compact_number, format_currency, format_percentage, palettes,
        utils, AnimationEasing, BarChart, BarChartProps, BarChartVariant, ChartAnimation,
        ChartAxis, ChartDataPoint, ChartMargin, ChartSeries, ChartTooltip, DonutChart, GaugeChart,
        LegendPosition, LineChart, LineChartProps, LineChartVariant, PieChart, PieChartProps,
        PieChartVariant, Sparkline, SparklineProps, SparklineVariant, TrendIndicator,
    };
}

// Re-export at crate root for convenience
pub use config::*;
pub use styles::*;
pub use theme::*;
