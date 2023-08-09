# systemstat-warp

By using the Rust crates systemstat and warp, I wrapped a Web API to monitor server performance.

```toml
[dependencies]
systemstat-warp = "1.0.0"
```

## Rust version requirements

systemstat-warp works with stable Rust, and typically works with the most recent prior
stable release as well.

## Building systemstat-warp

```shell
$ git clone https://github.com/meteorcat/systemstat-warp
$ cd systemstat-warp
$ cargo build
```

## Configuration

```toml
# examples in project directory 

# normal config
[default]
log = "debug" # logger level
log_file = "/var/log/systemstat-warp.log"


# web config
[web]
address = "127.0.0.1" # listen server
port = 18081 # listen port
auth = "xxx:yyy,zzz:aaa" # auth[username:password(SHA256)] header -> sys-cipher: xxx  
allow = "*" # allow access liste[default:*]
deny = "127.0.0.1,192.168.1.10,192.168.1.11" # deny access list

```

## Run systemstat-warp

```shell
systemstat-warp-cli config.toml
```

## Run by Systemd

```ini
# vim /etc/systemd/system/systemstat-warp.service
[Unit]
Description = Systemstat-warp Server Monitor
Documentation = https://github.com/meteorcat/systemstat-warp
After = network.target nss-lookup.target


[Service]
# 1. create systemstat user
# 2. cp systemstat-warp-cli /usr/bin
# 3. touch /etc/systemstat-warp.toml
User = systemstat
ExecStart = /usr/bin/systemstat-warp-cli /etc/systemstat-warp.toml
Restart = on-failure
RestartPreventExitStatus = 23
LimitNOFILE = 65535

[Install]
WantedBy = multi-user.target
```

# License

This project is licensed under either of

* MIT license ([LICENSE-MIT](LICENSE-MIT) or
  https://opensource.org/licenses/MIT)
