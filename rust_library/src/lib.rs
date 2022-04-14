// extern crate cppsrc-rsbind;

// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
include!("../../cpp_src/build/cppsrc-rsbind.rs");

fn main() {
    println!("Hello, world!");
    unsafe { hello_from_cpp() };
}
