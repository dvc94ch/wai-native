fn main() {
    println!("cargo:rustc-link-search=target/i686-unknown-linux-gnu/debug/");
    println!("cargo:rustc-link-lib=static=api_stripped");
}
