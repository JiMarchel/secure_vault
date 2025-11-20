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

  const response = await fetch("http://localhost:8000/api/user/sign-up", {
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

  const setCookieHeader = response.headers.get("Set-Cookie");
  console.log(setCookieHeader)
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

  const json_res = await response.json();

  if (json_res.message === "verif_otp" || json_res.message === "created") {
    redirect("/auth/verif-otp");
  } else if (json_res.message === "verif_password") {
    redirect("/auth/verif-password");
  } else {
  }

  return {};
}
