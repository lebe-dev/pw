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

## Image scan report

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

And for redis:

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
