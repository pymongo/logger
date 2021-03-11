fn main() {
    if !cfg!(target_os = "linux") {
        panic!("systemd lib only support linux");
    }
    println!("cargo:rustc-link-lib=dylib=systemd");
}
