use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub schema: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u64, // 秒
}
