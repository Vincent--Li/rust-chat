use std::fs::File;

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
    let reader = File::open("app.yml")?;

    
  }
}