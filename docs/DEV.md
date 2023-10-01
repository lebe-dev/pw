# Development

Install Rust 1.72+.

Prepare config for backend:

```shell
cp pw.yml-dev pw.yml
```

Start backend:

```shell
cargo run
````

Install NodeJS + npm:

```shell
cd ../webui
npm i
```

Start dev server:

```shell
npm run dev -- --port 4200
```

http://localhost:4200

## Nginx config

Add `pw.test` in `hosts`.

Use config for nginx:

```nginx
server {
    listen 80;

    server_name  pw.test;

    location /api {
        proxy_set_header Host $host;
        proxy_pass http://localhost:8080;
    }
}
```