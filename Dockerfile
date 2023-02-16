FROM rust:1.67-bullseye as builder
RUN sed -i 's/deb.debian.org/mirrors.ustc.edu.cn/g' /etc/apt/sources.list && \
    sed -i 's|security.debian.org/debian-security|mirrors.ustc.edu.cn/debian-security|g' /etc/apt/sources.list
RUN apt-get update
RUN apt-get install -yq musl-tools musl
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /workspace
COPY . .
RUN RUSTFLAGS='-C link-arg=-s' cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3
LABEL org.opencontainers.image.authors="yinheli"
RUN mkdir /app
WORKDIR /app
COPY --from=builder /workspace/target/x86_64-unknown-linux-musl/release/ucloud-alert-notifier .
EXPOSE 8080/tcp
CMD ./ucloud-alert-notifier
