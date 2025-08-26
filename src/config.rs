use serde::{Deserialize, Serialize};
use config::Config;
use std::path::PathBuf;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct AppConfig {
    pub simkl: SimklConfig,
    pub tmdb: TmdbConfig,
    pub tvdb: TvdbConfig,
    pub mal: MalConfig,
    pub amazon: AmazonConfig,
    pub output: OutputConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct SimklConfig {
    #[validate(length(min = 1, message = "Client ID cannot be empty"))]
    pub client_id: String,
    #[validate(length(min = 1, message = "Client secret cannot be empty"))]
    pub client_secret: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct TmdbConfig {
    #[validate(length(min = 1, message = "API key cannot be empty"))]
    pub api_key: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct TvdbConfig {
    #[validate(length(min = 1, message = "API key cannot be empty"))]
    pub api_key: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct MalConfig {
    #[validate(length(min = 1, message = "Client ID cannot be empty"))]
    pub client_id: String,
    #[validate(length(min = 1, message = "Client secret cannot be empty"))]
    pub client_secret: String,
}


#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct AmazonConfig {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 1, message = "Password cannot be empty"))]
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct OutputConfig {
    pub path: PathBuf,
}

impl AppConfig {
    pub fn load_with_cli_args(cli_args: &crate::cli::CliArgs) -> Result<Self, Box<dyn std::error::Error>> {
        // Get the executable's directory
        let exe_path = std::env::current_exe()?;
        let exe_dir = exe_path.parent().unwrap_or_else(|| std::path::Path::new("."));
        let config_path = exe_dir.join("config.json");

        // Config file should be generated during build - check if exists
        if !config_path.exists() {
            return Err(format!(
                "Config file not found at: {}. Please rebuild the project to generate the default config.",
                config_path.display()
            ).into());
        }

        let mut builder = Config::builder()
            .add_source(config::File::with_name(config_path.to_str().unwrap()).required(false));

        // Override with CLI arguments if provided
        if let Some(cli_config_path) = &cli_args.config {
            builder = builder.add_source(config::File::with_name(cli_config_path.to_str().unwrap()));
        }

        // Override specific values from CLI args
        if let Some(output_path) = &cli_args.output {
            builder = builder.set_override("output.path", output_path.to_str().unwrap())?;
        }

        let config = builder.build()?;
        let app_config: AppConfig = config.try_deserialize()?;

        // Validate the configuration
        app_config.validate().map_err(|e: validator::ValidationErrors| -> Box<dyn std::error::Error> {
            Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Configuration validation failed: {}", e)))
        })?;

        Ok(app_config)
    }

    pub fn validate(&self) -> Result<(), validator::ValidationErrors> {
        validator::Validate::validate(self)
    }
}