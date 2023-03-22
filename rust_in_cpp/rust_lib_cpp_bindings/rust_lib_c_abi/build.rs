


#[cfg(feature = "binding-generation")]
extern crate cbindgen;

use std::env;
use std::path::PathBuf;


fn main() {

    #[cfg(feature = "binding-generation")]
    if let Some(_) = env::var_os("RUST_LIB_GENERATE_BINDINGS") {
        generate_bindings();
    }

}

#[cfg(feature = "binding-generation")]
fn generate_bindings() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let package_name = env::var("CARGO_PKG_NAME").unwrap();
    let output_file = target_dir()
        .join(format!("{}.hpp", package_name))
        .display()
        .to_string();

    let config = cbindgen::Config {
        namespace: Some(String::from("ffi")),
        ..Default::default()
    };

    cbindgen::generate_with_config(&crate_dir, config)
      .unwrap()
      .write_to_file(&output_file);
}

/// Find the location of the `target/` directory. Note that this may be 
/// overridden by `cmake`, so we also need to check the `CARGO_TARGET_DIR` 
/// variable.
fn target_dir() -> PathBuf {
    if let Ok(target) = env::var("CARGO_TARGET_DIR") {
        PathBuf::from(target)
    } else {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target")
    }
}