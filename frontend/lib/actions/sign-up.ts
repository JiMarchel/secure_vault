"use server";

import z from "zod";
import { signUpSchema } from "../schemas/sign-up";
import { signUpActionResponse } from "../types";
import { cookies } from "next/headers";
import { redirect } from "next/navigation";

export async function signUpAction(
  _: signUpActionResponse | null,
  formData: FormData
): Promise<signUpActionResponse> {
  const rawData = {
    username: formData.get("username") as string,
    email: formData.get("email") as string,
  };

  const validateFields = signUpSchema.safeParse(rawData);

  if (!validateFields.success) {
    return {
      errors: z.flattenError(validateFields.error).fieldErrors,
      inputs: rawData,
    };
  }

  const baseApiUrl = process.env.BASE_API_URL;
  const response = await fetch(`${baseApiUrl}/sign-up`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(validateFields.data),
  });

  if (!response.ok) {
    const error = await response.json();
    return {
      messageApi: error.error,
      inputs: rawData,
    };
  }

  const json_res = await response.json();
  const cookieStore = await cookies();

  if (json_res.message === "verif_otp" || json_res.message === "created") {
    cookieStore.set({
      name: "sc-verif-otp",
      value: json_res.id,
      path: "/",
    });
    redirect("/auth/verif-otp");
  } else if (json_res.message === "verif_password") {
    cookieStore.set({
      name: "sc-verif-password",
      value: json_res.id,
      path: "/",
    });
    redirect("/auth/verif-password");
  } else {
  }

  return {};
}
