import {
  decryptUserIdentifier,
} from "~/lib/wasm/vault";
import type { SuccessResponse } from "~/utils/model/response";
import { AuthError } from "~/lib/errors";
import type { Identifier, loginType } from "~/utils/model/auth";
import { toast } from "vue-sonner";
import { errorHelper } from "~/lib/error-helper";

interface UserInfo {
  id: string;
  email: string;
  username: string;
}

interface AuthTokens {
  accessToken: string;
  refreshToken: string;
}

// DEK disimpan di closure, TIDAK reactive, TIDAK di localStorage
// Ini memastikan DEK tidak terekspos di Vue DevTools
let _dek: string | null = null;

function setDek(dek: string) {
  _dek = dek;
}

function getDek(): string | null {
  return _dek;
}

function clearDek() {
  _dek = null;
}

export function useAuth() {
  const config = useRuntimeConfig();

  const user = useState<UserInfo | null>("auth:user", () => null);
  const isAuthenticated = useState<boolean>(
    "auth:isAuthenticated",
    () => false
  );
  const isLoading = useState<boolean>("auth:isLoading", () => false);

  async function login(credentials: loginType): Promise<void> {
    isLoading.value = true;
    try {
      const identifierResponse = await $fetch<
        SuccessResponse<Identifier>
      >(`${config.public.apiBaseUrl}/user/identifier`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: { email: credentials.email },
      });

      if (!identifierResponse.data) {
        throw new AuthError("Wrong email or password");
      }

      const dekResult = await decryptUserIdentifier(
        credentials.password,
        identifierResponse.data
      );

      setDek(dekResult.dek);

      const authResponse = await $fetch<SuccessResponse<UserInfo>>(
        `${config.public.apiBaseUrl}/auth/login`,
        {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: {
            email: credentials.email,
          },
          credentials: "include", // Untuk httpOnly cookies
        }
      );

      if (!authResponse.data) {
        throw new AuthError("Authentication failed");
      }

      user.value = authResponse.data;
      isAuthenticated.value = true;

      await navigateTo("/dashboard");
    } finally {
      isLoading.value = false;
    }
  }

  async function logout(): Promise<void> {
    try {
      await $fetch(`${config.public.apiBaseUrl}/auth/logout`, {
        method: "DELETE",
        credentials: "include",
      });

      toast.success("Successfully logged out");
    } catch (error) {
      await errorHelper(error);
    } finally {
      clearDek();
      user.value = null;
      isAuthenticated.value = false;

      await navigateTo("/");
    }
  }

  // ============================================================================
  // Token Refresh (dipanggil otomatis oleh interceptor)
  // ============================================================================

  async function refreshAccessToken(): Promise<boolean> {
    try {
      await $fetch<SuccessResponse<AuthTokens>>(
        `${config.public.apiBaseUrl}/auth/refresh`,
        {
          method: "POST",
          credentials: "include", // Kirim httpOnly refresh token cookie
        }
      );
      return true;
    } catch {
      // Refresh gagal, force logout
      await logout();
      return false;
    }
  }

  // ============================================================================
  // Check Auth Status (untuk middleware/initial load)
  // ============================================================================

  async function checkAuth(): Promise<boolean> {
    if (isAuthenticated.value && user.value) {
      return true;
    }

    try {
      const response = await $fetch<SuccessResponse<UserInfo>>(
        `${config.public.apiBaseUrl}/auth/me`,
        {
          credentials: "include",
        }
      );

      if (response.data) {
        user.value = response.data;
        isAuthenticated.value = true;
        return true;
      }
    } catch {
      // Not authenticated
    }

    return false;
  }

  // ============================================================================
  // Vault Operations (menggunakan DEK)
  // ============================================================================

  function hasDek(): boolean {
    return _dek !== null;
  }

  // Expose DEK getter untuk vault operations
  // PENTING: Hanya gunakan untuk encrypt/decrypt, jangan expose ke UI
  function useDek(): string {
    const dek = getDek();
    if (!dek) {
      throw new AuthError("DEK not available. Please login again.", {
        status: 403,
      });
    }
    return dek;
  }

  // ============================================================================
  // Return
  // ============================================================================

  return {
    // State (readonly untuk komponen)
    user: readonly(user),
    isAuthenticated: readonly(isAuthenticated),
    isLoading: readonly(isLoading),

    // Actions
    login,
    logout,
    refreshAccessToken,
    checkAuth,

    // DEK operations
    hasDek,
    useDek,

  };
}
