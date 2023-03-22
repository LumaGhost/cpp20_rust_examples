

this example demonstrates using meson to compile rust so that rust and c++ code can live in the same meson build. this approach is potentially a lot simpler since there is only one build to manage, but, at the time of writing this, building rust with meson has the downside of not supporting external crates