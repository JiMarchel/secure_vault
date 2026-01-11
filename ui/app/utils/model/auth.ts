import type z from "zod";
import type { login, signUp } from "../validation/auth";

export type Identifier = {
  encryptedDek: string;
  nonce: string;
  salt: string;
  argon2Params: string;
  authVerifier: string;
};

export type SessionData = {
  access_token: string;
  refresh_token: string;
  token_type: string;
  expires_in: number;
};

export type signUpType = z.infer<typeof signUp>;

export type loginType = z.infer<typeof login>;
