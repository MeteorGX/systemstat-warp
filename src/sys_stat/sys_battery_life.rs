use std::time::Duration;
use serde::{Deserialize, Serialize};
use systemstat::Platform;
use systemstat::System;
use crate::{SysInfo};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SysBatteryLife {
    remaining_capacity: f32,
    remaining_time: Duration,
}


impl SysInfo<System, SysBatteryLife> for SysBatteryLife {
    fn sys(handler: &System) -> std::io::Result<SysBatteryLife> {
        let stat = handler.battery_life()?;
        Ok(Self {
            remaining_capacity: stat.remaining_capacity,
            remaining_time: stat.remaining_time,
        })
    }
}

