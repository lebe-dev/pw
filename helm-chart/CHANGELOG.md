# Changelog

All notable changes to the PW Helm chart will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.5.1] - 2026-02-05

### Security
- Migrated nginx sidecar from nginxinc/nginx-unprivileged:1.29.3-alpine-otel to nginx:1.29.5-alpine3.23-perl
- Fixed 32 HIGH/CRITICAL vulnerabilities found by Trivy in unprivileged nginx image
- Updated nginx version from 1.29.3 to 1.29.5
- Official nginx image maintains same unprivileged mode with runAsUser: 101

## [1.5.0] - 2026-02-05

### Added
- Dedicated metrics endpoint with separate service for Prometheus monitoring on port 8082
- Metrics service for external monitoring systems integration

### Security
- Enhanced Kubernetes security with hardened pod security contexts
- Non-root container execution (runAsUser: 1007)
- Read-only root filesystem for improved container security
- Dropped all Linux capabilities, added only NET_BIND_SERVICE
- Seccomp profile RuntimeDefault

### Changed
- Improved liveness and readiness probe configurations for both PW and Redis containers
- Updated Redis image to 8.4.0-alpine3.22 for stability improvements
- Enhanced health check reliability with better probe settings
- Optimized health check intervals and timeout values

## [1.4.1] - 2026-01-27

### Added
- Enhanced client IP detection with trusted proxies support
- Improved documentation for proxy configuration in Kubernetes environments

### Changed
- Updated application version to 1.14.0
- Improved client-IP detection when nginx sidecar is enabled
- Added comprehensive guide for trusted proxies in kubernetes deployment

### Fixed
- Client IP detection now works correctly with multiple proxies

## [1.4.0] - 2026-01-14

### Added
- Optional nginx sidecar container (enabled by default) for static asset caching and performance optimization
- Client cache layer implementation for improved frontend performance
- Support for static asset immutable caching with proper cache headers
- Gzip compression for text-based content delivered by nginx
- Security blocking of `/api/health` and `/api/metrics` endpoints from external access
- Nginx configuration options in values.yaml for customization

### Changed
- Updated README with nginx sidecar configuration documentation
- Adjusted PW application listen address when nginx sidecar is enabled
- Enhanced deployment templates to support optional nginx container
- Updated configmap to detect nginx sidecar enabled state

### Documentation
- Added comprehensive nginx sidecar configuration guide
- Added instructions for viewing nginx sidecar logs
- Added container status check commands for troubleshooting

## [1.3.1] - 2026-01-13

### Changed
- Updated application version to 1.13.1

## [1.3.0] - 2026-01-13

### Removed
- Removed rate limits feature due to complexity - use nginx sidecar for rate limiting instead

### Changed
- Helm chart updated for compatibility with app version 1.12.0+

## [1.1.0] - 2026-01-13

### Changed
- Minor helm chart updates and improvements

## [1.0.11] - 2026-01-11

### Changed
- Updated application version to 1.12.0
- Updated all dependent images to latest stable versions

## [1.0.10] - 2025-12-30

### Changed
- Minor chart updates and improvements

## [1.0.9] - 2025-12-05

### Added
- Support for new application feature: language UI selector
- Support for multiple new locales in application layer

### Changed
- Updated helm chart templates for language/locale support

## [1.0.8] - 2025-11-21

### Changed
- Updated Redis image to latest version (8.3.0+)
- Updated all container dependencies to latest stable versions
- Improved image version management

## [1.0.7] - 2025-10-07

### Security
- Updated Redis image version to 8.2.2 for security improvements

## [1.0.6] - 2025-09-02

### Changed
- Minor helm chart updates and improvements

## [1.0.5] - 2025-07-21

### Changed
- Updated chart configuration options

## [1.0.4] - 2025-07-21

### Added
- Helm chart image configuration for easier version management

### Changed
- Updated application version to 1.10.1
- Optimized chart image references

## [1.0.3] - 2025-07-19

### Added
- Support for IP whitelist configuration in Kubernetes deployment
- Ability to configure per-IP message and file size limits

### Changed
- Updated helm chart templates to support IP whitelist feature
- Enhanced configmap for IP limit settings

## [1.0.2] - 2025-07-09

### Security
- Updated Redis image version to 8.0.3 (critical CVE/RCE fix)

## [1.0.1] - 2025-06-19

### Changed
- Updated application version to 1.9.2
- Minor chart improvements

## [1.0.0] - 2025-06-18

### Added
- Official stable release of PW Helm chart
- Complete Kubernetes deployment support for PW application
- Redis backend with authentication
- Configurable TTL and download policies
- File upload support
- Ingress configuration for nginx ingress controller
- Service account and RBAC support
- Liveness and readiness probes
- Resource requests and limits configuration
- Security context for hardened container execution
- Environment variable configuration for all PW settings
- IP whitelist support for access control

### Documentation
- Comprehensive README with installation and configuration instructions
- Parameter documentation for all configurable options
- Examples for common deployment scenarios
- Troubleshooting guide

## [0.1.0] - 2025-06-05

### Added
- Initial Helm chart for PW application
- Basic Kubernetes deployment support
- Redis support
- Configuration templates
- Artifact Hub repository registration

[Unreleased]: https://github.com/lebe-dev/pw/compare/v1.5.0...HEAD
[1.5.0]: https://github.com/lebe-dev/pw/releases/tag/v1.5.0
[1.4.1]: https://github.com/lebe-dev/pw/releases/tag/v1.4.1
[1.4.0]: https://github.com/lebe-dev/pw/releases/tag/v1.4.0
[1.3.1]: https://github.com/lebe-dev/pw/releases/tag/v1.3.1
[1.3.0]: https://github.com/lebe-dev/pw/releases/tag/v1.3.0
[1.1.0]: https://github.com/lebe-dev/pw/releases/tag/v1.1.0
[1.0.11]: https://github.com/lebe-dev/pw/releases/tag/v1.0.11
[1.0.10]: https://github.com/lebe-dev/pw/releases/tag/v1.0.10
[1.0.9]: https://github.com/lebe-dev/pw/releases/tag/v1.0.9
[1.0.8]: https://github.com/lebe-dev/pw/releases/tag/v1.0.8
[1.0.7]: https://github.com/lebe-dev/pw/releases/tag/v1.0.7
[1.0.6]: https://github.com/lebe-dev/pw/releases/tag/v1.0.6
[1.0.5]: https://github.com/lebe-dev/pw/releases/tag/v1.0.5
[1.0.4]: https://github.com/lebe-dev/pw/releases/tag/v1.0.4
[1.0.3]: https://github.com/lebe-dev/pw/releases/tag/v1.0.3
[1.0.2]: https://github.com/lebe-dev/pw/releases/tag/v1.0.2
[1.0.1]: https://github.com/lebe-dev/pw/releases/tag/v1.0.1
[1.0.0]: https://github.com/lebe-dev/pw/releases/tag/v1.0.0
[0.1.0]: https://github.com/lebe-dev/pw/releases/tag/v0.1.0
