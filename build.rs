use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Get the output directory
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).parent().unwrap().parent().unwrap().parent().unwrap();
    
    // Create default config content
    let config_content = r#"{
  "simkl": {
    "client_id": "YOUR_SIMKL_CLIENT_ID",
    "client_secret": "YOUR_SIMKL_CLIENT_SECRET"
  },
  "tmdb": {
    "access_token": "YOUR_TMDB_ACCESS_TOKEN"
  },
  "tvdb": {
    "api_key": "YOUR_TVDB_API_KEY"
  },
  "imdb": {
    "api_key": "YOUR_IMDB_API_KEY"
  },
  "mal": {
    "client_id": "YOUR_MAL_CLIENT_ID",
    "client_secret": "YOUR_MAL_CLIENT_SECRET"
  },
  "amazon": {
    "email": "YOUR_AMAZON_EMAIL",
    "password": "YOUR_AMAZON_PASSWORD"
  },
  "output": {
    "path": "./export.csv"
  }
}"#;

    // Write config to target directory
    let config_path = dest_path.join("config.json");
    if !config_path.exists() {
        fs::write(&config_path, config_content).expect("Failed to write config.json");
        println!("cargo:warning=Generated default config file at: {}", config_path.display());
    } else {
        println!("cargo:warning=Config file already exists at: {}", config_path.display());
    }
}