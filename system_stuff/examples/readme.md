**overview**

these configurations specific to our system/setup, they are only shared as an example of what your enviornment setup could look like (:

**additional details**

[debian:](debian/) 
i originally started this project on a chromebook, and i recreated that enviornment using docker so that i could use a desktop for the later stages of development (: if you prefer to use linux and/or docker this example will probably be helpful. if you dont want to use docker, you can still reference the dockerfile for an idea of what/how to install prerequisites.

[windows:](windows/)
this is the configuration we used on a windows desktop environment. i do not wish to provide a guide for setting up windows for development (please refer to each respective tools installation instructions for proper instruction) but the tldr for how we set up that environment is as follows: install chocolatey and use it to install pkg-config; install rust and the visual studio backend; install python3+pip and use pip to install cmake, meson, ninja, and conan; use visual studio to install clang-cl.