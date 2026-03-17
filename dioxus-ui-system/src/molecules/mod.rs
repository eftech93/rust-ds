//! Atomic Design: Molecules
//!
//! Molecules are groups of atoms bonded together and are the smallest
//! fundamental units of a compound. These molecules take on their own
//! properties and serve as the backbone of our design systems.

pub mod input_group;
pub mod card;
pub mod badge;
pub mod alert;
pub mod avatar;
pub mod breadcrumb;
pub mod dialog;
pub mod dropdown_menu;
pub mod popover;
pub mod separator;
pub mod skeleton;
pub mod tooltip;
pub mod stepper;

// Re-export all molecule components
pub use input_group::InputGroup;
pub use card::{Card, CardProps, CardVariant, CardHeader, CardHeaderProps, CardContent, CardContentProps, CardFooter, CardFooterProps, CardFooterJustify};
pub use badge::{Badge, BadgeProps, BadgeVariant, BadgeSize, StatusBadge, StatusBadgeProps, StatusType};
pub use alert::{Alert, AlertProps, AlertVariant};
pub use avatar::{Avatar, AvatarProps, AvatarSize, AvatarGroup, AvatarGroupProps};
pub use breadcrumb::{Breadcrumb, BreadcrumbProps, BreadcrumbItem};
pub use dialog::{Dialog, DialogProps, DialogFooter, DialogFooterProps, DialogFooterAlign, AlertDialog, AlertDialogProps};
pub use dropdown_menu::{DropdownMenu, DropdownMenuProps, DropdownMenuItem, DropdownAlign, DropdownMenuSeparator, DropdownMenuLabel, DropdownMenuLabelProps};
pub use popover::{Popover, PopoverProps, PopoverPlacement, PopoverHeader, PopoverHeaderProps, PopoverFooter, PopoverFooterProps};
pub use separator::{Separator, SeparatorProps, SeparatorOrientation};
pub use skeleton::{Skeleton, SkeletonProps, SkeletonCircle, SkeletonCircleProps, SkeletonText, SkeletonTextProps, SkeletonCard, SkeletonCardProps};
pub use tooltip::{Tooltip, TooltipProps, TooltipPlacement, SimpleTooltip, SimpleTooltipProps};
pub use stepper::{StepItem, HorizontalStepper, HorizontalStepperProps, VerticalStepper, VerticalStepperProps, StepContent, StepContentProps, StepperActions, StepperActionsProps, StepperActionsAlign, StepItemComponent, StepItemProps};
