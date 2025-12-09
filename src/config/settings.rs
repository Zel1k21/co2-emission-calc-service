use dotenv::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub host: String,
    pub port: u16,
}

impl Settings {
    pub fn new() -> Self {
        dotenv().ok();
        Self {
            host: env::var("HOST").unwrap(),
            port: env::var("LISTEN_PORT").unwrap().parse().unwrap(),
        }
    }
}
