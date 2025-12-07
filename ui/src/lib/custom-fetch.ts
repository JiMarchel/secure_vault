import { APIError } from './error-handling'

export type SuccessResponse<T> = {
  data?: T
  message: string
}

export async function fetchAPI<T>(
  url: RequestInfo,
  options?: RequestInit,
): Promise<SuccessResponse<T>> {
  const response = await fetch(url, options)

  if (!response.ok) {
    const errorResponse = await response.json()
    const error = new APIError(errorResponse, response.status)

    if (response.status >= 500) {
      const errorMessage = encodeURIComponent(
        errorResponse.error?.message || 'Internal Server Error',
      )
      const requestId = encodeURIComponent(errorResponse.requestId)

      window.location.href = `/internal-error?error=${errorMessage}&requestId=${requestId}`
    }

    throw error
  }

  return response.json()
}
