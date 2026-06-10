use crate::config::database::DatabaseConfig;
use crate::config::server::ServerConfig;
use anyhow::Context;
use config::{Config, Environment, File};
use serde::Deserialize;
use std::env;
use std::sync::OnceLock;

mod database;
mod server;

static CONFIG: OnceLock<AppConfig> = OnceLock::new();
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}
impl AppConfig {
    #[allow(dead_code)]
    ///加载配置
    fn load() -> anyhow::Result<AppConfig> {
        let run_env = env::var("RUN_ENV").unwrap_or_else(|_| "dev".to_string());

        let config = Config::builder()
            // 先加载默认配置
            .add_source(File::with_name("config/default"))
            // 再加载环境配置覆盖默认值
            .add_source(File::with_name(&format!("config/{}", run_env)))
            // 支持环境变量覆盖，前缀 OHO__
            // 例如：OHO__DATABASE__PASSWORD=xxx
            .add_source(Environment::with_prefix("OHO").separator("__"))
            .build()?;
        config
            .try_deserialize::<Self>()
            .with_context(|| "failed to parse config")
    }
    #[allow(dead_code)]
    fn init() -> anyhow::Result<()> {
        let config = Self::load()?;
        CONFIG.set(config).ok();
        Ok(())
    }
    pub fn get() -> &'static AppConfig {
        CONFIG.get().expect("config is not initialized")
    }
}
