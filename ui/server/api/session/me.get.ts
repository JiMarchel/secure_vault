import type { SuccessResponse } from "~/utils/model/response";
import type { User } from "~/utils/model/user";

export default defineEventHandler(async (event) => {

  const cookie = getCookie(event, "auth_session");
  console.log("Fetching user data with auth_session:", cookie);

  const userData = await $fetch<SuccessResponse<User>>(
    `http://localhost:8000/api/session/me`,
    {
      headers: {
        cookie: `auth_session=${cookie}`,
      },
    }
  );

  return userData;
});
