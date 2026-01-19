import { decryptUserIdentifier } from "~/lib/wasm/vault";
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
    () => false,
  );
  const isLoading = useState<boolean>("auth:isLoading", () => false);

  async function login(credentials: loginType): Promise<void> {
    isLoading.value = true;
    try {
      const identifierResponse = await $fetch<SuccessResponse<Identifier>>(
        `${config.public.apiBaseUrl}/user/identifier`,
        {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: { email: credentials.email },
        },
      );

      if (!identifierResponse.data) {
        throw new AuthError("Wrong email or password");
      }

      let dekResult;
      try {
        dekResult = await decryptUserIdentifier(
          credentials.password,
          identifierResponse.data,
        );
      } catch (decryptError) {
        try {
          await $fetch(`${config.public.apiBaseUrl}/auth/report-failed`, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: { email: credentials.email },
          });
          throw new AuthError("Wrong email or password");
        } catch (reportError: unknown) {
          throw reportError;
        }
      }

      setDek(dekResult.dek);

      const authResponse = await $fetch<SuccessResponse<UserInfo>>(
        `${config.public.apiBaseUrl}/auth/login`,
        {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: {
            email: credentials.email,
            authVerifier: dekResult.authVerifier,
          },
          credentials: "include",
        },
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

  async function logout(silent = false): Promise<void> {
    try {
      await $fetch(`${config.public.apiBaseUrl}/auth/logout`, {
        method: "DELETE",
        credentials: "include",
      });

      if (!silent) {
        toast.success("Successfully logged out");
      }
    } catch (error) {
      if (!silent) {
        await errorHelper(error);
      }
    } finally {
      clearDek();
      user.value = null;
      isAuthenticated.value = false;

      await navigateTo("/");
    }
  }

  async function checkAuth(): Promise<boolean> {
    if (isAuthenticated.value && user.value) {
      return true;
    }

    try {
      const { $api } = useNuxtApp();
      const response = await $api<SuccessResponse<UserInfo>>("/auth/me");

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

  function hasDek(): boolean {
    return _dek !== null;
  }

  function useDek(): string {
    const dek = getDek();
    if (!dek) {
      throw new AuthError("DEK not available. Please login again.", {
        status: 403,
      });
    }
    return dek;
  }

  return {
    user: readonly(user),
    isAuthenticated: readonly(isAuthenticated),
    isLoading: readonly(isLoading),

    login,
    logout,
    checkAuth,

    hasDek,
    useDek,
  };
}
