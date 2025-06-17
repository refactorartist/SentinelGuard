use dotenvy;
use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database_uri: String,
}

impl AppConfig {
    pub fn from_env(load_env: Option<bool>) -> Result<Self, anyhow::Error> {
        if load_env == Some(true) {
            dotenvy::dotenv().ok();
        }

        let database_uri = match env::var("SENTINEL_GUARD_DATABASE_URI") {
            Ok(uri) => uri,
            Err(_) => {
                return Err(anyhow::anyhow!(
                    "SENTINEL_GUARD_DATABASE_URI environment variable is not set"
                ));
            }
        };

        let host = match env::var("SENTINEL_GUARD_HOST") {
            Ok(h) => h,
            Err(_) => {
                return Err(anyhow::anyhow!(
                    "SENTINEL_GUARD_HOST environment variable is not set"
                ));
            }
        };

        let port = match env::var("SENTINEL_GUARD_PORT") {
            Ok(p) => p,
            Err(_) => {
                return Err(anyhow::anyhow!(
                    "SENTINEL_GUARD_PORT environment variable is not set"
                ));
            }
        };

        Ok(Self {
            host,
            port: port.parse()?,
            database_uri,
        })
    }
}
