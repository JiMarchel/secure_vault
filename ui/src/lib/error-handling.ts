import type { ErrorResponse } from "@/model/response"

export class APIError extends Error {
  public status: number
  public errorResponse: ErrorResponse

  constructor(errorResponse: ErrorResponse, status: number) {
    super()
    this.errorResponse = errorResponse
    this.status = status
  }
}

export async function withCatch<
  T,
  TError extends Error = Error,
>(
  promise: Promise<T>,
  errorToCatch?: Array<new (...args: Array<any>) => TError>,
): Promise<[undefined, T] | [TError]> {
  return promise
    .then((data) => {
      return [undefined, data] as [undefined, T]
    })
    .catch((error) => {
      if (errorToCatch == undefined) {
        return [error]
      }

      if (errorToCatch.some((ErrorClass) => error instanceof ErrorClass)) {
        return [error]
      }

      throw error
    })
}
