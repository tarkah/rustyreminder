use failure::{Error, ResultExt};
use rustyreminder::errors::AppError;
use serde::{Deserialize, Serialize};

const FILE_PATH: &str = "config.ini";

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
    let mut config = config::Config::default();
    config
        .merge(config::File::with_name(FILE_PATH))
        .context(AppError::ConfigLoad { path: FILE_PATH })?;
    let config = config.try_into::<Config>().context(AppError::ConfigDeser)?;
    Ok(config)
}
