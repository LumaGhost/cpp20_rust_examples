project('cpp_bindings_rust_lib', 'cpp',
    version         : '0.0.0')

pymod = import('python')
py = pymod.find_installation('python3')

cargo_manifest_path = meson.project_source_root() + '/../rust_lib/Cargo.toml'
cargo_target_dir = meson.project_build_root() + '/rust_lib/'

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
                        install_dir: 'rust_lib',
                        output: ['librust_src.so', 'rust_src.hpp' ])
