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
        AlignItems, AspectRatio, AspectRatioProps, AspectRatios, AutoResizeTextArea,
        AutoResizeTextAreaProps, BackgroundColor, Blockquote, BlockquoteProps, BorderWidth, Box,
        BoxDisplay, BoxProps, Button, ButtonProps, ButtonSize, ButtonType, ButtonVariant, Caption,
        CaptionColor, CaptionProps, Center, Checkbox, CheckboxProps, DatePicker, DatePickerProps,
        DatePickerSize, Divider, DividerOrientation, DividerProps, DividerVariant, FlexDirection,
        FlexWrap, HStack, Heading, HeadingLevel, HeadingProps, Icon, IconBtn as IconButton2,
        IconButton, IconButtonProps, IconColor, IconProps, IconSize, Input, InputProps, InputTag,
        InputTagProps, InputType, JustifyContent, Label, LabelElement, LabelPosition, LabelProps,
        LoadingOverlay, LoadingOverlayProps, MultiSelect as MoleculeMultiSelect,
        MultiSelectProps as MoleculeMultiSelectProps, MutedText, NumberInput, NumberInputProps,
        Overflow, Paragraph, ParagraphProps, PasswordInput, PasswordInputProps, PasswordStrength,
        Position, Progress, ProgressProps, ProgressSize, ProgressVariant, Radio, RadioDirection,
        RadioGroup, RadioGroupProps, RadioProps, RadiusSize, RangeSlider, RangeSliderProps, Rating,
        RatingInput, RatingInputProps, RatingProps, ReviewSummary, ReviewSummaryProps, Select,
        SelectOption, SelectProps, ShadowSize, Skeleton, SkeletonAnimation, SkeletonAvatar,
        SkeletonAvatarProps, SkeletonCard as AtomSkeletonCard,
        SkeletonCardProps as AtomSkeletonCardProps, SkeletonList, SkeletonListProps, SkeletonProps,
        SkeletonShape, SkeletonText as AtomSkeletonText,
        SkeletonTextProps as AtomSkeletonTextProps, Slider, SliderMark, SliderProps, SliderSize,
        Spacer, SpacerDirection, SpacerProps, SpacerSize, SpacingSize, Spinner, SpinnerProps,
        SpinnerSize, SpinnerVariant, StepConnector, StepConnectorProps, StepIndicator,
        StepIndicatorProps, StepLabel, StepLabelProps, StepProgress, StepProgressProps, StepSize,
        StepState, Switch, SwitchProps, SwitchSize, Tag, TagData, TagGroup, TagGroupProps,
        TagProps, TagSize, TagVariant, TextArea, TextAreaProps, TextColor, TextSize, TextWeight,
        Toggle, ToggleProps, ToggleSize, ToggleVariant, VStack,
    };

    // Molecules
    pub use crate::molecules::{
        use_toast, ActionListItem, ActionListItemProps, Alert, AlertDialog, AlertDialogProps,
        AlertProps, AlertVariant, Avatar, AvatarGroup, AvatarGroupProps, AvatarProps, AvatarSize,
        Badge, BadgeProps, BadgeSize, BadgeVariant, Breadcrumb, BreadcrumbItem, BreadcrumbProps,
        Card, CardContent, CardContentProps, CardFooter, CardFooterJustify, CardFooterProps,
        CardHeader, CardHeaderProps, CardProps, CardVariant, Collapsible, CollapsibleGroup,
        CollapsibleGroupProps, CollapsibleProps, Combobox, ComboboxOption, ComboboxProps, Command,
        CommandEmpty, CommandEmptyProps, CommandGroup, CommandGroupProps, CommandInput,
        CommandInputProps, CommandItem, CommandItemProps, CommandList, CommandListProps,
        CommandLoading, CommandProps, CommandSeparator, CommandShortcut, CommandShortcutProps,
        Comment, CommentProps, ContextMenu, ContextMenuCheckboxItem, ContextMenuCheckboxItemProps,
        ContextMenuContent, ContextMenuContentProps, ContextMenuItem, ContextMenuItemProps,
        ContextMenuLabel, ContextMenuLabelProps, ContextMenuProps, ContextMenuSeparator,
        ContextMenuSub, ContextMenuSubProps, ContextMenuSubTrigger, ContextMenuSubTriggerProps,
        ContextMenuTrigger, ContextMenuTriggerProps, Dialog, DialogFooter, DialogFooterAlign,
        DialogFooterProps, DialogProps, DropdownAlign, DropdownMenu, DropdownMenuItem,
        DropdownMenuLabel, DropdownMenuLabelProps, DropdownMenuProps, DropdownMenuSeparator,
        ExpandableListItem, ExpandableListItemProps, HorizontalScroll, HorizontalScrollProps,
        HoverCard, HoverCardAlign, HoverCardAvatar, HoverCardAvatarProps, HoverCardContent,
        HoverCardContentProps, HoverCardFooter, HoverCardFooterProps, HoverCardHeader,
        HoverCardHeaderProps, HoverCardProps, HoverCardSide, HoverCardUserInfo,
        HoverCardUserInfoProps, IconToggleGroup, IconToggleItem, InputGroup, InputGroupProps,
        ListGroup, ListGroupProps, ListItem, ListItemProps, ListItemVariant, MediaAlign,
        MediaContent, MediaContentProps, MediaObject, MediaObjectProps, MultiCombobox,
        MultiComboboxProps, MultiSelect, MultiSelectProps, OtpInput, OtpInputProps,
        PageSizeSelector, PageSizeSelectorProps, Pagination, PaginationInfo, PaginationInfoProps,
        PaginationProps, PaginationSize, Popover, PopoverFooter, PopoverFooterProps, PopoverHeader,
        PopoverHeaderProps, PopoverPlacement, PopoverProps, QrCode, QrCodeLevel, QrCodeProps,
        QrCodeSvg, QrCodeSvgProps, ScrollArea, ScrollAreaProps, ScrollOrientation, ScrollViewport,
        ScrollViewportProps, Separator, SeparatorOrientation, SeparatorProps, Sheet, SheetFooter,
        SheetFooterAlign, SheetFooterProps, SheetProps, SheetSide, SimpleCollapsible,
        SimpleCollapsibleProps, SimpleTooltip, SimpleTooltipProps, SkeletonCard, SkeletonCardProps,
        SkeletonCircle, SkeletonCircleProps, SkeletonMolecule, SkeletonMoleculeProps, SkeletonText,
        SkeletonTextProps, Sonner, SonnerProps, SonnerVariant, StatusBadge, StatusBadgeProps,
        StatusType, StepContent, StepContentProps, StepItem, StepItemComponent, StepItemProps,
        StepperActions, StepperActionsAlign, StepperActionsProps, TimeInput, TimeInputProps,
        TimePicker, TimePickerProps, Toast, ToastAction, ToastManager, ToastPosition, ToastProps,
        ToastProvider, ToastProviderProps, ToastVariant, ToggleGroup, ToggleGroupProps,
        ToggleGroupType, ToggleItem, ToggleItemProps, Tooltip, TooltipPlacement, TooltipProps,
        UseSonner, VerticalScroll, VerticalScrollProps,
    };

    // Organisms
    pub use crate::organisms::{
        default_presets, use_carousel, use_resizable_panel_sizes, use_set_resizable_panel_size,
        use_tour, Accordion, AccordionItem, AccordionItem2, AccordionItem2Props, AccordionProps,
        ActionCard, ActionCardProps, AddColumnButton, AddColumnButtonProps, BannerAlert,
        BannerAlertProps, Calendar, CalendarMode, CalendarProps, Carousel, CarouselContent,
        CarouselContentProps, CarouselContext, CarouselDots, CarouselDotsProps, CarouselItem,
        CarouselItemProps, CarouselNext, CarouselNextProps, CarouselOptions, CarouselPrevious,
        CarouselPreviousProps, CarouselProps, ColumnAlign, CompactStepper, CompactStepperProps,
        ConfirmVariant, ConfirmationDialog, ConfirmationDialogProps, DataTable, DataTableFilter,
        DataTableProps, DateRangePicker, DateRangePickerProps, DateRangePreset,
        DeleteConfirmDialog, DeleteConfirmDialogProps, Direction, DualActionCard,
        DualActionCardProps, ExpandableCard, ExpandableCardProps, FileType, FileUpload,
        FileUploadProps, FilterOption, Footer, FooterLink, FooterLinkGroup, FooterProps,
        FooterVariant, FullRichText, FullRichTextProps, Header, HeaderNavLink, HeaderProps, Hero,
        HeroAlign, HeroCta, HeroProps, HeroSize, HeroWithImage, HeroWithImageProps, HorizontalCard,
        HorizontalCardProps, ImageActionCard, ImageActionCardProps, ImageCard, ImageCardProps,
        ImagePosition, ImageUploader, ImageUploaderProps, Kanban, KanbanCard, KanbanCardView,
        KanbanCardViewProps, KanbanColumn, KanbanColumnView, KanbanColumnViewProps, KanbanProps,
        KanbanTag, KanbanTagProps, Layout, LayoutNavItem, LayoutProps, LayoutType, MediaCard,
        MediaCardProps, MediaType, Menubar, MenubarAlign, MenubarCheckboxItem,
        MenubarCheckboxItemProps, MenubarContent, MenubarContentProps, MenubarItem,
        MenubarItemProps, MenubarLabel, MenubarLabelProps, MenubarMenu, MenubarMenuProps,
        MenubarProps, MenubarSeparator, MenubarSub, MenubarSubContent, MenubarSubContentProps,
        MenubarSubProps, MenubarSubTrigger, MenubarSubTriggerProps, MenubarTrigger,
        MenubarTriggerProps, MinimalRichText, MinimalRichTextProps, MobileMenuToggle, NavItem,
        Notification, NotificationCard, NotificationCardProps, NotificationCenter,
        NotificationCenterProps, NotificationType, Orientation, Placement, PricingCard,
        PricingCardProps, ProfileCard, ProfileCardProps, ResizableHandle, ResizableHandleProps,
        ResizablePanel, ResizablePanelGroup, ResizablePanelGroupProps, ResizablePanelProps,
        RichTextContent, RichTextContentProps, RichTextEditor, RichTextEditorProps,
        RichTextFeatures, RichTextToolbar, RichTextToolbarProps, SignOutDialog, SignOutDialogProps,
        SimpleCarousel, SimpleCarouselProps, SimpleFooter, SimpleFooterProps, SimpleRichText,
        SimpleRichTextProps, SocialProofBar, SocialProofBarProps, StatCard, StatCardProps,
        StepSummary, StepSummaryItem, StepSummaryProps, Stepper, StepperProps, StepperVariant,
        TabItem, TabPanel, TabPanelProps, TableColumn, TableFilter, TablePagination,
        TablePaginationProps, Tabs, TabsProps, TabsVariant, Timeline, TimelineContent,
        TimelineContentProps, TimelineDot, TimelineDotProps, TimelineDotSize, TimelineEvent,
        TimelineItem, TimelineItemProps, TimelineOppositeContent, TimelineOppositeContentProps,
        TimelinePosition, TimelineProps, TimelineSeparator, TimelineSeparatorProps, TouchCarousel,
        TouchCarouselProps, Tour, TourController, TourProps, TourStep, Tree, TreeNodeData,
        TreeProps, TreeWithState, UnsavedChangesDialog, UnsavedChangesDialogProps, UploadStatus,
        UploadedFile, UserMenu, UserMenuItem, UserMenuProps, VerticalTabs, VerticalTabsProps,
        Wizard, WizardProps, WizardStep,
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
