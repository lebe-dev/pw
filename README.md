# PW

PW is a project for sharing secrets (any confidential information). All data is encrypted in the browser.

![Screenshot of PW application for sharing secrets](pw-screenshot.png)


Demo: [https://pw.tinyops.ru](https://pw.tinyops.ru)

## Getting started

```shell
docker run -d --name pw -p 8080:8080 --rm -t tinyops/pw:1.0.0
```

Other options: [docker-compose.yml](docs/install/DOCKER.md) | [Linux Service](docs/install/BINARY-LINUX.md)

## Docs

- [FAQ](docs/FAQ.md)
- [Installation](docs/install/INSTALL.md)
- [Security](docs/SECURITY.md)
- [Localization](docs/LOCALE.md)
- [API](docs/API.md)
- [How to build](docs/BUILD.md)
- [Architecture](docs/ARCHITECTURE.md)

## Credits

- [kodgehopper](https://www.boringadv.com/2022/12/05/simple-encryption-in-rust/)

## Roadmap

1. Build static release
2. Support Windows
3. Configuration via environment variables
