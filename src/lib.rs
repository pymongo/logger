/**
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
*/
pub struct Logger {
    log_level: log::Level,
}

impl Logger {
    pub fn with_level(log_level: log::Level) -> Self {
        Self { log_level }
    }
    pub fn init(self) {
        // log::Level::Error=1, log::Level::Trace=5
        log::set_max_level(self.log_level.to_level_filter());
        log::set_boxed_logger(Box::new(self)).expect("logger has init");
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.log_level
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let mut file = record.file().unwrap_or_default();
        let file_str_len = file.len();
        if file_str_len > 30 {
            file = &file[file_str_len - 30..file_str_len];
        }
        println!(
            "{:<5}[{},{}:{}] {}",
            record.level(),
            record.module_path_static().unwrap_or_default(),
            file,
            record.line().unwrap_or_default(),
            record.args()
        );
    }

    fn flush(&self) {}
}

type size_t = usize;
use std::ffi::c_void;
use std::os::raw::c_int;

/** https://man7.org/linux/man-pages/man2/readv.2.html#DESCRIPTION
```c
struct iovec {
    void  *iov_base;    /* Starting address */
    size_t iov_len;     /* Number of bytes to transfer */
};
```
*/
#[derive(Clone, Debug)]
#[repr(C)]
struct iovec {
    iov_base: *const c_void,
    iov_len: size_t
}

impl From<&str> for iovec {
    fn from(s: &str) -> Self {
        Self {
            iov_base: s.as_ptr() as *const c_void,
            iov_len: s.len() as size_t
        }
    }
}

extern "C" {
    /// https://man7.org/linux/man-pages/man3/sd_journal_send.3.html
    /// int sd_journal_sendv(const struct iovec *iov, int n);
    fn sd_journal_sendv(iov: *const iovec, n: c_int) -> c_int;
}

struct LoggerSystemd {
    systemd_unit: iovec
}

impl LoggerSystemd {
    fn new(systemd_unit_name: &str) -> Self {
        Self {
            systemd_unit: format!("TARGET={}\0", systemd_unit_name).as_str().into()
        }
    }

    /**
遇到知识盲区了，调用systemd的sd_journal_sendv API需要传入iovec类型的数组
如果iovec数组内有个成员是通过 String::as_str().into() 转换的话，则API会调用失败。

我看rust-systemd源码用AsRef::as_ref将String转&str没有用as_str
为什么调用as_str()再取原始指针就不行呢？
    */
    fn log2(&self) {
        let priority_iovec: iovec = "PRIORITY=3".into();
        // 错误写法: format!().as_str().into::<iovec>()不能正确得到字符串地址
        let msg_iovec: iovec = "MESSAGE=log1".to_string().as_str().into();
        // ok: let msg_iovec: iovec = "MESSAGE=log1".into();
        let iovecs = vec![priority_iovec, msg_iovec];

        let ret: Vec<iovec> = vec!["PRIORITY=3", "MESSAGE=log2"].into_iter().map(|x| x.into()).collect();
        unsafe {
            sd_journal_sendv(iovecs.as_ptr(), iovecs.len() as c_int);
            sd_journal_sendv(ret.as_ptr(), ret.len() as c_int);
        }
    }
}
#[test]
fn feature() {
    let logger = LoggerSystemd::new("api");
    logger.log2();
}
