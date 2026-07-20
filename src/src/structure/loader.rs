use crate::structure::definition::Config;

use std::{error::Error, fs};

/// payload.toml を読み込む
pub fn load_config(path: &str) -> Result<Config, Box<dyn Error>> {
    //
    // Read File
    //
    let content = fs::read_to_string(path)?;

    //
    // Deserialize
    //
    let config: Config = toml::from_str(&content)?;

    //
    // Validate
    //
    validate(&config)?;

    Ok(config)
}

/// Config全体を検証
fn validate(config: &Config) -> Result<(), Box<dyn Error>> {
    if config.requests.is_empty() {
        return Err("payload.toml does not contain any [[requests]].".into());
    }

    for (index, request) in config.requests.iter().enumerate() {
        validate_request(index, request)?;
    }

    Ok(())
}

/// Request単位の検証
fn validate_request(
    index: usize,
    request: &crate::structure::definition::Request,
) -> Result<(), Box<dyn Error>> {
    //
    // URL Validation
    //
    if request.url.is_none() && (request.base_url.is_none() || request.path.is_none()) {
        return Err(format!(
            "Request #{} : either url or (base_url + path) must be specified.",
            index + 1
        )
        .into());
    }

    //
    // HTTP Method
    //
    request.http_method()?;

    //
    // Timeout
    //
    if request.timeout == 0 {
        return Err(format!(
            "Request #{} : timeout must be greater than zero.",
            index + 1
        )
        .into());
    }

    Ok(())
}
