import { createMiddleware } from "@tanstack/react-start"
import { getCookies } from '@tanstack/react-start/server'
import { redirect } from "@tanstack/react-router"
import type { signUpSession } from "@/model/session"
import type { User } from "@/model/user"
import { fetchAPI } from "@/lib/custom-fetch"
import { serverEnv } from "@/validation/env"

export const signUpMiddleware = createMiddleware({ type: "request" })
  .server(async ({ next, pathname }) => {
    const sessionCookie = getCookies()
    const hasSession = !!sessionCookie.auth_session

    const isVerificationRoute =
      pathname === "/verification/otp" ||
      pathname === "/verification/password"

    if (isVerificationRoute && !hasSession) {
      throw redirect({ to: "/" })
    }

    if (!hasSession) {
      return next()
    }

    const auth_session = `auth_session=${sessionCookie.auth_session}`

    const res = await fetchAPI<signUpSession>(`${serverEnv.API_BASE_URL}/session/check`, {
      headers: {
        Cookie: auth_session
      }
    })

    const stateToPath: Record<string, string> = {
      'verif_otp': '/verification/otp',
      'verif_password': '/verification/password',
    }

    const targetPath = stateToPath[res.data?.state || ''] || '/'

    if (pathname !== targetPath && !isVerificationRoute) {
      throw redirect({ to: targetPath })
    }

    // Jika di verification route tapi state tidak match, 
    // biarkan tetap di route tersebut (mungkin sedang process)
    if (isVerificationRoute && pathname !== targetPath) {
      // Log untuk debugging
      console.warn(`State mismatch: on ${pathname} but state suggests ${targetPath}`)
      // Tetap allow access
    }


    const user = await fetchAPI<User>(`${serverEnv.API_BASE_URL}/session/me`, {
      headers: {
        Cookie: auth_session
      }
    })

    return next({
      context: {
        user,
      }
    })
  })
