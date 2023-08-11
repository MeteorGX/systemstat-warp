use log::{error, info};
use tokio::io::AsyncReadExt;
use tokio::spawn;
use systemstat_warp::{AppConfig, DBHandler, LoggerHandler, RedisHandler, WebHandler};


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
        LoggerHandler::init(&basic)?;
        info!("[Basic] = {:?}",basic);
    }

    // initialize tasks
    let mut tasks = vec![];


    // network
    if let Some(web) = config.web {
        info!("[Web] = {:?}",web);
        tasks.push(spawn(async move {
            match WebHandler::init(&web).await {
                Ok(_) => (),
                Err(e) => {
                    error!("{:?}",e)
                }
            };
        }));
    }


    // database
    if let Some(db) = config.db {
        info!("[DB] = {:?}",db);
        tasks.push(spawn(async move {
            match DBHandler::init(&db).await {
                Ok(_) => (),
                Err(e) => {
                    error!("{:?}",e)
                }
            };
        }));
    }


    // redis
    if let Some(redis) = config.redis {
        info!("[REDIS] = {:?}",redis);
        tasks.push(spawn(async move {
            match RedisHandler::init(&redis).await {
                Ok(_) => (),
                Err(e) => {
                    error!("{:?}",e)
                }
            };
        }));
    }


    // execute tasks
    for task in tasks {
        task.await.unwrap();
    }

    Ok(())
}
