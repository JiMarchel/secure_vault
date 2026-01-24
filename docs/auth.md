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

## Login User

Endpoint : POST api/auth/login

Request Body :

```json
{
  "email": "string",
  "authVerifier": "string"
}
```

Response Body Success :

```json
{
  "data": {
    "id": "uuid",
    "username": "string",
    "email": "string"
  },
  "message": "Login success"
}
```

## Logout User

Endpoint : DELETE api/auth/logout

Headers :

- Cookie: sv_at=<token>; sv_rt=<token>

Response Body Success :

```json
{
  "data": null,
  "message": "Logged out successfully"
}
```

## Refresh Token

Endpoint : POST api/auth/refresh

Headers :

- Cookie: sv_rt=<token>

Response Body Success :

```json
{
  "data": {
    "accessToken": "string",
    "refreshToken": "string"
  },
  "message": "Token refreshed"
}
```

## Get Current User

Endpoint : GET api/auth/me

Headers :

- Cookie: sv_at=<token>

Response Body Success :

```json
{
  "data": {
    "id": "uuid",
    "username": "string",
    "email": "string"
  },
  "message": "User retrieved successfully"
}
```

## Verify OTP

Endpoint : PATCH api/auth/verif/otp

Request Body :

```json
{
  "otpCode": "string"
}
```

Response Body Success :

```json
{
  "data": null,
  "message": "Verified Otp success!"
}
```

## Update User Identifier

Endpoint : PATCH api/auth/verif/identifier

Request Body :

```json
{
  "encryptedDek": "string",
  "salt": "string",
  "nonce": "string",
  "argon2Params": "string",
  "authVerifier": "string"
}
```

Response Body Success :

```json
{
  "data": null,
  "message": "User identifier updated"
}
```

## Report Failed Attempt

Endpoint : POST api/auth/report-failed

Request Body :

```json
{
  "email": "string"
}
```

Response Body Success :

```json
{
  "data": null,
  "message": ""
}
```

## Unlock Account

Endpoint : POST api/auth/unlock-account

Request Body :

```json
{
  "token": "string"
}
```

Response Body Success :

```json
{
  "data": null,
  "message": "Account ... unlocked successfully"
}
```
