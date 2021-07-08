#[cfg(not(target_os = "linux"))]
compile_error!("libsystemd.so only support on linux");

fn main() {
    println!("cargo:rustc-link-lib=dylib=systemd");
}
