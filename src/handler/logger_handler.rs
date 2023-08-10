use log::SetLoggerError;
use simple_logger::SimpleLogger;
use crate::AppConfigByBasic;

pub struct LoggerHandler;


impl LoggerHandler {
    pub fn init(conf: &AppConfigByBasic) -> Result<(), SetLoggerError> {
        let logger_handler = SimpleLogger::new();

        // initialize logger level
        let logger_handler = match &conf.log {
            None => logger_handler,
            Some(active) => logger_handler.with_level(
                match active.as_str() {
                    "debug" => log::LevelFilter::Debug,
                    "info" => log::LevelFilter::Info,
                    "trace" => log::LevelFilter::Trace,
                    "off" => log::LevelFilter::Warn,
                    "warn" => log::LevelFilter::Off,
                    _ => log::LevelFilter::Error,
                }
            ),
        };


        // initialize logger timezone
        let logger_handler = match &conf.timezone {
            None => logger_handler,
            Some(active) => match active.as_str() {
                "local" => logger_handler.with_local_timestamps(),
                "utc" => logger_handler.with_utc_timestamps(),
                _ => logger_handler
            }
        };
        logger_handler.init()
    }
}
