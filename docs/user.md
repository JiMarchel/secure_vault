# User API

## Get User By Email

Endpoint : POST /user/get-user-by-email

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
