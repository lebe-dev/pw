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
$ trivy image tinyops/pw:1.10.2

2025-09-03T10:24:40+03:00       INFO    [vulndb] Need to update DB
2025-09-03T10:24:40+03:00       INFO    [vulndb] Downloading vulnerability DB...
2025-09-03T10:24:40+03:00       INFO    [vulndb] Downloading artifact...        repo="mirror.gcr.io/aquasec/trivy-db:2"
69.87 MiB / 69.87 MiB [-----------------------------------------------------------------------------------] 100.00% 6.90 MiB p/s 10s
2025-09-03T10:24:51+03:00       INFO    [vulndb] Artifact successfully downloaded       repo="mirror.gcr.io/aquasec/trivy-db:2"
2025-09-03T10:24:51+03:00       INFO    [vuln] Vulnerability scanning is enabled
2025-09-03T10:24:51+03:00       INFO    [secret] Secret scanning is enabled
2025-09-03T10:24:51+03:00       INFO    [secret] If your scanning is slow, please try '--scanners vuln' to disable secret scanning
2025-09-03T10:24:51+03:00       INFO    [secret] Please see also https://trivy.dev/v0.63/docs/scanner/secret#recommendation for faster secret detection
2025-09-03T10:24:52+03:00       INFO    Detected OS     family="alpine" version="3.22.1"
2025-09-03T10:24:52+03:00       WARN    This OS version is not on the EOL list  family="alpine" version="3.22"
2025-09-03T10:24:52+03:00       INFO    [alpine] Detecting vulnerabilities...   os_version="3.22" repository="3.22" pkg_num=21
2025-09-03T10:24:52+03:00       INFO    Number of language-specific files       num=0

Report Summary

┌───────────────────────────────────┬────────┬─────────────────┬─────────┐
│              Target               │  Type  │ Vulnerabilities │ Secrets │
├───────────────────────────────────┼────────┼─────────────────┼─────────┤
│ tinyops/pw:1.10.2 (alpine 3.22.1) │ alpine │        0        │    -    │
└───────────────────────────────────┴────────┴─────────────────┴─────────┘
Legend:
- '-': Not scanned
- '0': Clean (no security findings detected)
```

And for redis:

```bash
$ trivy image redis:8.2.1-alpine3.22

2025-09-03T10:26:08+03:00       INFO    [vuln] Vulnerability scanning is enabled
2025-09-03T10:26:08+03:00       INFO    [secret] Secret scanning is enabled
2025-09-03T10:26:08+03:00       INFO    [secret] If your scanning is slow, please try '--scanners vuln' to disable secret scanning
2025-09-03T10:26:08+03:00       INFO    [secret] Please see also https://trivy.dev/v0.63/docs/scanner/secret#recommendation for faster secret detection
2025-09-03T10:26:09+03:00       INFO    Detected OS     family="alpine" version="3.22.1"
2025-09-03T10:26:09+03:00       WARN    This OS version is not on the EOL list  family="alpine" version="3.22"
2025-09-03T10:26:09+03:00       INFO    [alpine] Detecting vulnerabilities...   os_version="3.22" repository="3.22" pkg_num=22
2025-09-03T10:26:09+03:00       INFO    Number of language-specific files       num=0

Report Summary

┌────────────────────────────────────────┬────────┬─────────────────┬─────────┐
│                 Target                 │  Type  │ Vulnerabilities │ Secrets │
├────────────────────────────────────────┼────────┼─────────────────┼─────────┤
│ redis:8.2.1-alpine3.22 (alpine 3.22.1) │ alpine │        0        │    -    │
└────────────────────────────────────────┴────────┴─────────────────┴─────────┘
Legend:
- '-': Not scanned
- '0': Clean (no security findings detected)
```
