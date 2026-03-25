//! Component configuration system
//!
//! Allows setting global defaults for all components, such as:
//! - Default spacing
//! - Default sizing
//! - Default styling variants
//!
//! # Example
//! ```rust,ignore
//! use dioxus_ui_system::config::ComponentConfig;
//!
//! // Configure global defaults
//! ComponentConfig::global()
//!     .button_defaults(|b| b
//!         .size(ButtonSize::Lg)
//!         .variant(ButtonVariant::Secondary))
//!     .card_defaults(|c| c
//!         .padding("24px".to_string()));
//! ```

use std::sync::{Mutex, OnceLock};

/// Global component configuration
#[derive(Clone, Debug, Default)]
pub struct Config {
    /// Default button size ("sm", "md", "lg", "icon")
    pub button_size: Option<String>,
    /// Default button variant ("primary", "secondary", "ghost", "destructive", "link")
    pub button_variant: Option<String>,
    /// Default card padding
    pub card_padding: Option<String>,
    /// Default input size
    pub input_size: Option<String>,
    /// Default spacing scale unit
    pub spacing_unit: Option<u8>,
    /// Default border radius
    pub border_radius: Option<String>,
    /// Whether to apply transitions globally
    pub enable_transitions: bool,
}

impl Config {
    /// Create a new config with defaults
    pub fn new() -> Self {
        Self {
            enable_transitions: true,
            ..Default::default()
        }
    }
    
    /// Set default button size
    pub fn with_button_size(mut self, size: impl Into<String>) -> Self {
        self.button_size = Some(size.into());
        self
    }
    
    /// Set default button variant
    pub fn with_button_variant(mut self, variant: impl Into<String>) -> Self {
        self.button_variant = Some(variant.into());
        self
    }
    
    /// Set default card padding
    pub fn with_card_padding(mut self, padding: impl Into<String>) -> Self {
        self.card_padding = Some(padding.into());
        self
    }
    
    /// Set default input size
    pub fn with_input_size(mut self, size: impl Into<String>) -> Self {
        self.input_size = Some(size.into());
        self
    }
    
    /// Set default spacing unit
    pub fn with_spacing_unit(mut self, unit: u8) -> Self {
        self.spacing_unit = Some(unit);
        self
    }
    
    /// Set default border radius
    pub fn with_border_radius(mut self, radius: impl Into<String>) -> Self {
        self.border_radius = Some(radius.into());
        self
    }
    
    /// Enable/disable transitions globally
    pub fn with_transitions(mut self, enabled: bool) -> Self {
        self.enable_transitions = enabled;
        self
    }
}

/// Global configuration singleton
static GLOBAL_CONFIG: OnceLock<Mutex<Config>> = OnceLock::new();

/// Initialize and get the global configuration
pub fn global_config() -> std::sync::MutexGuard<'static, Config> {
    GLOBAL_CONFIG
        .get_or_init(|| Mutex::new(Config::new()))
        .lock()
        .expect("Config mutex poisoned")
}

/// Set the global configuration
pub fn set_global_config(config: Config) {
    let mut global = global_config();
    *global = config;
}

/// Component configuration builder
/// 
/// Provides a fluent API for configuring global component defaults
pub struct ComponentConfig;

impl ComponentConfig {
    /// Start building configuration
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }
    
    /// Reset to default configuration
    pub fn reset() {
        set_global_config(Config::new());
    }
}

/// Configuration builder with fluent API
#[derive(Clone, Debug, Default)]
pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    /// Create a new config builder
    pub fn new() -> Self {
        Self {
            config: Config::new(),
        }
    }
    
    /// Build and apply the configuration
    pub fn build(self) {
        set_global_config(self.config);
    }
    
    /// Set default button size
    pub fn button_size(mut self, size: impl Into<String>) -> Self {
        self.config.button_size = Some(size.into());
        self
    }
    
    /// Set default button variant
    pub fn button_variant(mut self, variant: impl Into<String>) -> Self {
        self.config.button_variant = Some(variant.into());
        self
    }
    
    /// Set default card padding
    pub fn card_padding(mut self, padding: impl Into<String>) -> Self {
        self.config.card_padding = Some(padding.into());
        self
    }
    
    /// Set default input size
    pub fn input_size(mut self, size: impl Into<String>) -> Self {
        self.config.input_size = Some(size.into());
        self
    }
    
    /// Set default spacing unit
    pub fn spacing_unit(mut self, unit: u8) -> Self {
        self.config.spacing_unit = Some(unit);
        self
    }
    
    /// Set default border radius
    pub fn border_radius(mut self, radius: impl Into<String>) -> Self {
        self.config.border_radius = Some(radius.into());
        self
    }
    
    /// Enable/disable transitions
    pub fn transitions(mut self, enabled: bool) -> Self {
        self.config.enable_transitions = enabled;
        self
    }
}

/// Trait for components that can use global config
pub trait ConfigurableComponent {
    /// Get the component type name
    fn component_type() -> &'static str;
    
    /// Apply global defaults to props
    fn apply_defaults(props: &mut Self);
}

/// Helper macro to get config value or default
#[macro_export]
macro_rules! config_or_default {
    ($config_field:expr, $default:expr) => {
        $config_field.as_ref().map(|s| s.as_str()).unwrap_or($default)
    };
}
