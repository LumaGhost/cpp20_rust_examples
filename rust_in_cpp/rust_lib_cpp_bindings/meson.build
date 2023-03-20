project('rust_lib_cpp_bindings', 'cpp',
    version         : '0.0.0')

pymod = import('python')
py = pymod.find_installation('python3')

cargo_manifest_path = meson.project_source_root() + '/rust_lib_c_abi/Cargo.toml'
cargo_target_dir = meson.project_build_root() + '/rust_lib_c_abi/'

if get_option('cargo_mainifest_path') != ''
    cargo_manifest_path = get_option('cargo_mainifest_path')
endif

if get_option('cargo_target_dir') != ''
    cargo_target_dir = get_option('cargo_target_dir')
endif

rust_lib = custom_target('rust_lib',
                        command: [py, '@SOURCE_ROOT@/cargo_build.py',
                                    '--cargo_manifest_path', cargo_manifest_path,
                                    '--cargo_target_dir', cargo_target_dir],
                        install: true,
                        install_dir: 'rust_lib_c_abi',
                        output: ['librust_lib_c_abi.so', 'rust_lib_c_abi.hpp' ])
