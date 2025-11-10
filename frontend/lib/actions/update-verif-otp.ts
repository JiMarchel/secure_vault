"use server";

import { revalidatePath } from "next/cache";

export async function updateVerifOtp(_: unknown, cookieHeader?: string) {
  await fetch(`http://localhost:8000/api/user/session/resend-otp`, {
    method: "PATCH",
    headers: cookieHeader ? { Cookie: cookieHeader } : {},
  });

  revalidatePath("/auth/verif-otp");
}
