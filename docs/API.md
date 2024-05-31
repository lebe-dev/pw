# Backend API

## 1. Store secret

- URL: `/api/secret`
- Method: `POST`

Body:

```json
{
  "payload": "encrypted-data-in-base64",
  "ttl-times": 0,
  "ttl-unixtime": 0
}
```

Response body:

```json
{
  "url": "https://your-app.domain.com/secrets/SECRET-ID"
}
```

**Errors:**

- `400 Bad Request` - invalid request

## 2. Retrieve secret

- URL: `/api/secret/{secret-id}`
- Method: `GET`

**Errors:**

- `400 Bad Request` - secret wasn't found by id or invalid request

## 3. Remove secret

- URL: `/api/secret/{secret-id}`
- Method: `DELETE`

## 4. Get app config

- URL: `/api/config`
- Method: `GET`

## 4. Get app version

- URL: `/api/version`
- Method: `GET`