# Prime Video to Simkl CSV Exporter (Rust Edition)

A modern Rust-based tool to export your Amazon Prime Video watch history to a CSV file compatible with [Simkl](https://simkl.com/apps/import/csv/) import format.

## If you like my work
Help me pay off my home loan → [Donate on PayPal](https://paypal.me/ruggierocarlo)

## Features

- **Automatic scraping** of Prime Video watch history
- **Metadata enrichment** from multiple sources:
  - [Simkl](https://simkl.com/) - primary metadata provider
  - [TMDB](https://www.themoviedb.org/) - movie and TV show details
  - [TVDB](https://thetvdb.com/) - TV show metadata (optional)
  - [MyAnimeList](https://myanimelist.net/) - anime-specific metadata (optional)
- **Smart deduplication** - only includes last watched episode for TV shows
- **CSV generation** in Simkl import format
- **Automatic configuration** - creates config file on first run
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
    - Add it to your system PATH, OR
    - Specify the full path in config.json under "browser.driver_path"
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

3. Run the application to generate config file:
```bash
cargo run --release
```
This will create a `config.json` file in the target directory (`target/release/` or `target/debug/`).

4. Edit `config.json` with your credentials:
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
    "email": "YOUR_AMAZON_EMAIL",
    "password": "YOUR_AMAZON_PASSWORD"
  },
  "output": {
    "path": "./export.csv"
  }
}
```

> **Security Note**: The `config.json` contains sensitive credentials. Keep it secure and never commit to version control.

## Usage

Run the application:
```bash
cargo run --release
```

The application will:
1. Launch a browser window for Amazon login
2. Scrape your watch history
3. Enrich items with metadata
4. Generate `export.csv` in Simkl format

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
  - Complete login within 5 minutes
  - For 2FA, enter code within 60 seconds
  - Check `login-error.png` for diagnostics

- **Large Watch Histories**:
  - Processing may take time (1-2 items/second)
  - API rate limits are automatically handled

- **WebDriver Issues**:
  - Ensure ChromeDriver/GeckoDriver is installed and in PATH
  - Verify installation:
    ```bash
    chromedriver --version  # Should output version
    ```
  - Run WebDriver manually before starting the app:
    - ChromeDriver: `chromedriver --port=4444`
    - GeckoDriver: `geckodriver --port 4444`
  - Or specify WebDriver path in config.json:
    ```json
    "browser": {
      "driver_path": "C:/path/to/chromedriver.exe"
    }
    ```
  - If using Chrome, ensure Chrome browser is installed
  - If using Firefox, ensure Firefox browser is installed

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
