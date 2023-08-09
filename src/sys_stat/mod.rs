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
use warp::http;
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


pub type JsonReply = warp::reply::WithStatus<String>;

pub type JsonReplyResult = Result<JsonReply,warp::Rejection>;



pub trait SysReply<T:Serialize> {
    fn reject(stat:&T) -> JsonReply {
        match serde_json::to_string(&stat) {
            Ok(e) => warp::reply::with_status(e, http::StatusCode::OK),
            Err(e) => warp::reply::with_status(e.to_string(), http::StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub enum SysNetworkAddressEnum {
    Empty,
    Unsupported,
    IPv4(Ipv4Addr),
    IPv6(Ipv6Addr),
}
