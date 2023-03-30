
**overview**

for importing rust into c++ code currently only one approach is presented: [meson_calls_cargo](meson_calls_cargo). i felt one approach was sufficient since a meson project can be packaged with conan, and what can be done with meson can also generally be done with cmake or other modern build systems. for building rust and c++ together with meson see [meson_builds_cpp_and_rust](../cpp_in_rust/meson_builds_cpp_and_rust).