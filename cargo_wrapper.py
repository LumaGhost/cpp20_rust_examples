from posix import environ
import subprocess, os

if __name__ == "__main__":

    cmd_env = os.environ.copy()
    cmd_env["CARGO_CPP_BUILD_DIR"] = os.path.join(os.getcwd(), "cpp_library", "build")
    print(cmd_env["CARGO_CPP_BUILD_DIR"])
    cmd_env["CARGO_MESON_PATH"] = "meson"
    cmd_env["CARGO_MESON_SETUP_ARGS"] = "".join(["--native-file ", os.path.join(os.getcwd(), "system_stuff", "rust-clang-native.ini")])
    print(cmd_env["CARGO_MESON_SETUP_ARGS"])
    # if ur not passing args just dont set them
    #current_env["CARGO_MESON_COMPILE_ARGS"] = ""
    cmd_env["CARGO_MESON_INSTALL_ARGS"] = "--only-changed --dry-run"
    #current_env["CARGO_MESON_CONFIGURE_ARGS"] = ""
    cargo_cmd = ["cargo"]
    cargo_cmd.append("build")
    subprocess.run(cargo_cmd, cwd=os.path.join(os.getcwd(), "rust_library"), check=True, env=cmd_env)