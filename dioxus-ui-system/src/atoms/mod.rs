//! Atomic Design: Atoms
//!
//! Atoms are the basic building blocks of matter. Applied to web interfaces,
//! atoms are our HTML tags, such as a form label, an input, or a button.
//! Atoms can also include more abstract elements like color palettes, fonts,
//! and animations.

pub mod button;
pub mod input;
pub mod label;
pub mod icon;

// Re-export all atom components
pub use button::{Button, ButtonProps, ButtonVariant, ButtonSize, ButtonType, IconButton};
pub use input::{Input, InputProps, InputType};
pub use label::{Label, LabelProps, TextSize, TextWeight, TextColor, LabelElement, Heading, HeadingLevel, MutedText};
pub use icon::{Icon, IconProps, IconSize, IconColor, IconButton as IconBtn};
