


no conan. requires the cpp/c code to live in the same place as the rust bindings (or update the build rs to fetch it). you could write a py script to do everything and call it from build.rs, or just write the meson compile commands in build rs