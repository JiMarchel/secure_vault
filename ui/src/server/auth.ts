import { createServerFn } from '@tanstack/react-start'
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
