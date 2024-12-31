use std::fs::File;

use anyhow::{bail, Result};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
  pub server: ServerConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
  pub port: u16,
}

impl AppConfig {
  pub fn load() -> Result<Self> {
    // read from /etc/config/app.yml or ./app.yml or from env CHAT_CONFIG
    let ret = match (
      File::open("app.yml"),
      File::open("/etc/config/app.yml"),
      std::env::var("CHAT_CONFIG"),
    ) {
      (Ok(file), _, _) => serde_yaml::from_reader(file)?,
      (_, Ok(file), _) => serde_yaml::from_reader(file)?,
      (_, _, Ok(path)) => {
        println!("loading config from {}", path);
        serde_yaml::from_reader(File::open(path)?)?
      }
      _ => bail!("no config file found"),
    };
    Ok(ret)
  }
}
