FROM rust:1.72.0-slim-buster as app-build

WORKDIR /build

RUN mkdir -p /build/backend/static

RUN apt update -y && apt install elfutils xz-utils wget pkg-config libssl-dev perl make -y && \
    wget https://github.com/upx/upx/releases/download/v4.0.2/upx-4.0.2-amd64_linux.tar.xz && \
    unxz upx-4.0.2-amd64_linux.tar.xz && tar xvf upx-4.0.2-amd64_linux.tar && \
    cp upx-4.0.2-amd64_linux/upx /usr/bin/upx && chmod +x /usr/bin/upx

RUN cargo install dioxus-cli --locked
RUN rustup target add wasm32-unknown-unknown

COPY . /build

RUN cd webui && \
    cargo test && \
    dx build --release && \
    cp -r dist/. ../backend/static && \
    ls -liah ../backend/static

RUN cd backend && \
    cargo test && \
    cargo build --release && \
    eu-elfcompress ../target/release/backend && \
    strip ../target/release/backend

RUN upx -9 --lzma target/release/backend && \
    chmod +x target/release/backend

FROM debian:bullseye-slim

WORKDIR /app

RUN apt update -y && apt install -y openssl libssl-dev && \
    useradd -d /app pw

RUN chmod 700 /app && \
    chown -R pw: /app

COPY --from=app-build /build/target/release/backend /app/pw

RUN chown -R pw: /app && chmod +x /app/pw

USER pw

CMD ["/app/pw"]


