use anyhow::Result;
use konnektoren_core::prelude::PlayerProfile;

// Get the API URL from environment variables set in build.rs
fn get_api_url() -> String {
    option_env!("KONNEKTOREN_V1_API_URL")
        .unwrap_or("https://api.konnektoren.help/api/v1")
        .to_string()
}

pub async fn fetch_profile(id: String, name: String) -> Result<PlayerProfile> {
    let api_url = get_api_url();
    let url = format!("{}/profiles/{}", api_url, id);

    // Create a default profile
    let default_profile = PlayerProfile {
        id: id.clone(),
        name: name.clone(),
        xp: 0,
    };

    // Try to save the default profile first
    if let Err(e) = save_profile(default_profile.clone()).await {
        log::warn!("Failed to save default profile: {:?}", e);
    }

    // Try to fetch the profile
    let client = reqwest::Client::new();
    match client.get(&url).fetch_mode_no_cors().send().await {
        Ok(response) => match response.json::<PlayerProfile>().await {
            Ok(profile) => Ok(profile),
            Err(e) => {
                log::warn!("Error parsing profile JSON: {:?}", e);
                Ok(default_profile)
            }
        },
        Err(e) => {
            log::warn!("Error fetching profile: {:?}", e);
            Ok(default_profile)
        }
    }
}

pub async fn save_profile(profile: PlayerProfile) -> Result<()> {
    let api_url = get_api_url();
    let url = format!("{}/profiles", api_url);

    let client = reqwest::Client::new();
    match client.post(&url).json(&profile).send().await {
        Ok(_) => Ok(()),
        Err(e) => {
            log::warn!("Failed to save profile: {:?}", e);
            Err(anyhow::anyhow!("Failed to save profile: {:?}", e))
        }
    }
}
