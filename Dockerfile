FROM rust:1.36-buster

RUN apt-get -qq update; \
    apt-get install -qqy --no-install-recommends \
        autoconf automake cmake dpkg-dev file git make patch \
        libc-dev libc++-dev libgcc-7-dev libstdc++-7-dev  \
        dirmngr gnupg2 lbzip2 wget xz-utils libtinfo5; \
    rm -rf /var/lib/apt/lists/*

# Signing keys
ENV GPG_KEYS 09C4E7007CB2EFFB A2C794A986419D8A B4468DF4E95C63DC D23DD2C20DD88BA2 8F0871F202119294 0FC3042E345AD05D

# Retrieve keys
RUN gpg --batch --keyserver ha.pool.sks-keyservers.net --recv-keys $GPG_KEYS

# Version info
ENV LLVM_RELEASE 8
ENV LLVM_VERSION 8.0.0

# Install Clang and LLVM
COPY install.sh .
RUN ./install.sh

WORKDIR /usr/src/myapp
COPY . .

RUN cargo build
RUN cargo test
