use std::sync::Arc;
use log::{debug, error};
use systemstat::{Platform, System};
use systemstat::platform::PlatformImpl;
use warp::{Filter, reject, Reply};
use warp::filters::BoxedFilter;
use crate::{JsonReplyResult, SysBatteryLife, SysBootTime, SysCpuLoad, SysInfo, SysLoadAggregate, SysMemory, SysMount, SysNetworks, SysReply, SysSockets, SysSwap, SysUpTime};

pub struct SysRouteTable;


impl SysRouteTable {
    async fn access_sys_battery_life(sys: Arc<PlatformImpl>) -> JsonReplyResult {
        match SysBatteryLife::sys(&sys) {
            Ok(stat) => {
                Ok(SysBatteryLife::reject(&stat))
            }
            Err(e) => {
                error!("{:?}",e);
                Err(reject())
            }
        }
    }

    async fn access_sys_boot_time(sys: Arc<PlatformImpl>) -> JsonReplyResult {
        match SysBootTime::sys(&sys) {
            Ok(stat) => {
                Ok(SysBootTime::reject(&stat))
            }
            Err(e) => {
                error!("{:?}",e);
                Err(reject())
            }
        }
    }

    async fn access_sys_cpu_load(sys: Arc<PlatformImpl>) -> JsonReplyResult {
        match SysCpuLoad::sys(&sys) {
            Ok(stat) => {
                Ok(SysCpuLoad::reject(&stat))
            }
            Err(e) => {
                error!("{:?}",e);
                Err(reject())
            }
        }
    }


    async fn access_sys_load_aggregate(sys: Arc<PlatformImpl>) -> JsonReplyResult {
        match SysLoadAggregate::sys(&sys) {
            Ok(stat) => {
                Ok(SysLoadAggregate::reject(&stat))
            }
            Err(e) => {
                error!("{:?}",e);
                Err(reject())
            }
        }
    }

    async fn access_sys_memory(sys: Arc<PlatformImpl>) -> JsonReplyResult {
        match SysMemory::sys(&sys) {
            Ok(stat) => {
                Ok(SysMemory::reject(&stat))
            }
            Err(e) => {
                error!("{:?}",e);
                Err(reject())
            }
        }
    }

    async fn access_sys_mount(sys: Arc<PlatformImpl>) -> JsonReplyResult {
        match SysMount::sys(&sys) {
            Ok(stat) => {
                Ok(SysMount::reject(&stat))
            }
            Err(e) => {
                error!("{:?}",e);
                Err(reject())
            }
        }
    }


    async fn access_sys_network(sys: Arc<PlatformImpl>) -> JsonReplyResult {
        match SysNetworks::sys(&sys) {
            Ok(stat) => {
                Ok(SysNetworks::reject(&stat))
            }
            Err(e) => {
                error!("{:?}",e);
                Err(reject())
            }
        }
    }

    async fn access_sys_socket(sys: Arc<PlatformImpl>) -> JsonReplyResult {
        match SysSockets::sys(&sys) {
            Ok(stat) => {
                Ok(SysSockets::reject(&stat))
            }
            Err(e) => {
                error!("{:?}",e);
                Err(reject())
            }
        }
    }

    async fn access_sys_swap(sys: Arc<PlatformImpl>) -> JsonReplyResult {
        match SysSwap::sys(&sys) {
            Ok(stat) => {
                Ok(SysSwap::reject(&stat))
            }
            Err(e) => {
                error!("{:?}",e);
                Err(reject())
            }
        }
    }

    async fn access_sys_up_time(sys: Arc<PlatformImpl>) -> JsonReplyResult {
        match SysUpTime::sys(&sys) {
            Ok(stat) => {
                Ok(SysUpTime::reject(&stat))
            }
            Err(e) => {
                error!("{:?}",e);
                Err(reject())
            }
        }
    }


    pub async fn dispose(path: warp::path::FullPath, sys: Arc<System>) -> JsonReplyResult {
        debug!("access path = {:?}",path);
        match path.as_str() {
            "/sys/battery_life" => {
                let sys_clone = sys.clone();
                Self::access_sys_battery_life(sys_clone).await
            }
            "/sys/boot_time" => {
                let sys_clone = sys.clone();
                Self::access_sys_boot_time(sys_clone).await
            }
            "/sys/cpu_load" => {
                let sys_clone = sys.clone();
                Self::access_sys_cpu_load(sys_clone).await
            }
            "/sys/load_aggregate" => {
                let sys_clone = sys.clone();
                Self::access_sys_load_aggregate(sys_clone).await
            }
            "/sys/memory" => {
                let sys_clone = sys.clone();
                Self::access_sys_memory(sys_clone).await
            }
            "/sys/mount" => {
                let sys_clone = sys.clone();
                Self::access_sys_mount(sys_clone).await
            }
            "/sys/network" => {
                let sys_clone = sys.clone();
                Self::access_sys_network(sys_clone).await
            }
            "/sys/socket" => {
                let sys_clone = sys.clone();
                Self::access_sys_socket(sys_clone).await
            }
            "/sys/swap" => {
                let sys_clone = sys.clone();
                Self::access_sys_swap(sys_clone).await
            }
            "/sys/up_time" => {
                let sys_clone = sys.clone();
                Self::access_sys_up_time(sys_clone).await
            }

            _ => Err(warp::reject()),
        }
    }

    pub fn build_sys() -> BoxedFilter<(impl Reply, )> {
        let sys = Arc::new(System::new());
        return warp::any()
            .and(warp::path::full())
            .and_then(move |path| Self::dispose(path, sys.clone())).boxed();
    }
}
