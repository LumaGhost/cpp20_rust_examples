**overview**

in this example we have a cpp library that depends on a rust library, and the rust library is imported into meson using the `custom_target` function. the rust library has one dependency, and the cpp side has no other dependencies.

**layout**

[rust_lib_cpp_bindings](rust_lib_cpp_bindings) this folder represents a cpp library that provides bindings for a rust library. besides the dependency on the rust library this cpp library is header only

[rust_lib_c_abi](rust_lib_cpp_bindings/rust_lib_c_abi) this folder represents a cargo library with a c abi. this library can function as a regular cargo crate. however, for the purpose of importing this rust code into cpp i have the rust library living in source as a sub project of the cpp library. as far as i know cargo doesnt support installing library targets with cargo (i.e. we cant simply list the dependencies we want and install them like we did in the [cargo calls conan example](../../cpp_in_rust/cargo_calls_conan)), so we are responsible for fetching, building, and locating the rust library which to me is most simply accomplished with the rust code living in source as a subproject.

**additional thoughts**

caveats: i think this approach should be used with additional caution. i believe there is good reason for cargo not supporting the installation of library targets. however in this approach we hook cargo into meson to the point where meson can treat the resulting rust binary the same way it treats any cpp library (including support for installation). in other words we are sortof working around cargos rules and potentially assumptions that cargo makes. for these reasons i would recommend using caution with this approach especially if your project has other rust dependencies.

cargo_build.py: the purpose of this script may not be obvious, but the short answer is that we needed a small wrapper around cargo in order for it to behave in a way that worked nicely with the custom_target function in meson. 

binding generation: i chose not to use auto generated bindings in the final example (generating bindings during the build can add nuance in practice especially in terms of stability, testing, and documentation), but i still left in the code for generating c++ bindings for the rust library (can be found in the build.rs file [here](rust_lib_cpp_bindings/rust_lib_c_abi/build.rs)). the binding generation code was taken from here https://github.com/Michael-F-Bryan/rust-ffi-guide/blob/master/book/cbindgen.md#adding-cbindgen which is also a great source for learning about rust and c/c++ interop (:
