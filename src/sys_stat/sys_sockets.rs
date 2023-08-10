use serde::{Deserialize, Serialize};
use systemstat::{Platform, System};
use crate::{SysInfo};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SysSockets {
    pub tcp_sockets_in_use: usize,
    pub tcp_sockets_orphaned: usize,
    pub udp_sockets_in_use: usize,
    pub tcp6_sockets_in_use: usize,
    pub udp6_sockets_in_use: usize,
}

impl SysInfo<System, SysSockets> for SysSockets {
    fn sys(handler: &System) -> std::io::Result<SysSockets> {
        let stat = handler.socket_stats()?;
        Ok(Self {
            tcp_sockets_in_use: stat.tcp_sockets_in_use,
            tcp_sockets_orphaned: stat.tcp_sockets_orphaned,
            udp_sockets_in_use: stat.udp_sockets_in_use,
            tcp6_sockets_in_use: stat.tcp6_sockets_in_use,
            udp6_sockets_in_use: stat.udp6_sockets_in_use,
        })
    }
}




