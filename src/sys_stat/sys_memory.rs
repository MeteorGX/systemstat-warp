use serde::{Deserialize, Serialize};
use systemstat::{Platform, saturating_sub_bytes};
use systemstat::System;
use crate::{SysInfo, SysReply};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SysMemory {
    pub total: u64,
    pub total_str: String,
    pub free: u64,
    pub free_str: String,
    pub used: u64,
    pub used_str: String,
}


impl SysInfo<System, SysMemory> for SysMemory {
    fn sys(handler: &System) -> std::io::Result<SysMemory> {
        let state = handler.memory()?;
        let used = saturating_sub_bytes(state.total, state.free);
        Ok(Self {
            total: state.total.as_u64(),
            total_str: state.total.to_string(),
            free: state.free.as_u64(),
            free_str: state.free.to_string(),
            used: used.as_u64(),
            used_str: used.to_string(),
        })
    }
}


impl SysReply for SysMemory {}
