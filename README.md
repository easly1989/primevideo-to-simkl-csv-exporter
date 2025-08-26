# Prime Video to Simkl CSV Exporter (Rust Edition)

A modern Rust-based tool to export your Amazon Prime Video watch history to a CSV file compatible with [Simkl](https://simkl.com/apps/import/csv/) import format.

## If you like my work
Help me pay off my home loan → [Donate on PayPal](https://paypal.me/ruggierocarlo)

## Features

- **Manual login required** - secure authentication with Prime Video
- **Automated scraping** of Prime Video watch history after login
- **Metadata enrichment** from multiple sources:
  - [Simkl](https://simkl.com/) - primary metadata provider
  - [TMDB](https://www.themoviedb.org/) - movie and TV show details
  - [TVDB](https://thetvdb.com/) - TV show metadata (optional)
  - [MyAnimeList](https://myanimelist.net/) - anime-specific metadata (optional)
- **Smart deduplication** - only includes last watched episode for TV shows
- **CSV generation** in Simkl import format
- **Easy configuration** - generates config file during build with helpful comments
- **Comprehensive validation** of API keys and credentials
- **Concurrent processing** for fast metadata lookups
- **Robust error handling** with automatic retries

## Prerequisites

- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Amazon Prime Video account
- WebDriver for browser automation:
  - Chrome: [Download ChromeDriver](https://chromedriver.chromium.org/downloads) (match your Chrome version)
  - Firefox: [Download GeckoDriver](https://github.com/mozilla/geckodriver/releases)
  - **Installation**:
    - Download the appropriate driver for your browser
    - Add it to your system PATH
    - **Note**: Driver path configuration in config.json is not required - the application connects to localhost:4444 automatically
  - Verify installation by running in terminal:
    ```bash
    chromedriver --version  # For Chrome
    geckodriver --version   # For Firefox
    ```
- API credentials (required for metadata enrichment):
  - **Simkl** (Primary provider): [Create app](https://simkl.com/settings/developer/new/) → Get Client ID/Secret
  - **TMDB** (Movie/TV details): [Get API Key](https://www.themoviedb.org/settings/api) → Use "API Read Access Token"
  - **TVDB** (TV show metadata): [Get API Key](https://thetvdb.com/api-information) → Register for free
  - **MyAnimeList** (Anime metadata): [Create app](https://myanimelist.net/apiconfig/create) → Get Client ID/Secret

  **Note**: Without proper API keys, the application will fail with "All providers failed". You can skip optional providers by leaving them as placeholder values.

## Installation & Setup

1. Clone the repository:
```bash
git clone https://github.com/yourusername/primevideo-to-simkl-csv-exporter.git
cd primevideo-to-simkl-csv-exporter
```

2. Build the project:
```bash
cargo build --release
```
This will automatically generate a `config.json` file in the target directory (`target/release/`).

3. Edit `config.json` with your credentials:
**Important**: The config file must be properly edited before the application can function correctly. The application will exit with an error message if the config is not properly configured.
```json
{
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
  "mal": {
    "client_id": "YOUR_MAL_CLIENT_ID",
    "client_secret": "YOUR_MAL_CLIENT_SECRET"
  },
  "amazon": {
    "email": "YOUR_AMAZON_EMAIL (optional for manual login)",
    "password": "YOUR_AMAZON_PASSWORD (optional for manual login)"
  },
  "output": {
    "path": "./export.csv"
  }
}
```
Replace all placeholder values (starting with "YOUR_") with your actual API keys and credentials.

> **Security Note**: The `config.json` contains sensitive credentials. Keep it secure and never commit to version control.

## Usage

Run the application:
```bash
cargo run --release
```

The application will:
1. Launch a browser window
2. **Require manual login** to Amazon Prime Video (autologin has been disabled)
3. Guide you through the login process with clear instructions
4. **Wait for you to press Enter** in the terminal once you've logged in
5. Verify you're on the correct page and proceed with scraping
6. Enrich items with metadata
7. Generate `export.csv` in Simkl format

## CSV Format

The generated CSV contains these columns:

| Column          | Description                          |
|-----------------|--------------------------------------|
| `simkl_id`      | Simkl ID for the title               |
| `TVDB_ID`       | TVDB ID (if available)               |
| `TMDB`          | TMDB ID                              |
| `MAL_ID`        | MyAnimeList ID (for anime)           |
| `Type`          | "movie" or "tv"                      |
| `Title`         | Title of the movie/show              |
| `Year`          | Release year                         |
| `LastEpWatched` | Last episode watched (e.g., "s1e2")  |
| `Watchlist`     | Always "completed"                   |
| `WatchedDate`   | Date watched (YYYY-MM-DD)            |
| `Rating`        | Your rating (empty)                  |
| `Memo`          | Notes (empty)                        |

## Importing to Simkl

1. Visit [Simkl CSV Import](https://simkl.com/apps/import/csv/)
2. Upload `export.csv`
3. Follow the import wizard

## Troubleshooting

- **Login Issues**:
  - Complete manual login in the browser window when prompted
  - The application will guide you through the login process
  - Press Enter in the terminal once you've successfully logged in
  - Ensure you're on the Prime Video watch history page after login
  - For 2FA, complete the authentication process as required
  - If you get a login error, the application will show your current URL and specific instructions
  - URLs with "auth" parameters (like `ref_=atv_auth_red_aft`) are normal and won't trigger login errors

- **Large Watch Histories**:
  - Processing may take time (1-2 items/second)
  - API rate limits are automatically handled

- **WebDriver Issues**:
  - Ensure ChromeDriver/GeckoDriver is installed and in PATH
  - Verify installation:
    ```bash
    chromedriver --version  # Should output version
    ```
  - Start WebDriver manually before running the app:
    - ChromeDriver: `chromedriver --port=4444`
    - GeckoDriver: `geckodriver --port 4444`
  - If using Chrome, ensure Chrome browser is installed
  - If using Firefox, ensure Firefox browser is installed
  - **Note**: WebDriver path configuration is not required in config.json - the application connects automatically to localhost:4444

- **Metadata Failures**:
  - **"All providers failed" error**: Check that your API keys are properly set (not placeholder values)
  - **Invalid API keys**: Verify your credentials are correct and active
  - **Network issues**: Ensure you have internet connection
  - **Rate limits**: Some APIs have request limits - wait and retry if needed
  - **Optional providers**: You can leave TVDB/MAL as placeholders if you don't need them
  - Some obscure titles may not be found in any database

- **Browser Compatibility**:
  - Tested with latest Chrome and Firefox
  - Ensure browser automation is not blocked

## API Key Testing

To test if your API keys are working before running the full application:

1. **Simkl**: Visit [Simkl API](https://api.simkl.com/) and test your credentials
2. **TMDB**: Visit [TMDB API](https://www.themoviedb.org/documentation/api) and try a simple request
3. **TVDB**: Check your API key on their [developer page](https://thetvdb.com/api-information)
4. **MyAnimeList**: Test your credentials on their [API config page](https://myanimelist.net/apiconfig)

**Note**: The application will tell you which specific API is failing if you check the detailed error logs.

## Testing

### Run Configuration Tests
```bash
cargo test --test config_tests -- --nocapture
```

Configuration tests verify:
- ✅ Configuration loading from generated config.json
- ✅ Fallback behavior for missing config files
- ✅ Configuration validation and error handling
- ✅ API key detection and placeholder handling
- ✅ Configuration reloading and persistence
- ✅ Graceful error handling for invalid config

### Run API Integration Tests
```bash
cargo test --test api_integration_tests -- --nocapture
```

Integration tests verify:
- ✅ Mock API endpoint testing
- ✅ HTTP client functionality
- ✅ Error handling and rate limiting
- ✅ Response parsing and validation
- ✅ JSON data integrity
- ✅ URL construction
- ✅ Mock server setup

**Test Results**: 8/8 tests passing ✅

### Run All Tests
```bash
cargo test
```

### Test Configuration Setup

The test suite dynamically loads configuration from the generated `config.json` file:

1. **Build generates config**: `cargo build --release` creates `target/release/config.json`
2. **Tests load dynamically**: Configuration tests read from the generated file
3. **Fallback handling**: Tests gracefully handle missing or invalid configuration
4. **API key validation**: Tests detect and report on placeholder vs real API keys

### Quick Start for Your Setup

If you have a `config.json` file in your `target/release/` directory with real API keys, run:

**Windows PowerShell:**
```powershell
$env:CONFIG_PATH = "target/release/config.json"; cargo test --test config_tests -- --nocapture
```

**Windows Command Prompt:**
```cmd
set CONFIG_PATH=target/release/config.json && cargo test --test config_tests -- --nocapture
```

**To test with your release config (recommended):**
```powershell
$env:CONFIG_PATH = "target/release/config.json"; cargo test --release --test config_tests -- --nocapture
```

**Linux/macOS:**
```bash
CONFIG_PATH=target/release/config.json cargo test --test config_tests -- --nocapture
```

**Note**: Use `--release` flag if your config is in `target/release/config.json` and you want to test the release build.

### Environment Variables

You can set environment variables in several ways:

**Windows Command Prompt:**
```cmd
set CONFIG_PATH=C:\path\to\config.json
set SKIP_INTEGRATION_TESTS=1
cargo test
```

**PowerShell:**
```powershell
$env:CONFIG_PATH = "C:\path\to\config.json"
$env:SKIP_INTEGRATION_TESTS = "1"
cargo test
```

**PowerShell (single command):**
```powershell
$env:CONFIG_PATH = "C:\path\to\config.json"; cargo test -- --nocapture
```

**Windows Command Prompt (single command):**
```cmd
set CONFIG_PATH=C:\path\to\config.json && cargo test -- --nocapture
```

**Linux/macOS Terminal:**
```bash
export CONFIG_PATH=/path/to/config.json
export SKIP_INTEGRATION_TESTS=1
cargo test
```

**Linux/macOS (single command):**
```bash
CONFIG_PATH=/path/to/config.json SKIP_INTEGRATION_TESTS=1 cargo test -- --nocapture
```

**For CI/CD or permanent settings:**
- Create a `.env` file in the project root
- Use tools like `direnv` or IDE environment variable settings
- Set system-wide environment variables in OS settings

**Purpose:**
- `CONFIG_PATH=/path/to/config.json` - Use custom config location for tests
- `SKIP_INTEGRATION_TESTS=1` - Skip tests requiring real API keys

Tests verify configuration handling, metadata lookup logic, history processing, CSV generation, and error handling.

## License

MIT License

## Acknowledgements

- [Simkl](https://simkl.com/) for CSV import functionality
- [TMDB](https://www.themoviedb.org/) for movie/TV metadata
- [TVDB](https://thetvdb.com/) for TV show details
- [MyAnimeList](https://myanimelist.net/) for anime metadata
