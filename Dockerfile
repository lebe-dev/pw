FROM node:22.9.0-alpine3.20 as webui-build

ARG FALLBACK_LOCALE_ID=en

WORKDIR /build

COPY webui/ /build

RUN sed -i "s/'en'/'$FALLBACK_LOCALE_ID'/g" /build/src/routes/+layout.ts && \
    npm i && \
    npm run build

FROM rust:1.81.0-alpine3.20 as app-build

WORKDIR /build

RUN mkdir -p /build/static && \
    apk add nodejs npm musl-dev elfutils xz wget pkgconfig libressl-dev perl make && \
    wget https://github.com/upx/upx/releases/download/v4.0.2/upx-4.0.2-amd64_linux.tar.xz && \
    unxz upx-4.0.2-amd64_linux.tar.xz && tar xvf upx-4.0.2-amd64_linux.tar && \
    cp upx-4.0.2-amd64_linux/upx /usr/bin/upx && chmod +x /usr/bin/upx

COPY . /build
COPY --from=webui-build /build/build/ /build/backend/static/

COPY favicon.png /build/backend/static/

RUN cd backend && \
    cargo test && \
    cargo build --release && \
    eu-elfcompress ../target/release/backend && \
    strip ../target/release/backend && \
    upx -9 --lzma ../target/release/backend && \
    chmod +x ../target/release/backend

FROM alpine:3.20.3

WORKDIR /app

RUN apk add libressl-dev && \
    adduser -h /app -D pw && \
    chmod 700 /app && \
    chown -R pw: /app

COPY --from=app-build /build/backend/pw.yml-dist /app/pw.yml
COPY --from=app-build /build/target/release/backend /app/pw

RUN chown -R pw: /app && chmod +x /app/pw

USER pw

CMD ["/app/pw"]