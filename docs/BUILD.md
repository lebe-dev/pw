# How to build

## How to build docker image (Recommended)

1. Install [Docker](https://docs.docker.com/engine/install/)

2. Build image:

```shell
docker build --progress=plain --platform=linux/amd64 -t IMAGE-TAG .
```

## How to build standalone version

**1. Install [Rust](https://rust-lang.org/learn/get-started/)**

**2. Install [NodeJS and npm](https://nodejs.org/en/download)**

**3. Build frontend module**

```shell
cd frontend
npm i
npm run build
```

**4. Build app**

```shell
mkdir static
cp -r frontend/build/ static/
cargo build --release
cd ..

# Linux/MacOS
cp target/release/pw pw

# Windows
cp target/release/pw.exe pw.exe
```
