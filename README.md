# PW

PW is a project for sharing secrets.

## Getting started

```shell
docker run -p 8080:8080 --rm -t tinyops/pw:1.0.0
```

Other options: [docker-compose.yml](docs/install/DOCKER.md) | [Linux Service](docs/install/BINARY-LINUX.md) | [Windows Service](docs/install/BINARY-WINDOWS.md)

## Docs

- [FAQ](docs/FAQ.md)
- [API](docs/API.md)
- [How to build](docs/BUILD.md)
- [Architecture](docs/ARCHITECTURE.md)

## Credits

- [kodgehopper](https://www.boringadv.com/2022/12/05/simple-encryption-in-rust/)

## Roadmap

1. Configuration via environment variables
2. Add custom translation support
3. Improve UX