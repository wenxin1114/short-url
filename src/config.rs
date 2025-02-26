use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub base_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
}

impl Config {
    pub fn from_env() -> Self {
        Config {
            server: ServerConfig {
                host: env::var("SERVER_HOST").unwrap_or_else(|_| String::from("127.0.0.1")),
                port: env::var("SERVER_PORT").unwrap_or_else(|_| String::from("8080")).parse().unwrap(),
                base_url: env::var("BASE_URL").unwrap_or_else(|_| format!("http://{}:{}", env::var("SERVER_HOST").unwrap_or_else(|_| String::from("localhost")), env::var("SERVER_PORT").unwrap_or_else(|_| String::from("8080")))),
            },
        }
    }
}