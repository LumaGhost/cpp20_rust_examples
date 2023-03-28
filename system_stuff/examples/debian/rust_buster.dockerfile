FROM rust:1.68-buster

RUN apt-get update && apt-get install -y python3 python3-pip clang-tools-11 libc++-11-dev libc++abi-11-dev

RUN pip3 install meson==1.0.1 ninja==1.10.2.3 conan==2.0.1

RUN curl -OL https://github.com/Kitware/CMake/releases/download/v3.25.3/cmake-3.25.3-SHA-256.txt && \
    curl -OL https://github.com/Kitware/CMake/releases/download/v3.25.3/cmake-3.25.3.tar.gz && \
    sha256sum -c --ignore-missing cmake-3.25.3-SHA-256.txt && \
    tar -xf cmake-3.25.3.tar.gz && \
    cd cmake-3.25.3 && \
    ./bootstrap -- -DCMAKE_BUILD_TYPE:STRING=Release && \
    make -j 4
