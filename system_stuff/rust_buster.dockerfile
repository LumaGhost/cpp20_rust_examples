FROM rust:1.60-buster

# apt list --installed. any marked [installed] i think were explicitly installed
RUN apt-get update && apt-get install -y python3 python3-pip clang-tools-11 libc++-11-dev libc++abi-11-dev

RUN pip3 install meson ninja

RUN cargo install bindgen --version 0.59.2 --root /home/from_docker_build/cargo_installs/


# btw conan remotes and profiles shouldnt be image specific, they can change a bit more often than the environment. esp the remotes

# RUN conan remote update bincrafters https://bincrafters.jfrog.io/artifactory/api/conan/public-conan && \
#     conan remote remove bintray && \
#     conan config set general.revisions_enabled=1

# RUN conan profile new clang --detect && \
#     conan profile update settings.compiler.libcxx=libstdc++11 clang && \
#     conan profile update settings.compiler=clang clang && \
#     conan profile update settings.compiler.version=13 clang && \
#     conan profile update env.CC=/llvm-project/build/bin/clang clang && \
#     conan profile update env.CXX=/llvm-project/build/bin/clang++ clang
    
# RUN apt-get install -y gdb

# ENV CONAN_SYSREQUIRES_MODE=enabled