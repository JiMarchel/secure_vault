export type User = {
  id: string;
  username: string;
  email: string;
  encryptedDek?: string;
  salt?: string;
  argon2Params?: string;
  isEmailVerified: boolean;
  nonce?: string;
  createdAt: string;
}

export type Otp = {
  otpCode: string;
  otpExpiresAt: string
}
