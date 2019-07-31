use failure::Error;
use failure::ResultExt;
use rustyreminder::errors::AppError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Smtp {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub smtp: Smtp,
}

pub fn get_config() -> Result<Config, Error> {
    let path = "config.ini";
    let mut config = config::Config::default();
    config
        .merge(config::File::with_name(path))
        .context(AppError::ConfigLoad { path })?;
    let config = config.try_into::<Config>().context(AppError::ConfigDeser)?;
    Ok(config)
}
