

**introduction**

hello! this project is a collection of examples on importing modern c++ code into rust projects and rust code into c++ projects. 

the main motivation of this project is to explore options that existing c++ codebases could use to introduce rust gradually or use rust as a small part of an existing c++ project, and to provide more in depth/advanced examples for using modern build tools like meson and cargo in a cross platform scenario.

a key difference between this project and other examples i've seen on rust and c++ interop is that this project focuses on c++ projects that use modern toolchains and build systems (cmake, meson, cargo, newer c++ compilers, etc). 

**overview**

[cpp_in_rust](cpp_in_rust/)
a collection of examples for importing c++20 code into rust. currently there are 3 examples: using cargo to import c++ code package with conan, using cargo to import a c++ project built with meson, and using meson to build both a rust library and its c++ library depencency. for more details see the [readme](cpp_in_rust/readme.md) for that section of the project.

[rust_in_cpp](rust_in_cpp/)
examples for importing rust into c++ projects. currently there is just one example where meson is used to invoke cargo. however, this example could generalize in a straightforward way to use conan or cmake. for more details see the [readme](rust_in_cpp/readme.md) for that section of the project.

[system_stuff](system_stuff/)
this folder includes environment spefific config used for CI. we also include our local development setups as examples. these configurations should not be required for working with this project, though feel free to reference them when setting up your environment. see [setup](#setup_anchor) for more details.


<a name="setup_anchor"></a>

**setup**

prerequisites: rust, cmake, meson, ninja, conan >2.0, pkg-config, a c++ compiler new enough to have support for concepts and std::numbers.

each workflow is tested on windows, linux, and mac.

for more details on prerequisites feel free to check out our [setup examples](system_stuff/examples) or [CI configuration](system_stuff/ci), and for workflow/build commands refer to individual subprojects (or our github actions).


**additional thoughts**

meson is used throughout as the preferred c++ build system because i wanted to get more experience with meson. however all of these examples should still generalize for other c++ build systems especially cmake. feel free to experiment with other build systems (: 

where there is a choice between static or dynamic/shared libraries i chose to use shared libraries. i realise some people may need one or the other (or both) depending on use cases. i chose to use shared libraries througout mainly because it seemed more challenging to me, but these approaches should still work for static libraries. 

i tried to make these examples relatively robust and representative of my experience with production code, but still keep in mind these are just examples at the end of the day (:

