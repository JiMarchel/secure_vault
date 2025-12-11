import { createServerFn } from '@tanstack/react-start'
import { setCookie } from '@tanstack/react-start/server'
import { useAppSession } from '@/utils/session'

export const saveTokenToSessionFn = createServerFn({ method: 'POST' })
  .inputValidator(
    (data: {
      access_token: string
      refresh_token: string
      token_type: string
      expires_in: number
    }) => data,
  )
  .handler(async ({ data }) => {
    const session = await useAppSession()

    await session.update({
      access_token: data.access_token,
      refresh_token: data.refresh_token,
      token_type: data.token_type,
      expires_in: data.expires_in,
    })

    return { success: true }
  })

export const deleteAuthSessionCookieFn = createServerFn({ method: 'POST' })
  .handler(() => {
    // Delete the auth_session cookie by setting it with expired date
    setCookie('auth_session', '', {
      httpOnly: true,
      secure: true,
      sameSite: 'lax',
      path: '/',
      maxAge: 0, // Immediately expire
    })

    return { success: true }
  })
