
#include <iostream>
#include <numbers>
#include <rust_lib_cpp_bindings.hpp>


int main() {
    ffi::meep();
    std::cout << "hello (:" << std::endl;
    std::cout << std::numbers::pi << std::endl;
}