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
      proxy_pass http://localhost:8080;
    }
    
    gzip            on;
    gzip_comp_level 5;
    gzip_min_length 256;
    gzip_proxied    any;
    gzip_vary on;
    gzip_types      application/atom+xml
                  application/javascript
                  application/json
                  application/ld+json
                  application/manifest+json
                  application/rss+xml
                  application/geo+json
                  application/vnd.ms-fontobject
                  application/x-web-app-manifest+json
                  application/xhtml+xml
                  application/xml
                  application/rdf+xml
                  font/otf
                  application/wasm
                  image/bmp
                  image/svg+xml
                  text/cache-manifest
                  text/css
                  text/javascript
                  text/plain
                  text/markdown
                  text/vcard
                  text/calendar
                  text/vnd.rim.location.xloc
                  text/vtt
                  text/x-component
                  text/x-cross-domain-policy;
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