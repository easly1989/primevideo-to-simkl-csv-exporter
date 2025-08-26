use async_trait::async_trait;
use reqwest::Client;
use crate::{
    config::TmdbConfig,
    error::AppError,
    metadata::{MediaType, MetadataResult, MediaIds, MetadataProvider},
};

pub struct TmdbClient {
    client: Client,
    config: TmdbConfig,
}

impl TmdbClient {
    pub fn new(config: TmdbConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    async fn search_internal(
        &self,
        title: &str,
        media_type: MediaType,
        year: Option<i32>,
    ) -> Result<Vec<MetadataResult>, AppError> {
        let type_param = match media_type {
            MediaType::Movie => "movie",
            MediaType::Tv => "tv",
        };

        let mut query = vec![
            ("query".to_string(), title.to_string()),
            ("include_adult".to_string(), "false".to_string()),
        ];

        if let Some(y) = year {
            query.push(("year".to_string(), y.to_string()));
        }

        let url = format!("https://api.themoviedb.org/3/search/{}", type_param);

        let response = self.client
            .get(&url)
            .query(&query)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .send()
            .await?;

        if response.status().is_success() {
            let results: TmdbSearchResponse = response.json().await?;
            Ok(results.results.into_iter().map(|item| item.into()).collect())
        } else {
            Err(AppError::MetadataError(format!(
                "TMDB API error: {}",
                response.status()
            )))
        }
    }

    async fn get_details_internal(
        &self,
        tmdb_id: &str,
        media_type: MediaType,
    ) -> Result<MetadataResult, AppError> {
        let type_param = match media_type {
            MediaType::Movie => "movie",
            MediaType::Tv => "tv",
        };

        let url = format!(
            "https://api.themoviedb.org/3/{}/{}?append_to_response=external_ids",
            type_param,
            tmdb_id
        );

        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .send()
            .await?;

        if response.status().is_success() {
            let details: TmdbDetailsResponse = response.json().await?;
            Ok(details.into())
        } else {
            Err(AppError::MetadataError(format!(
                "TMDB API error: {}",
                response.status()
            )))
        }
    }
}

#[async_trait]
impl MetadataProvider for TmdbClient {
    fn name(&self) -> &'static str {
        "TMDB"
    }

    async fn search(
        &self,
        title: &str,
        media_type: MediaType,
        year: Option<i32>,
    ) -> Result<Vec<MetadataResult>, AppError> {
        self.search_internal(title, media_type, year).await
    }

    async fn get_details(
        &self,
        id: &str,
        media_type: MediaType,
    ) -> Result<MetadataResult, AppError> {
        self.get_details_internal(id, media_type).await
    }
}

#[derive(serde::Deserialize)]
struct TmdbSearchResponse {
    results: Vec<TmdbItem>,
}

#[derive(serde::Deserialize)]
struct TmdbItem {
    id: i32,
    title: String,
    name: String,
    release_date: Option<String>,
    first_air_date: Option<String>,
    media_type: Option<String>,
}

#[derive(serde::Deserialize)]
struct TmdbDetailsResponse {
    id: i32,
    title: Option<String>,
    name: Option<String>,
    release_date: Option<String>,
    first_air_date: Option<String>,
    external_ids: TmdbExternalIds,
}

#[derive(serde::Deserialize)]
struct TmdbExternalIds {
    #[allow(dead_code)]
    imdb_id: Option<String>,
    tvdb_id: Option<i32>,
}

impl From<TmdbItem> for MetadataResult {
    fn from(item: TmdbItem) -> Self {
        let title = if item.title.is_empty() { item.name } else { item.title };
        let year = item.release_date.or(item.first_air_date)
            .and_then(|d| d.split('-').next().map(|s| s.to_string()));

        MetadataResult {
            ids: MediaIds {
                tmdb: Some(item.id.to_string()),
                ..Default::default()
            },
            title,
            year,
            media_type: match item.media_type.as_deref() {
                Some("tv") => MediaType::Tv,
                Some("movie") => MediaType::Movie,
                _ => MediaType::Movie, // Default to movie if unclear
            },
        }
    }
}

impl From<TmdbDetailsResponse> for MetadataResult {
    fn from(details: TmdbDetailsResponse) -> Self {
        let has_title = details.title.is_some();
        let title = details.title.or(details.name).unwrap_or_default();
        let year = details.release_date.or(details.first_air_date)
            .and_then(|d| d.split('-').next().map(|s| s.to_string()));

        MetadataResult {
            ids: MediaIds {
                tmdb: Some(details.id.to_string()),
                tvdb: details.external_ids.tvdb_id.map(|id| id.to_string()),
                ..Default::default()
            },
            title,
            year,
            media_type: if has_title {
                MediaType::Movie
            } else {
                MediaType::Tv
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tmdb_item_conversion() {
        let item = TmdbItem {
            id: 123,
            title: "Inception".to_string(),
            name: "".to_string(),
            release_date: Some("2010-07-16".to_string()),
            first_air_date: None,
            media_type: Some("movie".to_string()),
        };

        let result: MetadataResult = item.into();

        assert_eq!(result.title, "Inception");
        assert_eq!(result.ids.tmdb, Some("123".to_string()));
        assert_eq!(result.year, Some("2010".to_string()));
        assert_eq!(result.media_type, crate::models::MediaType::Movie);
    }

    #[test]
    fn test_tmdb_tv_item_conversion() {
        let item = TmdbItem {
            id: 456,
            title: "".to_string(),
            name: "Breaking Bad".to_string(),
            release_date: None,
            first_air_date: Some("2008-01-20".to_string()),
            media_type: Some("tv".to_string()),
        };

        let result: MetadataResult = item.into();

        assert_eq!(result.title, "Breaking Bad");
        assert_eq!(result.ids.tmdb, Some("456".to_string()));
        assert_eq!(result.year, Some("2008".to_string()));
        assert_eq!(result.media_type, MediaType::Tv);
    }

    #[test]
    fn test_tmdb_details_conversion() {
        let details = TmdbDetailsResponse {
            id: 123,
            title: Some("Inception".to_string()),
            name: None,
            release_date: Some("2010-07-16".to_string()),
            first_air_date: None,
            external_ids: TmdbExternalIds {
                imdb_id: Some("tt1375666".to_string()),
                tvdb_id: Some(12345),
            },
        };

        let result: MetadataResult = details.into();

        assert_eq!(result.title, "Inception");
        assert_eq!(result.ids.tmdb, Some("123".to_string()));
        assert_eq!(result.ids.tvdb, Some("12345".to_string()));
        assert_eq!(result.year, Some("2010".to_string()));
        assert_eq!(result.media_type, MediaType::Movie);
    }

    #[test]
    fn test_tmdb_tv_details_conversion() {
        let details = TmdbDetailsResponse {
            id: 456,
            title: None,
            name: Some("Breaking Bad".to_string()),
            release_date: None,
            first_air_date: Some("2008-01-20".to_string()),
            external_ids: TmdbExternalIds {
                imdb_id: Some("tt0903747".to_string()),
                tvdb_id: Some(12345),
            },
        };

        let result: MetadataResult = details.into();

        assert_eq!(result.title, "Breaking Bad");
        assert_eq!(result.ids.tmdb, Some("456".to_string()));
        assert_eq!(result.ids.tvdb, Some("12345".to_string()));
        assert_eq!(result.year, Some("2008".to_string()));
        assert_eq!(result.media_type, MediaType::Tv);
    }

    #[test]
    fn test_client_creation() {
        let config = TmdbConfig {
            api_key: "test_api_key".to_string(),
        };

        let client = TmdbClient::new(config);

        assert_eq!(client.name(), "TMDB");
        assert_eq!(client.config.api_key, "test_api_key");
    }
}