export type Identifier = {
  encryptedDek: string;
  nonce: string;
  salt: string;
  argon2Params: string;
}

export type SessionData = {
  access_token: string
  refresh_token: string
  token_type: string
  expires_in: number
}