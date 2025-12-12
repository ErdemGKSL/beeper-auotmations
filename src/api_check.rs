use crate::app_state::SharedAppState;
use beeper_desktop_api::BeeperClient;

/// Validate API credentials using the shared AppState
pub async fn validate_api_with_state(state: &SharedAppState) -> bool {
    let config = match state.get_config() {
        Ok(cfg) => cfg,
        Err(_) => return false,
    };

    let client = BeeperClient::new(&config.api.token, &config.api.url);
    match client.get_accounts().await {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// Validate API credentials directly with url and token
pub async fn validate_api(url: &str, token: &str) -> bool {
    let client = BeeperClient::new(token, url);
    match client.get_accounts().await {
        Ok(_) => true,
        Err(_) => false,
    }
}
