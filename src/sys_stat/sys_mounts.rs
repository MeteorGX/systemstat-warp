use serde::{Deserialize, Serialize};
use systemstat::{Platform, saturating_sub_bytes};
use systemstat::System;
use crate::{SysInfo, SysReply};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SysMount {
    pub files: usize,
    pub files_total: usize,
    pub files_avail: usize,
    pub free: u64,
    pub free_str: String,
    pub avail: u64,
    pub avail_str: String,
    pub total: u64,
    pub total_str: String,
    pub used: u64,
    pub used_str: String,
    pub name_max: usize,
    pub fs_type: String,
    pub fs_mounted_from: String,
    pub fs_mounted_on: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SysMounts {
    pub mounts: Vec<SysMount>,
}

impl SysInfo<System, SysMounts> for SysMounts {
    fn sys(handler: &System) -> std::io::Result<SysMounts> {
        let mounts = handler.mounts()?;
        let mut data = Vec::with_capacity(mounts.len());
        for i in 0..mounts.len() {
            let filesystem = &mounts[i];
            let used = saturating_sub_bytes(filesystem.total, filesystem.free);
            data.push(SysMount {
                files: filesystem.files,
                files_total: filesystem.files_total,
                files_avail: filesystem.files_avail,
                free: filesystem.free.as_u64(),
                free_str: filesystem.free.to_string(),
                avail: filesystem.avail.as_u64(),
                avail_str: filesystem.avail.to_string(),
                total: filesystem.total.as_u64(),
                total_str: filesystem.total.to_string(),
                used: used.as_u64(),
                used_str: used.to_string(),
                name_max: filesystem.name_max,
                fs_type: filesystem.fs_type.clone(),
                fs_mounted_from: filesystem.fs_mounted_from.clone(),
                fs_mounted_on: filesystem.fs_mounted_on.clone(),
            })
        }
        Ok(Self {
            mounts: data,
        })
    }
}

impl SysReply for SysMounts {}

