use std::env;
use dotenvy;

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

        let database_uri = match env::var("BURAQ_DATABASE_URI") {
            Ok(uri) => uri,
            Err(_) => {
                return Err(anyhow::anyhow!(
                    "BURAQ_DATABASE_URI environment variable is not set"
                ));
            }
        };

        let host = match env::var("BURAQ_HOST") {
            Ok(h) => h,
            Err(_) => {
                return Err(anyhow::anyhow!(
                    "BURAQ_HOST environment variable is not set"
                ));
            }
        };

        let port = match env::var("BURAQ_PORT") {
            Ok(p) => p,
            Err(_) => {
                return Err(anyhow::anyhow!(
                    "BURAQ_PORT environment variable is not set"
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
