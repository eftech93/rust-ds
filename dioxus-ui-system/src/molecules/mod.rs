//! Atomic Design: Molecules
//!
//! Molecules are groups of atoms bonded together and are the smallest
//! fundamental units of a compound. These molecules take on their own
//! properties and serve as the backbone of our design systems.

pub mod input_group;
pub mod card;
pub mod badge;

// Re-export all molecule components
pub use input_group::InputGroup;
pub use card::{Card, CardProps, CardVariant, CardHeader, CardHeaderProps, CardContent, CardContentProps, CardFooter, CardFooterProps, CardFooterJustify};
pub use badge::{Badge, BadgeProps, BadgeVariant, BadgeSize, StatusBadge, StatusBadgeProps, StatusType};
