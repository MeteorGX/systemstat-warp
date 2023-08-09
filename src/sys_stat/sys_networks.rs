use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use systemstat::{IpAddr, Platform};
use systemstat::platform::PlatformImpl;
use crate::{SysInfo, SysNetworkAddressEnum, SysReply};

#[derive(Serialize, Deserialize, Debug)]
pub struct SysNetworkAddress {
    pub addr: SysNetworkAddressEnum,
    pub netmask: SysNetworkAddressEnum,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SysNetworks {
    pub name: String,
    pub addrs: Vec<SysNetworkAddress>,
}

impl SysInfo<PlatformImpl, BTreeMap<String, SysNetworks>> for SysNetworks {
    fn sys(handler: &PlatformImpl) -> std::io::Result<BTreeMap<String, SysNetworks>> {
        let stat = handler.networks()?;
        let mut data = BTreeMap::new();
        for (k, v) in &stat {
            let mut addrs = Vec::with_capacity(v.addrs.len());
            for i in 0..v.addrs.len() {
                let addr = &v.addrs[i];
                addrs.push(SysNetworkAddress {
                    addr: match addr.addr {
                        IpAddr::Empty => SysNetworkAddressEnum::Empty,
                        IpAddr::Unsupported => SysNetworkAddressEnum::Unsupported,
                        IpAddr::V4(addr) => SysNetworkAddressEnum::IPv4(addr),
                        IpAddr::V6(addr) => SysNetworkAddressEnum::IPv6(addr),
                    },
                    netmask: match addr.netmask {
                        IpAddr::Empty => SysNetworkAddressEnum::Empty,
                        IpAddr::Unsupported => SysNetworkAddressEnum::Unsupported,
                        IpAddr::V4(addr) => SysNetworkAddressEnum::IPv4(addr),
                        IpAddr::V6(addr) => SysNetworkAddressEnum::IPv6(addr),
                    },
                })
            }

            data.insert(k.clone(), Self {
                name: v.name.clone(),
                addrs,
            });
        }
        Ok(data)
    }
}


impl SysReply<SysNetworks> for SysNetworks {}


impl SysReply<BTreeMap<String, SysNetworks>> for SysNetworks {}
