# Rust logger for systemd

## logger output example
```text
Jan 13 07:45:37 localhost api[32163]: WARN [os_info::imp::lsb_release,2.0.8/src/linux/lsb_release.rs:49] lsb_release command failed with Os { code: 2, kind: NotFound, message: "No such file or directory" }
Jan 13 07:45:37 localhost api[32163]: INFO [api::routes,api/src/routes.rs:63] ServerInfo {
Jan 13 07:45:37 localhost api[32163]: git_commit_hash: "5143b8de17ee633ccaa2ad18c878e77d0554da8c",
Jan 13 07:45:37 localhost api[32163]: git_commit_message: "update logger",
Jan 13 07:45:37 localhost api[32163]: git_commit_date: "Wed Jan 13 15:43:02 2021 +0800",
Jan 13 07:45:37 localhost api[32163]: compile_at: "Wed Jan 13 07:43:40 UTC 2021",
Jan 13 07:45:37 localhost api[32163]: rust_version: "rustc 1.49.0 (e1884a8e3 2020-12-29)",
Jan 13 07:45:37 localhost api[32163]: cargo_pkg_version: "0.1.0",
Jan 13 07:45:37 localhost api[32163]: }
Jan 13 07:45:37 localhost api[32163]: INFO [,ec823/warp-0.2.5/src/server.rs:133] Server::run; addr=127.0.0.1:7000
Jan 13 07:45:37 localhost api[32163]: INFO [,ec823/warp-0.2.5/src/server.rs:134] listening on http://127.0.0.1:7000
Jan 13 07:48:01 localhost api[32163]: INFO [async_graphql::extensions::logger,2.4.7/src/extensions/logger.rs:52] [Query] query:
```

## How log time
Use systemd/journalctl to log time

> journalctl -u service_name -f

> journalctl -u service_name -n 20 --no-pager

## How to query log in time range

> journalctl --since "2021-01-10 17:15:00" --until "now" -u > journalctl -u service_name -n 20 --no-pager
 -n 20 --no-pager
