import { NextRequest, NextResponse } from "next/server";

export function middleware(request: NextRequest) {
  const otpCookie = request.cookies.has("sc-verif-otp");
  const { pathname } = request.nextUrl;

  if (otpCookie && pathname !== "/auth/verif-otp") {
    return NextResponse.redirect(new URL("/auth/verif-otp", request.url));
  }

  if (!otpCookie && pathname === "/auth/verif-otp") {
    return NextResponse.redirect(new URL("/", request.url));
  }

  return NextResponse.next();
}

export const config = {
  matcher: ["/((?!api|_next/static|_next/image|favicon.ico).*)"],
};
