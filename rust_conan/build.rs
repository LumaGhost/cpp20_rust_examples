use std::io::Write;

fn main() {

    // conan profile name and install folder could be envs
    let install_folder_conan = std::env::current_dir().unwrap().join("conan_build");

    let conan_install = 
        std::process::Command::new("conan")
            .args(["install", "-if", install_folder_conan.to_str().unwrap(), "-pr", "clang-sys", "."])
            .output().expect("conan install failed");
    std::io::stdout().write_all(&conan_install.stdout).unwrap();
    std::io::stderr().write_all(&conan_install.stderr).unwrap();
    assert!(conan_install.status.success());

    // is there a like... match let? im used to if let but cant in rust because option doesnt convert to bool.
    let current_pgk_config_path = std::env::var_os("PKG_CONFIG_PATH");
    match current_pgk_config_path {
        Some(p) => std::env::set_var("PKG_CONFIG_PATH", format!("{}:{}", p.to_str().unwrap(), install_folder_conan.to_str().unwrap())),
        None => std::env::set_var("PKG_CONFIG_PATH", format!("{}",install_folder_conan.to_str().unwrap())),
    }
    println!("PKG_CONFIG_PATH:{}", std::env::var_os("PKG_CONFIG_PATH").unwrap().to_str().unwrap());
    
    pkg_config::Config::new().probe("fmt").unwrap();
    pkg_config::Config::new().probe("zlib").unwrap();
    // will fail if it cant find the lib (:
    // pkg_config::Config::new().probe("asdfasdfasdfsdfasdf").unwrap();
    // also dosent work if u dont set the PKG_CONFIG_PATH to contain the dir where conan is putting the .pc files (:

    println!("cargo:rerun-if-changed={}", install_folder_conan.to_str().unwrap());
    println!("cargo:rerun-if-changed={}", "build.rs");
    // looks like the rpath is already set correctly? thanks to the .pc files or the pkgconfig crate? idk lol
    //println!("cargo:rustc-env=LD_LIBRARY_PATH={}", cpp_lib_dir.to_str().unwrap())
    
}