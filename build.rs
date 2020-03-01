use std::env;
use std::path::PathBuf;

fn main() {
    let lib_dir = env::var("ONESDK_LIB_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", lib_dir);
    println!("cargo:rustc-link-lib=static=onesdk_static");
}
