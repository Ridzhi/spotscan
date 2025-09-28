use crate::domain::TgUsername;
use config;
use deadpool_postgres;
use serde_derive::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub env: Option<String>,
    pub port: u16,
    pub secret: String,
    pub pg: deadpool_postgres::Config,
    pub tg: Tg,
}

impl Config {
    pub fn from_env() -> Self {
        let cfg = config::Config::builder()
            .add_source(
                config::Environment::with_prefix("SPOTSCAN")
                    .try_parsing(true)
                    .separator("_"),
            )
            .build()
            .unwrap();

        cfg.try_deserialize::<Config>().unwrap()
    }
}

#[derive(Clone, Deserialize)]
pub struct Tg {
    pub bottoken: String,
    pub botusername: TgUsername,
    pub apiid: i32,
    pub apihash: String,
}
