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
pub mod checkbox;
pub mod radio;
pub mod switch;
pub mod select;
pub mod textarea;
pub mod step;

// Re-export all atom components
pub use button::{Button, ButtonProps, ButtonVariant, ButtonSize, ButtonType, IconButton};
pub use input::{Input, InputProps, InputType};
pub use label::{Label, LabelProps, TextSize, TextWeight, TextColor, LabelElement, Heading, HeadingLevel, MutedText};
pub use icon::{Icon, IconProps, IconSize, IconColor, IconButton as IconBtn};
pub use checkbox::{Checkbox, CheckboxProps};
pub use radio::{Radio, RadioProps, RadioGroup, RadioGroupProps, RadioDirection};
pub use switch::{Switch, SwitchProps, SwitchSize};
pub use select::{Select, SelectProps, SelectOption, MultiSelect, MultiSelectProps};
pub use textarea::{TextArea, TextAreaProps, AutoResizeTextArea, AutoResizeTextAreaProps};
pub use step::{StepIndicator, StepIndicatorProps, StepConnector, StepConnectorProps, StepLabel, StepLabelProps, StepState, StepSize};
