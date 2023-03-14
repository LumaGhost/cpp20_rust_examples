#pragma once


#ifdef _WIN32
  #define CPP_LIBRARY_EXPORT __declspec(dllexport)
#else
  #define CPP_LIBRARY_EXPORT
#endif

CPP_LIBRARY_EXPORT void cpp_library();
