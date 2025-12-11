export interface WasmModule {
  default: () => Promise<void>;
  encrypt_user_identifier: (masterPassword: string) => string;
  decrypt_user_identifier: (masterPassword: string, vaultData: string) => string;
  encrypt_vault_item: (dek: string, plaintext: string) => string;
  decrypt_vault_item: (dek: string, vaultItemJson: string) => string;
}

export interface UserIdentifier {
  encryptedDek: string;
  nonce: string;
  salt: string;
  argon2Params: string;
}

export interface LoginData {
  dek: string;
}

export interface VaultItem {
  encryptedData: string;
  nonce: string;
}

export interface DecryptedVaultItem {
  plaintext: string;
}

export interface WasmResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}