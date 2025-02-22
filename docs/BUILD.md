# How to build

## How to build docker image

1. Install [Docker](https://docs.docker.com/engine/install/)

2. Build image:

```shell
docker build --progress=plain -t pw:1.7.0 .
```

## How to build standalone version

**1. Install Rust 1.85 or later**

**2. Install NodeJS and npm**

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

# Linux
cp target/release/pw pw

# Windows
cp target/release/pw.exe pw.exe
```
