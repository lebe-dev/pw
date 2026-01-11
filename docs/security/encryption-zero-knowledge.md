# Encryption & Zero-Knowledge Architecture

## Overview

This application implements a [zero-knowledge architecture](https://en.wikipedia.org/wiki/Zero-knowledge_proof), meaning the backend server never has access to unencrypted data. All encryption and decryption happens in your browser, ensuring that:
- Backend operators cannot read secrets
- Network traffic only contains encrypted data
- Encryption keys never reach the server

## How Encryption Works

### 1. Secret Creation

When you create a secret:

1. **Browser generates encryption key** OR you provide your own password
2. **Browser encrypts data** (text or file) using AES-256-GCM encryption
3. **Browser generates unique Secret URL** containing:
   - **Secret ID**: Used to retrieve encrypted data from backend
   - **Encryption key**: Embedded in URL fragment (after `#`)
4. **Backend stores ONLY encrypted data** - it cannot decrypt the content

### 2. Storage Security

- Backend stores encrypted data **in memory only** (Redis)
- **No disk storage** is used for secrets
- Backend **cannot decrypt secrets** (zero-knowledge principle)
- Each secret has a **limited lifetime** (TTL: 1 hour to 7 days)

### 3. Data Cleanup

Secrets are automatically removed through:

- **Immediate cleanup**: When someone retrieves the secret (for one-time secrets)
- **TTL-based cleanup**: Redis automatically expires secrets after configured time
- **Download policy**: 
  - **OneTime**: Secret deleted after first retrieval
  - **Unlimited**: Secret remains until TTL expires

## How Decryption Works

### 1. User Opens Secret URL

When someone opens a secret URL:

1. Browser **extracts** secret ID and encryption key from URL
2. Browser sends **only the secret ID** to backend (not the encryption key)
3. Encryption key **stays in the browser**

### 2. Backend Returns Encrypted Data

- Backend returns the **encrypted payload** for the given secret ID
- Backend does **NOT have the encryption key**
- If secret doesn't exist or expired: returns `400 Bad Request`

### 3. Browser Decrypts

1. Browser **decrypts data** using the encryption key from URL
2. Decrypted content is **shown to user**
3. Data is **never transmitted unencrypted** over the network

## Security Guarantees

### Zero-Knowledge Design

- **Server operators cannot read secrets**: The backend only stores encrypted data
- **Network security**: All network traffic contains only encrypted payloads
- **Client-side encryption**: Encryption keys never leave your browser (except embedded in the URL you share)

### What the Backend Can See

The backend can only see:
- Encrypted data (unreadable ciphertext)
- Secret ID (random identifier)
- Metadata (content type, size, TTL settings)
- IP address and access patterns

The backend **cannot** see:
- Original secret content
- Encryption keys
- Decrypted data

## Technical Details

### Encryption Algorithm

- **Algorithm**: AES-256-GCM (Galois/Counter Mode)
- **Key derivation**: Browser-generated random keys or PBKDF2 for passwords
- **Authenticated encryption**: Prevents tampering and ensures integrity

### Storage Backend

- **Technology**: Redis (in-memory key-value store)
- **Persistence**: None (data lost on restart by design)
- **TTL**: Native Redis expiration for automatic cleanup

## References

- Architecture documentation: `docs/ARCHITECTURE.md`
- API documentation: `docs/API.md`
- Frontend encryption: Browser's Web Crypto API
- Backend storage: `src/secret/storage.rs`
