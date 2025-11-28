"use server";
import { OtpVerifActionResponse } from "../types";
import { otpVerifSchema } from "../schemas/verif-otp";
import z from "zod";
import { redirect } from "next/navigation";
import { getSession } from "./get-session-cookie";
import { cookies } from "next/headers";

export async function verifOtpAction(
  _: OtpVerifActionResponse | null,
  formData: FormData
): Promise<OtpVerifActionResponse> {
  const authSession = await getSession();

  const rawData = {
    otp_code: formData.get("otp_code") as string,
  };

  const validateFields = otpVerifSchema.safeParse(rawData);

  if (!validateFields.success) {
    return {
      errors: z.flattenError(validateFields.error).fieldErrors,
      inputs: rawData,
    };
  }

  const response = await fetch("http://localhost:8000/api/user/verify-otp", {
    method: "PATCH",
    headers: { "Content-Type": "application/json", Cookie: authSession },
    body: JSON.stringify(rawData),
  });

  if (!response.ok) {
    const res = await response.json();
    console.log(res);
    return {
      messageApi: res.error,
      inputs: { otp_code: rawData.otp_code },
    };
  }

  const setCookieHeader = response.headers.get("Set-Cookie");
  if (setCookieHeader) {
    const cookieStore = await cookies();
    const cookieParts = setCookieHeader.split(";")[0].split("=");
    const cookieName = cookieParts[0];
    const cookieValue = cookieParts[1];

    cookieStore.set({
      name: cookieName,
      value: cookieValue,
      httpOnly: true,
      path: "/",
      sameSite: "lax",
      maxAge: 60 * 60 * 24,
    });
  }

  redirect("/auth/verif-password");
}
