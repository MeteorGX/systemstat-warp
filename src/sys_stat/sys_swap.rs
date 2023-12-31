use serde::{Deserialize, Serialize};
use systemstat::{Platform, saturating_sub_bytes, System};
use crate::{SysInfo};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SysSwap {
    pub total: u64,
    pub total_str: String,
    pub free: u64,
    pub free_str: String,
    pub used: u64,
    pub used_str: String,
}


impl SysInfo<System, SysSwap> for SysSwap {
    fn sys(handler: &System) -> std::io::Result<SysSwap> {
        let stat = handler.swap()?;
        let used = saturating_sub_bytes(stat.total, stat.free);
        Ok(Self {
            total: stat.total.as_u64(),
            total_str: stat.total.to_string(),
            free: stat.free.as_u64(),
            free_str: stat.free.to_string(),
            used: used.as_u64(),
            used_str: used.to_string(),
        })
    }
}

