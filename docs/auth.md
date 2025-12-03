# Auth API

## Register User

Endpoint : POST api/auth/

Request Body :

```json
{
  "username": "string",
  "email": "string"
}
```

Response Body Success :

```json
{
  "data": {
    "username": "string",
    "email": "string"
  },
  "message": "created"
}
```

## Verify OTP

Endpoint : PATCH api/auth/verif/otp

Request Body :

```json
{
  "otp_code": "string"
}
```

Response Body Success :

```json
{
  "data": null,
  "message": "email_verified"
}
```

## Update User Identifier

Endpoint : PATCH api/auth/verif/identifier

Request Body :

```json
{
  "encrypted_dek": "string",
  "salt": "string",
  "nonce": "string",
  "argon2_params": "string"
}
```

Response Body Success :

```json
{
  "data": {
    "access_token": "string",
    "refresh_token": "string",
    "token_type": "Bearer",
    "expires_in": 900
  },
  "message": "User identifier updated"
}
```
