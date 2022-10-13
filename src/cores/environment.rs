use std::sync::Arc;

use crate::utils::common;
use dotenv::var;

#[derive(Clone)]
pub struct Config {
    pub app: ConfigApp,
    pub db: ConfigDatabase,
    pub log: ConfigLog,
    pub jwt: ConfigJWT,
}

#[derive(Clone)]
pub struct ConfigApp {
    pub url: String,
    pub name: String,
}

#[derive(Clone)]
pub struct ConfigDatabase {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub pass: String,
}

#[derive(Clone)]
pub struct ConfigLog {
    pub level: String,
    pub is_json: bool,
    pub file: String,
}

#[derive(Clone)]
pub struct ConfigJWT {
    pub secret: String,
    pub audience: String,
}

impl Config {
    pub fn set() -> Arc<Self> {
        dotenv::dotenv().ok();
        std::env::set_var("RUST_LOG", "actix_web=debug");

        Arc::new(Self {
            app: ConfigApp {
                url: var("APP_URL").expect("APP_URL must be set"),
                name: var("APP_NAME").unwrap_or_default(),
            },
            db: ConfigDatabase {
                host: var("DB_HOST").expect("DB_HOST must be set"),
                port: var("DB_PORT")
                    .expect("DB_PORT must be set")
                    .parse::<u16>()
                    .unwrap(),
                name: var("DB_NAME").expect("DB_NAME must be set"),
                user: var("DB_USER").unwrap_or_default(),
                pass: var("DB_PASS").unwrap_or_default(),
            },
            log: ConfigLog {
                level: var("LOG_LEVEL").unwrap_or("INFO".to_owned()),
                is_json: common::string_to_bool(
                    var("LOG_IS_JSON")
                        .unwrap_or("false".to_owned())
                        .to_lowercase(),
                )
                .unwrap_or_default(),
                file: var("LOG_FILE").unwrap_or_default(),
            },
            jwt: ConfigJWT {
                secret: var("JWT_SECRET").unwrap_or_default(),
                audience: var("JWT_AUDIENCE").unwrap_or_default(),
            },
        })
    }
}
