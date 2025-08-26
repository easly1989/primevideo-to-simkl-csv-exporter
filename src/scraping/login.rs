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
    println!("🔐 MANUAL LOGIN REQUIRED");
    println!("========================");
    println!("Please complete the following steps in the browser window:");
    println!("1. Navigate to Prime Video and sign in with your Amazon account");
    println!("2. Go to your watch history page");
    println!("3. Once logged in, press Enter in this terminal to proceed");
    println!();
    println!("The browser window should open automatically. Please log in and press Enter when ready...");

    // Navigate to global Prime Video domain
    client
        .goto("https://www.primevideo.com/settings/watch-history")
        .await
        .map_err(|e| AppError::BrowserError(e.to_string()))?;

    // Simple approach: Wait for user to press Enter
    println!("⏳ Waiting for you to press Enter...");

    // Use a simpler blocking approach for stdin
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("✅ Proceeding with login check...");
        }
        Err(e) => {
            return Err(AppError::AuthError(format!("Failed to read input: {}", e)));
        }
    }

    // Do a final URL check
    let current_url = client
        .current_url()
        .await
        .map_err(|e| AppError::BrowserError(e.to_string()))?;

    let url_str = current_url.to_string();
    println!("📍 Current URL: {}", url_str);

    // Check if we're on the watch history page (positive check)
    let is_on_watch_history = url_str.contains("watch-history");

    // More precise check for login pages - only check path, not query parameters
    let url_path = current_url.path();
    let is_on_login_page = url_path.contains("signin") ||
                          url_path.contains("/login") ||
                          url_path.contains("/auth");

    if !is_on_watch_history {
        if is_on_login_page {
            println!("⚠️  You appear to be on a login page. Please log in to Prime Video first.");
            return Err(AppError::AuthError("Please log in to Prime Video first".into()));
        } else {
            println!("⚠️  You don't appear to be on the watch history page. Please navigate to your watch history.");
            return Err(AppError::AuthError("Please navigate to your Prime Video watch history page".into()));
        }
    }

    // If we're on watch history page, we're good to go
    println!("✅ Confirmed: You're on the watch history page!");

    println!("✅ Login check completed - proceeding with scraping...");
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

    // Check URL patterns first (quick check)
    let url_check = url_str.contains("watch-history") &&
        !url_str.contains("signin") &&
        !url_str.contains("auth");

    if !url_check {
        return Ok(false);
    }

    // Additional checks for login indicators
    // Look for watch history content or user account elements
    let page_content_checks = [
        // Check for watch history specific elements
        "[data-testid='watch-history']",
        ".watch-history",
        "[data-automation-id='watch-history']",
        // Check for user account/navigation elements that indicate login
        "[data-testid='account-menu']",
        ".account-menu",
        "[data-automation-id='account-menu']",
        // Check for Prime Video navigation or content
        "[data-testid='av-nav-main']",
        ".av-nav-main",
        // Check for absence of login forms
        "input[name='email']",
        "input[name='password']",
    ];

    for selector in page_content_checks {
        // If we find login form elements, user is not logged in
        if let Ok(_) = client.find(Locator::Css(selector)).await {
            if selector.contains("email") || selector.contains("password") {
                return Ok(false); // Login form detected
            }
        }
    }

    Ok(true)
}