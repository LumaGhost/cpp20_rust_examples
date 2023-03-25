#include <iostream>
#include <type_traits>
#include <numbers>
#include <concepts>

#include <lib_c.hpp>

// toy concept to demonstrate usage of c++20 functionality
template<class T>
concept IntsOnly = std::is_same_v<T, int>;


template <IntsOnly T>
static void print_uwu() {
    std::cout << "uwu" << std::endl;
}

template <class T>
static void print_uwu_requires() requires IntsOnly<T> {
    std::cout << "uwu" << std::endl;
}

// c++20 usage with a c abi
extern "C" void hello_from_cpp() {
  print_uwu<int>();
  print_uwu_requires<int>();
  std::cout << std::numbers::pi << std::endl;
}