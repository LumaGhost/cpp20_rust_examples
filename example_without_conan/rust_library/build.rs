
fn main() {


    let cpp_lib_dir = std::env::var_os("CARGO_USER_CPP_LIB_DIR").map(|p|std::path::PathBuf::from(p)).unwrap();
    let cpp_lib_name = std::env::var_os("CARGO_USER_CPP_LIB_NAME").map(|p|std::path::PathBuf::from(p)).unwrap();
    println!("cpp lib dir: {}", cpp_lib_dir.to_str().unwrap());
    println!("exists: {}", cpp_lib_dir.exists());

    println!("cargo:rerun-if-changed={}", cpp_lib_dir.to_str().unwrap());
    println!("cargo:rerun-if-changed={}", "build.rs");
    println!("cargo:rustc-link-search=native={}", cpp_lib_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib={}", cpp_lib_name.to_str().unwrap());
    // ideally we'd set rpath but... https://github.com/rust-lang/cargo/issues/5077
    println!("cargo:rustc-env=LD_LIBRARY_PATH={}", cpp_lib_dir.to_str().unwrap())
    
}