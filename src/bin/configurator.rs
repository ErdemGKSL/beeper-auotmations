use anyhow::Result;
use beeper_auotmations::config::Config;
use beeper_auotmations::app_state::SharedAppState;
use beeper_auotmations::tui::{show_config_screen, show_main_screen, show_loading_screen, show_notification_screen, MenuOption};
use beeper_auotmations::api_check::validate_api;

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let config = Config::load()?;
    let default_config = config.clone();

    // Initialize shared app state
    let app_state = SharedAppState::new(config);

    // Check if API is configured, if not show configuration screen first
    let current_config = app_state.get_config().unwrap_or_else(|_| default_config.clone());
    if !current_config.is_api_configured() {
        let updated_config = show_config_screen(current_config)?;
        app_state.update_config(updated_config.clone()).ok();
        
        if !updated_config.is_api_configured() {
            eprintln!("✗ Configuration is incomplete. Cannot continue without API configuration.");
            return Ok(());
        }
    }

    // Validate API credentials
    {
        let cfg = app_state.get_config().unwrap_or_else(|_| default_config.clone());
        let url = cfg.api.url.clone();
        let token = cfg.api.token.clone();
        let is_valid = show_loading_screen("Validating API credentials...", async move {
            validate_api(&url, &token).await
        }).await?;
        
        if !is_valid {
            std::thread::sleep(std::time::Duration::from_millis(1500));
            let current_config = app_state.get_config().unwrap_or_else(|_| default_config.clone());
            let updated_config = show_config_screen(current_config)?;
            app_state.update_config(updated_config.clone()).ok();
            
            if !updated_config.is_api_configured() {
                return Ok(());
            }

            // Validate again after reconfiguration
            let url = updated_config.api.url.clone();
            let token = updated_config.api.token.clone();
            let is_valid_retry = show_loading_screen("Validating API credentials...", async move {
                validate_api(&url, &token).await
            }).await?;
            
            if !is_valid_retry {
                eprintln!("✗ API credentials are still invalid. Cannot continue.");
                return Ok(());
            }
        }
    }

    // Main application loop
    loop {
        // Show main screen
        let current_config = app_state.get_config().unwrap_or_else(|_| default_config.clone());
        match show_main_screen(current_config)? {
            Some(MenuOption::Module(idx)) => {
                // Handle module selection
                match idx {
                    0 => {
                        // Notification Manager
                        show_notification_screen(app_state.clone())?;
                    }
                    1 => {
                        // Auto Response - TODO
                    }
                    _ => {}
                }
            }
            Some(MenuOption::ChangeConfiguration) => {
                // Show configuration screen
                let current_config = app_state.get_config().unwrap_or_else(|_| default_config.clone());
                match show_config_screen(current_config) {
                    Ok(new_config) => {
                        // Verify and validate configuration
                        if new_config.is_api_configured() {
                            let url = new_config.api.url.clone();
                            let token = new_config.api.token.clone();
                            let is_valid = show_loading_screen("Validating API credentials...", async move {
                                let r = validate_api(&url, &token).await;
                                // wait 1500 ms for user to read message
                                tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
                                r
                            }).await?;
                            
                            if !is_valid {
                                eprintln!("⚠ Configuration saved but API credentials are invalid.");
                                break;
                            }
                            
                            // Update app state with new config
                            app_state.update_config(new_config).ok();
                        } else {
                            eprintln!("✗ Configuration is incomplete.");
                        }
                    }
                    Err(e) => {
                        eprintln!("✗ Error loading configuration: {}", e);
                    }
                }
                // Loop back to main screen
            }
            Some(MenuOption::Exit) | None => {
                break;
            }
        }
    }

    Ok(())
}
