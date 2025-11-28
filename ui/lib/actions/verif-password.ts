"use server";
import { cookies } from "next/headers";
import { VerifPasswordActionResponse } from "../types";
import { verifPasswordSchema } from "../schemas/verif-password";
import z from "zod";
import { getSession } from "./get-session-cookie";

export async function verifPasswordAction(
  _: VerifPasswordActionResponse,
  formData: FormData
): Promise<VerifPasswordActionResponse> {
  const cookieStore = await cookies();
  const authSession = await getSession();
  
  const rawData = {
    password: formData.get("password") as string,
    confirm_password: formData.get("confirm_password") as string,
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

  return {
    // errors: {},
    // inputs: data,
    // messageApi: "Password verification successful.",
  };
}
