export interface SuccessResponse<T> {
  data?: T;
  message: string;
}

export interface ErrorResponse {
  error: ErrorDetail;
  requestId?: string;
}

export interface ErrorDetail {
  message: string;
  details?: unknown;
}

// export class ApiError extends Error {
//   constructor(
//     message: string,
//     public statusCode: number,
//     public statusText: string,
//     public errorDetail?: ErrorDetail,
//     public requestId?: string
//   ) {
//     super(message);
//     this.name = "ApiError";
//   }

//   is(code: number): boolean {
//     return this.statusCode === code;
//   }

//   isClientError(): boolean {
//     return this.statusCode >= 400 && this.statusCode < 500;
//   }

//   isServerError(): boolean {
//     return this.statusCode >= 500;
//   }
// }
