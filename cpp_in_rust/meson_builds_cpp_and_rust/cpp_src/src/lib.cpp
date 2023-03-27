#include <iostream>
#include <type_traits>
#include <numbers>
#include <concepts>

#include <lib_c.h>

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

extern "C" void hello_from_cpp() {
  print_uwu<int>();
  print_uwu_requires<int>();
  std::cout << std::numbers::pi << std::endl;
}