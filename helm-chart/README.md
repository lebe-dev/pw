# PW Helm Chart

This Helm chart deploys PW (Secure Secret Share Service) on a Kubernetes cluster using the Helm package manager.

## Features

- Secure secret sharing with client-side encryption (AES-256)
- Redis backend with authentication
- Configurable TTL and download policies
- File upload support
- Optional nginx sidecar for static asset caching and performance optimization

## Changelog

For detailed information about changes in each release, see [CHANGELOG.md](CHANGELOG.md).

For releases from version 1.5.0 onwards, changelog information is also available on [Artifact Hub](https://artifacthub.io/packages/helm).

## Prerequisites

- Kubernetes 1.19+
- Helm 3.18.4+

## Installing the Chart

To install the chart with the release name `pw`:

```bash
helm repo add tinyops https://tinyops.ru/helm-charts/
helm repo update
helm upgrade --install --create-namespace -n pw pw tinyops/pw --version 1.4.0
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
| `nginx.image.repository`       | Nginx image repository           | `nginx`                                |
| `nginx.image.tag`             | Nginx image tag                  | `1.29.5-alpine3.23-perl`               |
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
- Supports both IPv4 and IPv6 addresses, CIDR notation supported
- Can specify hostnames that resolve to IP addresses
- Environment variable `PW_IP_LIMITS_TRUSTED_PROXIES` overrides YAML configuration
- Format: JSON array of strings (e.g., `["192.168.1.1", "10.0.0.0/8"]`)

### Important: Trusted Proxies in Kubernetes

When deploying PW in Kubernetes with ingress-nginx and the nginx sidecar enabled, you **MUST** configure trusted proxies correctly to ensure client IP addresses are properly detected.

#### Required Trusted Proxies

```yaml
pw:
  config:
    ipLimits:
      enabled: true
      trustedProxies:
        - "127.0.0.1"           # Nginx sidecar on localhost
        - "10.244.0.0/16"       # Pod network (adjust to your cluster's pod CIDR)
        # Add your external load balancer IPs if needed:
        - "123.123.105.67"       # External LoadBalancer IP
```

**Why these are required:**

1. **`127.0.0.1`**: The nginx sidecar runs in the same pod and connects to the PW app via localhost. The app must trust localhost to read client IP from HTTP headers.

2. **Pod Network CIDR** (e.g., `10.244.0.0/16`): The ingress-nginx controller pods connect to the PW service from within the pod network. The app must trust these IPs to extract the real client IP from X-Forwarded-For headers set by ingress-nginx.

3. **External LoadBalancer IPs** (optional): If using MetalLB or cloud load balancer, add these IPs to the trusted list.

#### Finding Your Pod Network CIDR

```bash
# Method 1: Check node pod CIDR
kubectl get nodes -o jsonpath='{.items[*].spec.podCIDR}' | tr ' ' '\n' | head -1

# Method 2: Check existing pod IPs
kubectl get pods -A -o wide | awk '{print $6}' | grep -E '^[0-9]+\.' | head -5
# If you see IPs like 10.244.x.x, use 10.244.0.0/16
# If you see IPs like 10.42.x.x, use 10.42.0.0/16
```

### Nginx Ingress Controller Configuration

For proper client IP detection, the ingress-nginx controller **MUST** be configured with these settings:

```yaml
controller:
  config:
    use-forwarded-headers: "true"
    compute-full-forwarded-for: "true"
    forwarded-for-header: "X-Forwarded-For"
```

These settings enable nginx to:
- Trust X-Forwarded-For headers from upstream proxies
- Compute the full forwarded-for chain
- Set `$remote_addr` to the real client IP (extracted from X-Forwarded-For)

**Without these settings**, the ingress-nginx controller will set `$remote_addr` to the load balancer or node IP, not the actual client IP.

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

### IP Whitelist Not Working / Always Getting Default Limits

If IP-based limits are not being applied correctly (users always get default limits instead of whitelist limits), check the following:

#### 1. Enable debug logging to see what IP the app detects:

```bash
kubectl set env deployment/<release-name>-pw PW_LOG_LEVEL=debug -c pw
```

Check logs for client IP detection:
```bash
kubectl logs -l app.kubernetes.io/component=pw -c pw --tail=50 | grep "extracted client IP"
```

Expected output:
```
DEBUG - extracted client IP: 203.0.113.195 (connection IP: 127.0.0.1)
```

#### 2. If you see `127.0.0.1` as extracted client IP:

This means the app is not trusting the nginx sidecar. Add `127.0.0.1` to trustedProxies:

```yaml
pw:
  config:
    ipLimits:
      trustedProxies:
        - "127.0.0.1"
```

#### 3. If you see pod network IP (e.g., `10.244.x.x`) as extracted client IP:

This means the app is not trusting ingress-nginx pods. Add your pod network CIDR:

```yaml
pw:
  config:
    ipLimits:
      trustedProxies:
        - "10.244.0.0/16"  # Adjust to your cluster's pod CIDR
```

#### 4. If you see node IP (e.g., `10.220.x.x`) but expect external IP:

This means:
- You're testing from inside the cluster network, OR
- Ingress-nginx is not configured with `use-forwarded-headers: "true"`

Check ingress-nginx controller config:
```bash
kubectl get configmap ingress-nginx-controller -n ingress-nginx -o yaml | grep -E "use-forwarded|compute-full"
```

Expected output:
```yaml
use-forwarded-headers: "true"
compute-full-forwarded-for: "true"
forwarded-for-header: "X-Forwarded-For"
```

If these are missing, update your ingress-nginx values and apply.

#### 5. Verify nginx sidecar is forwarding headers correctly:

Check nginx sidecar logs for X-Forwarded-For header:
```bash
kubectl logs -l app.kubernetes.io/component=pw -c nginx --tail=20
```

Look for the last field in log lines (X-Forwarded-For):
```
10.244.194.42 - - [02/Feb/2026:10:16:34 +0000] "GET /api/config HTTP/1.1" 200 72 "-" "curl/8.7.1" "203.0.113.195"
```

The last field should contain the real client IP, not `"-"` or pod/node IPs.

#### 6. Complete troubleshooting checklist:

- [ ] Ingress-nginx has `use-forwarded-headers: "true"`
- [ ] Ingress-nginx has `compute-full-forwarded-for: "true"`
- [ ] PW trustedProxies includes `127.0.0.1` (nginx sidecar)
- [ ] PW trustedProxies includes pod network CIDR (e.g., `10.244.0.0/16`)
- [ ] Nginx sidecar uses `$http_x_real_ip` and `$http_x_forwarded_for` (not `$remote_addr`)
- [ ] IP whitelist is enabled: `ipLimits.enabled: true`
- [ ] Client IP is in the whitelist with correct CIDR notation

### Testing from within cluster

When testing from a node or pod inside the cluster, the detected IP will be the node/pod IP, not your external IP. To test with external IPs:

```bash
# Test from outside the cluster
curl https://pw.example.com/api/config

# Or use a service like httpbin to verify headers
curl -H "X-Forwarded-For: 203.0.113.195" https://pw.example.com/api/config
```

## License

This chart is licensed under the same license as the PW project.
