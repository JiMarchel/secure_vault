// import { ApiError, ErrorResponse, SuccessResponse } from "../types/api-response";

// export interface FetchOptions extends RequestInit {
//   params?: Record<string, string>;
//   cookieHeader?: string;
// }

// export class ApiClient {
//   private baseUrl: string;

//   constructor(baseUrl: string = "http://localhost:8000/api") {
//     this.baseUrl = baseUrl;
//   }

//   private buildUrl(endPoint: string, params?: Record<string, string>): string {
//     const url = new URL(endPoint, this.baseUrl);

//     if (params) {
//       Object.entries(params).forEach(([key, value]) => {
//         url.searchParams.append(key, value);
//       });
//     }

//     return url.toString();
//   }

//   private async handleResponse<T>(response: Response): Promise<T> {
//     const contentType = response.headers.get("content-type");
//     const isJson = contentType?.includes("application/json");

//     if (!response.ok) {
//       if (isJson) {
//         try {
//           const errorBody: ErrorResponse = await response.json();

//           throw new ApiError(
//             errorBody.error.message,
//             response.status,
//             response.statusText,
//             errorBody.error,
//             errorBody.requestId
//           );
//         } catch (error) {
//           if (error instanceof ApiError) throw error;

//           throw new ApiError(
//             "Failed to parse error response",
//             response.status,
//             response.statusText
//           );
//         }
//       } else {
//         const text = await response.text();

//         throw new ApiError(
//           text || response.statusText,
//           response.status,
//           response.statusText
//         );
//       }
//     }

//     if (!isJson) {
//       return "" as T;
//     }

//     try {
//       const data: SuccessResponse<T> = await response.json();

//       return (data.data ?? data) as T;
//     } catch (error) {
//       throw new ApiError(
//         "Failed to parse success response",
//         response.status,
//         response.statusText
//       );
//     }
//   }

//   async request<T>(endpoint: string, options: FetchOptions = {}): Promise<T> {
//     const { params, cookieHeader, headers, ...fetchOptions } = options;

//     const url = this.buildUrl(endpoint, params);

//     const config: RequestInit = {
//       ...fetchOptions,
//       headers: {
//         "Content-Type": "application/json",
//         ...(cookieHeader && { Cookie: cookieHeader }),
//         ...headers,
//       },
//     };

//     try {
//       const response = await fetch(url, config);
//       return this.handleResponse<T>(response);
//     } catch (error) {
//       if (error instanceof ApiError) {
//         throw error;
//       }

//       throw new ApiError(
//         error instanceof Error ? error.message : "Network Error",
//         0,
//         "Network Error"
//       );
//     }
//   }

//   async get<T>(endpoint: string, options?: FetchOptions): Promise<T> {
//     return this.request<T>(endpoint, { ...options, method: "GET" });
//   }

//   async post<T>(
//     endpoint: string,
//     data?: unknown,
//     options?: FetchOptions
//   ): Promise<T> {
//     return this.request<T>(endpoint, {
//       ...options,
//       method: "POST",
//       body: data ? JSON.stringify(data) : undefined,
//     });
//   }

//   async put<T>(
//     endpoint: string,
//     data?: unknown,
//     options?: FetchOptions
//   ): Promise<T> {
//     return this.request<T>(endpoint, {
//       ...options,
//       method: "PUT",
//       body: data ? JSON.stringify(data) : undefined,
//     });
//   }

//   async patch<T>(
//     endpoint: string,
//     data?: unknown,
//     options?: FetchOptions
//   ): Promise<T> {
//     return this.request<T>(endpoint, {
//       ...options,
//       method: "PATCH",
//       body: data ? JSON.stringify(data) : undefined,
//     });
//   }

//   async delete<T>(endpoint: string, options?: FetchOptions): Promise<T> {
//     return this.request<T>(endpoint, { ...options, method: "DELETE" });
//   }
// }

// export const apiClient = new ApiClient();