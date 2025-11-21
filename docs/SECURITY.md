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
$ trivy image tinyops/pw:1.10.3
2025-11-21T16:27:41+03:00	INFO	[vulndb] Need to update DB
2025-11-21T16:27:41+03:00	INFO	[vulndb] Downloading vulnerability DB...
2025-11-21T16:27:41+03:00	INFO	[vulndb] Downloading artifact...	repo="mirror.gcr.io/aquasec/trivy-db:2"
76.00 MiB / 76.00 MiB [-------------------------------------------------------------------------------------------------------------] 100.00% 3.82 MiB p/s 20s
2025-11-21T16:28:03+03:00	INFO	[vulndb] Artifact successfully downloaded	repo="mirror.gcr.io/aquasec/trivy-db:2"
2025-11-21T16:28:03+03:00	INFO	[vuln] Vulnerability scanning is enabled
2025-11-21T16:28:03+03:00	INFO	[secret] Secret scanning is enabled
2025-11-21T16:28:03+03:00	INFO	[secret] If your scanning is slow, please try '--scanners vuln' to disable secret scanning
2025-11-21T16:28:03+03:00	INFO	[secret] Please see https://trivy.dev/v0.67/docs/scanner/secret#recommendation for faster secret detection
2025-11-21T16:28:03+03:00	INFO	Detected OS	family="alpine" version="3.22.2"
2025-11-21T16:28:03+03:00	INFO	[alpine] Detecting vulnerabilities...	os_version="3.22" repository="3.22" pkg_num=21
2025-11-21T16:28:03+03:00	INFO	Number of language-specific files	num=0

Report Summary

┌───────────────────────────────────┬────────┬─────────────────┬─────────┐
│              Target               │  Type  │ Vulnerabilities │ Secrets │
├───────────────────────────────────┼────────┼─────────────────┼─────────┤
│ tinyops/pw:1.10.3 (alpine 3.22.2) │ alpine │        0        │    -    │
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
