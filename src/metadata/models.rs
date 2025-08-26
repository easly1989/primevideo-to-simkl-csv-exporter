use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ServiceType {
    Simkl,
    Tmdb,
    Tvdb,
    Mal,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataResult {
    pub ids: MediaIds,
    pub title: String,
    pub year: Option<String>,
    pub media_type: crate::models::MediaType,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MediaIds {
    pub simkl: Option<String>,
    pub tvdb: Option<String>,
    pub tmdb: Option<String>,
    pub mal: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub simkl: RateLimit,
    pub tmdb: RateLimit,
    pub tvdb: RateLimit,
    pub mal: RateLimit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    pub calls: u32,
    pub per_seconds: u64,
}

pub type PriorityOrder = Vec<ServiceType>;