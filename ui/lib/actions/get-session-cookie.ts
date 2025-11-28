"use server";

import { cookies } from "next/headers";

export async function getSession() {
  const cookie = await cookies();
  const cookieHeader = cookie.get("auth_session") || undefined;
  const authSession = `${cookieHeader?.name}=${cookieHeader?.value}`;
  return authSession;
}
