import { SuccessResponse } from "../types/api-response";
import { CheckSessionResponse } from "../types/user";

export const checkUserSession = async (
  cookieHeader?: string
): Promise<SuccessResponse<CheckSessionResponse>> => {
  const response = await fetch("http://localhost:8000/api/user/check-session", {
    headers: cookieHeader ? { Cookie: cookieHeader } : {},
  });
  return response.json();
};
