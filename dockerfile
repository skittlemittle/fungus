ARG DEB_VERSION="11.7"

FROM debian:${DEB_VERSION}

RUN dpkg --add-architecture armhf

# Add cross compilers and dependencies
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install --yes \
    curl git crossbuild-essential-armhf pkg-config \
    libasound2-dev:armhf \
    && apt-get clean && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/

# Install Rust system-wide
ENV RUSTUP_HOME=/opt/rust
ENV CARGO_HOME=/opt/rust
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path
ENV PATH="/opt/rust/bin:${PATH}"

RUN rustup target add armv7-unknown-linux-gnueabihf
ENV CARGO_BUILD_TARGET=armv7-unknown-linux-gnueabihf

# Configure Rust linkers for cross compilation
ENV CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=/usr/bin/arm-linux-gnueabihf-gcc

# Configure pkg-config for cross compilation
ENV PKG_CONFIG_ALLOW_CROSS=1/
ENV PKG_CONFIG_PATH_armv7_unknown_linux_gnueabihf=/usr/lib/arm-linux-gnueabihf/pkgconfig/

RUN chmod a+rw /opt/rust

# Set the working directory to /src, where we will mount our project
WORKDIR /src
