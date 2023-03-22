import argparse
import glob
import os
import shutil
import subprocess


parser = argparse.ArgumentParser(
                    prog='ProgramName',
                    description='What the program does',
                    epilog='Text at the bottom of help')
parser.add_argument('--cargo_manifest_path')
parser.add_argument('--cargo_target_dir')

def copy(src_dir, dst_dir, pattern):
    for file in glob.glob(os.path.join(src_dir, pattern)):
        shutil.copy(file, dst_dir)


def main():
    args = parser.parse_args()
    os.environ['CARGO_TARGET_DIR'] = args.cargo_target_dir
    subprocess.run(['cargo', 'build', '-vv', '--release', '--manifest-path', args.cargo_manifest_path], check=True, env=os.environ)
    copy(os.path.join(args.cargo_target_dir,"release"), os.getcwd(), '*.so')
    copy(os.path.join(args.cargo_target_dir,"release"), os.getcwd(), '*.dll')
    subprocess.run(['ls'], check=True)

if __name__ == "__main__":
    main()