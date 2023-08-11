# systemstat-warp

By using the Rust crates systemstat and tokio, I wrapped a Web API to monitor server performance.

```toml
[dependencies]
systemstat-warp = "1.0.0"
```

## Rust version requirements

systemstat-warp works with stable Rust, and typically works with the most recent prior
stable release as well.

## Building systemstat-warp

```shell
$ git clone https://github.com/MeteorGX/systemstat-warp.git
$ cd systemstat-warp
$ cargo build --release
```

## Example

- Url: [monitor.meteorcat.net](https://monitor.meteorcat.net)
- Username: `meteorcat`
- Password: `meteorcat`

## Configuration

```toml
# normal config
[default]
log = "debug" # logger level


# web config
[web]
address = "127.0.0.1" # listen server
port = 18081 # listen port
static_dir = "./html/" # static directory
cipher = "sha256" # cipher method
auth = "meteorcat:fe67eaa1f8b3b85a3a135795128abf9f4594f95128bef04c276f7dbcf1198b78" # auth[username:password(SHA256)]
```

## Run by Systemd

```ini
# vim /etc/systemd/system/systemstat-warp.service
[Unit]
Description = Systemstat-warp Server Monitor
Documentation = https://github.com/MeteorGX/systemstat-warp
After = network.target nss-lookup.target


[Service]
# 1. create systemstat user
# 2. cp systemstat-warp-cli /usr/bin
# 3. touch /etc/systemstat-warp.toml
User = nobody
ExecStart = /usr/bin/systemstat-warp-cli /etc/systemstat-warp.toml
Restart = on-failure
RestartPreventExitStatus = 23
LimitNOFILE = 65535

[Install]
WantedBy = multi-user.target
```

## Forward Nginx

```nginx
# monitor.conf
server {
    listen 80;
    server_name monitor.meteorcat.net;
    location / {
      proxy_pass http://127.0.0.1:18081;
      proxy_redirect off;
      proxy_set_header Host $host;
      proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
      proxy_set_header X-real-ip $remote_addr;
    }
}
```

# License

This project is licensed under either of

* MIT license ([LICENSE-MIT](LICENSE-MIT) or
  https://opensource.org/licenses/MIT)
