## Credit to Andrew Scorpils
## https://github.com/Scorpil/docker-rust/blob/master/1.5/Dockerfile
##
## We needed to build this on Ubuntu instead of Debian

## This is changed ##
FROM ubuntu:16.04
#####################

MAINTAINER Andrew Scorpil "dev@scorpil.com"

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install \
       ca-certificates \
       curl \
       gcc \
       libc6-dev \
       -qqy \
       --no-install-recommends \
    && rm -rf /var/lib/apt/lists/*

ENV RUST_ARCHIVE=rust-1.14.0-x86_64-unknown-linux-gnu.tar.gz
ENV RUST_DOWNLOAD_URL=https://static.rust-lang.org/dist/$RUST_ARCHIVE

RUN curl -fOSL $RUST_DOWNLOAD_URL
RUN curl $RUST_DOWNLOAD_URL.sha256 | sha256sum -c -

RUN mkdir /rust
RUN tar -C /rust -xzf $RUST_ARCHIVE --strip-components=1
RUN rm $RUST_ARCHIVE
WORKDIR /rust
RUN ./install.sh

WORKDIR /

### Our custom stuff ###
RUN apt-get update && \
    apt-get install \
       pkg-config \
       libczmq-dev \
       -qqy \
    && rm -rf /var/lib/apt/lists/*
RUN cargo install rustfmt
ENV PATH="/root/.cargo/bin:${PATH}"
