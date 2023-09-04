# Installation (Windows)

## 1. Prepare application

```shell
mkdir c:\\pw
```

Copy `pw.exe` to `c:\\pw`.

```shell
copy pw.yml-dist c:\pw\
```

Edit `pw.yml` for your needs.

## 2. Create service

Use great tool [NSSM](https://nssm.cc/usage) to create Windows service.

Start the service.

Application will be available on http://localhost:8080.

## Related

- [Nginx configuration](NGINX.md)