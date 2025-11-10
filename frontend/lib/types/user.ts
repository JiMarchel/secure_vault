export type UserType = {
  id: string;
  username: string;
  email: string;
  encrypted_dek: null | Array<number>;
  salt: null | Array<number>;
  argon2_params: null | Array<string>;
  is_email_verified: boolean;
  nonce: null | Array<number>;
  created_at: string;
};