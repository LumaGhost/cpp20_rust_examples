
project('rust_calls_cpp', 'cpp', 'rust',
    version         : '0.0.0')

cpplib = shared_library('cppsrc', 'src/lib.cpp',
    include_directories : include_directories('include'),
    override_options : ['cpp_std=c++2a'],
    install : true)

rust = import('unstable-rust')

generated = rust.bindgen(
    input : 'include/lib_c.hpp',
    output : 'cppsrc-rsbind.rs',
    include_directories : include_directories('include'),
    args : ['--no-rustfmt-bindings'],
)

# shared_library('cppsrc-rsbind', generated, link_with : [cpplib], install : true)

# fails clean setup due to the generated file not existing yet. seems like a bug?
#  or im not using the right tool or something. they say just generated[0] should work
#  and file a bug otherwise https://mesonbuild.com/Reference-manual_returned_custom_tgt.html#custom_tgtfull_path
#  so that makes me feel more like its a bug but idk.
#  yea if u add this line after building once it works lol
#install_data(generated[0].full_path(), 
#    install_dir : get_option('generated_rust_install_dir') / 'cppsrc-rsbind/include-rs')
