FROM node:25.9.0-alpine3.23 AS frontend-build

ARG FALLBACK_LOCALE_ID=en

WORKDIR /build

COPY frontend/ /build
COPY Cargo.toml /build/Cargo.toml

RUN APP_VERSION=$(grep version /build/Cargo.toml | head -1 | cut -d ' ' -f 3 | tr -d '"') && \
    sed -i "s/\"version\": \"[^\"]*\"/\"version\": \"$APP_VERSION\"/" /build/package.json && \
    sed -i "s/'en'/'$FALLBACK_LOCALE_ID'/g" /build/src/routes/+layout.ts && \
    yarn && \
    yarn build

FROM rust:1.97.1-alpine AS app-build

WORKDIR /build

RUN mkdir -p /build/static && \
    apk --no-cache add nodejs npm musl-dev elfutils pkgconfig libressl-dev perl make mold upx

COPY Cargo.toml Cargo.lock pw.yml-dist /build/
COPY src/ /build/src/
COPY --from=frontend-build /build/build/ /build/static/

COPY favicon.png /build/static/

RUN cargo build --release && \
    eu-elfcompress target/release/pw && \
    strip target/release/pw && \
    upx -9 --lzma target/release/pw && \
    chmod +x target/release/pw

FROM alpine:3.24

WORKDIR /app

RUN addgroup -g 10001 pw && \
    adduser -h /app -D -u 10001 -G pw pw && \
    chmod 700 /app && \
    chown -R pw: /app

COPY --from=app-build /build/pw.yml-dist /app/pw.yml
COPY --from=app-build /build/target/release/pw /app/pw

RUN chown -R pw: /app && chmod +x /app/pw

USER pw

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD wget -q -O- http://localhost:8080/api/health || exit 1

CMD ["/app/pw"]
