# Backend API

## 1. Store secret

- URL: `/api/secret`
- Method: `POST`

Request body:

```json
{
  "id": "string",
  "contentType": "Text" | "File",
  "metadata": {
    "name": "string",
    "type": "string",
    "size": 0
  },
  "payload": "encrypted-data-in-base64",
  "ttl": "OneHour" | "TwoHours" | "OneDay" | "OneWeek",
  "downloadPolicy": "OneTime" | "Unlimited"
}
```

Response:
- `200 OK` - secret stored successfully
- `400 Bad Request` - invalid request (e.g., file upload disabled when content type is File)
- `500 Internal Server Error` - storage error

## 2. Retrieve secret

- URL: `/api/secret/{id}`
- Method: `GET`

Response body (on success):

```json
{
  "id": "string",
  "contentType": "Text" | "File",
  "metadata": {
    "name": "string",
    "type": "string",
    "size": 0
  },
  "payload": "encrypted-data-in-base64",
  "ttl": "OneHour" | "TwoHours" | "OneDay" | "OneWeek",
  "downloadPolicy": "OneTime" | "Unlimited"
}
```

Response codes:
- `200 OK` - secret found and returned
- `400 Bad Request` - secret not found by id
- `500 Internal Server Error` - storage error

## 3. Remove secret

- URL: `/api/secret/{id}`
- Method: `DELETE`

Response codes:
- `200 OK` - secret removed successfully
- `500 Internal Server Error` - storage error

## 4. Get app config

- URL: `/api/config`
- Method: `GET`

Response body:

```json
{
  "messageMaxLength": 0,
  "fileUploadEnabled": true,
  "fileMaxSize": 0
}
```

Response codes:
- `200 OK` - config returned

## 5. Get app version

- URL: `/api/version`
- Method: `GET`

Response body: Plain text version string

Response codes:
- `200 OK` - version returned