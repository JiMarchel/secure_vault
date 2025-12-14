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
    "encrypted_dek": "string", // optional
    "salt": "string", // optional
    "argon2_params": "string", // optional
    "is_email_verified": true,
    "nonce": "string", // optional
    "created_at": "timestamp"
  },
  "message": "Current user fetched successfully"
}
```

## Get OTP

Endpoint : GET api/session/otp

Headers :
- Cookie: auth_session = <verif_otp>

Response Body Success :

```json
{
  "data": {
    "otp_code": "string",
    "otp_expires_at": "timestamp"
  },
  "message": "OTP fetched successfully"
}
```

## Get OTP

Endpoint : GET api/session/otp/expire

Headers :
- Cookie: auth_session = <verif_otp>

Response Body Success :

```json
{
  "data": {
    "otp_expires_at": "timestamp"
  },
  "message": "Success get otp expires"
}
```


## Resend OTP

Endpoint : PATCH api/session/otp/resend

Headers :
- Cookie: auth_session = <verif_otp>

Response Body Success :

```json
{
  "data": null,
  "message": "Success resend otp"
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
