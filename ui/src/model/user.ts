export type User = {
  id: string
  username: string
  email: string
  encryptedDek?: string
  salt?: string
  argon2Params?: string
  isEmailVerified: boolean
  nonce?: string
  createdAt: string
}

export type otpExpiresAt = {
  otpExpiresAt: string
}

export type Identifier = {
  encryptedDek: string;
  nonce: string;
  salt: string;
  argon2Params: string;
}
