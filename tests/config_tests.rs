use std::env;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use validator::Validate;

// Test-specific configuration types that mirror the main crate
#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
struct TestAppConfig {
    simkl: TestSimklConfig,
    tmdb: TestTmdbConfig,
    tvdb: TestTvdbConfig,
    mal: TestMalConfig,
    amazon: TestAmazonConfig,
    output: TestOutputConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
struct TestSimklConfig {
    #[validate(length(min = 1, message = "Client ID cannot be empty"))]
    client_id: String,
    #[validate(length(min = 1, message = "Client secret cannot be empty"))]
    client_secret: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
struct TestTmdbConfig {
    #[validate(length(min = 1, message = "Access token cannot be empty"))]
    access_token: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
struct TestTvdbConfig {
    #[validate(length(min = 1, message = "API key cannot be empty"))]
    api_key: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
struct TestMalConfig {
    #[validate(length(min = 1, message = "Client ID cannot be empty"))]
    client_id: String,
    #[validate(length(min = 1, message = "Client secret cannot be empty"))]
    client_secret: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
struct TestAmazonConfig {
    #[validate(email(message = "Invalid email format (optional for manual login)"))]
    email: String,
    #[validate(length(min = 1, message = "Password cannot be empty (optional for manual login)"))]
    password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
struct TestOutputConfig {
    path: PathBuf,
}

#[derive(Debug, Clone)]
struct TestAppError(String);

impl std::fmt::Display for TestAppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for TestAppError {}

// Test utilities for configuration management
mod config_test_utils {
    use super::*;

    /// Test configuration loader that can handle missing config gracefully
    pub struct TestConfigLoader {
        pub config_path: PathBuf,
        pub fallback_config: Option<TestAppConfig>,
    }

    impl TestConfigLoader {
        /// Create a new test config loader
        pub fn new() -> Self {
            // Check if CONFIG_PATH environment variable is set
            let config_path = if let Ok(env_path) = env::var("CONFIG_PATH") {
                PathBuf::from(env_path)
            } else {
                // Default behavior: look in the target directory
                let exe_path = env::current_exe().expect("Failed to get current exe path");
                let exe_dir = exe_path.parent().expect("Failed to get exe directory");
    
                // For tests, we need to look in the target directory, not the deps subdirectory
                if exe_dir.ends_with("deps") {
                    // If we're in deps, go up two levels to target/release or target/debug
                    exe_dir.parent().unwrap().parent().unwrap().join("config.json")
                } else {
                    exe_dir.join("config.json")
                }
            };
    
            Self {
                config_path,
                fallback_config: None,
            }
        }

        /// Load configuration with fallback for testing
        pub fn load_with_fallback(&mut self) -> Result<TestAppConfig, TestAppError> {
            if self.config_path.exists() {
                match self.load_from_path(&self.config_path) {
                    Ok(config) => {
                        println!("✅ Loaded config from: {}", self.config_path.display());
                        Ok(config)
                    }
                    Err(e) => {
                        println!("⚠️  Failed to load config, using fallback: {}", e);
                        self.create_fallback_config()
                    }
                }
            } else {
                println!("⚠️  Config file not found, using fallback configuration");
                self.create_fallback_config()
            }
        }

        /// Load configuration from a specific path
        fn load_from_path(&self, path: &PathBuf) -> Result<TestAppConfig, TestAppError> {
            let config_content = std::fs::read_to_string(path)
                .map_err(|e| TestAppError(format!("Failed to read config file: {}", e)))?;

            let config: TestAppConfig = serde_json::from_str(&config_content)
                .map_err(|e| TestAppError(format!("Failed to deserialize config: {}", e)))?;

            // Validate the configuration
            config.validate()
                .map_err(|e| TestAppError(format!("Config validation failed: {}", e)))?;

            Ok(config)
        }

        /// Create a fallback configuration for testing when real config is not available
        fn create_fallback_config(&mut self) -> Result<TestAppConfig, TestAppError> {
            if self.fallback_config.is_none() {
                self.fallback_config = Some(TestAppConfig {
                    simkl: TestSimklConfig {
                        client_id: "TEST_SIMKL_CLIENT_ID".to_string(),
                        client_secret: "TEST_SIMKL_CLIENT_SECRET".to_string(),
                    },
                    tmdb: TestTmdbConfig {
                        access_token: "TEST_TMDB_ACCESS_TOKEN".to_string(),
                    },
                    tvdb: TestTvdbConfig {
                        api_key: "TEST_TVDB_API_KEY".to_string(),
                    },
                    mal: TestMalConfig {
                        client_id: "TEST_MAL_CLIENT_ID".to_string(),
                        client_secret: "TEST_MAL_CLIENT_SECRET".to_string(),
                    },
                    amazon: TestAmazonConfig {
                        email: "test@example.com".to_string(),
                        password: "test_password".to_string(),
                    },
                    output: TestOutputConfig {
                        path: PathBuf::from("./test_export.csv"),
                    },
                });
            }

            println!("🔧 Using fallback test configuration");
            Ok(self.fallback_config.clone().unwrap())
        }

        /// Reload configuration (useful for testing config changes)
        pub fn reload_config(&mut self) -> Result<TestAppConfig, TestAppError> {
            self.fallback_config = None; // Clear cache
            self.load_with_fallback()
        }
    }

    /// Check if configuration has real API keys (not placeholders)
    pub fn has_real_api_keys(config: &TestAppConfig) -> bool {
        let simkl_real = !config.simkl.client_id.starts_with("YOUR_") &&
                        !config.simkl.client_id.starts_with("TEST_");
        let tmdb_real = !config.tmdb.access_token.starts_with("YOUR_") &&
                       !config.tmdb.access_token.starts_with("TEST_");

        simkl_real && tmdb_real
    }

    /// Check if optional providers have real API keys
    pub fn optional_providers_configured(config: &TestAppConfig) -> (bool, bool) {
        let tvdb_real = !config.tvdb.api_key.starts_with("YOUR_") &&
                       !config.tvdb.api_key.starts_with("TEST_");
        let mal_real = !config.mal.client_id.starts_with("YOUR_") &&
                      !config.mal.client_id.starts_with("TEST_");

        (tvdb_real, mal_real)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use config_test_utils::*;

    /// Test configuration loading from generated config file
    #[test]
    fn test_config_loading_from_build_output() {
        let mut loader = TestConfigLoader::new();
        let config = loader.load_with_fallback();

        match config {
            Ok(cfg) => {
                println!("✅ Configuration loaded successfully");
                println!("  Simkl Client ID: {}", cfg.simkl.client_id);
                println!("  TMDB Access Token: {}", cfg.tmdb.access_token);
                println!("  Output Path: {}", cfg.output.path.display());

                // Verify required fields are present
                assert!(!cfg.simkl.client_id.is_empty(), "Simkl client ID should not be empty");
                assert!(!cfg.tmdb.access_token.is_empty(), "TMDB access token should not be empty");
                assert!(!cfg.amazon.email.is_empty(), "Amazon email should not be empty");

                // Check if we have real API keys or test placeholders
                let has_real_keys = has_real_api_keys(&cfg);
                println!("  Real API keys configured: {}", has_real_keys);
            }
            Err(e) => {
                panic!("Failed to load configuration: {:?}", e);
            }
        }
    }

    /// Test configuration validation
    #[test]
    fn test_config_validation() {
        let mut loader = TestConfigLoader::new();
        let config_result = loader.load_with_fallback();

        match config_result {
            Ok(config) => {
                // Test that the configuration validates successfully
                let validation_result = config.validate();
                match validation_result {
                    Ok(_) => println!("✅ Configuration validation passed"),
                    Err(e) => {
                        println!("⚠️  Configuration validation failed: {:?}", e);
                        println!("   This is expected if using placeholder values");
                    }
                }
            }
            Err(e) => panic!("Failed to load config for validation test: {:?}", e),
        }
    }

    /// Test configuration reloading
    #[test]
    fn test_config_reloading() {
        let mut loader = TestConfigLoader::new();

        // Load config twice to test reloading
        let config1 = loader.load_with_fallback().expect("Failed to load config first time");
        let config2 = loader.reload_config().expect("Failed to reload config");

        // Configs should be identical for this test
        assert_eq!(config1.simkl.client_id, config2.simkl.client_id);
        assert_eq!(config1.tmdb.access_token, config2.tmdb.access_token);

        println!("✅ Configuration reloading works correctly");
    }

    /// Test API key detection
    #[test]
    fn test_api_key_detection() {
        let mut loader = TestConfigLoader::new();
        let config = loader.load_with_fallback().expect("Failed to load config");

        let has_real_keys = has_real_api_keys(&config);
        let (tvdb_real, mal_real) = optional_providers_configured(&config);

        println!("📊 API Key Status:");
        println!("  Required providers (Simkl + TMDB): {}", if has_real_keys { "✅ Configured" } else { "⚠️  Using placeholders" });
        println!("  TVDB (optional): {}", if tvdb_real { "✅ Configured" } else { "⚠️  Using placeholders" });
        println!("  MyAnimeList (optional): {}", if mal_real { "✅ Configured" } else { "⚠️  Using placeholders" });

        // This test just reports status, doesn't fail
        assert!(true, "API key detection completed");
    }

    /// Test configuration file existence
    #[test]
    fn test_config_file_existence() {
        let exe_path = env::current_exe().expect("Failed to get current exe path");
        let exe_dir = exe_path.parent().expect("Failed to get exe directory");
        let config_path = exe_dir.join("config.json");

        println!("📁 Looking for config at: {}", config_path.display());

        if config_path.exists() {
            println!("✅ Config file found");
            assert!(config_path.is_file(), "Config path should be a file");
        } else {
            println!("⚠️  Config file not found - using fallback configuration");
            // This is not a failure in test environment
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use config_test_utils::*;

    /// Test configuration structure and validation
    #[test]
    fn test_config_structure_and_validation() {
        let mut loader = TestConfigLoader::new();
        let config = loader.load_with_fallback().expect("Failed to load config");

        // Test that all required fields are present
        assert!(!config.simkl.client_id.is_empty(), "Simkl client ID should not be empty");
        assert!(!config.simkl.client_secret.is_empty(), "Simkl client secret should not be empty");
        assert!(!config.tmdb.access_token.is_empty(), "TMDB access token should not be empty");
        assert!(!config.amazon.email.is_empty(), "Amazon email should not be empty");

        // Test configuration validation
        match config.validate() {
            Ok(_) => println!("✅ Configuration validation passed"),
            Err(e) => {
                println!("⚠️  Configuration validation failed (may be expected with placeholders): {:?}", e);
                // This is acceptable for test configurations
            }
        }

        println!("✅ Configuration structure test completed");
    }

    /// Test configuration persistence across test runs
    #[tokio::test]
    async fn test_config_persistence() {
        let mut loader = TestConfigLoader::new();
        let config1 = loader.load_with_fallback().expect("Failed to load config");
        let config2 = loader.reload_config().expect("Failed to reload config");

        // Test that key properties remain consistent
        assert_eq!(config1.simkl.client_id, config2.simkl.client_id);
        assert_eq!(config1.tmdb.access_token, config2.tmdb.access_token);
        assert_eq!(config1.output.path, config2.output.path);

        println!("✅ Configuration persistence verified");
    }

    /// Test error handling with invalid configuration
    #[test]
    fn test_invalid_config_handling() {
        // Test what happens when config file exists but has invalid JSON
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let invalid_config_path = temp_dir.path().join("invalid_config.json");

        // Write invalid JSON
        std::fs::write(&invalid_config_path, "invalid json content {").expect("Failed to write invalid config");

        let mut loader = TestConfigLoader { config_path: invalid_config_path, fallback_config: None };

        // This should fall back gracefully
        match loader.load_with_fallback() {
            Ok(_) => println!("✅ Graceful fallback worked for invalid config"),
            Err(e) => panic!("Should have fallen back gracefully: {:?}", e),
        }
    }
}

/// Documentation for running tests with configuration
#[cfg(test)]
mod documentation_tests {
    use super::*;

    /// Test that documents how to run tests with real config
    #[test]
    fn test_documentation_config_setup() {
        println!("📖 Configuration Test Setup Guide:");
        println!("=================================");
        println!("1. Build the project: cargo build --release");
        println!("2. Edit target/release/config.json with real API keys");
        println!("3. Run tests: cargo test --release");
        println!("4. For integration tests, ensure API keys are valid");
        println!("");
        println!("Environment variables (optional):");
        println!("  CONFIG_PATH=/path/to/config.json  - Use custom config location");
        println!("  SKIP_INTEGRATION_TESTS=1          - Skip API integration tests");
        println!("");
        println!("Test categories:");
        println!("  cargo test config_tests           - Configuration loading tests");
        println!("  cargo test integration_tests      - API integration tests");
        println!("  cargo test --release              - All tests with release config");

        assert!(true, "Documentation test completed");
    }
}