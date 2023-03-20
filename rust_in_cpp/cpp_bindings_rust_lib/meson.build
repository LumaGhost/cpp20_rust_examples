project('cpp_bindings_rust_lib', 'cpp',
    version         : '0.0.0')

'''
run_command('cargo', 'build', '-vv', '--manifest-path', get_option('cargo_mainifest_path'), 
            env: {'CARGO_TARGET_DIR': get_option('cargo_target_dir')}, check: true)

cxx_cc = meson.get_compiler('cpp')
rust_dylib = cxx_cc.find_library('cppsrc',
    dirs : get_option('cargo_mainifest_path') + '/release/',
    static : false)
'''

pymod = import('python')
py = pymod.find_installation('python3')

rust_lib = custom_target('rust_lib',
                        command: [py, '/workspaces/cpp20_rust/rust_in_cpp/cpp_bindings_rust_lib/cargo_build.py',
                                    '--cargo_manifest_path', get_option('cargo_mainifest_path'),
                                    '--cargo_target_dir', get_option('cargo_target_dir')],
                        install: true,
                        install_dir: 'rust_lib',
                        output: ['librust_src.so', 'rust_src.hpp' ])

'''
install_headers('include/rust_lib_bindings.hpp')

cpplib = shared_library('cpp_library_c_api', 'src/lib.cpp',
    include_directories : include_directories('include'),
    override_options : ['cpp_std=c++2a'],
    install : true)
'''
