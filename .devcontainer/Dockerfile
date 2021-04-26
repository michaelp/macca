# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.166.1/containers/rust/.devcontainer/base.Dockerfile

FROM mcr.microsoft.com/vscode/devcontainers/rust:0-1
RUN curl -s https://packagecloud.io/install/repositories/wasmcloud/core/script.deb.sh | sudo bash
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install build-essential cmake cppcheck valgrind clang lldb llvm gdb wasmcloud wash\
    && cargo install cargo-generate\
    && rustup target add wasm32-unknown-unknown\
    && apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/*

# [Optional] Uncomment this section to install additional packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>
