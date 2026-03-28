//! New Atom component documentation pages (Phase 1-4)

use crate::docs_ui::{CodeBlock, DocPage, ExampleBox, Section};
use dioxus::prelude::*;
use dioxus_ui_system::atoms::{
    AspectRatio, AspectRatios, NumberInput, PasswordInput, Toggle, ToggleSize, ToggleVariant,
};
use dioxus_ui_system::prelude::*;

/// Toggle documentation page
#[component]
pub fn TogglePage() -> Element {
    let mut pressed = use_signal(|| false);

    rsx! {
        DocPage {
            title: "Toggle",
            description: "A two-state button that can be either on or off.",

            Section { title: "Basic",
                ExampleBox {
                    Toggle {
                        pressed: pressed(),
                        on_pressed_change: move |p| pressed.set(p),
                        "Bold"
                    }
                }
                CodeBlock { code: r#"let mut pressed = use_signal(|| false);

Toggle {
    pressed: pressed(),
    on_pressed_change: move |p| pressed.set(p),
    "Bold"
}"#.to_string() }
            }

            Section { title: "Variants",
                ExampleBox {
                    HStack { gap: SpacingSize::Md,
                        Toggle { pressed: true, variant: ToggleVariant::Default, "Default" }
                        Toggle { pressed: true, variant: ToggleVariant::Outline, "Outline" }
                        Toggle { pressed: true, variant: ToggleVariant::Ghost, "Ghost" }
                    }
                }
            }

            Section { title: "Sizes",
                ExampleBox {
                    HStack { gap: SpacingSize::Md, style: "align-items: center;",
                        Toggle { pressed: true, size: ToggleSize::Sm, "Small" }
                        Toggle { pressed: true, size: ToggleSize::Md, "Medium" }
                        Toggle { pressed: true, size: ToggleSize::Lg, "Large" }
                    }
                }
            }
        }
    }
}

/// Number Input documentation page
#[component]
pub fn NumberInputPage() -> Element {
    let mut value = use_signal(|| 0.0);

    rsx! {
        DocPage {
            title: "Number Input",
            description: "An input with increment and decrement buttons for numeric values.",

            Section { title: "Basic",
                ExampleBox {
                    Box { style: "max-width: 200px;",
                        NumberInput {
                            value: value(),
                            on_change: move |v| value.set(v),
                        }
                    }
                }
                CodeBlock { code: r#"let mut value = use_signal(|| 0.0);

NumberInput {
    value: value(),
    on_change: move |v| value.set(v),
}"#.to_string() }
            }

            Section { title: "With Min/Max",
                ExampleBox {
                    Box { style: "max-width: 200px;",
                        NumberInput {
                            value: 50.0,
                            on_change: move |_| {},
                            min: Some(0.0),
                            max: Some(100.0),
                            step: 10.0,
                        }
                    }
                }
            }

            Section { title: "With Precision",
                ExampleBox {
                    Box { style: "max-width: 200px;",
                        NumberInput {
                            value: 3.14,
                            on_change: move |_| {},
                            precision: Some(2),
                            step: 0.01,
                        }
                    }
                }
            }
        }
    }
}

/// Aspect Ratio documentation page
#[component]
pub fn AspectRatioPage() -> Element {
    rsx! {
        DocPage {
            title: "Aspect Ratio",
            description: "A container that maintains a consistent aspect ratio for its content.",

            Section { title: "Basic",
                ExampleBox {
                    Box { style: "max-width: 400px;",
                        AspectRatio { ratio: 16.0/9.0,
                            Box {
                                style: "background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); display: flex; align-items: center; justify-content: center; color: white; font-weight: 600;",
                                "16:9 Aspect Ratio"
                            }
                        }
                    }
                }
                CodeBlock { code: r#"AspectRatio { ratio: 16.0/9.0,
    img { src: "image.jpg", style: "width: 100%; height: 100%; object-fit: cover;" }
}"#.to_string() }
            }

            Section { title: "Common Ratios",
                ExampleBox {
                    VStack { gap: SpacingSize::Md,
                        Box { style: "max-width: 200px;",
                            AspectRatio { ratio: AspectRatios::SQUARE,
                                Box { style: "background: #3b82f6; display: flex; align-items: center; justify-content: center; color: white;", "1:1" }
                            }
                        }
                        Box { style: "max-width: 300px;",
                            AspectRatio { ratio: AspectRatios::WIDESCREEN,
                                Box { style: "background: #22c55e; display: flex; align-items: center; justify-content: center; color: white;", "16:9" }
                            }
                        }
                        Box { style: "max-width: 200px;",
                            AspectRatio { ratio: AspectRatios::PHOTO,
                                Box { style: "background: #f59e0b; display: flex; align-items: center; justify-content: center; color: white;", "3:2" }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Password Input documentation page
#[component]
pub fn PasswordInputPage() -> Element {
    rsx! {
        DocPage {
            title: "Password Input",
            description: "A password input with show/hide toggle and optional strength indicator.",

            Section { title: "Basic",
                ExampleBox {
                    Box { style: "max-width: 300px;",
                        PasswordInput {
                            value: "password123".to_string(),
                            on_change: move |_| {},
                            label: Some("Password".to_string()),
                        }
                    }
                }
                CodeBlock { code: r#"PasswordInput {
    value: password(),
    on_change: move |v| password.set(v),
    label: Some("Password".to_string()),
}"#.to_string() }
            }

            Section { title: "With Strength Indicator",
                ExampleBox {
                    Box { style: "max-width: 300px;",
                        PasswordInput {
                            value: "MyStr0ng!Pass".to_string(),
                            on_change: move |_| {},
                            strength_indicator: true,
                            label: Some("Password".to_string()),
                        }
                    }
                }
            }

            Section { title: "With Error",
                ExampleBox {
                    Box { style: "max-width: 300px;",
                        PasswordInput {
                            value: "123".to_string(),
                            on_change: move |_| {},
                            error: Some("Password must be at least 8 characters".to_string()),
                            label: Some("Password".to_string()),
                        }
                    }
                }
            }
        }
    }
}
