

use std::io::Write;


fn run_subprocess(cmd: &mut std::process::Command, cmd_name: &str) {
    let out = cmd.output().expect(cmd_name);
    std::io::stdout().write_all(&out.stdout).unwrap();
    std::io::stderr().write_all(&out.stderr).unwrap();
    assert!(out.status.success());
}


fn main() {


    // should potentially be env vars. at least the native file
    let native_file = std::env::current_dir().unwrap().join("..").join("..").join("system_stuff").join("rust-clang-native.ini");
    let meson_cwd =  std::env::current_dir().unwrap().join("..").join("cpp_library");
    let build_dir = meson_cwd.join("build");

    // meson setup
    run_subprocess(std::process::Command::new("meson").current_dir(meson_cwd.to_str().unwrap())
        .args(["setup", "--native-file", native_file.to_str().unwrap(), build_dir.to_str().unwrap()]),
        "meson setup");

    // meson compile
    run_subprocess(std::process::Command::new("meson").current_dir(meson_cwd.to_str().unwrap())
        .args(["compile", "-C", build_dir.to_str().unwrap()]),
        "meson compile");

    // meson install?

    let cpp_lib_dir = &build_dir;
    // let cpp_lib_dir = std::path::Path::new("/workspaces/cpp20_rust/example_without_conan/cpp_library/build");
    let cpp_lib_name = "cppsrc";
    println!("cpp lib dir: {}", cpp_lib_dir.to_str().unwrap());
    println!("exists: {}", cpp_lib_dir.exists());

    println!("cargo:rerun-if-changed={}", cpp_lib_dir.to_str().unwrap());
    println!("cargo:rerun-if-changed={}", "build.rs");
    println!("cargo:rustc-link-search=native={}", cpp_lib_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib={}", cpp_lib_name);
    // ideally we'd set rpath but... https://github.com/rust-lang/cargo/issues/5077
    println!("cargo:rustc-env=LD_LIBRARY_PATH={}", cpp_lib_dir.to_str().unwrap())
    
}