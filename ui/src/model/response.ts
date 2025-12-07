type ErrorDetail = {
  message: string
  details?: {
    validationErrors?: [
      {
        field: string
        message: string
      },
    ]
  }
}

export type ErrorResponse = {
  error: ErrorDetail
  requestId?: string
}

export type SuccessResponse<T> = {
  data?: T
  message: string
}