export interface UserResponse{
  id: string;
  username: string;
  email: string;
  encryptedDek: null | Array<number>;
  salt: null | Array<number>;
  argon2Params: null | Array<string>;
  isEmailVerified: boolean;
  nonce: null | Array<number>;
  createdAt: string;
};

export interface CheckSessionResponse {
  authenticated: boolean;
}

export interface OtpResponse {
  otpCode: string;
  otpExpiresAt: string;
}