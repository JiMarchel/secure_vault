import type { SuccessResponse } from "~/utils/model/response";

interface AuthTokens {
  accessToken: string;
  refreshToken: string;
}

export default defineNuxtPlugin(() => {
  const config = useRuntimeConfig();

  let isRefreshing = false;
  let refreshSubscribers: Array<(success: boolean) => void> = [];

  function subscribeTokenRefresh(cb: (success: boolean) => void) {
    refreshSubscribers.push(cb);
  }

  function onRefreshComplete(success: boolean) {
    refreshSubscribers.forEach((cb) => cb(success));
    refreshSubscribers = [];
  }

  async function refreshToken(): Promise<boolean> {
    try {
      await $fetch<SuccessResponse<AuthTokens>>(
        `${config.public.apiBaseUrl}/auth/refresh`,
        {
          method: "POST",
          credentials: "include",
        },
      );
      return true;
    } catch {
      return false;
    }
  }

  const $api = $fetch.create({
    baseURL: config.public.apiBaseUrl,
    credentials: "include",

    async onResponseError({ response, request, options }) {
      const status = response.status;

      if (status !== 401) {
        return;
      }

      // Avoid infinite loop on refresh endpoint
      if (String(request).includes("/auth/refresh")) {
        const { logout } = useAuth();
        await logout(true);
        return;
      }

      // Don't retry on report-failed - its 401 is intentional (wrong password)
      if (String(request).includes("/auth/report-failed")) {
        return;
      }

      if (!isRefreshing) {
        isRefreshing = true;

        const success = await refreshToken();

        isRefreshing = false;
        onRefreshComplete(success);

        if (!success) {
          const { logout } = useAuth();
          await logout(true);
          return;
        }

        // Leader request retries immediately
        const retryOptions = {
          ...options,
          credentials: "include" as RequestCredentials,
        };
        return $fetch(request, retryOptions as Parameters<typeof $fetch>[1]);
      }

      // Wait for ongoing refresh to complete
      return new Promise<void>((resolve, reject) => {
        subscribeTokenRefresh(async (success) => {
          if (success) {
            try {
              const retryOptions = {
                ...options,
                credentials: "include" as RequestCredentials,
              };
              await $fetch(
                request,
                retryOptions as Parameters<typeof $fetch>[1],
              );
              resolve();
            } catch (retryError) {
              reject(retryError);
            }
          } else {
            reject(new Error("Token refresh failed"));
          }
        });
      });
    },
  });

  return {
    provide: {
      api: $api,
    },
  };
});
