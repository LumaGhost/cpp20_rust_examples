project('rust_calls_cpp', 'rust', 'cpp',
    version         : '0.0.0')

cpplib = shared_library('cppsrc', 'cpp_src/src/lib.cpp',
    include_directories : ['cpp_src/include'],
    override_options : ['cpp_std=c++2a'])

executable('hello_cpp_rust', 'rust_src/main.rs', link_with : cpplib)
