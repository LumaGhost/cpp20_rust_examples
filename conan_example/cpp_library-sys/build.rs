use std::io::Write;

fn run_subprocess(cmd: &mut std::process::Command, cmd_name: &str) {
    let out = cmd.output().expect(cmd_name);
    std::io::stdout().write_all(&out.stdout).unwrap();
    std::io::stderr().write_all(&out.stderr).unwrap();
    assert!(out.status.success());
}

fn main() {

    // conan profile name and install folder could be envs
    let install_folder_conan = std::env::current_dir().unwrap().join("conan_build");

    let mut conan_install = std::process::Command::new("conan");
    
    conan_install.args(["install", "--build=missing", "-of", install_folder_conan.to_str().unwrap()]);

    if let Some(conan_profile) = std::env::var_os("CARGO_CONAN_PROFILE").map(|p|std::path::PathBuf::from(p)) {
        conan_install.args(["-pr:h", conan_profile.to_str().unwrap(), "-pr:b", conan_profile.to_str().unwrap()]);
    }
    conan_install.arg(".");
    run_subprocess(&mut conan_install, "conan install");

    // appending to pkgconfig search path
    if let Some(current_pgk_config_path) = std::env::var_os("PKG_CONFIG_PATH") {
        std::env::set_var("PKG_CONFIG_PATH", format!("{}:{}", current_pgk_config_path.to_str().unwrap(), install_folder_conan.to_str().unwrap()))
    } else {
        std::env::set_var("PKG_CONFIG_PATH", format!("{}",install_folder_conan.to_str().unwrap()))
    }
    println!("PKG_CONFIG_PATH:{}", std::env::var_os("PKG_CONFIG_PATH").unwrap().to_str().unwrap());
    
    let cpp_lib = pkg_config::Config::new().atleast_version("0.1").statik(false).probe("cpp_library").unwrap();
    println!("link paths: {:?}", cpp_lib.link_paths);
    // will fail if it cant find the lib (:
    // pkg_config::Config::new().probe("asdfasdfasdfsdfasdf").unwrap();
    // also dosent work if u dont set the PKG_CONFIG_PATH to contain the dir where conan is putting the .pc files (:

    println!("cargo:rerun-if-changed={}", install_folder_conan.to_str().unwrap());
    println!("cargo:rerun-if-changed={}", "build.rs");
    // looks like the rpath is already set correctly? thanks to the .pc files or the pkgconfig crate? idk lol
    // println!("cargo:rustc-env=LD_LIBRARY_PATH={}", cpp_lib_dir.to_str().unwrap())
    
}