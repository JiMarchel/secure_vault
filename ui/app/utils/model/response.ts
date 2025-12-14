type ErrorDetail = {
  message: string
  details?: {
    validationErrors?: Array<{
      field: string
      message: string
    }>
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

