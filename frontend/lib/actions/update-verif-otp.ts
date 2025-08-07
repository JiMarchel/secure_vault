"use server";

import { revalidatePath } from "next/cache";

export async function updateVerifOtp(_: unknown, formData: FormData) {
  const rawData = {
    id: formData.get("id") as string,
  };
  console.log(rawData);

  await fetch(
    `${process.env.BASE_API_URL}/users/otp-code/update/${rawData.id}`,
    {
      method: "PATCH",
      body: JSON.stringify(rawData),
    }
  );

  revalidatePath("/auth/verif-otp");
}
