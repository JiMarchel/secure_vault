# Vault API

## Create Vault

Endpoint : POST api/vault/

Headers :

- Cookie: sv_at=<access_token>

Request Body :

```json
{
  "title": "string",
  "itemType": "Password" | "CreditCard" | "Note" | "Contact",
  "encryptedData": "string",
  "nonce": "string"
}
```

Response Body Success :

```json
{
  "data": null,
  "message": "Successfully create new vault"
}
```

## Get All Vaults

Endpoint : GET api/vault/all

Headers :

- Cookie: sv_at=<access_token>

Response Body Success :

```json
{
  "data": [
    {
      "id": "uuid",
      "userId": "uuid",
      "title": "string",
      "itemType": "Password" | "CreditCard" | "Note" | "Contact",
      "encryptedData": "string",
      "nonce": "string",
      "createdAt": "timestamp",
      "updatedAt": "timestamp"
    }
  ],
  "message": "Successfully get all vaults"
}
```

## Search Vaults

Endpoint : GET api/vault/search

Headers :

- Cookie: sv_at=<access_token>

Query Parameters :

- title: string

Response Body Success :

```json
{
  "data": [
    {
      "id": "uuid",
      "userId": "uuid",
      "title": "string",
      "itemType": "string",
      "encryptedData": "string",
      "nonce": "string",
      "createdAt": "timestamp",
      "updatedAt": "timestamp"
    }
  ],
  "message": "Successfully search vault"
}
```

## Update Vault

Endpoint : PUT api/vault/

Headers :

- Cookie: sv_at=<access_token>

Request Body :

```json
{
  "id": "uuid",
  "title": "string",
  "itemType": "Password" | "CreditCard" | "Note" | "Contact",
  "encryptedData": "string",
  "nonce": "string"
}
```

Response Body Success :

```json
{
  "data": null,
  "message": "Successfully update vault"
}
```

## Delete Vault

Endpoint : DELETE api/vault/{id}

Headers :

- Cookie: sv_at=<access_token>

Path Parameters :

- id: uuid

Response Body Success :

```json
{
  "data": null,
  "message": "Successfully delete vault"
}
```
