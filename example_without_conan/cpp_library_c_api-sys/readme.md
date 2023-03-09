

trying to follow conventions for rust crates that link to native librarires https://doc.rust-lang.org/cargo/reference/build-scripts.html#-sys-packages

the more standard approach with [cc::Build](https://docs.rs/cc/latest/cc/struct.Build.html) isnt used here since we want access to a complete c++ toolchain, rather than just the systems c/c++ compiler. 

there are issues with this approach, such as difficulties with configuring meson (currently the user can optionally specify a meson native file as an env var when calling cargo. other meson arguments could be configured using this approach, but overall working with env vars in this way isnt user friendly imo).

cargo build -vv
cargo test -- --nocapture