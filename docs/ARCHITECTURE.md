# Backend architecture

## 1. Save secret

- Backend application [stores secrets in memory cache](https://github.com/lebe-dev/pw/blob/main/backend/src/secret/storage.rs) (Redis) with limited lifespan (TTL)
- Frontend app:
  - Encrypt secret data
  - Generate secret ID
  - Send to backend ID and encrypted secret
  - Generate encoded URL with secret ID and decryption key and shows to the end user

## 2. Retrieve secret by URL

- User open URL
- Frontend decodes URL slug part and extract:
  - Secret ID
  - Encryption Key
- Frontend sends secret ID to backend
- Backend returns encrypted payload for given Secret ID or return `400 Bad Request`
- Frontend decrypt data and shows to user

## 3. How TTL works

- Backend store secrets inside [in-memory key-value storage](https://github.com/lebe-dev/pw/blob/main/backend/src/secret/storage.rs) (Redis) with limited lifetime. Max lifetime is one day.
- User can restrict how many times URL can be used 