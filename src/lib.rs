use std::os::raw::{c_int, c_char};
/// Rust logger for systemd-journal
/// Only target_os=linux will use systemd API, other os just print to STDOUT
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

/**
## libsystemd reference
- https://man7.org/linux/man-pages/man3/sd_journal_send.3.html
- https://man7.org/linux/man-pages/man7/systemd.journal-fields.7.html
- http://0pointer.de/blog/projects/journal-submit.html
- https://github.com/jmesmon/rust-systemd/blob/master/libsystemd-sys/src/journal.rs
*/
#[link(name = "systemd", kind="dylib")]
extern "C" {
    //fn sd_journal_send(format: *const c_char, ...);
    fn sd_journal_send(priority: *const c_char, message: *const c_char, terminator: *const c_char) -> c_int;
}

#[test]
fn test_sd_journal_send() {
    unsafe {
        sd_journal_send(
            "PRIORITY=4\0".as_ptr() as *const c_char,
            "MESSAGE=hello\0".as_ptr() as *const c_char,
            std::ptr::null() as *const c_char
        );
    }
}

#[cfg(test)]
mod test_sd_journal_sendv {
    use std::os::raw::{c_int, c_void};

    #[link(name = "systemd", kind="dylib")]
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
        iov_len: usize
    }

    impl From<&str> for iovec {
        fn from(s: &str) -> Self {
            Self {
                iov_base: s.as_ptr() as *const c_void,
                iov_len: s.len()
            }
        }
    }

    #[test]
    fn test_sd_journal_sendv() {
        let priority_iovec: iovec = "PRIORITY=3".into();
        // 错误写法: `format!("").as_str().into::<iovec>()` 或 `"MESSAGE=log1".to_string().as_str().into()`
        // 错误写法不能正确得到字符串地址
        let msg_iovec: iovec = "MESSAGE=log1".into();
        let iovecs = vec![priority_iovec, msg_iovec];

        let ret: Vec<iovec> = vec!["PRIORITY=3", "MESSAGE=log2", "UNIT=api"].into_iter().map(|x| x.into()).collect();
        unsafe {
            sd_journal_sendv(iovecs.as_ptr(), iovecs.len() as c_int);
            sd_journal_sendv(ret.as_ptr(), ret.len() as c_int);
        }
    }
}
