use std::time::Duration;
use serde::{Deserialize, Serialize};
use systemstat::Platform;
use systemstat::System;
use crate::{SysInfo};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SysCpuLoad {
    pub user: f32,
    pub user_str: String,
    pub nice: f32,
    pub nice_str: String,
    pub sys: f32,
    pub sys_str: String,
    pub interrupt: f32,
    pub interrupt_str: String,
    pub idle: f32,
    pub idle_str: String,
}


impl SysCpuLoad {
    pub async fn async_sys(handler: &System) -> std::io::Result<Self> {
        let cpu = handler.cpu_load_aggregate()?;
        tokio::time::sleep(Duration::from_secs(1)).await;
        let cpu = cpu.done()?;
        Ok(Self {
            user: cpu.user,
            user_str: format!("{}%", cpu.user * 100.0),
            nice: cpu.nice,
            nice_str: format!("{}%", cpu.nice * 100.0),
            sys: cpu.system,
            sys_str: format!("{}%", cpu.system * 100.0),
            interrupt: cpu.interrupt,
            interrupt_str: format!("{}%", cpu.interrupt * 100.0),
            idle: cpu.idle,
            idle_str: format!("{}%", cpu.idle * 100.0),
        })
    }
}


impl SysInfo<System, SysCpuLoad> for SysCpuLoad {
    fn sys(handler: &System) -> std::io::Result<SysCpuLoad> {
        let cpu = handler.cpu_load_aggregate()?;
        std::thread::sleep(Duration::from_secs(1));
        let cpu = cpu.done()?;
        Ok(Self {
            user: cpu.user,
            user_str: format!("{}%", cpu.user * 100.0),
            nice: cpu.nice,
            nice_str: format!("{}%", cpu.nice * 100.0),
            sys: cpu.system,
            sys_str: format!("{}%", cpu.system * 100.0),
            interrupt: cpu.interrupt,
            interrupt_str: format!("{}%", cpu.interrupt * 100.0),
            idle: cpu.idle,
            idle_str: format!("{}%", cpu.idle * 100.0),
        })
    }
}




