use std::time::Duration;
use serde::{Deserialize, Serialize};
use systemstat::Platform;
use systemstat::platform::PlatformImpl;
use crate::{SysInfo, SysReply};

#[derive(Serialize, Deserialize, Debug)]
pub struct SysBatteryLife {
    remaining_capacity:f32,
    remaining_time:Duration,
}


impl SysInfo<PlatformImpl,SysBatteryLife> for SysBatteryLife {
    fn sys(handler: &PlatformImpl) -> std::io::Result<SysBatteryLife> {
        let stat = handler.battery_life()?;
        Ok(Self{
            remaining_capacity: stat.remaining_capacity,
            remaining_time: stat.remaining_time,
        })
    }
}

impl SysReply<SysBatteryLife> for SysBatteryLife {}
