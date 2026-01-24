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
    "encryptedDek": "string", // optional
    "salt": "string", // optional
    "argon2Params": "string", // optional
    "isEmailVerified": true,
    "nonce": "string", // optional
    "authVerifier": "string", // optional
    "createdAt": "timestamp"
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
    "encryptedDek": "string",
    "salt": "string",
    "argon2Params": "string",
    "nonce": "string",
    "authVerifier": "string"
  },
  "message": "User identifier fetched successfully"
}
```
