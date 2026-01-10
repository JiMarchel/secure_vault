import type { UserIdentifier } from "~/lib/wasm/type";
import type { SuccessResponse } from "~/utils/model/response";

export default defineEventHandler(async (event) => {
  const config = useRuntimeConfig();
  const emailBody = await readBody(event)

  const data = await $fetch<SuccessResponse<UserIdentifier>>(
    `${config.apiBaseUrl}/user/identifier`,
    {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: { email: emailBody.email },
    }
  );

  console.log(data)
  return data;
});
