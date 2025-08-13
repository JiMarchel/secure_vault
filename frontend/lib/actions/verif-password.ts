"use server";
import { cookies } from "next/headers";
import { VerifPasswordActionResponse } from "../types";
import { verifPasswordSchema } from "../schemas/verif-password";
import z from "zod";
import { createVault } from "../crypto/vault";

export async function verifPasswordAction(
  _: VerifPasswordActionResponse,
  formData: FormData
): Promise<VerifPasswordActionResponse> {
  const cookieStore = await cookies();
  const rawData = {
    password: formData.get("password") as string,
    confirm_password: formData.get("confirm_password") as string,
    id: cookieStore.get("sc-verif-password")?.value as string,
  };

  const validateFields = verifPasswordSchema.safeParse(rawData);
  if (!validateFields.success) {
    console.error(z.flattenError(validateFields.error).fieldErrors);
    return {
      errors: z.flattenError(validateFields.error).fieldErrors,
      inputs: rawData,
    };
  }

  console.log("Raw Data:", rawData);


  const baseApiUrl = process.env.BASE_API_URL;

  return {
    // errors: {},
    // inputs: data,
    // messageApi: "Password verification successful.",
  };
}
