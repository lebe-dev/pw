# Development

Start redis:

```shell
docker-compose -f docker-compose-dev.yml up -d
```

Install Rust 1.87+.

Prepare config for backend:

```bash
cp pw.yml-dev pw.yml
```

Start backend:

```bash
cargo run
```

Install NodeJS 20.x + npm then install dependencies:

```bash
cd ../frontend
npm i
```

Start dev server:

```bash
npm run dev -- --port 4200
```

Open http://localhost:4200.

## Nginx config

Add `pw.test` in `hosts`.

Use config for nginx:

```nginx
server {
    listen 80;

    server_name pw.test;

    location /api {
        proxy_set_header Host $host;
        proxy_pass http://localhost:8080;
    }

    location / {
        proxy_set_header Host $host;
        proxy_pass http://localhost:4200;
    }
}
```
