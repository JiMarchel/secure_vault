"use server";
import { OtpVerifActionResponse } from "../types";
import { otpVerifSchema } from "../schemas/verif-otp";
import z from "zod";
import { redirect } from "next/navigation";
import { cookies } from "next/headers";

export async function verifOtpAction(
  _: OtpVerifActionResponse | null,
  formData: FormData
): Promise<OtpVerifActionResponse> {
  const cookieStore = await cookies();
  const rawData = {
    otp_code: formData.get("otp") as string,
    id: formData.get("id") as string,
  };

  const validateFields = otpVerifSchema.safeParse(rawData);

  if (!validateFields.success) {
    return {
      errors: z.flattenError(validateFields.error).fieldErrors,
      inputs: rawData,
    };
  }

  const baseApiUrl = process.env.BASE_API_URL;
  const response = await fetch(`${baseApiUrl}/users/otp-code/verif`, {
    method: "PATCH",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(rawData),
  });

  if (!response.ok) {
    const res = await response.json();
    return {
      messageApi: res.error,
      inputs: { otp_code: rawData.otp_code },
    };
  }

  const res = await response.json();

  cookieStore.set({
    name: "sc-verif-password",
    value: res.id,
    path: "/",
  });

  cookieStore.delete("sc-verif-otp")
  redirect("/auth/verif-password");
}
