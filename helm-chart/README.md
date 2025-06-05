# PW Helm Chart

This Helm chart deploys PW (Secure Secret Share Service) on a Kubernetes cluster using the Helm package manager.

## Features

- Secure secret sharing with client-side encryption (AES-256)
- Redis backend with authentication
- Configurable TTL and download policies
- File upload support
- Kubernetes-native deployment with proper security contexts
- Service accounts and RBAC ready
- Ingress support with TLS
- Health checks and resource management

## Prerequisites

- Kubernetes 1.19+
- Helm 3.2.0+

## Installing the Chart

To install the chart with the release name `my-pw`:

```bash
helm install my-pw ./pw
```

The command deploys PW on the Kubernetes cluster in the default configuration. The [Parameters](#parameters) section lists the parameters that can be configured during installation.

## Uninstalling the Chart

To uninstall/delete the `my-pw` deployment:

```bash
helm delete my-pw
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
| `pw.image.tag`                             | PW image tag                          | `1.8.1`              |
| `pw.image.pullPolicy`                      | PW image pull policy                  | `IfNotPresent`       |
| `pw.replicaCount`                          | Number of PW replicas                 | `1`                  |
| `pw.config.listen`                         | PW listen address                     | `0.0.0.0:8080`       |
| `pw.config.logLevel`                       | PW log level                          | `info`               |
| `pw.config.messageMaxLength`               | Maximum message length                | `3127`               |
| `pw.config.encryptedMessageMaxLength`      | Maximum encrypted message length      | `15000`              |
| `pw.config.fileUploadEnabled`              | Enable file upload                    | `true`               |
| `pw.config.fileMaxSize`                    | Maximum file size in bytes            | `1048576`            |
| `pw.service.type`                          | PW service type                       | `ClusterIP`          |
| `pw.service.port`                          | PW service port                       | `8080`               |
| `pw.resources.limits.cpu`                  | PW CPU limit                          | `500m`               |
| `pw.resources.limits.memory`               | PW memory limit                       | `128Mi`              |
| `pw.resources.requests.cpu`                | PW CPU request                        | `100m`               |
| `pw.resources.requests.memory`             | PW memory request                     | `64Mi`               |

### Redis parameters

| Name                           | Description                      | Value                    |
| ------------------------------ | -------------------------------- | ------------------------ |
| `redis.image.repository`       | Redis image repository           | `redis`                  |
| `redis.image.tag`             | Redis image tag                  | `8.0.2-alpine3.21`       |
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

## Security Considerations

1. **TLS Required**: PW requires HTTPS in production due to WebCrypto API requirements
2. **Redis Authentication**: Enabled by default with auto-generated passwords
3. **No Persistent Storage**: All data is stored in Redis memory with TTL
4. **Client-side Encryption**: All secrets are encrypted in the browser before transmission
5. **Service Account**: Dedicated service account with minimal permissions

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
kubectl logs -l app.kubernetes.io/name=pw,app.kubernetes.io/component=pw
kubectl logs -l app.kubernetes.io/name=pw,app.kubernetes.io/component=redis
```

## License

This chart is licensed under the same license as the PW project.
