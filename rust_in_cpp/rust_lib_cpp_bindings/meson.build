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


dylib_extension = '.so'
if host_machine.system() == 'windows'
    dylib_extension = '.dll'
endif 

install_dir = 'rust_lib_c_abi'

rust_lib = custom_target('rust_lib',
                        command: [py, '@SOURCE_ROOT@/cargo_build.py',
                                    '--cargo_manifest_path', cargo_manifest_path,
                                    '--cargo_target_dir', cargo_target_dir],
                        install: true,
                        install_dir: install_dir + '/lib',
                        output: ['librust_lib_c_abi' + dylib_extension])

inc = include_directories('include')

install_headers('include/rust_lib_cpp_bindings.hpp',
                install_dir: install_dir + '/include/rust_lib_c_abi')

rust_lib_cpp_bindings = declare_dependency(include_directories: inc,
                                           link_with: rust_lib[0])

subdir('test')
