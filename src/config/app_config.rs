use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfigByBasic {
    pub log: Option<String>,
    // debug/info/warn/error
    pub timezone: Option<String>,
    // timezone
    pub log_file: Option<String>, // /tmp/wrap.log
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfigByWeb {
    pub address: String,
    // "127.0.0.1"
    pub port: u32,
    // 18081
    pub static_dir: Option<String>,
    // static dir
    pub username: Option<String>,
    pub auth: Option<String>,
    // "md5/rc4/sha1"
    pub password: Option<String>,
    pub allow: Option<String>,
    // 192.168.1.xxx,192.168.yyy.xxx
    pub deny: Option<String>, // 192.168.1.xxx,192.168.yyy.xxx
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub basic: Option<AppConfigByBasic>,
    pub web: Option<AppConfigByWeb>,
}


impl AppConfig {
    pub fn parse(ctx: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(ctx)
    }
}
