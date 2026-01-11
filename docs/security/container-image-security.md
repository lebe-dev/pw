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

### Main Application Image

**Image**: `tinyops/pw:1.11.0`

**Trivy Version**: 0.68.1  
**Scan Date**: 2025-12-05  
**Vulnerability DB**: Version 2 (Updated: 2025-12-05)

```bash
$ trivy -v

Version: 0.68.1
Vulnerability DB:
  Version: 2
  UpdatedAt: 2025-12-05 12:26:38.481889982 +0000 UTC
  NextUpdate: 2025-12-06 12:26:38.481889712 +0000 UTC
  DownloadedAt: 2025-12-05 12:58:07.216851 +0000 UTC
Java DB:
  Version: 1
  UpdatedAt: 2025-11-20 00:57:10.039747638 +0000 UTC
  NextUpdate: 2025-11-23 00:57:10.039747518 +0000 UTC
  DownloadedAt: 2025-11-20 07:42:07.774353 +0000 UTC

$ trivy image tinyops/pw:1.11.0
2025-12-05T16:02:41+03:00	INFO	[vuln] Vulnerability scanning is enabled
2025-12-05T16:02:41+03:00	INFO	[secret] Secret scanning is enabled
2025-12-05T16:02:41+03:00	INFO	[secret] If your scanning is slow, please try '--scanners vuln' to disable secret scanning
2025-12-05T16:02:41+03:00	INFO	[secret] Please see https://trivy.dev/docs/v0.68/guide/scanner/secret#recommendation for faster secret detection
2025-12-05T16:02:41+03:00	INFO	Detected OS	family="alpine" version="3.23.0"
2025-12-05T16:02:41+03:00	WARN	This OS version is not on the EOL list	family="alpine" version="3.23"
2025-12-05T16:02:41+03:00	INFO	[alpine] Detecting vulnerabilities...	os_version="3.23" repository="3.23" pkg_num=16
2025-12-05T16:02:41+03:00	INFO	Number of language-specific files	num=0

Report Summary

┌───────────────────────────────────┬────────┬─────────────────┬─────────┐
│              Target               │  Type  │ Vulnerabilities │ Secrets │
├───────────────────────────────────┼────────┼─────────────────┼─────────┤
│ tinyops/pw:1.11.0 (alpine 3.23.0) │ alpine │        0        │    -    │
└───────────────────────────────────┴────────┴─────────────────┴─────────┘
Legend:
- '-': Not scanned
- '0': Clean (no security findings detected)
```

**Summary**: ✅ **0 vulnerabilities detected**

**Base OS**: Alpine Linux 3.23.0  
**Packages scanned**: 16

### Redis Image

**Image**: `redis:8.4.0-alpine3.22`

**Scan Date**: 2025-11-21

```bash
$ trivy image redis:8.4.0-alpine3.22

2025-11-21T16:28:46+03:00	INFO	[vuln] Vulnerability scanning is enabled
2025-11-21T16:28:46+03:00	INFO	[secret] Secret scanning is enabled
2025-11-21T16:28:46+03:00	INFO	[secret] If your scanning is slow, please try '--scanners vuln' to disable secret scanning
2025-11-21T16:28:46+03:00	INFO	[secret] Please see https://trivy.dev/v0.67/docs/scanner/secret#recommendation for faster secret detection
2025-11-21T16:28:54+03:00	INFO	Detected OS	family="alpine" version="3.22.2"
2025-11-21T16:28:54+03:00	INFO	[alpine] Detecting vulnerabilities...	os_version="3.22" repository="3.22" pkg_num=22
2025-11-21T16:28:54+03:00	INFO	Number of language-specific files	num=0

Report Summary

┌────────────────────────────────────────┬────────┬─────────────────┬─────────┐
│                 Target                 │  Type  │ Vulnerabilities │ Secrets │
├────────────────────────────────────────┼────────┼─────────────────┼─────────┤
│ redis:8.4.0-alpine3.22 (alpine 3.22.2) │ alpine │        0        │    -    │
└────────────────────────────────────────┴────────┴─────────────────┴─────────┘
Legend:
- '-': Not scanned
- '0': Clean (no security findings detected)
```

**Summary**: ✅ **0 vulnerabilities detected**

**Base OS**: Alpine Linux 3.22.2  
**Packages scanned**: 22

## Running Scans Locally

### Prerequisites

Install Trivy on your system:

**macOS:**
```bash
brew install trivy
```

**Linux (Debian/Ubuntu):**
```bash
wget -qO - https://aquasecurity.github.io/trivy-repo/deb/public.key | sudo apt-key add -
echo "deb https://aquasecurity.github.io/trivy-repo/deb $(lsb_release -sc) main" | sudo tee -a /etc/apt/sources.list.d/trivy.list
sudo apt-get update
sudo apt-get install trivy
```

**Linux (RHEL/CentOS):**
```bash
sudo rpm -ivh https://github.com/aquasecurity/trivy/releases/download/v0.68.1/trivy_0.68.1_Linux-64bit.rpm
```

**Docker:**
```bash
docker run aquasec/trivy:latest image your-image:tag
```

### Scan Application Image

```bash
# Scan specific version
trivy image tinyops/pw:1.11.0
```

### Scan Redis Image

```bash
trivy image redis:8.4.0-alpine3.22
```

**Scan local Dockerfile:**
```bash
trivy config Dockerfile
```

**Scan filesystem:**
```bash
trivy fs /path/to/project
```

## Security Best Practices

### Image Selection

1. **Use Alpine-based images**: Minimal attack surface with fewer packages
2. **Pin specific versions**: Avoid `latest` tag for reproducibility
3. **Regularly update base images**: Keep up with security patches

### Current Images

Our application uses security-focused base images:

- **Application**: `alpine:3.23.0`
  - Minimal Linux distribution
  - Small size (~5 MB)
  - Fast security updates
  
- **Redis**: `redis:8.4.0-alpine3.22`
  - Official Redis image
  - Alpine-based for minimal footprint
  - Regular security updates

## Additional Resources

- **Trivy Documentation**: https://trivy.dev/docs/
- **Alpine Security**: https://alpinelinux.org/security/
- **Redis Security**: https://redis.io/docs/security/
- **CVE Database**: https://cve.mitre.org/

## References

### Related Documentation

- Docker setup: `docs/install/DOCKER.md`
- Kubernetes deployment: `docs/install/KUBERNETES.md`
- Build instructions: `docs/BUILD.md`
