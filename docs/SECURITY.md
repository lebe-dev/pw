# Security

- Browser generates encryption key or you can specify your own password
- Browser encrypt data (text/file)
- Browser generate and encode unique Secret URL which contains secret id and encryption key
- Backend stores ONLY encrypted data
- Backend doesn't use disk to store data
- Backend do cleanup for in-memory storage:
  - Each time when someone retrieve secret
  - By redis TTL schedule
- App runs with [distroless non-root container](https://github.com/GoogleContainerTools/distroless)

## Decryption

- User visits secret URL and send secret id to backend
- Backend returns encrypted data to client side (browser)
- Browser decrypt data and shows to end user
