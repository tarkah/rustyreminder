use rustyreminder::errors::*;
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

pub fn get_config() -> Result<Config> {
    let mut _config = config::Config::default();
    _config
        .merge(config::File::with_name("config.ini"))
        .chain_err(|| ErrorKind::ConfigLoad("config.ini".into()))?;
    let config = _config
        .try_into::<Config>()
        .chain_err(|| ErrorKind::ConfigDeser)?;
    Ok(config)
}
