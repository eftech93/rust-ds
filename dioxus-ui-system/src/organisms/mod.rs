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

// Re-export all organism components
pub use header::{Header, HeaderProps, NavItem, HeaderNavLink, MobileMenuToggle, UserMenu, UserMenuProps, UserMenuItem};
pub use data_table::{DataTable, DataTableProps, TableColumn, ColumnAlign, Pagination, PaginationProps, TableFilter, FilterOption, DataTableFilter};
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
