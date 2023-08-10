use log::info;
use tokio::io::AsyncReadExt;
use systemstat_warp::{AppConfig, LoggerHandler, WebHandler};


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


    // logger handler
    let config = AppConfig::parse(config_context.as_str())?;
    if let Some(basic) = config.basic {
        info!("[Basic] = {:?}",basic);
        LoggerHandler::init(&basic)?;
    }


    // network
    if let Some(web) = config.web {
        info!("[Web] = {:?}",web);
        WebHandler::init(&web).await?;
    }


    Ok(())
}
