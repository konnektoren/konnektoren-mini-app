use anyhow::Result;
use konnektoren_core::prelude::PlayerProfile;

pub async fn fetch_profile(id: String, name: String) -> Result<PlayerProfile> {
    let url = format!("https://api.konnektoren.help/api/v1/profiles/{}", id);

    save_profile(PlayerProfile {
        id: id.clone(),
        name: name.clone(),
        xp: 0,
    })
    .await?;

    let client = reqwest::Client::new();
    let response = client.get(url).fetch_mode_no_cors().send().await?;
    let profile = response.json::<PlayerProfile>().await?;
    Ok(profile)
}

pub async fn save_profile(profile: PlayerProfile) -> Result<()> {
    let url = "https://api.konnektoren.help/api/v1/profiles";
    let client = reqwest::Client::new();
    let _response = client.post(url).json(&profile).send().await?;
    Ok(())
}
