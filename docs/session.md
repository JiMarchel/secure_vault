# Session API

## Get Current User

Endpoint : GET api/session/me

Headers :

- Cookie: auth_session = <verif_otp | verif_password>

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
  "message": "Current user fetched successfully"
}
```

## Get OTP Status

Endpoint : GET api/session/otp/status

Headers :

- Cookie: auth_session = <verif_otp>

Response Body Success :

```json
{
  "data": {
    "hasOtp": true, // boolean
    "expiresAt": "timestamp", // nullable
    "canResend": true, // boolean
    "resendAfter": 60 // nullable (seconds)
  },
  "message": "OTP status retrieved successfully"
}
```

## Resend OTP

Endpoint : PATCH api/session/otp/resend

Headers :

- Cookie: auth_session = <verif_otp>

Response Body Success :

```json
{
  "data": {
    "success": true,
    "cooldownSeconds": 60
  },
  "message": "OTP resent successfully"
}
```

## Verify OTP

Endpoint : POST api/session/otp/verify

Headers :

- Cookie: auth_session = <verif_otp>

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
  "message": "OTP verified successfully"
}
```

## Check Session

Endpoint : GET api/session/check

Headers :

- Cookie: auth_session = <verif_otp | verif_password>

Response Body Success :

```json
{
  "data": {
    "authenticated": false,
    "state": "verif_otp" || "verif_password"
  },
  "message": "Session checked"
}
```
