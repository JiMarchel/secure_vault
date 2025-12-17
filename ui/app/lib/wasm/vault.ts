import type {
  DecryptedVaultItem,
  LoginData,
  UserIdentifier,
  VaultItem,
  WasmModule,
  WasmResponse,
} from './type'

let wasmModule: WasmModule | null = null

async function initWasm(): Promise<WasmModule> {
  if (wasmModule) {
    return wasmModule
  }

  try {
    // Dynamic import with type casting
    const module =
      (await import('../../../enc_dec/pkg/enc_dec.js')) as unknown as WasmModule

    // Initialize WASM
    await module.default()

    wasmModule = module

    return wasmModule
  } catch (error) {
    console.error('Failed to load WASM:', error)
    throw new Error(`WASM initialization failed: ${error}`)
  }
}

function parseWasmResponse<T>(json: string): T {
  const response = JSON.parse(json) as WasmResponse<T>

  if (!response.success) {
    throw new Error(response.error || 'WASM operation failed')
  }

  if (!response.data) {
    throw new Error('WASM response missing data')
  }

  return response.data
}

export async function encryptUserIdentifier(
  masterPassword: string,
): Promise<UserIdentifier> {
  const wasm = await initWasm()
  const json = wasm.encrypt_user_identifier(masterPassword)
  return parseWasmResponse<UserIdentifier>(json)
}

export async function decryptUserIdentifier(
  masterPassword: string,
  vaultData: UserIdentifier,
): Promise<LoginData> {
  const wasm = await initWasm()
  const json = wasm.decrypt_user_identifier(
    masterPassword,
    JSON.stringify(vaultData),
  )
  return parseWasmResponse<LoginData>(json)
}

export async function encryptVaultItem(
  dek: string,
  plaintext: string,
): Promise<VaultItem> {
  const wasm = await initWasm()
  const json = wasm.encrypt_vault_item(dek, plaintext)
  return parseWasmResponse<VaultItem>(json)
}

export async function decryptVaultItem(
  dek: string,
  vaultItem: VaultItem,
): Promise<DecryptedVaultItem> {
  const wasm = await initWasm()
  const json = wasm.decrypt_vault_item(dek, JSON.stringify(vaultItem))
  return parseWasmResponse<DecryptedVaultItem>(json)
}
