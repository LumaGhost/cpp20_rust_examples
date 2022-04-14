from posix import environ
import subprocess, os

if __name__ == "__main__":

    meson_dir = os.path.join(os.getcwd(), "cpp_library")
    build_dir = os.path.join(meson_dir, "build")

    subprocess.run(["meson", "setup", "--native-file", os.path.join(os.getcwd(), "system_stuff", "rust-clang-native.ini"), build_dir], 
                    cwd=meson_dir, check=True)

    subprocess.run(["meson", "compile", "-C", build_dir], 
                    cwd=meson_dir, check=True)

    subprocess.run(["meson", "install", "-C", build_dir, "--only-changed", "--dry-run"], 
                    cwd=meson_dir, check=True)
    
    cargo_env = os.environ.copy()
    cargo_env["CARGO_USER_CPP_LIB_DIR"] = os.path.join(os.getcwd(), "cpp_library", "build")
    print(cargo_env["CARGO_USER_CPP_LIB_DIR"])
    cargo_env["CARGO_USER_CPP_LIB_NAME"] = "cppsrc"
    print(cargo_env["CARGO_USER_CPP_LIB_NAME"])

    cargo_cmd = ["cargo"]
    cargo_cmd.append("build")

    subprocess.run(cargo_cmd, cwd=os.path.join(os.getcwd(), "rust_library"), check=True, env=cargo_env)