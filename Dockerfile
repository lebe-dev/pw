FROM node:25.2.1-alpine3.22 AS frontend-build

ARG FALLBACK_LOCALE_ID=en

WORKDIR /build

COPY frontend/ /build

RUN sed -i "s/'en'/'$FALLBACK_LOCALE_ID'/g" /build/src/routes/+layout.ts && \
    yarn && \
    yarn build

FROM rust:1.91.1-alpine3.22 AS app-build

WORKDIR /build

RUN mkdir -p /build/static && \
    apk add nodejs npm musl-dev elfutils xz wget pkgconfig libressl-dev perl make mold && \
    wget https://github.com/upx/upx/releases/download/v5.0.2/upx-5.0.2-amd64_linux.tar.xz && \
    unxz upx-5.0.2-amd64_linux.tar.xz && tar xvf upx-5.0.2-amd64_linux.tar && \
    cp upx-5.0.2-amd64_linux/upx /usr/bin/upx && chmod +x /usr/bin/upx

COPY . /build
COPY --from=frontend-build /build/build/ /build/static/

COPY favicon.png /build/static/

RUN cargo test && \
    cargo build --release && \
    eu-elfcompress target/release/pw && \
    strip target/release/pw && \
    upx -9 --lzma target/release/pw && \
    chmod +x target/release/pw

FROM alpine:3.22.2

WORKDIR /app

RUN apk update && \
    adduser -h /app -D pw && \
    chmod 700 /app && \
    chown -R pw: /app

COPY --from=app-build /build/pw.yml-dist /app/pw.yml
COPY --from=app-build /build/target/release/pw /app/pw

RUN chown -R pw: /app && chmod +x /app/pw

USER pw

CMD ["/app/pw"]
