use fantoccini::{Client, Locator};
use crate::error::AppError;
use std::time::Duration;

pub enum LoginMethod {
    Manual,
    Automated { email: String, password: String },
}

async fn fill_form_field(
    client: &mut Client,
    selector: &str,
    value: &str
) -> Result<(), AppError> {
    client
        .wait()
        .at_most(Duration::from_secs(10))
        .for_element(Locator::Css(selector))
        .await
        .map_err(|e| AppError::BrowserError(e.to_string()))?
        .send_keys(value)
        .await
        .map_err(|e| AppError::BrowserError(e.to_string()))?;
    Ok(())
}

async fn click_element(
    client: &mut Client,
    selector: &str
) -> Result<(), AppError> {
    client
        .wait()
        .at_most(Duration::from_secs(10))
        .for_element(Locator::Css(selector))
        .await
        .map_err(|e| AppError::BrowserError(e.to_string()))?
        .click()
        .await
        .map_err(|e| AppError::BrowserError(e.to_string()))?;
    Ok(())
}

pub async fn handle_login(
    client: &mut Client,
    method: LoginMethod,
) -> Result<(), AppError> {
    match method {
        LoginMethod::Manual => manual_login(client).await,
        LoginMethod::Automated { email, password } => {
            automated_login(client, &email, &password).await
        }
    }
}

async fn manual_login(client: &mut Client) -> Result<(), AppError> {
    // Navigate to global Prime Video domain
    client
        .goto("https://www.primevideo.com/settings/watch-history")
        .await
        .map_err(|e| AppError::BrowserError(e.to_string()))?;

    // Wait for user to manually login
    tokio::time::sleep(std::time::Duration::from_secs(300)).await; // 5 minute timeout

    // Verify we're on watch-history page
    let current_url = client
        .current_url()
        .await
        .map_err(|e| AppError::BrowserError(e.to_string()))?;
    
    if !current_url.as_str().contains("watch-history") {
        return Err(AppError::AuthError("Manual login failed - not on watch history page".into()));
    }

    Ok(())
}

async fn automated_login(
    client: &mut Client,
    email: &str,
    password: &str,
) -> Result<(), AppError> {
    // Use regional Amazon site based on TLD in email
    let domain = if email.contains(".co.uk") {
        "amazon.co.uk"
    } else if email.contains(".de") {
        "amazon.de"
    } else if email.contains(".it") {
        "amazon.it"
    } else {
        "amazon.com"
    };
    let login_url = format!("https://www.{}/ap/signin", domain);

    client
        .goto(&login_url)
        .await
        .map_err(|e| AppError::BrowserError(e.to_string()))?;

    // Use helper functions for form interaction
    fill_form_field(client, "input[name='email'], input[name='ap_email']", email).await?;
    click_element(client, "#continue").await?;
    fill_form_field(client, "input[name='password'], input[name='ap_password']", password).await?;
    click_element(client, "#signInSubmit").await?;

    // Handle 2FA if present
    if let Ok(_element) = client
        .find(Locator::Css("#auth-mfa-otpcode, .cvf-widget-input-code"))
        .await
    {
        return Err(AppError::AuthError(
            "2FA detected - manual login required".into(),
        ));
    }

    // Verify login success
    if !is_logged_in(client).await? {
        return Err(AppError::AuthError("Automated login failed".into()));
    }

    Ok(())
}

async fn is_logged_in(client: &mut Client) -> Result<bool, AppError> {
    let current_url = client
        .current_url()
        .await
        .map_err(|e| AppError::BrowserError(e.to_string()))?;

    let url_str = current_url.to_string();
    Ok(url_str.contains("watch-history") &&
       !url_str.contains("signin") &&
       !url_str.contains("auth"))
}