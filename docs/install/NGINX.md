# Nginx configuration

For example, you have a host `pw.company.com`.

Create `/etc/nginx/conf.d/pw.company.com.conf` with content:

```nginx
server {
    listen 443 ssl;
    
    server_name pw.company.com;

    ssl_certificate /etc/nginx/ssl/postman/postman.crt;
    ssl_certificate_key /etc/nginx/ssl/postman/postman.key;
    
    location / {
      proxy_pass http://localhost:8080
    }
}

server {
    listen 80;
    server_name pw.company.com;
   
    return 301 https://$host$request_uri;
}
```

test config and reload if ok:

```shell
$ nginx -t
nginx: the configuration file /etc/nginx/nginx.conf syntax is ok
nginx: configuration file /etc/nginx/nginx.conf test is successful

systemctl reload nginx
```