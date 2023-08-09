use std::net::SocketAddr;
use lazy_static::lazy_static;
use log::{debug, error};
use systemstat::{Platform, System};
use warp::{Filter, Rejection, Reply};
use warp::http::StatusCode;
use crate::{AppConfigByWeb, SysInfo, SysReply, SystemInfo};

#[derive(Clone)]
pub struct WebRouteTable {
    conf: AppConfigByWeb,
}

lazy_static! {
    static ref WEB_SYSTEM_HANDLER:System = System::new();
}

impl WebRouteTable {
    pub fn create(conf: AppConfigByWeb) -> Self {
        Self {
            conf,
        }
    }

    async fn sys() -> Result<Box<dyn Reply>, Rejection> {
        let sys = &WEB_SYSTEM_HANDLER;
        match SystemInfo::sys(&sys) {
            Ok(stat) => Ok(Box::new(stat.reject())),
            Err(e) => {
                error!("{:?}",e);
                Ok(Box::new(StatusCode::BAD_REQUEST))
            }
        }
    }


    pub async fn build(&self, socket: SocketAddr) {
        if let Some(dir) = &self.conf.static_dir {
            debug!("static dir = {}",dir);
            let dir_name = dir.clone();
            let routes = warp::fs::dir(dir_name)
                .or(warp::path("sys")
                    .and_then(Self::sys));
            warp::serve(routes)
                .run(socket)
                .await;
        } else {
            warp::serve(warp::path("sys").and_then(Self::sys))
                .run(socket)
                .await;
        }
    }
}
