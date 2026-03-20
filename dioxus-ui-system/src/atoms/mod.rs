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
pub mod box_component;
pub mod heading;
pub mod divider;
pub mod progress;
pub mod spinner;
pub mod skeleton;
pub mod rating;
pub mod date_picker;
pub mod slider;
pub mod tag;

// Re-export all atom components
pub use button::{Button, ButtonProps, ButtonVariant, ButtonSize, ButtonType, IconButton};
pub use input::{Input, InputProps, InputType};
pub use label::{Label, LabelProps, TextSize, TextWeight, TextColor, LabelElement, MutedText};
pub use icon::{Icon, IconProps, IconSize, IconColor, IconButton as IconBtn};
pub use checkbox::{Checkbox, CheckboxProps};
pub use radio::{Radio, RadioProps, RadioGroup, RadioGroupProps, RadioDirection};
pub use switch::{Switch, SwitchProps, SwitchSize};
pub use select::{Select, SelectProps, SelectOption, MultiSelect, MultiSelectProps};
pub use textarea::{TextArea, TextAreaProps, AutoResizeTextArea, AutoResizeTextAreaProps};
pub use step::{StepIndicator, StepIndicatorProps, StepConnector, StepConnectorProps, StepLabel, StepLabelProps, StepState, StepSize};
pub use box_component::{
    Box, BoxProps, BoxDisplay, FlexDirection, FlexWrap, JustifyContent, AlignItems,
    SpacingSize, RadiusSize, ShadowSize, BackgroundColor, BorderWidth, Overflow, Position,
    VStack, HStack, Center,
};
pub use heading::{Heading, HeadingProps, HeadingLevel, Paragraph, ParagraphProps, Caption, CaptionProps, CaptionColor, Blockquote, BlockquoteProps};
pub use divider::{Divider, DividerProps, DividerOrientation, DividerVariant, Spacer, SpacerProps, SpacerSize, SpacerDirection};
pub use progress::{Progress, ProgressProps, ProgressVariant, ProgressSize, LabelPosition, StepProgress, StepProgressProps};
pub use spinner::{Spinner, SpinnerProps, SpinnerVariant, SpinnerSize, LoadingOverlay, LoadingOverlayProps};
pub use skeleton::{Skeleton, SkeletonProps, SkeletonShape, SkeletonAnimation, SkeletonText, SkeletonTextProps, SkeletonCard, SkeletonCardProps, SkeletonAvatar, SkeletonAvatarProps, AvatarSize, SkeletonList, SkeletonListProps};
pub use rating::{Rating, RatingProps, RatingInput, RatingInputProps, ReviewSummary, ReviewSummaryProps};
pub use date_picker::{DatePicker, DatePickerProps, DatePickerSize};
pub use slider::{Slider, SliderProps, SliderSize, SliderMark, RangeSlider, RangeSliderProps};
pub use tag::{Tag, TagProps, TagVariant, TagSize, TagGroup, TagGroupProps, TagData, InputTag, InputTagProps};
