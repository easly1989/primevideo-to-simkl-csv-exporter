use fantoccini::{Client, ClientBuilder};
use crate::error::AppError;
use std::time::Duration;

pub struct BrowserController {
    client: Option<Client>,
    #[allow(unused)]
    headless: bool, // Reserved for future headless browser configuration
    #[allow(unused)]
    timeout: Duration, // Reserved for future timeout configuration
}

impl BrowserController {
    pub fn new(headless: bool, timeout_secs: u64) -> Self {
        Self {
            client: None,
            headless,
            timeout: Duration::from_secs(timeout_secs),
        }
    }

    pub async fn start(&mut self) -> Result<(), AppError> {
        let builder = ClientBuilder::native();

        // Note: Headless mode configuration would need to be implemented
        // based on the specific WebDriver being used and may not be
        // supported by the current version of fantoccini

        let client = builder
            .connect("http://localhost:4444")
            .await
            .map_err(|e| AppError::BrowserError(e.to_string()))?;

        self.client = Some(client);
        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<(), AppError> {
        if let Some(client) = self.client.take() {
            let mut client = client;
            client.close().await.map_err(|e| AppError::BrowserError(e.to_string()))?;
        }
        Ok(())
    }

    pub async fn restart(&mut self) -> Result<(), AppError> {
        self.shutdown().await?;
        self.start().await
    }

    pub fn client(&self) -> Option<&Client> {
        self.client.as_ref()
    }

}