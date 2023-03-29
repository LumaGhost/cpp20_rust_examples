use std::io::Write;

fn run_subprocess(cmd: &mut std::process::Command, cmd_name: &str) {
    let out = cmd.output().expect(cmd_name);
    std::io::stdout().write_all(&out.stdout).unwrap();
    std::io::stderr().write_all(&out.stderr).unwrap();
    assert!(out.status.success());
}

fn prepare_ld_path(paths: &Vec<std::path::PathBuf>) -> String {
    let mut ld_path = String::new();
    for path in paths {
        ld_path.push_str(path.to_str().unwrap());
        if std::env::consts::OS == "windows" {
            ld_path.push(';');
        } else {
            ld_path.push(':');
        }
    }
    ld_path
}

fn main() {

    // conan profile name and install folder could be envs
    let install_folder_conan = std::env::current_dir().unwrap().join("conan_build");

    let mut conan_install = std::process::Command::new("conan");
    
    conan_install.args(["install", "-vtrace", "--build=missing", "-of", install_folder_conan.to_str().unwrap()]);

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
    
    let cpp_lib = pkg_config::Config::new().atleast_version("0.3").statik(false).probe("cpp_library").unwrap();

    println!("cargo:rerun-if-changed={}", install_folder_conan.to_str().unwrap());
    println!("cargo:rerun-if-changed={}", "build.rs");
    println!("cargo:rustc-env=LD_LIBRARY_PATH={}", prepare_ld_path(&cpp_lib.link_paths));
    println!("cargo:rustc-env=DYLD_LIBRARY_PATH={}", prepare_ld_path(&cpp_lib.link_paths));
}