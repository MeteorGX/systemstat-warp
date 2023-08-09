use std::time::Duration;
use serde::{Deserialize, Serialize};
use systemstat::Platform;
use systemstat::platform::PlatformImpl;
use crate::{SysInfo, SysReply};

#[derive(Serialize, Deserialize, Debug)]
pub struct SysUpTime {
    duration: Duration,
}


impl SysInfo<PlatformImpl, SysUpTime> for SysUpTime {
    fn sys(handler: &PlatformImpl) -> std::io::Result<SysUpTime> {
        let stat = handler.uptime()?;
        Ok(Self {
            duration: stat
        })
    }
}


impl SysReply<SysUpTime> for SysUpTime {}

