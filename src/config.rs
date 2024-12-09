// config.rs
use crate::error::{TelemetryError, TelemetryResult};
use crate::utils::{is_interactive, prompt_yes_no};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct TelemetryConfig {
    /// Whether telemetry is enabled
    pub enabled: bool,
    /// Unique instance ID
    pub instance_id: String,
    /// Timestamp of when config was created
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Optional custom config path
    pub config_path: Option<PathBuf>,
}

impl TelemetryConfig {
    /// Creates a new config instance
    pub fn new(app_name: &str, custom_path: Option<PathBuf>) -> TelemetryResult<Self> {
        let config_path = Self::get_config_path(app_name, custom_path.clone());

        // If config file exists, load it
        if config_path.exists() {
            let file = std::fs::File::open(&config_path)
                .map_err(|e| TelemetryError::ConfigError(format!("Failed to open config file: {}", e)))?;
            
            return serde_json::from_reader(file)
                .map_err(|e| TelemetryError::ConfigError(format!("Failed to parse config: {}", e)));
        }

        // If we're not in interactive mode, disable telemetry
        if !is_interactive() {
            return Ok(Self {
                enabled: false,
                instance_id: uuid::Uuid::new_v4().to_string(),
                created_at: chrono::Utc::now(),
                config_path: Some(config_path),
            });
        }

        // Prompt user for telemetry consent
        println!("Help us improve ZKsync by sending anonymous usage data.");
        println!("We collect:");
        println!("  - Basic usage statistics");
        println!("  - Error reports");
        println!("  - Platform information");
        println!();
        println!("We DO NOT collect:");
        println!("  - Personal information");
        println!("  - Sensitive configuration");
        println!("  - Private keys or addresses");
        
        let enabled = prompt_yes_no("Would you like to enable telemetry?");

        let config = Self {
            enabled,
            instance_id: uuid::Uuid::new_v4().to_string(),
            created_at: chrono::Utc::now(),
            config_path: Some(config_path.clone()),
        };

        // Save the config
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| TelemetryError::ConfigError(format!("Failed to create config directory: {}", e)))?;
        }

        let file = std::fs::File::create(&config_path)
            .map_err(|e| TelemetryError::ConfigError(format!("Failed to create config file: {}", e)))?;
        
        serde_json::to_writer_pretty(file, &config)
            .map_err(|e| TelemetryError::ConfigError(format!("Failed to write config: {}", e)))?;

        Ok(config)
    }

    /// Gets the configuration file path
    pub fn get_config_path(app_name: &str, custom_path: Option<PathBuf>) -> PathBuf {
        if let Some(path) = custom_path {
            path
        } else {
            let proj_dirs = directories::ProjectDirs::from("com", "matter-labs", app_name)
                .expect("Failed to get project directories");
            proj_dirs.config_dir().join("telemetry.json")
        }
    }
}