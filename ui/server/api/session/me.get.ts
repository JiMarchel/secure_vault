import type { SuccessResponse } from "~/utils/model/response";
import type { User } from "~/utils/model/user";

export default defineEventHandler(async (event) => {

  const config = useRuntimeConfig();
  const cookie = getCookie(event, "auth_session");

  const userData = await $fetch<SuccessResponse<User>>(
    `${config.apiBaseUrl}/session/me`,
    {
      headers: {
        cookie: `auth_session=${cookie}`,
      },
    }
  );

  return userData;
});
