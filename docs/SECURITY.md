# Security

- Browser generates encryption key or you can specify your own password
- Browser encrypt data (text/file)
- Browser generate and encode unique Secret URL which contains secret id and encryption key
- Backend stores ONLY encrypted data
- Backend doesn't use disk to store data
- Backend do cleanup for in-memory storage:
  - Each time when someone retrieve secret
  - By redis TTL schedule

## Decryption

- User visits secret URL and send secret id to backend
- Backend returns encrypted data to client side (browser)
- Browser decrypt data and shows to end user

## Why not distroless?

Because it has [issue with libc6](https://github.com/GoogleContainerTools/distroless/issues/1808).

```bash
trivy image tinyops/pw:1.9.0
2025-06-18T22:20:16+03:00       INFO    [vuln] Vulnerability scanning is enabled
2025-06-18T22:20:16+03:00       INFO    [secret] Secret scanning is enabled
2025-06-18T22:20:16+03:00       INFO    [secret] If your scanning is slow, please try '--scanners vuln' to disable secret scanning
2025-06-18T22:20:16+03:00       INFO    [secret] Please see also https://trivy.dev/v0.63/docs/scanner/secret#recommendation for faster secret detection
2025-06-18T22:20:17+03:00       INFO    Detected OS     family="alpine" version="3.21.3"
2025-06-18T22:20:17+03:00       INFO    [alpine] Detecting vulnerabilities...   os_version="3.21" repository="3.21" pkg_num=20
2025-06-18T22:20:17+03:00       INFO    Number of language-specific files       num=0

Report Summary

┌──────────────────────────────────┬────────┬─────────────────┬─────────┐
│              Target              │  Type  │ Vulnerabilities │ Secrets │
├──────────────────────────────────┼────────┼─────────────────┼─────────┤
│ tinyops/pw:1.9.0 (alpine 3.21.3) │ alpine │        0        │    -    │
└──────────────────────────────────┴────────┴─────────────────┴─────────┘
Legend:
- '-': Not scanned
- '0': Clean (no security findings detected)
```
