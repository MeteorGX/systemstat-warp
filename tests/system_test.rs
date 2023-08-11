

#[cfg(test)]
mod tests {
    use systemstat::{Platform, System};
    use systemstat_warp::*;

    #[tokio::test]
    async fn sys_works() -> Result<(), Box<dyn std::error::Error>> {
        let sys = System::new();
        let up_time = SysUpTime::sys(&sys)?;
        println!("{:?}", up_time);

        let boot_time = SysBootTime::sys(&sys)?;
        println!("{:?}", boot_time);

        let cpu_load = SysCpuLoad::async_sys(&sys).await?;
        println!("{:?}", cpu_load);

        // only linux/unix
        // let cpu_load_aggregate = SysLoadAggregate::sys(&sys)?;
        // println!("{:?}",cpu_load_aggregate);

        let memory = SysMemory::sys(&sys)?;
        println!("{:?}", memory);

        let sockets = SysSockets::sys(&sys)?;
        println!("{:?}", sockets);

        let mounts = SysMounts::sys(&sys)?;
        println!("{:?}", mounts);

        let swap = SysSwap::sys(&sys)?;
        println!("{:?}", swap);

        //let battery = SysBatteryLife::sys(&sys)?;
        //println!("{:?}",battery);

        let networks = SysNetworkMap::sys(&sys)?;
        println!("{:?}", networks);

        Ok(())
    }


}