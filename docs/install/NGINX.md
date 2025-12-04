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
      
      # Client IP headers for IP-based limits feature
      proxy_set_header X-Real-IP $remote_addr;
      proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
      proxy_set_header X-Forwarded-Proto $scheme;
      proxy_set_header Host $host;
    }
    
    location ~* \.(?:jpg|jpeg|gif|png|ico|js|svg|woff|woff2|ttf|css)$ {
      expires max;

      access_log off;
      add_header Cache-Control "public";
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

```bash
$ nginx -t

nginx: the configuration file /etc/nginx/nginx.conf syntax is ok
nginx: configuration file /etc/nginx/nginx.conf test is successful

systemctl reload nginx
```

## Client IP Configuration for IP-based Limits

When using the IP-based limits feature, nginx must be configured to properly forward client IP addresses. The configuration above includes the necessary headers:

- `X-Real-IP` - Direct client IP address
- `X-Forwarded-For` - Complete IP chain including intermediary proxies
- `X-Forwarded-Proto` - Original protocol (http/https)
- `Host` - Original host header

### Behind Load Balancer or CDN

If nginx is behind another proxy/load balancer, configure real IP detection:

```nginx
# Trust IP headers from these sources
set_real_ip_from 10.0.0.0/8;
set_real_ip_from 172.16.0.0/12;
set_real_ip_from 192.168.0.0/16;
set_real_ip_from 127.0.0.1;

# Use X-Forwarded-For header to determine real IP
real_ip_header X-Forwarded-For;
real_ip_recursive on;
```

Add this configuration in the `server` block before the `location` directives.
