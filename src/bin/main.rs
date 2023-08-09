use std::net::SocketAddr;
use log::info;
use simple_logger::SimpleLogger;
use tokio::io::AsyncReadExt;
use systemstat_warp::{AppConfig, SysRouteTable};



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // usage:
    //      systemstat-warp-cli.exe app.toml
    let mut args = std::env::args();
    let config_filename = args
        .nth(1)
        .ok_or(std::io::Error::from_raw_os_error(1))?;

    // file exists?
    let mut config_fd = tokio::fs::OpenOptions::new()
        .read(true)
        .write(false)
        .open(config_filename).await?;

    // read file
    let mut config_context = String::new();
    config_fd.read_to_string(&mut config_context).await?;


    // convert struct
    let config = AppConfig::parse(config_context.as_str())?;
    let logger = match config.basic {
        Some(active) => (
            active.log.unwrap_or("error".to_owned()),
            active.timezone.unwrap_or("local".to_owned()),
            active.log_file
        ),
        None => (
            "error".to_owned(),
            "local".to_owned(),
            None
        ),
    };

    // initialize logger
    match logger.1.as_str() {
        "local" => SimpleLogger::new().with_local_timestamps(),
        "utc" => SimpleLogger::new().with_utc_timestamps(),
        _ => SimpleLogger::new()
    }.with_level(match logger.0.as_str() {
        "debug" => log::LevelFilter::Debug,
        "info" => log::LevelFilter::Info,
        "trace" => log::LevelFilter::Trace,
        "off" => log::LevelFilter::Warn,
        "warn" => log::LevelFilter::Off,
        _ => log::LevelFilter::Error,
    }).init()?;



    // network
    if let Some(web) = config.web {
        info!("[Web] = {:?}",web);

        // address
        let address = format!("{}:{}",web.address,web.port);
        let socket = address.parse::<SocketAddr>()?;

        // server
        warp::serve(SysRouteTable::build_sys())
            .run(socket)
            .await;
    }


    Ok(())
}
