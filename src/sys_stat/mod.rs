mod sys_boot_time;
mod sys_load_aggregate;
mod sys_memory;
mod sys_sockets;
mod sys_mounts;
mod sys_battery_life;
mod sys_swap;
mod sys_cpu_load;
mod sys_networks;
mod sys_up_time;


use std::net::{Ipv4Addr, Ipv6Addr};
use serde::{Deserialize, Serialize};
use systemstat::System;
pub use sys_boot_time::*;
pub use sys_load_aggregate::*;
pub use sys_memory::*;
pub use sys_sockets::*;
pub use sys_mounts::*;
pub use sys_battery_life::*;
pub use sys_swap::*;
pub use sys_cpu_load::*;
pub use sys_networks::*;
pub use sys_up_time::*;


pub trait SysInfo<H, T> {
    fn sys(handler: &H) -> std::io::Result<T>;
}






#[derive(Serialize, Deserialize, Debug)]
pub enum SysNetworkAddressEnum {
    Empty,
    Unsupported,
    IPv4(Ipv4Addr),
    IPv6(Ipv6Addr),
}


#[derive(Serialize, Deserialize)]
pub struct SystemInfo {
    pub battery_life: Option<SysBatteryLife>,
    pub boot_time: Option<SysBootTime>,
    pub cpu_load: Option<SysCpuLoad>,
    pub load_aggregate: Option<SysLoadAggregate>,
    pub memory: Option<SysMemory>,
    pub swap: Option<SysSwap>,
    pub up_time: Option<SysUpTime>,
    pub sockets: Option<SysSockets>,
    //pub mounts: Option<SysMounts>,
    // pub networks: Option<SysNetworkMap>,
}

impl Default for SystemInfo{
    fn default() -> Self {
        Self{
            battery_life: None,
            boot_time: None,
            cpu_load: None,
            load_aggregate: None,
            memory: None,
            swap: None,
            up_time: None,
            sockets: None,
            // mounts: None,
            // networks: None,
        }
    }
}

impl SysInfo<System, SystemInfo> for SystemInfo {
    fn sys(handler: &System) -> std::io::Result<SystemInfo> {
        Ok(Self {
            battery_life: match SysBatteryLife::sys(handler) {
                Ok(stat) => Some(stat),
                Err(_e) => None,
            },
            boot_time: match SysBootTime::sys(handler) {
                Ok(stat) => Some(stat),
                Err(_e) => None,
            },
            cpu_load: match SysCpuLoad::sys(handler) {
                Ok(stat) => Some(stat),
                Err(_e) => None,
            },
            load_aggregate: match SysLoadAggregate::sys(handler) {
                Ok(stat) => Some(stat),
                Err(_e) => None,
            },
            memory: match SysMemory::sys(handler) {
                Ok(stat) => Some(stat),
                Err(_e) => None,
            },
            swap: match SysSwap::sys(handler) {
                Ok(stat) => Some(stat),
                Err(_e) => None,
            },
            up_time: match SysUpTime::sys(handler) {
                Ok(stat) => Some(stat),
                Err(_e) => None,
            },
            sockets: match SysSockets::sys(handler) {
                Ok(stat) => Some(stat),
                Err(_e) => None,
            },
            // mounts: match SysMounts::sys(handler) {
            //     Ok(stat) => Some(stat),
            //     Err(_e) => None,
            // },
            // networks: match SysNetworkMap::sys(handler) {
            //     Ok(stat) => Some(stat),
            //     Err(_e) => None,
            // },

        })
    }
}

