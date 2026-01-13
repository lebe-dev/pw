# IP Whitelist

## Overview

IP-based access restrictions with custom payload limits. This feature allows you to whitelist specific IP addresses to bypass limits or have custom restrictions.

## IP Whitelist

### Overview

Whitelist specific IP addresses to:
- **Set custom per-IP limits** for message length and file size
- **Allow trusted internal services** unrestricted access

### Supported IP Formats

- **IPv4 exact**: `192.168.1.100`
- **IPv4 CIDR**: `192.168.1.0/24`, `10.0.0.0/8`
- **IPv6 exact**: `2001:db8::1`
- **IPv6 CIDR**: `2001:db8::/32`

### Configuration

**YAML (`pw.yml`):**
```yaml
ip-limits:
  enabled: false  # Disabled by default (opt-in)
  trusted-proxies: []
  whitelist:
    - ip: "192.168.1.100"
      message-max-length: 8192
      file-max-size: 10485760  # 10 MB
    - ip: "10.0.0.0/8"
      # Only override message length, use default for file size
      message-max-length: 16384
    - ip: "203.0.113.0/24"
      # No custom limits, just bypass rate limiting
```

**Environment Variables:**
```bash
PW_IP_LIMITS_ENABLED=true
PW_IP_LIMITS_WHITELIST='[{"ip":"192.168.1.100","message-max-length":8192}]'
PW_IP_LIMITS_TRUSTED_PROXIES='["10.0.0.1","172.16.0.0/12"]'
```

### Per-IP Custom Limits

Configure custom limits for specific IPs:

- **`message-max-length`**: Maximum characters (1 to 65,535)
- **`file-max-size`**: Maximum bytes (up to 10 GB = 10,737,418,240 bytes)
- Both limits are **optional** - override only what you need
- Non-whitelisted IPs use **default application limits**

**Example: VIP user with higher limits:**
```yaml
whitelist:
  - ip: "203.0.113.100"
    message-max-length: 32768     # 32 KB text
    file-max-size: 104857600      # 100 MB files
```

## Trusted Proxies (Critical for Reverse Proxy Setups)

### Security Implications

When your application runs behind a reverse proxy (nginx, Cloudflare, HAProxy, etc.), the backend sees the **proxy's IP**, not the **client's real IP**. The real client IP is passed via HTTP headers like `X-Forwarded-For` or `X-Real-IP`.

**⚠️ Security Risk**: Without validation, attackers can **spoof these headers** to bypass IP restrictions.

### How It Works

The application validates proxy headers based on configuration:

- **`ip-limits.enabled: true` + empty `trusted-proxies`**: Headers **IGNORED** (secure by default)
- **`ip-limits.enabled: true` + configured `trusted-proxies`**: Only requests **from listed proxies** trust headers
- **`ip-limits.enabled: false`**: Headers **trusted** (backward compatible, less secure)

### Header Priority

The application checks headers in this order:

1. **`X-Forwarded-For`** (uses first IP in comma-separated list)
2. **`X-Real-IP`**
3. **Direct connection IP** (fallback)

### Configuration Examples

**Scenario 1: nginx reverse proxy at `10.0.0.1`**

```yaml
ip-limits:
  enabled: true
  trusted-proxies:
    - "10.0.0.1"  # nginx server IP
  whitelist:
    - ip: "203.0.113.10"  # Real client IP (not proxy IP)
```

**Scenario 2: Multiple proxies in CIDR range**

```yaml
ip-limits:
  enabled: true
  trusted-proxies:
    - "10.0.0.0/8"        # Internal proxy network
    - "172.16.0.0/12"     # Another proxy range
  whitelist:
    - ip: "192.168.1.0/24"  # Whitelisted client network
```

**Scenario 3: Cloudflare setup**

```yaml
ip-limits:
  enabled: true
  trusted-proxies:
    # Add all Cloudflare IP ranges
    - "173.245.48.0/20"
    - "103.21.244.0/22"
    - "103.22.200.0/22"
    - "103.31.4.0/22"
    - "141.101.64.0/18"
    - "108.162.192.0/18"
    - "190.93.240.0/20"
    # ... add remaining Cloudflare ranges
  whitelist:
    - ip: "198.51.100.0/24"  # Your corporate network
```

**Scenario 4: Direct connection (no proxy)**

```yaml
ip-limits:
  enabled: true
  trusted-proxies: []  # Empty = secure by default
  whitelist:
    - ip: "192.168.1.100"
```

## Use Cases

### 1. Internal Services Bypass

Whitelist your internal services to bypass rate limits:

```yaml
ip-limits:
  enabled: true
  trusted-proxies: []
  whitelist:
    - ip: "10.0.0.0/8"      # Internal network
    - ip: "172.16.0.0/12"   # Another internal range
```

### 2. VIP Users with Higher Limits

Give specific IPs higher message/file limits:

```yaml
ip-limits:
  enabled: true
  trusted-proxies: []
  whitelist:
    - ip: "203.0.113.100"
      message-max-length: 32768       # 32 KB
      file-max-size: 104857600        # 100 MB
    - ip: "203.0.113.101"
      message-max-length: 65535       # Max allowed
      file-max-size: 1073741824       # 1 GB
```

### 3. Development/Testing Environment

Bypass all limits for local development:

```yaml
ip-limits:
  enabled: true
  trusted-proxies: []
  whitelist:
    - ip: "127.0.0.1"       # localhost
    - ip: "::1"             # localhost IPv6
```

### 4. API Integration Partner

Allow API integration partner unrestricted access:

```yaml
ip-limits:
  enabled: true
  trusted-proxies:
    - "10.0.0.5"  # Your nginx proxy
  whitelist:
    - ip: "198.51.100.50"  # Partner's IP
      message-max-length: 16384
```

## Validation

Configuration is validated at application startup:

- **Invalid IP formats**: Rejected (must be valid IPv4/IPv6 or CIDR)
- **Duplicate IPs**: Detected and rejected
- **File size limits**: Maximum 10 GB (10,737,418,240 bytes)
- **Message length limits**: Maximum 65,535 characters

**Example validation error:**
```
ERROR: Invalid IP format in whitelist: '192.168.1.256/24'
ERROR: Duplicate IP in whitelist: '10.0.0.1'
ERROR: File size exceeds maximum: 20000000000 (max: 10737418240)
```

## Troubleshooting

### Wrong IP Address Detected

1. Check if behind a proxy - configure `trusted-proxies`:
```yaml
ip-limits:
  enabled: true
  trusted-proxies:
    - "your-proxy-ip"
```

2. Verify proxy sends correct headers (`X-Forwarded-For` or `X-Real-IP`)

### Whitelist Not Working

1. Ensure IP limits are enabled:
```yaml
ip-limits:
  enabled: true  # Must be true
```

2. Check IP format is valid (use exact IP or CIDR notation)

3. Verify IP matches what backend sees (check logs)

## References

### Implementation Files

- Client IP extraction: `src/middleware/client_ip.rs`
- Configuration models: `src/config/model.rs`
- Validation: `src/config/validation.rs`
- Limits service: `src/limits/service.rs`

### Related Documentation

- API Documentation: `docs/API.md`
- Architecture: `docs/ARCHITECTURE.md`
- Installation: `docs/install/INSTALL.md`
