# PW

PW is a project for sharing secrets (any confidential information). All data is encrypted in the browser.

![Screenshot of PW application for sharing secrets](pw-screenshot.png)

![Screenshot of PW application for sharing secrets](pw-screenshot-dark.png)

[Demo](https://pw.tinyops.ru)

## Getting started

```shell
docker compose up -d
```

Then visit http://localhost:8080.

Other options: [Kubernetes](docs/install/KUBERNETES.md)

## Features

- Secure: All data is encrypted in the browser ([details](docs/SECURITY.md))
- Supported media types: text, file
- BLAZING FAST 🌝 (Svelte+Rust)
- Dynamic configuration limits based on ip whitelist
- Low resources usage:
  ```shell
  CONTAINER ID   NAME              CPU %     MEM USAGE / LIMIT    MEM %     NET I/O           BLOCK I/O      PIDS
  94d9d31ddf83   pw-cache          1.27%     7.977MiB / 1.69GiB   0.46%     16.2kB / 0B       0B / 0B        6
  0d3c9c52165a   pw                0.00%     4.082MiB / 1.69GiB   0.24%     63.2kB / 224kB    0B / 0B        2
  ```
- Localization support: en, es, de, ru ([details](docs/LOCALE.md))
- Dark theme support

## Docs

- [FAQ](docs/FAQ.md)
- [Installation](docs/install/INSTALL.md)
- [Security](docs/SECURITY.md)
- [Localization](docs/LOCALE.md)
- [API](docs/API.md)
- [How to build](docs/BUILD.md)
- [Architecture](docs/ARCHITECTURE.md)

## Roadmap

- Backend: improve return codes

## Thanks

- [Nicco](https://github.com/cupcakearmy), author of [cryptgeon](https://github.com/cupcakearmy/cryptgeon)
