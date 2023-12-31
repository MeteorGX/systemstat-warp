use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfigByBasic {
    // debug/info/warn/error
    pub log: Option<String>,

    // timezone
    pub timezone: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfigByWeb {
    // "127.0.0.1"
    pub address: String,
    // 18081
    pub port: u32,
    // static dir
    pub static_dir: Option<String>,
    // cipher method: default sha256
    pub cipher: Option<String>,
    // auth: username:password[cipher],username:password,......
    pub auth: Option<String>,
    // 192.168.1.xxx,192.168.yyy.xxx
    pub allow: Option<String>,
    // 192.168.1.xxx,192.168.yyy.xxx
    pub deny: Option<String>,
}


/// database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfigByDB {
    pub url: String,
}


/// redis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfigByRedis {
    pub url: String,
    pub interval: Option<u64>,
    pub timezone: Option<String>,
    pub prefix: Option<String>,
    pub max_attempts: Option<u32>,
    pub min_delay: Option<u32>,
    pub max_delay: Option<u32>,
    pub mult: Option<u32>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub basic: Option<AppConfigByBasic>,
    pub web: Option<AppConfigByWeb>,
    pub db: Option<AppConfigByDB>,
    pub redis: Option<AppConfigByRedis>,
}


impl AppConfig {
    pub fn parse(ctx: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(ctx)
    }
}
