# Installation (Linux)

## 1. Install redis

## 2. Install pw

Create unprivileged user `pw`:

```bash
useradd pw -d /opt/pw

# Set strong password
passwd pw
```

Copy `pw` to `/opt/pw`.

Prepare config:

```bash
cp pw.yml-dist /opt/pw/pw.yml
```

Edit `/opt/pw/pw.yml` for your needs.

Update permissions:

```bash
chmod -R 500 /opt/pw
```

## Create service

Create file `/etc/systemd/system/pw.service` with content:

```ini
[Unit]
Description=PW - Secure Secret Share Service
After=network.target remote-fs.target nss-lookup.target

[Service]
Type=simple
User=pw

ExecStart=/opt/pw/pw

WorkingDirectory=/opt/pw

KillMode=process

[Install]
WantedBy=multi-user.target
```

Then start:

```bash
systemcl daemon-reload
systemctl enable --now pw
```

Application will be available on http://localhost:8080.

## Related

- [Nginx configuration](NGINX.md)
