import { NextRequest, NextResponse } from "next/server";
import { checkUserSession } from "@/lib/query/check-session";

export async function middleware(request: NextRequest) {
  const { pathname } = request.nextUrl;

  const cookieHeader = request.headers.get("cookie") || "";

  const data = await checkUserSession(cookieHeader); 

  if (data?.message === "verif_otp") {
    if (pathname !== "/auth/verif-otp") {
      return NextResponse.redirect(new URL("/auth/verif-otp", request.url));
    }
  } else if (data?.message === "verif_password") {
    if (pathname !== "/auth/verif-password") {
      return NextResponse.redirect(
        new URL("/auth/verif-password", request.url)
      );
    }
  } else if (data?.authenticated) {
    if (
      pathname === "/auth/verif-otp" ||
      pathname === "/auth/verif-password" ||
      pathname === "/"
    ) {
      return NextResponse.redirect(new URL("/dashboard", request.url));
    }
  }
}

export const config = {
  matcher: ["/((?!api|_next/static|_next/image|favicon.ico).*)"],
};
