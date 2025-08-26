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
- API credentials:
  - [Simkl Client ID/Secret](https://simkl.com/settings/developer/new/)
  - [TMDB API Read Access Token](https://www.themoviedb.org/settings/api)
  - (Optional) [TVDB API Key](https://thetvdb.com/api-information)
  - (Optional) [MyAnimeList Client ID/Secret](https://myanimelist.net/apiconfig/create)

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
4. Scrape your watch history after successful login
5. Enrich items with metadata
6. Generate `export.csv` in Simkl format

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
  - Complete manual login within 10 minutes when prompted
  - The application will guide you through the login process
  - Ensure you're on the Prime Video watch history page after login
  - For 2FA, complete the authentication process as required
  - Check `login-error.png` for diagnostics if issues persist

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
  - Verify API keys are valid
  - Some obscure titles may not be found

- **Browser Compatibility**:
  - Tested with latest Chrome and Firefox
  - Ensure browser automation is not blocked

## Testing

Run all tests:
```bash
cargo test
```

Tests verify:
- Configuration handling
- Metadata lookup logic
- History processing
- CSV generation
- Error handling

## License

MIT License

## Acknowledgements

- [Simkl](https://simkl.com/) for CSV import functionality
- [TMDB](https://www.themoviedb.org/) for movie/TV metadata
- [TVDB](https://thetvdb.com/) for TV show details
- [MyAnimeList](https://myanimelist.net/) for anime metadata
