# Security

- Browser generates encryption key 
- Browser encrypt data
- Browser generate and encode unique Secret URL which contains secret id and encryption key
- Backend stores ONLY encrypted data

## Decryption

- User visits secret URL and send secret id to backend
- Backend returns encrypted data to client side (browser)
- Browser decrypt data and shows to end user

## Docker

- App runs with minimal privileges, with user `pw`
- App doesn't have write permission inside docker container
- Directory `/app` doesn't have write permissions.