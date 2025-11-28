import { SuccessResponse } from "../types/api-response";
import { OtpResponse } from "../types/user";

export const getOtpCode = async (cookieHeader?: string): Promise<SuccessResponse<OtpResponse>> => {
  const response = await fetch(
    "http://localhost:8000/api/user/session/get-otp",
    {
      headers: cookieHeader ? { Cookie: cookieHeader } : {},
    }
  );

  if (!response.ok) {
    const res = await response.json();
    throw new Error(
      `${res.error}, did you try to change the Uuid? please don't do that.&&${response.status} - ${response.statusText}`
    );
  }

  return response.json();
};