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

const DEK_STORAGE_KEY = "app:dek";

function setDek(dek: string) {
  if (import.meta.client) {
    sessionStorage.setItem(DEK_STORAGE_KEY, dek);
  }
}

function getDek(): string | null {
  if (import.meta.client) {
    return sessionStorage.getItem(DEK_STORAGE_KEY);
  }
  return null;
}

function clearDek() {
  if (import.meta.client) {
    sessionStorage.removeItem(DEK_STORAGE_KEY);
  }
}

export function useAuth() {
  const config = useRuntimeConfig();

  const user = useState<UserInfo | null>("auth:user", () => null);
  const isAuthenticated = useState<boolean>(
    "auth:isAuthenticated",
    () => false,
  );
  const isLoading = useState<boolean>("auth:isLoading", () => false);
  const needsUnlock = useState<boolean>("auth:needsUnlock", () => false);

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
      needsUnlock.value = false;

      await navigateTo("/dashboard");
    } finally {
      isLoading.value = false;
    }
  }

  async function unlockVault(password: string): Promise<void> {
    const { $api } = useNuxtApp();

    try {
      const identifierResponse = await $api<SuccessResponse<Identifier>>(
        `${config.public.apiBaseUrl}/user/identifier`,
        {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: { email: user?.value?.email },
        },
      );

      if (!identifierResponse.data) {
        throw new AuthError("Failed to get vault data");
      }

      const dekResult = await decryptUserIdentifier(
        password,
        identifierResponse.data,
      );

      setDek(dekResult.dek);
      needsUnlock.value = false;

      toast.success("Vault unlocked successfully");
    } catch (error) {
      try {
        await $api(`${config.public.apiBaseUrl}/auth/report-failed`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: { email: user?.value?.email },
        });
      } catch {}
      await errorHelper(error);
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
      needsUnlock.value = false;
      await navigateTo("/");
    }
  }

  async function checkAuth(): Promise<boolean> {
    if (isAuthenticated.value && user.value && hasDek()) {
      return true;
    }

    try {
      const { $api } = useNuxtApp();
      const response = await $api<SuccessResponse<UserInfo>>("/auth/me");

      if (response.data) {
        user.value = response.data;
        isAuthenticated.value = true;

        if (!hasDek()) {
          needsUnlock.value = true;
          return true;
        }

        return true;
      }
    } catch {
      clearDek();
      user.value = null;
      isAuthenticated.value = false;
      needsUnlock.value = false;
    }

    return false;
  }

  function hasDek(): boolean {
    return getDek() !== null;
  }

  function useDek(): string {
    const dek = getDek();

    if (!dek) {
      needsUnlock.value = true;
      throw new AuthError("Vault locked. Please unlock to continue.", {
        status: 403,
      });
    }

    return dek;
  }

  return {
    user: readonly(user),
    isAuthenticated: readonly(isAuthenticated),
    isLoading: readonly(isLoading),
    needsUnlock: readonly(needsUnlock),

    login,
    logout,
    checkAuth,
    unlockVault,

    hasDek,
    useDek,
  };
}
