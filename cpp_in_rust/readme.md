**overview**

the following approaches for importing c++ code into rust are presented. each approach focuses on an example c++ library with a c api, and making that library available in rust code.

the purpose of these descriptions is to provide an overview of each approach and tradeoffs to help the reader figure out which approach may be more interesting/relevant. for more details on the each implementation and approach see their respective subfolders.

[cargo calls conan](cargo_calls_conan): conan is used to manage c++ dependencies, and cargo invokes conan in order to fetch and locate c++ dependencies. functionality is contained behind cargos interface and the project mostly functions as a typical rust project besides needing to manage a conanfile.txt. i would recommend this approach as the default. this approach should generalze to any build system thats supported by conan.

[cargo calls meson](cargo_calls_meson): the c++ library is compiled with meson (the c++ code in this case doesnt have dependencies. managing c++ dependencies without conan is left as an exercise to the reader :p ), and cargo invokes meson. i would recommend this approach if conan isnt an option or if there are other plans for managing c++ dependencies. this approach should generalize to other build tools with a command line interface such as cmake.

[meson builds cpp and rust](meson_builds_cpp_and_rust): both the rust library and the c++ library it depends on are compiled with meson. again the c++ library doesnt have any dependencies. the rust side also doesnt have any dependencies either since, as far as i know, external rust crates arent supported in meson yet. the lack of dependency support may be a deal breaker for most projects, but this example still has the benefit of containing everyhing within one build system (i also personally think its very cool that one build system can compile both languages and something besides cargo can be used for rust projects).


**additional thoughts**

for cargo based projects i tried to follow the following conventions for rust crates that wrap native libraries https://doc.rust-lang.org/cargo/reference/build-scripts.html#-sys-packages

