# Rust logger for systemd-journald

## logger output example
```text
[w@w-manjaro journal]$ journalctl --no-hostname -uapi -pinfo --no-pager -n 50
Mar 17 16:57:29 api[793078]: INFO [api::config,api/src/config.rs:87] config_file_path="api/config/api.toml"
Mar 17 16:57:34 api[793078]: INFO [api::app,api/src/app.rs:128] ServerInfo {
                                 git_commit_hash: "3425b945bed73267b628d37bd1cd504d0d00b86c",
                                 git_commit_message: "fix login response empty",
                                 git_commit_date: "Fri Mar 12 22:06:42 2021 +0800",
                                 compile_at: "Wed Mar 17 02:52:45 PM CST 2021",
                                 rust_version: "rustc 1.52.0-nightly (d6eaea1c8 2021-03-14)",
                                 cargo_pkg_version: "0.1.0",
                             }
Mar 17 16:57:34 api[793078]: INFO [,ec823/warp-0.3.0/src/server.rs:133] Server::run; addr=127.0.0.1:7000
Mar 17 16:57:34 api[793078]: INFO [,ec823/warp-0.3.0/src/server.rs:134] listening on http://127.0.0.1:7000
Mar 18 10:51:25 api[793078]: INFO [warp::filters::log,/warp-0.3.0/src/filters/log.rs:37] 127.0.0.1:40018 "GET /api HTTP/1.1" 200 "-" "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36" 3.268563ms
Mar 18 10:51:27 api[793078]: INFO [warp::filters::log,/warp-0.3.0/src/filters/log.rs:37] 127.0.0.1:40046 "POST /api HTTP/1.1" 200 "http://localhost:7000/api" "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36" 63.721151ms
Mar 18 10:51:52 api[793078]: INFO [async_graphql::extensions::logger,.5.14/src/extensions/logger.rs:52] [Query] query: "{test}", variables: {}
Mar 18 10:51:52 api[793078]: ERROR[async_graphql::extensions::logger,.5.14/src/extensions/logger.rs:108] [Error] pos: [2:3], query: "{test}", variables: {}Unknown field "test" on type "QueryRoot".
```

journalctl query by time range example:

> journalctl --since "2021-01-10 17:15:00" --until "now" -u > journalctl -u service_name -n 20 --no-pager
 -n 20 --no-pager
