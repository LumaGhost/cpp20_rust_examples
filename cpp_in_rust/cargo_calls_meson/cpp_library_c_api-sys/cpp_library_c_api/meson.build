
project('cpp_library_c_api', 'cpp',
    version         : '0.0.0')

cpplib = shared_library('cpp_library_c_api', 'src/lib.cpp',
    include_directories : include_directories('include'),
    override_options : ['cpp_std=c++20'],
    install : true)
