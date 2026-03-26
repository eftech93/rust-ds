//! Atomic Design: Organisms
//!
//! Organisms are groups of molecules joined together to form a relatively
//! complex, distinct section of an interface.

pub mod header;
pub mod data_table;
pub mod tabs;
pub mod accordion;
pub mod layout;
pub mod cards;
pub mod stepper;
pub mod charts;
pub mod footer;
pub mod notification_center;
pub mod hero;
pub mod file_upload;
pub mod confirmation_dialog;
pub mod resizable;
pub mod date_range_picker;
pub mod calendar;
pub mod carousel;
pub mod tree;
pub mod timeline;
pub mod menubar;
pub mod tour;
pub mod image_uploader;
pub mod rich_text;
pub mod kanban;

// Re-export all organism components
pub use header::{Header, HeaderProps, NavItem, HeaderNavLink, MobileMenuToggle, UserMenu, UserMenuProps, UserMenuItem};
pub use data_table::{DataTable, DataTableProps, TableColumn, ColumnAlign, Pagination as TablePagination, PaginationProps as TablePaginationProps, TableFilter, FilterOption, DataTableFilter};
pub use tabs::{Tabs, TabsProps, TabItem, TabsVariant, TabPanel, TabPanelProps, VerticalTabs, VerticalTabsProps};
pub use accordion::{Accordion, AccordionProps, AccordionItem, AccordionItem2, AccordionItem2Props};
pub use layout::{Layout, LayoutProps, LayoutType, LayoutNavItem};
pub use cards::{
    ActionCard, ActionCardProps,
    DualActionCard, DualActionCardProps,
    ImageCard, ImageCardProps,
    ImageActionCard, ImageActionCardProps,
    ProfileCard, ProfileCardProps,
    PricingCard, PricingCardProps,
    HorizontalCard, HorizontalCardProps,
    NotificationCard, NotificationCardProps, NotificationType,
    StatCard, StatCardProps,
    ExpandableCard, ExpandableCardProps,
    MediaCard, MediaCardProps, MediaType,
};
pub use stepper::{
    Stepper, StepperProps, StepperVariant,
    Wizard, WizardProps, WizardStep,
    StepSummary, StepSummaryProps, StepSummaryItem,
    CompactStepper, CompactStepperProps,
};
pub use footer::{Footer, FooterProps, FooterVariant, FooterLinkGroup, FooterLink, SimpleFooter, SimpleFooterProps};
pub use notification_center::{NotificationCenter, NotificationCenterProps, Notification, BannerAlert, BannerAlertProps};
pub use hero::{Hero, HeroProps, HeroAlign, HeroSize, HeroCta, HeroWithImage, HeroWithImageProps, ImagePosition, SocialProofBar, SocialProofBarProps};
pub use file_upload::{FileUpload, FileUploadProps, UploadedFile};
pub use confirmation_dialog::{ConfirmationDialog, ConfirmationDialogProps, ConfirmVariant, DeleteConfirmDialog, DeleteConfirmDialogProps, UnsavedChangesDialog, UnsavedChangesDialogProps, SignOutDialog, SignOutDialogProps};
pub use resizable::{
    ResizablePanelGroup, ResizablePanelGroupProps, Direction,
    ResizablePanel, ResizablePanelProps,
    ResizableHandle, ResizableHandleProps,
    use_resizable_panel_sizes, use_set_resizable_panel_size,
};
pub use date_range_picker::{DateRangePicker, DateRangePickerProps, DateRangePreset, default_presets};
pub use calendar::{
    Calendar, CalendarProps, CalendarMode,
};
pub use carousel::{
    Carousel, CarouselProps, CarouselContent, CarouselContentProps,
    CarouselItem, CarouselItemProps, CarouselPrevious, CarouselPreviousProps,
    CarouselNext, CarouselNextProps, CarouselDots, CarouselDotsProps,
    CarouselOptions, Orientation, SimpleCarousel, SimpleCarouselProps,
    TouchCarousel, TouchCarouselProps, use_carousel, CarouselContext,
};
pub use tree::{
    Tree, TreeProps, TreeNodeData, TreeWithState,
};
pub use timeline::{
    Timeline, TimelineProps, TimelinePosition,
    TimelineItem, TimelineItemProps,
    TimelineDot, TimelineDotProps, TimelineDotSize,
    TimelineContent, TimelineContentProps,
    TimelineSeparator, TimelineSeparatorProps,
    TimelineOppositeContent, TimelineOppositeContentProps,
    TimelineEvent, SimpleTimeline, SimpleTimelineProps,
};
pub use menubar::{
    Menubar, MenubarProps,
    MenubarMenu, MenubarMenuProps,
    MenubarTrigger, MenubarTriggerProps,
    MenubarContent, MenubarContentProps, MenubarAlign,
    MenubarItem, MenubarItemProps,
    MenubarSeparator,
    MenubarSub, MenubarSubProps,
    MenubarSubTrigger, MenubarSubTriggerProps,
    MenubarSubContent, MenubarSubContentProps,
    MenubarLabel, MenubarLabelProps,
    MenubarCheckboxItem, MenubarCheckboxItemProps,
};
pub use tour::{
    Tour, TourProps, TourStep, Placement,
    use_tour, TourController,
};
pub use image_uploader::{
    ImageUploader, ImageUploaderProps,
    UploadedImage, UploadStatus,
};
pub use rich_text::{
    RichTextEditor, RichTextEditorProps,
    RichTextToolbar, RichTextToolbarProps,
    RichTextContent, RichTextContentProps,
    RichTextFeatures,
    SimpleRichText, SimpleRichTextProps,
    MinimalRichText, MinimalRichTextProps,
    FullRichText, FullRichTextProps,
};
pub use kanban::{
    Kanban, KanbanProps,
    KanbanColumn, KanbanCard,
    KanbanColumnView, KanbanColumnViewProps,
    KanbanCardView, KanbanCardViewProps,
    KanbanTag, KanbanTagProps,
    AddColumnButton, AddColumnButtonProps,
    SimpleKanban, SimpleKanbanProps,
};
