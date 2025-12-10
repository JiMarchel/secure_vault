let wasmModule: any = null;

async function initWasm() {
  if (wasmModule) {
    return wasmModule;
  }

  try {
    console.log("Loading WASM module...");
    // Import the WASM module
    const module = await import("../../enc_dec/pkg/enc_dec.js");

    // Initialize the WASM module
    await module.default();

    wasmModule = module;

    console.log("WASM module loaded successfully");
    return wasmModule;
  } catch (error) {
    console.error("Failed to load WASM:", error);
    wasmModule = null;
    throw error;
  }
}

export async function encryptUserIdentifier(masterPassword: string) {
  try {
    console.log("Encrypting user identifier...");

    // Load WASM module
    const wasm = await initWasm();

    if (!wasm.encrypt_user_identifier) {
      throw new Error(
        "encrypt_user_identifier function not found in WASM module"
      );
    }

    console.log("Calling encrypt_user_identifier function...");
    const json = wasm.encrypt_user_identifier(masterPassword);
    // console.log("WASM function returned:", json);

    const result = JSON.parse(json) as {
      encryptedDek: string;
      nonce: string;
      salt: string;
      argon2Params: string;
    };

    console.log("User identifier encrypted successfully:", result);
    return result;
  } catch (error) {
    console.error("Failed to encrypt user identifier:", error);
    throw new Error("Failed to encrypt user identifier: " + (error as Error).message);
  }
}
