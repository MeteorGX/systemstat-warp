use crate::AppConfigByRedis;
use fred::{prelude::*};
use systemstat::{Platform, System};

pub struct RedisHandler {
    _client: RedisClient,
    _sys: System,
}

impl RedisHandler {
    pub async fn init(conf: &AppConfigByRedis) -> Result<(), Box<dyn std::error::Error>> {
        let config = RedisConfig::from_url(&conf.url.as_str())?;
        let perf = PerformanceConfig::default();
        let policy = ReconnectPolicy::new_exponential(match &conf.max_attempts {
            None => 0,
            Some(active) => *active
        }, match &conf.min_delay {
            None => 100,
            Some(active) => *active
        }, match &conf.max_delay {
            None => 30_000,
            Some(active) => *active
        }, match &conf.mult {
            None => 2,
            Some(active) => *active
        });
        let client = RedisClient::new(config, Some(perf), Some(policy));
        //let mut error_rx = client.on_error();
        //error_rx.recv().await?;


        //let _ = client.connect();
        //let _ = client.wait_for_connect().await?;

        let _owner = Self {
            _client:client,
            _sys: System::new(),
        };

        let mut _interval = tokio::time::interval(tokio::time::Duration::from_secs(match &conf.interval {
            None => 3,
            Some(active) => *active,
        }));


        let _timezone = match &conf.timezone {
            None => "local".to_string(),
            Some(active) => active.clone()
        };

        let _prefix = match &conf.prefix{
            None => "".to_string(),
            Some(active) => active.clone(),
        };


        // each
        // loop {
        //     interval.tick().await;
        //     if let Ok(stat) = SystemInfo::sys(&owner.sys) {
        //         let now = match timezone.as_str() {
        //             "utc" => chrono::Utc::now().timestamp(),
        //             &_ => chrono::Local::now().timestamp(),
        //         };
        //
        //         // battery_life
        //         if let Some(battery_life) = stat.battery_life {}
        //
        //
        //         // cpu_load
        //         if let Some(cpu_load) = stat.cpu_load {
        //             if let Ok(json) =  serde_json::to_string(&cpu_load){
        //                 let key = format!("{}{}",prefix,"cpu_load");
        //                 debug!("save {} = {}",key,json);
        //             }
        //         }
        //
        //     };
        // }
        Ok(())
    }
}
