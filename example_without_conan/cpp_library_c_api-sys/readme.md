

trying to follow conventions for rust crates that link to native librarires https://doc.rust-lang.org/cargo/reference/build-scripts.html#-sys-packages

the more standard approach using the [cc crate](https://docs.rs/crate/cc/latest) isnt used here since we want access to a complete c++ toolchain and build system, rather than just the c/c++ compiler directly. we could use the CXX and CXXFLAGS enviornment variables to configure the cc api sufficeintly for this example, but that still wouldnt be a substitute for a c++ build system especially for larger more complex existing c++ projects. note: this example is attempting to be representative of importing an existing c++ project into rust.

there are issues with this approach, such as difficulties with configuring meson/the underlying c++ build system (currently the user can optionally specify a meson native file as an env var when calling cargo. other meson arguments could be configured using this approach, but overall working with env vars in this way isnt user friendly imo), and a category of issues involving potential dependency conflicts (for example two different -sys crates that depend on different versions of some common c++ library). the approach using conan attempts to address some of these issues. 

cargo build -vv
cargo test -- --nocapture