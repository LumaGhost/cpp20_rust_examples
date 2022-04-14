
use std::process::Command;
use std::io::{self, Write};
use std::env;

struct MesonCommandBuilder {
    // should include meson. we wont assume the name of the meson binary. "meson" by default
    meson_path: std::path::PathBuf,
    // directory that contains the meson.build you want to work with. . by default
    project_dir: std::path::PathBuf,
}

struct MesonCommand {
    meson_path: std::path::PathBuf,
    project_dir: std::path::PathBuf,
}


impl MesonCommandBuilder {
    fn new() -> Self {
        MesonCommandBuilder{meson_path: std::path::PathBuf::from("meson"),
                        project_dir: std::path::PathBuf::from(".")}
    }

    fn project_dir<'a>(&'a mut self, dir: std::path::PathBuf) -> &'a mut Self {
        self.project_dir = dir;
        self
    }

    fn meson_path<'a>(&'a mut self, path: std::path::PathBuf) -> &'a mut Self {
        self.meson_path = path;
        self
    }

    fn build(& self) -> MesonCommand {
        MesonCommand{meson_path: self.meson_path.clone(),
                    project_dir: self.project_dir.clone()}
    }
}

impl MesonCommand {

    fn setup<I, S>(&self, build_dir: &str, meson_args: I) -> Command 
    where
        I: IntoIterator<Item=S>,
        S: AsRef<std::ffi::OsStr> {
        let mut cmd = Command::new(self.meson_path.clone());
        cmd.current_dir(self.project_dir.clone())
            .arg("setup")
            .args(meson_args)
            .arg(build_dir);
        cmd
    }

    fn configure<I, S>(&self, build_dir: &str, meson_args: I) -> Command 
    where
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>, {
        let mut cmd = Command::new(self.meson_path.clone());
        cmd.current_dir(self.project_dir.clone())
            .arg("configure")
            .args(meson_args)
            .arg(build_dir);
        cmd
    }

    fn compile<I, S>(&self, build_dir: &str, meson_args: I) -> Command 
    where
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>, {
        let mut cmd = Command::new(self.meson_path.clone());
        cmd.current_dir(self.project_dir.clone())
            .arg("compile")
            .arg("-C")
            .arg(build_dir)
            .args(meson_args);
        cmd
    }

    fn install<I, S>(&self, build_dir: &str, meson_args: I) -> Command 
    where
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>, {
        let mut cmd = Command::new(self.meson_path.clone());
        cmd.current_dir(self.project_dir.clone())
            .arg("install")
            .arg("-C")
            .arg(build_dir)
            .args(meson_args);
        cmd
    }
}


fn main() {
    // let cpp_proj_dir = std::env::current_dir().unwrap().join("..").join("cpp_library");
    let cpp_proj_dir = std::path::PathBuf::from("../cpp_library");

    let cpp_build_dir = env::var_os("CARGO_CPP_BUILD_DIR").map(|p|std::path::PathBuf::from(p)).unwrap();
    println!("cpp build dir: {}", cpp_build_dir.to_str().unwrap());
    println!("exists: {}", cpp_build_dir.exists());
    let meson_path = env::var_os("CARGO_MESON_PATH").unwrap();
    let meson_setup_args = env::var_os("CARGO_MESON_SETUP_ARGS").map_or(Vec::new(), |s| Vec::from([s]));
    let meson_compile_args = env::var_os("CARGO_MESON_COMPILE_ARGS").map_or(Vec::new(), |s| Vec::from([s]));
    let meson_install_args = env::var_os("CARGO_MESON_INSTALL_ARGS").map_or(Vec::new(), |s| Vec::from([s]));
    let meson_configure_args = env::var_os("CARGO_MESON_CONFIGURE_ARGS").map_or(Vec::new(), |s| Vec::from([s]));

    println!("cargo:rerun-if-changed={}", cpp_proj_dir.to_str().unwrap());
    println!("cargo:rerun-if-changed={}", "build.rs");
    println!("cargo:rustc-link-search=native={}", cpp_build_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib=cppsrc");
    // ideally we'd set rpath but... https://github.com/rust-lang/cargo/issues/5077
    println!("cargo:rustc-env=LD_LIBRARY_PATH={}", cpp_build_dir.to_str().unwrap());

    let meson = MesonCommandBuilder::new()
        .project_dir(cpp_proj_dir.clone())
        .meson_path(std::path::PathBuf::from(meson_path))
        .build();

    if !cpp_build_dir.exists() {
        println!("meson setup");
        let setup_out = meson.setup(cpp_build_dir.to_str().unwrap(), &meson_setup_args)
            .output().expect("meson setup failed");
        io::stdout().write_all(&setup_out.stdout).unwrap();
        io::stderr().write_all(&setup_out.stderr).unwrap();
        assert!(setup_out.status.success(), "setup failed");
    }

    if !meson_configure_args.is_empty() {
        let config_out = meson.configure(cpp_build_dir.to_str().unwrap(), &meson_configure_args)
                .output().expect("meson configure failed");
        io::stdout().write_all(&config_out.stdout).unwrap();
        io::stderr().write_all(&config_out.stderr).unwrap();
        assert!(config_out.status.success());
    }

    let compile_out = meson.compile(cpp_build_dir.to_str().unwrap(), &meson_compile_args)
            .output().expect("meson compile failed");
    io::stdout().write_all(&compile_out.stdout).unwrap();
    io::stderr().write_all(&compile_out.stderr).unwrap();
    assert!(compile_out.status.success());
        
    
    let output = meson.install(cpp_build_dir.to_str().unwrap(), &meson_install_args)
            .output().expect("meson install failed");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    assert!(output.status.success());

   // assert!(false);
    
}