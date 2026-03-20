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
