# How to build

## How to build docker image

1. Install [Docker](https://docs.docker.com/engine/install/)

2. Build image:

```shell
docker build -t pw:1.0.0 .
```

## How to build standalone version

**1. Install Rust 1.72 or later**

**2. Install Dioxus CLI**

```shell
cargo install dioxus-cli --locked
```

**3. Build webui module**

```shell
cd webui
dx build --release
```

**4. Build app**

```shell
mkdir backend/static
cp -r ../webui/dist/ static/
cd backend
cargo build --release
cd ..

# Linux
cp target/release/backend pw

# Windows
cp target/release/backend.exe pw.exe
```