use chrono::{DateTime, NaiveDateTime};

use crate::cores::env::{Config, ConfigApp, ConfigDatabase, ConfigJWT, ConfigLog};

pub fn test_uuid() -> uuid::Uuid {
    uuid::Uuid::parse_str("7fb305c1-9cb2-4cd9-a57a-7508ec07ecce").unwrap()
}

pub fn test_time() -> NaiveDateTime {
    DateTime::parse_from_rfc3339("2022-10-10T04:24:42.995338+00:00")
        .unwrap()
        .naive_utc()
}

pub fn test_actor() -> uuid::Uuid {
    uuid::Uuid::parse_str("78554741-b71a-4ce6-8061-2ac77c7fff0c").unwrap()
}

impl Config {
    pub fn default() -> Self {
        Self {
            app: Config::app_default(),
            db: Config::db_default(),
            log: Config::log_default(),
            jwt: Config::jwt_default(),
        }
    }
    pub fn app_default() -> ConfigApp {
        ConfigApp {
            url: String::new(),
            name: String::new(),
        }
    }
    pub fn db_default() -> ConfigDatabase {
        ConfigDatabase {
            host: String::new(),
            name: String::new(),
            user: String::new(),
            pass: String::new(),
            port: 0,
        }
    }
    pub fn log_default() -> ConfigLog {
        ConfigLog {
            level: String::new(),
            is_json: false,
            file: String::new(),
        }
    }
    pub fn jwt_default() -> ConfigJWT {
        ConfigJWT {
            secret: String::new(),
            audience: String::new(),
        }
    }
}
