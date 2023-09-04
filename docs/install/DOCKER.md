# How to install PW with docker

**1. Install Docker and docker-compose**

- [Official manual](https://docs.docker.com/engine/install/)

**2. Create app directory**

```shell
mkdir -p /opt/pw
```

**3. Create docker-compose.yml**

```yaml
version: '3.3'

services:
  app:
    container_name: pw
    image: tinyops/pw:1.0.0
    restart: always
    volumes:
      - ./pw.yml:/app/app.yml
      - ./logs:/app/logs
    ports:
      - "8080:8080"
```

**4. Run**

```shell
docker-compose up -d
```

Application will be available on http://localhost:8080.

## Related

- [Nginx configuration](NGINX.md)