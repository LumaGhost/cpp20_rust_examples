FROM rust:1.60-buster

# apt list --installed. any marked [installed] i think were explicitly installed
RUN apt-get update && apt-get install -y python3 python3-pip clang-tools-11 libc++-11-dev libc++abi-11-dev

RUN pip3 install meson ninja conan

RUN cargo install bindgen --version 0.59.2 --root /home/from_docker_build/cargo_installs/


# btw conan remotes and profiles shouldnt be image specific, they can change a bit more often than the environment. esp the remotes
# jk i agree with remotes. but at least default profiles are part of the environemnt. 

RUN conan profile new clang-sys --detect && \
    conan profile update settings.compiler.libcxx=libc++ clang-sys && \
    conan profile update settings.compiler=clang clang-sys && \
    conan profile update settings.compiler.version=11 clang-sys && \
    conan profile update env.CC=/usr/bin/clang-11 clang-sys && \
    conan profile update env.CXX=/usr/bin/clang++-11 clang-sys
    
# RUN apt-get install -y gdb

# ENV CONAN_SYSREQUIRES_MODE=enabled