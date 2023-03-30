
**overview**

in this example meson is used to build a rust library and the cpp library that it depends on.

**layout**

the layout here is meant to represent rust and cpp source living in the same project. the cpp source and rust source are in cpp_src and rust_src respectively.

**workflow**

this example is just a regular meson project, so messon commands e.g. `meson setup`, `meson compile`, `meson test` etc. will work.

**additional thoughts**

dependencies: this example doesnt use any additional dependencies on the rust or cpp side. this is due to both the difficulty of managing dependencies in cpp without a package manager, and the current lack of supports for external rust crates in meson.
