use async_trait::async_trait;
use crate::error::AppError;
use crate::metadata::models::{MediaType, MetadataResult};

#[async_trait]
pub trait MetadataProvider: Send + Sync {
    fn name(&self) -> &'static str;
    
    async fn search(
        &self,
        title: &str,
        media_type: MediaType,
        year: Option<i32>,
    ) -> Result<Vec<MetadataResult>, AppError>;
    
    #[allow(unused)]
    async fn get_details(
        &self,
        id: &str,
        media_type: MediaType,
    ) -> Result<MetadataResult, AppError> {
        // Note: This method is part of the public API but may not be used
        // by the current CLI application. It's available for library users
        // who want to get detailed information about a specific item.
        let _ = id;
        let _ = media_type;
        Err(AppError::MetadataError("get_details not implemented".into()))
    }
}
