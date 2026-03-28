//! Atomic Design: Atoms
//!
//! Atoms are the basic building blocks of matter. Applied to web interfaces,
//! atoms are our HTML tags, such as a form label, an input, or a button.
//! Atoms can also include more abstract elements like color palettes, fonts,
//! and animations.

pub mod aspect_ratio;
pub mod box_component;
pub mod button;
pub mod checkbox;
pub mod date_picker;
pub mod divider;
pub mod heading;
pub mod icon;
pub mod input;
pub mod label;
pub mod number_input;
pub mod password_input;
pub mod progress;
pub mod radio;
pub mod rating;
pub mod select;
pub mod skeleton;
pub mod slider;
pub mod spinner;
pub mod step;
pub mod switch;
pub mod tag;
pub mod textarea;
pub mod toggle;

// Re-export all atom components
pub use aspect_ratio::{AspectRatio, AspectRatioProps, AspectRatios};
pub use box_component::{
    AlignItems, BackgroundColor, BorderWidth, Box, BoxDisplay, BoxProps, Center, FlexDirection,
    FlexWrap, HStack, JustifyContent, Overflow, Position, RadiusSize, ShadowSize, SpacingSize,
    VStack,
};
pub use button::{Button, ButtonProps, ButtonSize, ButtonType, ButtonVariant, IconButton};
pub use checkbox::{Checkbox, CheckboxProps};
pub use date_picker::{DatePicker, DatePickerProps, DatePickerSize};
pub use divider::{
    Divider, DividerOrientation, DividerProps, DividerVariant, Spacer, SpacerDirection,
    SpacerProps, SpacerSize,
};
pub use heading::{
    Blockquote, BlockquoteProps, Caption, CaptionColor, CaptionProps, Heading, HeadingLevel,
    HeadingProps, Paragraph, ParagraphProps,
};
pub use icon::{Icon, IconButton as IconBtn, IconColor, IconProps, IconSize};
pub use input::{Input, InputProps, InputType};
pub use label::{Label, LabelElement, LabelProps, MutedText, TextColor, TextSize, TextWeight};
pub use number_input::{NumberInput, NumberInputProps};
pub use password_input::{PasswordInput, PasswordInputProps, PasswordStrength};
pub use progress::{
    LabelPosition, Progress, ProgressProps, ProgressSize, ProgressVariant, StepProgress,
    StepProgressProps,
};
pub use radio::{Radio, RadioDirection, RadioGroup, RadioGroupProps, RadioProps};
pub use rating::{
    Rating, RatingInput, RatingInputProps, RatingProps, ReviewSummary, ReviewSummaryProps,
};
pub use select::{MultiSelect, MultiSelectProps, Select, SelectOption, SelectProps};
pub use skeleton::{
    AvatarSize, Skeleton, SkeletonAnimation, SkeletonAvatar, SkeletonAvatarProps, SkeletonCard,
    SkeletonCardProps, SkeletonList, SkeletonListProps, SkeletonProps, SkeletonShape, SkeletonText,
    SkeletonTextProps,
};
pub use slider::{RangeSlider, RangeSliderProps, Slider, SliderMark, SliderProps, SliderSize};
pub use spinner::{
    LoadingOverlay, LoadingOverlayProps, Spinner, SpinnerProps, SpinnerSize, SpinnerVariant,
};
pub use step::{
    StepConnector, StepConnectorProps, StepIndicator, StepIndicatorProps, StepLabel,
    StepLabelProps, StepSize, StepState,
};
pub use switch::{Switch, SwitchProps, SwitchSize};
pub use tag::{
    InputTag, InputTagProps, Tag, TagData, TagGroup, TagGroupProps, TagProps, TagSize, TagVariant,
};
pub use textarea::{AutoResizeTextArea, AutoResizeTextAreaProps, TextArea, TextAreaProps};
pub use toggle::{Toggle, ToggleProps, ToggleSize, ToggleVariant};
