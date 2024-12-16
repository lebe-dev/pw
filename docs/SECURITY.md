# Security

- Browser generates encryption key 
- Browser encrypt data
- Browser generate and encode unique Secret URL which contains secret id and encryption key
- Backend stores ONLY encrypted data
- Backend doesn't use disk to store data
- Backend do cleanup for in-memory storage:
  - Each time when someone retrieve secret
  - By schedule (interval depends on option `secrets-cleanup-schedule`)

## Decryption

- User visits secret URL and send secret id to backend
- Backend returns encrypted data to client side (browser)
- Browser decrypt data and shows to end user

## Docker

- App runs with minimal privileges, with user `pw`
