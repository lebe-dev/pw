# PW Helm Chart

This Helm chart deploys PW (Secure Secret Share Service) on a Kubernetes cluster using the Helm package manager.

## Features

- Secure secret sharing with client-side encryption (AES-256)
- Redis backend with authentication
- Configurable TTL and download policies
- File upload support
- Optional nginx sidecar for static asset caching and performance optimization

## Prerequisites

- Kubernetes 1.19+
- Helm 3.18.4+

## Installing the Chart

To install the chart with the release name `pw`:

```bash
helm repo add tinyops https://tinyops.ru/helm-charts/
helm repo update
helm upgrade --install --create-namespace -n pw pw tinyops/pw --version 1.3.1
```

The command deploys PW on the Kubernetes cluster in the default configuration. The [Parameters](#parameters) section lists the parameters that can be configured during installation.

## Uninstalling the Chart

To uninstall/delete the `pw` deployment:

```bash
helm delete pw
```

## Parameters

### Global parameters

| Name                       | Description                                     | Value |
| -------------------------- | ----------------------------------------------- | ----- |
| `global.imagePullSecrets`  | Global Docker registry secret names as an array | `[]`  |

### PW Application parameters

| Name                                        | Description                           | Value                |
| ------------------------------------------- | ------------------------------------- | -------------------- |
| `pw.image.repository`                       | PW image repository                   | `tinyops/pw`         |
| `pw.image.tag`                             | PW image tag                          | ``              |
| `pw.image.pullPolicy`                      | PW image pull policy                  | `IfNotPresent`       |
| `pw.replicaCount`                          | Number of PW replicas                 | `1`                  |
| `pw.config.listen`                         | PW listen address                     | `0.0.0.0:8080`       |
| `pw.config.logLevel`                       | PW log level                          | `info`               |
| `pw.config.messageMaxLength`               | Maximum message length                | `3127`               |
| `pw.config.fileUploadEnabled`              | Enable file upload                    | `true`               |
| `pw.config.fileMaxSize`                    | Maximum file size in bytes            | `1048576`            |

**Note**: The encrypted message max length is calculated dynamically as `max(messageMaxLength, fileMaxSize) * 1.35` to account for encryption overhead. You can optionally override this by setting the `PW_ENCRYPTED_MESSAGE_MAX_LENGTH` environment variable.
| `pw.config.ipLimits.enabled`               | Enable IP whitelist limits            | `false`              |
| `pw.config.ipLimits.whitelist`             | Array of IP whitelist entries         | `[]`                 |
| `pw.config.ipLimits.trustedProxies`        | Array of trusted proxy IPs            | `[]`                 |
| `pw.service.type`                          | PW service type                       | `ClusterIP`          |
| `pw.service.port`                          | PW service port                       | `8080`               |
| `pw.resources.limits.cpu`                  | PW CPU limit                          | `500m`               |
| `pw.resources.limits.memory`               | PW memory limit                       | `128Mi`              |
| `pw.resources.requests.cpu`                | PW CPU request                        | `100m`               |
| `pw.resources.requests.memory`             | PW memory request                     | `64Mi`               |

### Nginx Sidecar parameters

| Name                           | Description                      | Value                                  |
| ------------------------------ | -------------------------------- | -------------------------------------- |
| `nginx.enabled`                | Enable nginx sidecar             | `true`                                 |
| `nginx.image.repository`       | Nginx image repository           | `nginxinc/nginx-unprivileged`          |
| `nginx.image.tag`             | Nginx image tag                  | `1.29.3-alpine-otel`                   |
| `nginx.image.pullPolicy`      | Nginx image pull policy          | `IfNotPresent`                         |
| `nginx.port`                  | Nginx container port             | `8080`                                 |
| `nginx.backendPort`           | Backend PW app port              | `8081`                                 |
| `nginx.resources`             | Nginx resource limits/requests   | `{}`                                   |
| `nginx.config`                | Nginx configuration file         | See values.yaml for full config        |

### Redis parameters

| Name                           | Description                      | Value                    |
| ------------------------------ | -------------------------------- | ------------------------ |
| `redis.image.repository`       | Redis image repository           | `redis`                  |
| `redis.image.tag`             | Redis image tag                  | `8.4.0-alpine3.22`       |
| `redis.image.pullPolicy`      | Redis image pull policy          | `IfNotPresent`           |
| `redis.replicaCount`          | Number of Redis replicas         | `1`                      |
| `redis.auth.enabled`          | Enable Redis authentication      | `true`                   |
| `redis.auth.password`         | Redis password (auto-generated)  | `""`                     |
| `redis.config.maxMemory`      | Redis max memory                 | `128mb`                  |
| `redis.service.type`          | Redis service type               | `ClusterIP`              |
| `redis.service.port`          | Redis service port               | `6379`                   |

### Ingress parameters

| Name                      | Description                        | Value            |
| ------------------------- | ---------------------------------- | ---------------- |
| `ingress.enabled`         | Enable ingress record generation   | `true`           |
| `ingress.className`       | IngressClass that will be used     | `nginx`          |
| `ingress.annotations`     | Additional annotations for Ingress | `{}`             |
| `ingress.hosts`          | An array with hostnames            | `[{host: "pw.company.com", paths: [{path: "/", pathType: "Prefix"}]}]` |
| `ingress.tls`            | TLS configuration for ingress      | `[]`             |

### Security parameters

| Name                           | Description                       | Value   |
| ------------------------------ | --------------------------------- | ------- |
| `serviceAccount.create`        | Create service account            | `true`  |
| `serviceAccount.annotations`   | Service account annotations       | `{}`    |
| `serviceAccount.name`         | Service account name              | `""`    |
| `podSecurityContext.fsGroup`   | Pod security context fsGroup      | `1000`  |
| `securityContext.runAsNonRoot` | Run containers as non-root user   | `true`  |
| `securityContext.runAsUser`    | Run containers as specific user   | `1000`  |

## Nginx Sidecar Configuration

PW includes an optional nginx sidecar container that provides:

- **Static asset caching**: Immutable caching for hashed assets (CSS, JS, fonts, images) with `Cache-Control: public, immutable`
- **Gzip compression**: Automatic compression for text-based content
- **Security**: Blocks external access to `/api/health` and `/api/metrics` endpoints
- **Performance**: Offloads static asset serving from the main application

Enabled by default.

## IP Whitelist Configuration

PW supports IP-based access restrictions with custom payload limits. When enabled, only IPs in the whitelist can access the service, and each IP can have individual limits for message and file sizes.

### Example Configuration

```yaml
pw:
  config:
    ipLimits:
      enabled: true
      whitelist:
        - ip: "192.168.1.100"
          messageMaxLength: 8192
          fileMaxSize: 104857600
        - ip: "10.0.0.0/8"
          messageMaxLength: 4096
        - ip: "172.16.1.5"
          fileMaxSize: 209715200
        - ip: "2001:db8::1"
          messageMaxLength: 16384
          fileMaxSize: 52428800
```

### Whitelist Entry Fields

| Field               | Type   | Required | Description                           |
| ------------------- | ------ | -------- | ------------------------------------- |
| `ip`                | string | Yes      | IP address or CIDR block (IPv4/IPv6) |
| `messageMaxLength`  | number | No       | Custom message size limit in bytes    |
| `fileMaxSize`       | number | No       | Custom file size limit in bytes      |

### Trusted Proxies

When PW is deployed behind a reverse proxy or load balancer, configure trusted proxies to correctly identify client IP addresses:

```yaml
pw:
  config:
    ipLimits:
      trustedProxies:
        - "192.168.1.1"       # Nginx proxy
        - "10.0.0.1"          # Load balancer
        - "proxy.example.com" # Named proxy
```

### Notes on Trusted Proxies

- Trusted proxy IPs are used to extract the real client IP from proxy headers
- Supports both IPv4 and IPv6 addresses
- Can specify hostnames that resolve to IP addresses
- Environment variable `PW_IP_LIMITS_TRUSTED_PROXIES` overrides YAML configuration
- Format: JSON array of strings (e.g., `["192.168.1.1", "10.0.0.1"]`)

### Notes

- Environment variables `PW_IP_LIMITS_ENABLED` and `PW_IP_LIMITS_WHITELIST` override YAML configuration
- IP addresses support both IPv4 and IPv6 formats
- CIDR notation is supported (e.g., `10.0.0.0/8`, `2001:db8::/32`)
- If limits are not specified for an IP, global defaults are used
- Duplicate IP entries are not allowed and will cause validation errors

## Security Considerations

1. **TLS Required**: PW requires HTTPS in production due to WebCrypto API requirements
2. **Redis Authentication**: Enabled by default with auto-generated passwords
3. **No Persistent Storage**: All data is stored in Redis memory with TTL
4. **Client-side Encryption**: All secrets are encrypted in the browser before transmission
5. **Service Account**: Dedicated service account with minimal permissions
6. **IP Whitelist**: Optional IP-based access control with custom payload limits

## Accessing the Application

After installation, get the application URL:

```bash
# If using ingress
echo "https://$(kubectl get ingress -o jsonpath='{.items[0].spec.rules[0].host}')"

# If using port-forward
kubectl port-forward svc/my-pw-pw 8080:8080
echo "http://localhost:8080"
```

## Troubleshooting

### Check pod status:

```bash
kubectl get pods -l app.kubernetes.io/name=pw
```

### View logs:

```bash
# PW application logs
kubectl logs -l app.kubernetes.io/name=pw,app.kubernetes.io/component=pw -c pw

# Nginx sidecar logs (if enabled)
kubectl logs -l app.kubernetes.io/name=pw,app.kubernetes.io/component=pw -c nginx

# Redis logs
kubectl logs -l app.kubernetes.io/name=pw,app.kubernetes.io/component=redis
```

### Check container status:

```bash
# Check if nginx sidecar is running
kubectl get pods -l app.kubernetes.io/component=pw -o jsonpath='{.items[0].spec.containers[*].name}'
# Expected output with nginx enabled: pw nginx
# Expected output with nginx disabled: pw
```

## License

This chart is licensed under the same license as the PW project.
