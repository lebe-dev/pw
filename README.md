# PW

PW is a project for sharing secrets.

![Screenshot of PW application for sharing secrets](pw-screenshot.png)

## Getting started

Prepare [pw.yml](backend/pw.yml-dist) and run:

```shell
docker run -p 8080:8080 --rm -v ./pw.yml:/app/pw.yml -t tinyops/pw:1.0.0
```

Other options: [docker-compose.yml](docs/install/DOCKER.md) | [Linux Service](docs/install/BINARY-LINUX.md) | [Windows Service](docs/install/BINARY-WINDOWS.md)

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

1. Short links support
2. Configuration via environment variables
3. Improve UX