export interface UserResponse {
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
