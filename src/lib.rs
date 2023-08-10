mod sys_stat;
mod config;

mod cipher;
mod handler;


pub use sys_stat::*;
pub use config::*;
pub use cipher::*;
pub use handler::*;


#[cfg(test)]
mod tests {
    use systemstat::{Platform, System};
    use super::*;


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


    #[tokio::test]
    async fn cipher_works() -> Result<(), Box<dyn std::error::Error>> {
        let source = "meteorCat123";

        let cipher256 = Cipher256::create(source.as_bytes());
        println!("sha256({}) = {:?}", source, cipher256.cipher());

        let cipher512 = Cipher512::create(source.as_bytes());
        println!("sha512({}) = {:?}", source, cipher512.cipher());


        Ok(())
    }


    #[tokio::test]
    async fn users_works() -> Result<(), Box<dyn std::error::Error>> {
        // first user
        let username_first = "MeteorCat";
        let password_first = "MeteorCatPassword";
        let cipher_first = Cipher256::create(password_first.as_bytes());
        let hash_first = cipher_first.cipher().unwrap_or_default();

        // last user
        let username_last = "MeteorCatLast";
        let password_last = "MeteorCatPasswordLast";
        let cipher_last = Cipher256::create(password_last.as_bytes());
        let hash_last = cipher_last.cipher().unwrap_or_default();

        // merge user
        let line = format!("{}:{},{}:{}", username_first, hash_first, username_last, hash_last);
        println!("Users = {}",line);

        // parse users
        let users:UserConfig = line.as_str().into();
        println!("User Struct = {:?}",users);


        Ok(())
    }
}
