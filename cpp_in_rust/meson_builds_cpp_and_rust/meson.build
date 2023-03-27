project('rust_calls_cpp', 'rust', 'cpp',
    version         : '0.0.0')

cpplib = shared_library('cpplib', 'cpp_src/src/lib.cpp',
    include_directories : ['cpp_src/include'],
    override_options : ['cpp_std=c++20'],
    install: true)

rustlib = shared_library('rustlib', 'rust_src/lib.rs', link_with: cpplib, install: true)

rust = import('unstable-rust')
rust.test('hello_cpp_rust', rustlib)
