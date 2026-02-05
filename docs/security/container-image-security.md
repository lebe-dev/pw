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

## Related Documentation

- Docker setup: `docs/install/DOCKER.md`
- Kubernetes deployment: `docs/install/KUBERNETES.md`
- Build instructions: `docs/BUILD.md`
