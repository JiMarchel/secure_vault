"use server";
import { cookies } from "next/headers";
import { OtpVerifActionResponse } from "../types";
import { otpVerifSchema } from "../schemas/verif-otp";
import z from "zod";

export async function verifOtpAction(
  _: OtpVerifActionResponse | null,
  formData: FormData
): Promise<OtpVerifActionResponse> {
  const rawData = { otp: formData.get("otp") as string };
  const cookieStore = cookies();

  const validateFields = otpVerifSchema.safeParse(rawData);

  if (!validateFields.success) {
    return {
      errors: z.flattenError(validateFields.error).fieldErrors,
      inputs: rawData,
    };
  }

  const baseApiUrl = process.env.BASE_API_URL;

  return {};
}
