# User API

## Get User By Email

Endpoint : GET api/user/by-email

Request Body :

```json
{
  "email": "string"
}
```

Response Body Success :

```json
{
  "data": {
    "id": "uuid",
    "username": "string",
    "email": "string",
    "encrypted_dek": "string", // optional
    "salt": "string", // optional
    "argon2_params": "string", // optional
    "is_email_verified": true,
    "nonce": "string", // optional
    "created_at": "timestamp"
  },
  "message": "User fetched successfully"
}
```
## POST User Identifier

Endpoint : POST api/user/identifier

Request Body :

```json
{
  "email": "string"
}
```

Response Body Success :

```json
{
  "data": {
    "encrypted_dek": "string",
    "salt": "string",
    "argon2_params": "string",
    "nonce": "string"
  },
  "message": "User identifier fetched successfully"
}
```