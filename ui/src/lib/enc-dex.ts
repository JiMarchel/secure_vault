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

export async function createUserIdentifier(masterPassword: string) {
  try {
    console.log("Creating user identifier...");

    // Load WASM module
    const wasm = await initWasm();

    if (!wasm.create_user_identifier) {
      throw new Error(
        "create_user_identifier function not found in WASM module"
      );
    }

    console.log("Calling create_user_identifier function...");
    const json = wasm.create_user_identifier(masterPassword);
    // console.log("WASM function returned:", json);

    const result = JSON.parse(json) as {
      encryptedDek: string;
      nonce: string;
      salt: string;
      argon2Params: string;
    };

    console.log("Vault created successfully:", result);
    return result;
  } catch (error) {
    console.error("Failed to create vault:", error);
    throw new Error("Failed to create vault: " + (error as Error).message);
  }
}
