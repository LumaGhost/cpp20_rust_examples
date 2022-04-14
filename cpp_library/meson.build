
project('rust_calls_cpp', 'cpp',
    version         : '0.0.0')

cpplib = shared_library('cppsrc', 'src/lib.cpp',
    include_directories : include_directories('include'),
    override_options : ['cpp_std=c++2a'],
    install : true)
