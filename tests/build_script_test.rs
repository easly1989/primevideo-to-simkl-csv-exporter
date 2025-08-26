use std::fs;
use tempfile::tempdir;

mod build_support {
    include!("../build_support.rs");
}

#[test]
fn test_build_script_generates_config() {
    // Create a temporary directory for the test
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let dest_path = temp_dir.path();
    
    // Call the config generation function
    build_support::generate_config_if_needed(dest_path);
    
    // Check that config was created
    let config_path = dest_path.join("config.json");
    assert!(config_path.exists(), "Config file was not created");
    
    // Verify config content
    let config_content = fs::read_to_string(&config_path).expect("Failed to read config file");
    assert!(config_content.contains("YOUR_SIMKL_CLIENT_ID"), "Config missing Simkl ID");
    assert!(config_content.contains("YOUR_TMDB_ACCESS_TOKEN"), "Config missing TMDB token");
    assert!(config_content.contains("YOUR_AMAZON_EMAIL"), "Config missing Amazon email");
}