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

message(rust_lib.full_path())
message(rust_lib[0].full_path())
message(rust_lib[1].full_path())

rust_lib_dep = declare_dependency(include_directories: include_directories('include'),
                                  link_with: rust_lib[0])

tests = executable('cpp_binding_tests', meson.project_source_root() + '/test/rust_lib_cpp_bindings_tests.cpp', dependencies: rust_lib_dep)
test('binding tests', tests)
