# Security

This document provides an overview of the security architecture. For detailed information, see the specific documentation:

- **[Encryption & Zero-Knowledge Architecture](security/encryption-zero-knowledge.md)** - How data is encrypted client-side and why the backend cannot decrypt secrets
- **[IP Whitelist](security/ip-whitelist.md)** - Access control and trusted proxy configuration
- **[Container Image Security](security/container-image-security.md)** - Vulnerability scanning results and security scanning procedures

## Security Layers

### 1. Zero-Knowledge Encryption

All encryption happens in the browser, ensuring maximum privacy:

- All encryption happens in the browser
- Backend stores only encrypted data
- Encryption keys never transmitted to server
- Backend operators cannot read secrets

**[Read more →](security/encryption-zero-knowledge.md)**

### 2. Access Control

IP-based access control:

- IP whitelist support
- Per-IP custom limits
- Trusted proxy validation

**[Read more →](security/ip-whitelist.md)**

### 3. Container Security

Regularly scanned images with zero vulnerabilities:

- Regular vulnerability scanning with Trivy
- Alpine-based minimal images
- Zero known vulnerabilities

**[Read more →](security/container-image-security.md)**

## Quick Start

### For Users

Secrets are encrypted in your browser before transmission. The server cannot decrypt your data. Share the generated URL securely.

**Key points:**
- Your secret is encrypted before leaving your browser
- The server never sees unencrypted data
- Share Secret URLs only through secure channels
- Secrets expire automatically (max 7 days)

### For Administrators

1. Review [encryption architecture](security/encryption-zero-knowledge.md) to understand zero-knowledge model
2. Configure [IP whitelist](security/ip-whitelist.md) if needed
3. Monitor [container security](security/container-image-security.md) scan results

**Configuration files:**
- Main config: `pw.yml`
- Environment variables: See individual security docs

## Security Contact

If you discover a security vulnerability, please report it to the project maintainers. Do not create public GitHub issues for security vulnerabilities.
