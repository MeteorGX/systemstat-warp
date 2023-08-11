use crate::AppConfigByDB;

pub struct DBHandler {}

impl DBHandler {
    /// todo: database initialize
    /// error: could not find native static library `mysqlclient`, perhaps an -L flag is missing?
    pub async fn init(conf: &AppConfigByDB) -> Result<(), Box<dyn std::error::Error>> {
        let _url = &conf.url;
        //Some(MysqlConnection::establish(&url))
        //Some(PgConnection::establish(&url))

        Ok(())
    }
}

