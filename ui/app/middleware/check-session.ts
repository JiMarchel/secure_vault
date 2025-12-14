import type { SuccessResponse } from "~/utils/model/response";
import type { signUpSession } from "~/utils/model/session";

export default defineNuxtRouteMiddleware(async (to) => {
  if (import.meta.client) return;

  const config = useRuntimeConfig();
  const { value: authSession } = useCookie("auth_session");

  const { data } =await  useFetch<SuccessResponse<signUpSession>>(
    `${config.public.apiBaseUrl}/session/check`,
    {
      headers: {
        Cookie: `auth_session=${authSession}`,
      },
    }
  );

  const stateToPath: Record<string, string> = {
    verif_otp: "/verif/otp",
    verif_password: "/verif/password",
  };

  const targetPath = stateToPath[data.value?.data?.state || ""] || "/";

  if (data.value?.data && to.path !== targetPath) {
    return navigateTo(targetPath);
  }
});
