FROM rust:1.72.0-alpine3.18 as app-build

WORKDIR /build

RUN mkdir -p /build/backend/static

RUN apk add --no-cache musl-dev elfutils xz wget pkgconfig libressl-dev perl make && \
    wget https://github.com/upx/upx/releases/download/v4.0.2/upx-4.0.2-amd64_linux.tar.xz && \
    unxz upx-4.0.2-amd64_linux.tar.xz && tar xvf upx-4.0.2-amd64_linux.tar && \
    cp upx-4.0.2-amd64_linux/upx /usr/bin/upx && chmod +x /usr/bin/upx && \
    cargo install dioxus-cli --locked && \
    rustup target add wasm32-unknown-unknown

COPY . /build

RUN cd webui && \
    cargo test && \
    dx build --release && \
    cp -r dist/. ../backend/static && \
    cd ../backend && \
    cargo test && \
    cargo build --release && \
    eu-elfcompress ../target/release/backend && \
    strip ../target/release/backend && \
    upx -9 --lzma ../target/release/backend && \
    chmod +x ../target/release/backend

FROM alpine:3.18.3

WORKDIR /app

RUN apk add openssl libressl-dev && \
    adduser -h /app -D pw && \
    chmod 700 /app && \
    chown -R pw: /app

COPY --from=app-build /build/target/release/backend /app/pw

RUN chown -R pw: /app && chmod +x /app/pw

USER pw

CMD ["/app/pw"]


