
#pragma once

#ifdef _WIN32
  #define CPP_LIBRARY_EXPORT __declspec(dllexport)
#else
  #define CPP_LIBRARY_EXPORT
#endif

CPP_LIBRARY_EXPORT extern "C" void hello_from_cpp();