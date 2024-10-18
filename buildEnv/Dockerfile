FROM debian:buster

# Install dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    nasm \
    xorriso \
    grub-pc-bin \
    grub-common \
    curl \
    make \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl https://sh.rustup.rs/ -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

VOLUME /root/env
WORKDIR /root/env

# Command to execute when the container starts
CMD ["make", "build-x86_64"]