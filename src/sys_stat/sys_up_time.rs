use serde::{Deserialize, Serialize};
use systemstat::{Platform, System};
use crate::{SysInfo, SysReply};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SysUpTime {
    secs: u64,
    nanos: u128,
    micros: u128,
    millis: u128,
}


impl SysInfo<System, SysUpTime> for SysUpTime {
    fn sys(handler: &System) -> std::io::Result<SysUpTime> {
        let stat = handler.uptime()?;
        Ok(Self {
            secs: stat.as_secs(),
            nanos: stat.as_nanos(),
            micros: stat.as_micros(),
            millis: stat.as_millis(),
        })
    }
}


impl SysReply for SysUpTime {}

