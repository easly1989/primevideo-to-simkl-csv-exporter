use async_trait::async_trait;
use reqwest::Client;
use crate::{
    config::SimklConfig,
    error::AppError,
    metadata::{MediaType, MetadataResult, MediaIds, MetadataProvider},
};

pub struct SimklClient {
    client: Client,
    config: SimklConfig,
}

impl SimklClient {
    pub fn new(config: SimklConfig) -> Self {
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
            MediaType::Tv => "show",
        };

        let mut query = vec![
            ("q".to_string(), title.to_string()),
            ("type".to_string(), type_param.to_string()),
        ];

        if let Some(y) = year {
            query.push(("year".to_string(), y.to_string()));
        }

        let response = self.client
            .get("https://api.simkl.com/search")
            .header("Authorization", format!("Bearer {}", self.config.client_secret))
            .header("simkl-api-key", &self.config.client_id)
            .query(&query)
            .send()
            .await?;

        if response.status().is_success() {
            let results: Vec<SimklSearchItem> = response.json().await?;
            Ok(results.into_iter().map(|item| item.into()).collect())
        } else {
            Err(AppError::MetadataError(format!(
                "Simkl API error: {}",
                response.status()
            )))
        }
    }

    async fn get_details_internal(
        &self,
        simkl_id: &str,
        media_type: MediaType,
    ) -> Result<MetadataResult, AppError> {
        let type_param = match media_type {
            MediaType::Movie => "movies",
            MediaType::Tv => "shows",
        };

        let url = format!(
            "https://api.simkl.com/{}/{}?extended=full",
            type_param,
            simkl_id
        );

        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.config.client_secret))
            .header("simkl-api-key", &self.config.client_id)
            .send()
            .await?;

        if response.status().is_success() {
            let details: SimklDetailsResponse = response.json().await?;
            Ok(details.into())
        } else {
            Err(AppError::MetadataError(format!(
                "Simkl API error: {}",
                response.status()
            )))
        }
    }
}

#[async_trait]
impl MetadataProvider for SimklClient {
    fn name(&self) -> &'static str {
        "Simkl"
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
struct SimklSearchItem {
    title: String,
    year: Option<String>,
    ids: SimklIds,
}

#[derive(serde::Deserialize)]
struct SimklIds {
    simkl: String,
    imdb: Option<String>,
    tmdb: Option<String>,
    tvdb: Option<String>,
}

#[derive(serde::Deserialize)]
struct SimklDetailsResponse {
    title: String,
    year: Option<String>,
    ids: SimklIds,
}

impl From<SimklSearchItem> for MetadataResult {
    fn from(item: SimklSearchItem) -> Self {
        MetadataResult {
            ids: MediaIds {
                simkl: Some(item.ids.simkl),
                imdb: item.ids.imdb,
                tmdb: item.ids.tmdb,
                tvdb: item.ids.tvdb,
                ..Default::default()
            },
            title: item.title,
            year: item.year,
            media_type: MediaType::Movie, // Will be overridden
        }
    }
}

impl From<SimklDetailsResponse> for MetadataResult {
    fn from(details: SimklDetailsResponse) -> Self {
        MetadataResult {
            ids: MediaIds {
                simkl: Some(details.ids.simkl),
                imdb: details.ids.imdb,
                tmdb: details.ids.tmdb,
                tvdb: details.ids.tvdb,
                ..Default::default()
            },
            title: details.title,
            year: details.year,
            media_type: MediaType::Movie, // Will be overridden
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simkl_search_item_conversion() {
        let item = SimklSearchItem {
            title: "Inception".to_string(),
            year: Some("2010".to_string()),
            ids: SimklIds {
                simkl: "123".to_string(),
                imdb: Some("tt1375666".to_string()),
                tmdb: Some("12345".to_string()),
                tvdb: None,
            },
        };

        let result: MetadataResult = item.into();

        assert_eq!(result.title, "Inception");
        assert_eq!(result.year, Some("2010".to_string()));
        assert_eq!(result.ids.simkl, Some("123".to_string()));
        assert_eq!(result.ids.imdb, Some("tt1375666".to_string()));
        assert_eq!(result.ids.tmdb, Some("12345".to_string()));
        assert_eq!(result.ids.tvdb, None);
        assert_eq!(result.media_type, MediaType::Movie);
    }

    #[test]
    fn test_simkl_details_conversion() {
        let details = SimklDetailsResponse {
            title: "Breaking Bad".to_string(),
            year: Some("2008".to_string()),
            ids: SimklIds {
                simkl: "456".to_string(),
                imdb: Some("tt0903747".to_string()),
                tmdb: Some("12345".to_string()),
                tvdb: Some("789".to_string()),
            },
        };

        let result: MetadataResult = details.into();

        assert_eq!(result.title, "Breaking Bad");
        assert_eq!(result.year, Some("2008".to_string()));
        assert_eq!(result.ids.simkl, Some("456".to_string()));
        assert_eq!(result.ids.imdb, Some("tt0903747".to_string()));
        assert_eq!(result.ids.tmdb, Some("12345".to_string()));
        assert_eq!(result.ids.tvdb, Some("789".to_string()));
        assert_eq!(result.media_type, MediaType::Movie);
    }

    #[test]
    fn test_simkl_item_with_missing_fields() {
        let item = SimklSearchItem {
            title: "Unknown Show".to_string(),
            year: None,
            ids: SimklIds {
                simkl: "999".to_string(),
                imdb: None,
                tmdb: None,
                tvdb: None,
            },
        };

        let result: MetadataResult = item.into();

        assert_eq!(result.title, "Unknown Show");
        assert_eq!(result.year, None);
        assert_eq!(result.ids.simkl, Some("999".to_string()));
        assert_eq!(result.ids.imdb, None);
        assert_eq!(result.ids.tmdb, None);
        assert_eq!(result.ids.tvdb, None);
        assert_eq!(result.media_type, MediaType::Movie);
    }

    #[test]
    fn test_client_creation() {
        let config = SimklConfig {
            client_id: "test_client".to_string(),
            client_secret: "test_secret".to_string(),
        };

        let client = SimklClient::new(config);

        assert_eq!(client.name(), "Simkl");
        assert_eq!(client.config.client_id, "test_client");
        assert_eq!(client.config.client_secret, "test_secret");
    }
}