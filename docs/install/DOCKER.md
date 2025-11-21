# How to install PW with docker

**1. Install Docker and docker-compose**

- [Official manual](https://docs.docker.com/engine/install/)

**2. Create app directory**

```bash
mkdir -p /opt/pw
```

**3. Use [docker-compose.yml](../../docker-compose.yml)**

**4. Run**

```bash
docker compose up -d
```

Application will be available on http://localhost:8080.

## Related

- [Nginx configuration](NGINX.md)
