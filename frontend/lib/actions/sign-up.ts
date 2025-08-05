"use server";

import z from "zod";
import { signUpSchema } from "../schemas/sign-up";
import { signUpActionResponse } from "../types/sign-up";

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
  try {
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

    const json_res = await response.json()

    if(json_res.message === "verif_otp"){
        console.log("di verif otp", json_res)
    }else if(json_res.message === "verif_password"){

    }else {

    }
  } catch (error) {
    console.log(error);
  }

  return {};
}
