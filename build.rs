fn main() {
    println!(
        "cargo:rustc-env=DECACHE_VERSION={}",
        std::env::var("DECACHE_VERSION").unwrap_or("0.1.2".into())
    );
    println!(
        "cargo:rustc-env=BUILD_DATE={}",
        chrono::Local::now().to_rfc3339()
    );
    println!(
        "cargo:rustc-env=BUILD_TARGET={}",
        std::env::var("TARGET").unwrap()
    );
}
