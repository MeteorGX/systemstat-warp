use serde::{Deserialize, Serialize};
use systemstat::Platform;
use systemstat::System;
use crate::{SysInfo, SysReply};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SysLoadAggregate {
    pub one: f32,
    pub five: f32,
    pub fifteen: f32,
}


impl SysInfo<System, SysLoadAggregate> for SysLoadAggregate {
    fn sys(handler: &System) -> std::io::Result<SysLoadAggregate> {
        let stat = handler.load_average()?;
        Ok(SysLoadAggregate {
            one: stat.one,
            five: stat.five,
            fifteen: stat.fifteen,
        })
    }
}


impl SysReply for SysLoadAggregate {}
