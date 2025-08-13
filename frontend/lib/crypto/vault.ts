import init, {
  create_vault_wasm,
  InitOutput,
} from "../../vault_wasm/pkg/vault_wasm";

let wasmReady: Promise<InitOutput> | null = null;

export async function ensureWasmReady() {
  if (!wasmReady) {
    wasmReady = init();
  }
  await wasmReady;
}

export async function createVault(masterPassword: string) {
  await ensureWasmReady();
  const json = create_vault_wasm(masterPassword);
  return JSON.parse(json) as {
    encrypted_dek: string;
    nonce: string;
    salt: string;
    argon2_params: string;
  };
}
