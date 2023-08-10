use serde::{Deserialize, Serialize};
use systemstat::Platform;
use systemstat::System;
use crate::sys_stat::SysInfo;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SysBootTime {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub millis: u16,
    pub nanos: u32,
}


impl SysInfo<System, SysBootTime> for SysBootTime {
    fn sys(handler: &System) -> std::io::Result<SysBootTime> {
        let boot_time = handler.boot_time()?;
        Ok(SysBootTime {
            year: boot_time.year(),
            month: boot_time.month().into(),
            day: boot_time.day(),
            hour: boot_time.hour(),
            minute: boot_time.minute(),
            second: boot_time.second(),
            millis: boot_time.millisecond(),
            nanos: boot_time.nanosecond(),
        })
    }
}





