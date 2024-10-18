FROM randomdude/gcc-cross-x86_64-elf

RUN apt-get update 
RUN apt-get upgrade -y
RUN apt-get install -y nasm
RUN apt-get install -y xorriso
RUN apt-get install -y grub-pc-bin
RUN apt-get install -y grub-common
RUN apt-get install -y curl
RUN apt-get install -y cargo
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup target add x86_64-unknown-none
RUN rustup toolchain install nightly
RUN rustup default nightly


# Install Rust
# RUN curl https://sh.rustup.rs/ -sSf | sh -s -- -y
# ENV PATH="/root/.cargo/bin:${PATH}"

VOLUME /root/env
WORKDIR /root/env

# Command to execute when the container starts
#CMD ["make", "build-x86_64"]