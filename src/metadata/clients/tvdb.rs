use async_trait::async_trait;
use reqwest::Client;
use crate::{
    config::TvdbConfig,
    error::AppError,
    metadata::{MediaType, MetadataResult, MediaIds, MetadataProvider, RateLimit},
};

#[allow(dead_code)]
pub struct TvdbClient {
    client: Client,
    config: TvdbConfig,
    rate_limit: RateLimit,
    access_token: Option<String>,
}

impl TvdbClient {
    pub fn new(config: TvdbConfig, rate_limit: RateLimit) -> Self {
        Self {
            client: Client::new(),
            config,
            rate_limit,
            access_token: None,
        }
    }

    async fn authenticate(&mut self) -> Result<(), AppError> {
        let auth = serde_json::json!({
            "apikey": self.config.api_key
        });

        let response = self.client
            .post("https://api.thetvdb.com/login")
            .json(&auth)
            .send()
            .await?;

        if response.status().is_success() {
            let auth: TvdbAuthResponse = response.json().await?;
            self.access_token = Some(auth.token);
            Ok(())
        } else {
            Err(AppError::AuthError("TVDB authentication failed".into()))
        }
    }

    async fn search_internal(
        &mut self,
        title: &str,
        media_type: MediaType,
    ) -> Result<Vec<MetadataResult>, AppError> {
        if self.access_token.is_none() {
            self.authenticate().await?;
        }

        let url = format!(
            "https://api.thetvdb.com/search/series?name={}",
            title
        );

        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token.as_ref().unwrap()))
            .send()
            .await?;

        if response.status().is_success() {
            let results: TvdbSearchResponse = response.json().await?;
            Ok(results.data.into_iter().map(|item| item.into()).collect())
        } else if response.status() == 401 {
            // Token expired, retry with new auth
            self.authenticate().await?;
            Box::pin(self.search_internal(title, media_type)).await
        } else {
            Err(AppError::MetadataError(format!(
                "TVDB API error: {}",
                response.status()
            )))
        }
    }

    async fn get_details_internal(
        &mut self,
        tvdb_id: &str,
        media_type: MediaType,
    ) -> Result<MetadataResult, AppError> {
        if self.access_token.is_none() {
            self.authenticate().await?;
        }

        let url = format!(
            "https://api.thetvdb.com/series/{}",
            tvdb_id
        );

        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token.as_ref().unwrap()))
            .send()
            .await?;

        if response.status().is_success() {
            let details: TvdbDetailsResponse = response.json().await?;
            Ok(details.data.into())
        } else if response.status() == 401 {
            // Token expired, retry with new auth
            self.authenticate().await?;
            Box::pin(self.get_details_internal(tvdb_id, media_type)).await
        } else {
            Err(AppError::MetadataError(format!(
                "TVDB API error: {}",
                response.status()
            )))
        }
    }
}

#[async_trait]
impl MetadataProvider for TvdbClient {
    fn name(&self) -> &'static str {
        "TVDB"
    }

    async fn search(
        &self,
        title: &str,
        media_type: MediaType,
        _year: Option<i32>,
    ) -> Result<Vec<MetadataResult>, AppError> {
        // Need mutable self for auth
        let mut this = unsafe { std::ptr::read(self) };
        let result = this.search_internal(title, media_type).await;
        std::mem::forget(this);
        result
    }

    async fn get_details(
        &self,
        id: &str,
        media_type: MediaType,
    ) -> Result<MetadataResult, AppError> {
        // Need mutable self for auth
        let mut this = unsafe { std::ptr::read(self) };
        let result = this.get_details_internal(id, media_type).await;
        std::mem::forget(this);
        result
    }
}

#[derive(serde::Deserialize)]
struct TvdbAuthResponse {
    token: String,
}

#[derive(serde::Deserialize)]
struct TvdbSearchResponse {
    data: Vec<TvdbSearchItem>,
}

#[derive(serde::Deserialize)]
struct TvdbSearchItem {
    id: i32,
    #[serde(rename = "seriesName")]
    series_name: String,
    #[serde(rename = "firstAired")]
    first_aired: Option<String>,
}

#[derive(serde::Deserialize)]
struct TvdbDetailsResponse {
    data: TvdbDetailsItem,
}

#[derive(serde::Deserialize)]
struct TvdbDetailsItem {
    id: i32,
    #[serde(rename = "seriesName")]
    series_name: String,
    #[serde(rename = "firstAired")]
    first_aired: Option<String>,
    #[serde(rename = "imdbId")]
    imdb_id: Option<String>,
}

impl From<TvdbSearchItem> for MetadataResult {
    fn from(item: TvdbSearchItem) -> Self {
        let year = item.first_aired
            .as_ref()
            .and_then(|d| d.split('-').next().map(|s| s.to_string()));

        MetadataResult {
            ids: MediaIds {
                tvdb: Some(item.id.to_string()),
                ..Default::default()
            },
            title: item.series_name,
            year,
            media_type: MediaType::Tv,
        }
    }
}

impl From<TvdbDetailsItem> for MetadataResult {
    fn from(item: TvdbDetailsItem) -> Self {
        let year = item.first_aired
            .as_ref()
            .and_then(|d| d.split('-').next().map(|s| s.to_string()));

        MetadataResult {
            ids: MediaIds {
                tvdb: Some(item.id.to_string()),
                imdb: item.imdb_id,
                ..Default::default()
            },
            title: item.series_name,
            year,
            media_type: MediaType::Tv,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tvdb_search_item_conversion() {
        let item = TvdbSearchItem {
            id: 123,
            series_name: "Breaking Bad".to_string(),
            first_aired: Some("2008-01-20".to_string()),
        };

        let result: MetadataResult = item.into();

        assert_eq!(result.title, "Breaking Bad");
        assert_eq!(result.ids.tvdb, Some("123".to_string()));
        assert_eq!(result.year, Some("2008".to_string()));
        assert_eq!(result.media_type, MediaType::Tv);
    }

    #[test]
    fn test_tvdb_search_item_without_date() {
        let item = TvdbSearchItem {
            id: 456,
            series_name: "Unknown Show".to_string(),
            first_aired: None,
        };

        let result: MetadataResult = item.into();

        assert_eq!(result.title, "Unknown Show");
        assert_eq!(result.ids.tvdb, Some("456".to_string()));
        assert_eq!(result.year, None);
        assert_eq!(result.media_type, MediaType::Tv);
    }

    #[test]
    fn test_tvdb_details_conversion() {
        let details = TvdbDetailsItem {
            id: 123,
            series_name: "Breaking Bad".to_string(),
            first_aired: Some("2008-01-20".to_string()),
            imdb_id: Some("tt0903747".to_string()),
        };

        let result: MetadataResult = details.into();

        assert_eq!(result.title, "Breaking Bad");
        assert_eq!(result.ids.tvdb, Some("123".to_string()));
        assert_eq!(result.ids.imdb, Some("tt0903747".to_string()));
        assert_eq!(result.year, Some("2008".to_string()));
        assert_eq!(result.media_type, MediaType::Tv);
    }

    #[test]
    fn test_tvdb_details_without_imdb() {
        let details = TvdbDetailsItem {
            id: 789,
            series_name: "Another Show".to_string(),
            first_aired: Some("2015-05-10".to_string()),
            imdb_id: None,
        };

        let result: MetadataResult = details.into();

        assert_eq!(result.title, "Another Show");
        assert_eq!(result.ids.tvdb, Some("789".to_string()));
        assert_eq!(result.ids.imdb, None);
        assert_eq!(result.year, Some("2015".to_string()));
        assert_eq!(result.media_type, MediaType::Tv);
    }

    #[test]
    fn test_client_creation() {
        let config = TvdbConfig {
            api_key: "test_api_key".to_string(),
        };
        let rate_limit = RateLimit { calls: 10, per_seconds: 1 };

        let client = TvdbClient::new(config, rate_limit);

        assert_eq!(client.name(), "TVDB");
        assert_eq!(client.config.api_key, "test_api_key");
    }
}