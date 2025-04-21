use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_path = env::var("BASE_PATH").unwrap_or_else(|_| String::from(""));
    let domain = env::var("DOMAIN").unwrap_or_else(|_| String::from("konnektoren.help"));

    let konnektoren_v1_api_url = env::var("KONNEKTOREN_V1_API_URL")
        .unwrap_or_else(|_| String::from("https://api.konnektoren.help/api/v1"));

    println!("cargo:rustc-env=BASE_PATH={}", base_path);
    println!("cargo:rustc-env=DOMAIN={}", domain);
    println!(
        "cargo:rustc-env=KONNEKTOREN_V1_API_URL={}",
        konnektoren_v1_api_url
    );

    Ok(())
}
