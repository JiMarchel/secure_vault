import type { ErrorResponse } from "~/utils/model/response";

export interface FieldError {
  field: string;
  message: string;
}

export interface AppErrorOptions {
  message: string;
  status?: number;
  requestId?: string;
  fieldErrors?: FieldError[];
  cause?: unknown;
}

/**
 * Base error class that matches API's ErrorResponse format.
 * All custom errors should extend this class.
 */
export class AppError extends Error {
  readonly status: number;
  readonly requestId?: string;
  readonly fieldErrors?: FieldError[];
  readonly data: ErrorResponse;

  constructor(options: AppErrorOptions) {
    super(options.message);
    this.name = "AppError";
    this.status = options.status ?? 400;
    this.requestId = options.requestId;
    this.fieldErrors = options.fieldErrors;
    this.cause = options.cause;

    this.data = {
      error: {
        message: options.message,
        details: options.fieldErrors
          ? { validationErrors: options.fieldErrors }
          : undefined,
      },
      requestId: options.requestId,
    };
  }

  /**
   * Create AppError from API ErrorResponse
   */
  static fromResponse(response: ErrorResponse, status: number = 400): AppError {
    return new AppError({
      message: response.error.message,
      status,
      requestId: response.requestId,
      fieldErrors: response.error.details?.validationErrors,
    });
  }
}

/**
 * Authentication errors (login, session, token)
 */
export class AuthError extends AppError {
  constructor(
    message: string,
    options: Partial<Omit<AppErrorOptions, "message">> = {}
  ) {
    super({ message, status: 401, ...options });
    this.name = "AuthError";
  }
}

/**
 * WASM/Encryption errors
 */
export class CryptoError extends AppError {
  constructor(
    message: string,
    options: Partial<Omit<AppErrorOptions, "message">> = {}
  ) {
    super({ message, status: 400, ...options });
    this.name = "CryptoError";
  }
}

/**
 * Validation errors (form, input)
 */
export class ValidationError extends AppError {
  constructor(
    message: string,
    fieldErrors: FieldError[],
    options: Partial<Omit<AppErrorOptions, "message" | "fieldErrors">> = {}
  ) {
    super({ message, status: 422, fieldErrors, ...options });
    this.name = "ValidationError";
  }
}

/**
 * Not found errors
 */
export class NotFoundError extends AppError {
  constructor(
    message: string = "Resource not found",
    options: Partial<Omit<AppErrorOptions, "message">> = {}
  ) {
    super({ message, status: 404, ...options });
    this.name = "NotFoundError";
  }
}

/**
 * Server/Internal errors
 */
export class ServerError extends AppError {
  constructor(
    message: string = "Internal server error",
    options: Partial<Omit<AppErrorOptions, "message">> = {}
  ) {
    super({ message, status: 500, ...options });
    this.name = "ServerError";
  }
}

/**
 * Check if error is an AppError instance
 */
export function isAppError(error: unknown): error is AppError {
  return error instanceof AppError;
}

/**
 * Check if error matches API error structure (from $fetch)
 */
export function isApiError(
  error: unknown
): error is { data?: ErrorResponse; status: number } {
  return (
    typeof error === "object" &&
    error !== null &&
    "data" in error &&
    "status" in error
  );
}

/**
 * Convert any error to AppError for consistent handling
 */
export function toAppError(error: unknown): AppError {
  if (isAppError(error)) {
    return error;
  }

  if (isApiError(error) && error.data) {
    return AppError.fromResponse(error.data, error.status);
  }

  if (error instanceof Error) {
    return new AppError({
      message: error.message,
      cause: error,
    });
  }

  return new AppError({
    message: String(error) || "An unexpected error occurred",
  });
}
