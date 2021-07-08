#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation,
    clippy::doc_markdown,
    clippy::non_ascii_literal
)]
use std::os::raw::{c_char, c_int};

/**
## libsystemd reference
- https://man7.org/linux/man-pages/man3/sd_journal_send.3.html
- https://man7.org/linux/man-pages/man7/systemd.journal-fields.7.html
- http://0pointer.de/blog/projects/journal-submit.html
- https://github.com/jmesmon/rust-systemd/blob/master/libsystemd-sys/src/journal.rs
*/
#[link(name = "systemd", kind = "dylib")]
extern "C" {
    /// fn sd_journal_send(format: *const c_char, ...);
    fn sd_journal_send(
        priority: *const c_char,
        message: *const c_char,
        terminator: *const c_char,
    ) -> c_int;
}

pub fn rust_log_level_to_syslog_rpiority(log_level: log::Level) -> libc::c_int {
    match log_level {
        // LOG_EMERG = 0
        // LOG_ALERT = 1
        // LOG_CRIT = 2
        log::Level::Error => libc::LOG_ERR,    // 4
        log::Level::Warn => libc::LOG_WARNING, // 5
        // LOG_NOTIC = 6
        log::Level::Info => libc::LOG_INFO, // 7
        log::Level::Debug | log::Level::Trace => libc::LOG_DEBUG, // 8
    }
}

/// Rust logger for systemd-journal
/// Only target_os=linux will use systemd API, other os just print to STDOUT
pub struct Logger;

impl Logger {
    pub fn init(self) {
        if !cfg!(target_os = "linux") {
            eprintln!("warning: journal_logger use libsystemd.so is only support on Linux");
        }
        log::set_max_level(log::Level::Trace.to_level_filter());
        log::set_boxed_logger(Box::new(self)).expect("logger has init");
    }
}

impl log::Log for Logger {
    /// use journalctl's -p argument to filter by log_level
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        //metadata.level() <= self.log_level
        true
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
        let log_message = format!(
            "{:<5}[{},{}:{}] {}",
            record.level(),
            record.module_path_static().unwrap_or_default(),
            file,
            record.line().unwrap_or_default(),
            record.args()
        );

        let priority = rust_log_level_to_syslog_rpiority(record.level());
        let priority_str = format!("PRIORITY={}\0", priority);
        let message_str = format!("MESSAGE={}\0", log_message);
        unsafe {
            sd_journal_send(
                priority_str.as_ptr().cast(),
                message_str.as_ptr().cast(),
                "\0".as_ptr().cast(),
            );
        }
    }

    fn flush(&self) {}
}

#[test]
fn test_logger() {
    Logger.init();
    log::trace!("log_level=trace");
    log::debug!("log_level=debug");
    log::info!("log_level=info");
    log::warn!("log_level=warn");
    log::error!("log_level=error");
}

#[test]
fn test_sd_journal_send() {
    unsafe {
        sd_journal_send(
            "PRIORITY=4\0".as_ptr().cast(),
            "MESSAGE=hello\0".as_ptr().cast(),
            std::ptr::null::<*const c_char>().cast(),
        );
    }
}

#[cfg(test)]
mod test_sd_journal_sendv {
    use std::os::raw::{c_int, c_void};

    #[link(name = "systemd", kind = "dylib")]
    extern "C" {
        fn sd_journal_sendv(iov: *const iovec, n: c_int) -> c_int;
    }

    /** https://man7.org/linux/man-pages/man2/readv.2.html#DESCRIPTION
    ## 错误记录: String::as_str().as_ptr() as *const c_void就不能找到正确的字符串内存地址
    遇到知识盲区了，调用systemd的sd_journal_sendv API需要传入iovec类型的数组
    如果iovec数组内有个成员是通过 String::as_str().into() 转换的话，则API会调用失败。
    我看rust-systemd源码用AsRef::as_ref将String转&str没有用as_str
    为什么调用as_str()再取原始指针就不行呢？

    ```text
    // 错误写法: `format!("").as_str().into::<iovec>()` 或 `"MESSAGE=log1".to_string().as_str().into()`
    // 错误不能正确得到字符串地址
    let msg_iovec: iovec = "MESSAGE=log1".into();
    ```
    */
    #[repr(C)]
    struct iovec {
        iov_base: *const c_void,
        iov_len: usize,
    }

    impl From<&str> for iovec {
        fn from(s: &str) -> Self {
            Self {
                iov_base: s.as_ptr().cast(),
                iov_len: s.len(),
            }
        }
    }

    #[test]
    fn test_sd_journal_sendv() {
        let priority_iovec: iovec = "PRIORITY=3".into();
        // 错误写法: `format!("").as_str().into()` 或 `"MESSAGE=log1".to_string().as_str().into()`
        // 错误写法不能正确得到字符串地址
        // 正确写法: let msg_iovec: iovec = "MESSAGE=log1".into();
        let msg_iovec: iovec = format!("MESSAGE={}", "log1").as_str().into();
        let iovecs = vec![priority_iovec, msg_iovec];

        let ret: Vec<iovec> = vec!["PRIORITY=3", "MESSAGE=log2", "UNIT=api"]
            .into_iter()
            .map(|x| x.into())
            .collect();
        unsafe {
            sd_journal_sendv(iovecs.as_ptr(), iovecs.len() as c_int);
            sd_journal_sendv(ret.as_ptr(), ret.len() as c_int);
        }
    }
}
