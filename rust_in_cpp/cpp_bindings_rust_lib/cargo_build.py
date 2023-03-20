import argparse
import os
import shutil
import subprocess


parser = argparse.ArgumentParser(
                    prog='ProgramName',
                    description='What the program does',
                    epilog='Text at the bottom of help')
parser.add_argument('--cargo_manifest_path')

def main():
    cargo_target_dir = "/workspaces/cpp20_rust/rust_in_cpp/rust_lib/rust_target"
    args = parser.parse_args()
    subprocess.run(['cargo', 'build', '-vv', '--release', '--manifest-path', args.cargo_manifest_path], check=True)
    shutil.copyfile(os.path.join(cargo_target_dir,"release","librust_src.so"), os.path.join(os.getcwd(), "librust_src.so"))
    shutil.copyfile(os.path.join(cargo_target_dir,"rust_src.hpp"), os.path.join(os.getcwd(), "rust_src.hpp"))

if __name__ == "__main__":
    main()