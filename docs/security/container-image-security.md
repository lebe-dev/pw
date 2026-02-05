# Container Image Security

## Overview

Container images are regularly scanned for security vulnerabilities using [Trivy](https://trivy.dev/), an industry-standard open-source security scanner. This ensures that both the application and its dependencies remain secure and free from known vulnerabilities.

## Scanning Process

Images are scanned for:

- **OS vulnerabilities**: Security issues in Alpine Linux packages
- **Package vulnerabilities**: Known CVEs in installed packages
- **Secret detection**: Accidentally committed secrets or credentials
- **Security misconfigurations**: Insecure container configurations

## Latest Scan Results

See [trivy-scan-report.txt](trivy-scan-report.txt).

## Nginx Image Selection

The helm chart uses the official `nginx:1.29.5-alpine3.23-perl` image instead of `nginxinc/nginx-unprivileged` due to security considerations:

- **Security**: Trivy scanner detected 32 HIGH/CRITICAL vulnerabilities in the unprivileged image
- **Compatibility**: Official nginx image runs in unprivileged mode with `runAsUser: 101` (same as unprivileged variant)
- **Configuration**: Nginx config uses `/tmp` paths and port 8080 to work without root privileges

The official image is regularly updated and scanned. See latest scan results for both application and nginx images in the reports.

## Related Documentation

- Docker setup: `docs/install/DOCKER.md`
- Kubernetes deployment: `docs/install/KUBERNETES.md`
- Build instructions: `docs/BUILD.md`
