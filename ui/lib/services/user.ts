// import { CheckSessionResponse, OtpResponse, UserResponse } from "../types/user";
// import { apiClient } from "./client";

// export const userService = {
//   getUserMe: async (cookieHeader?: string): Promise<UserResponse> => {
//     return apiClient.get<UserResponse>("/user/session/get-me", {
//       cookieHeader,
//     });
//   },

//   checkSession: async (
//     cookieHeader?: string
//   ): Promise<CheckSessionResponse> => {
//     return apiClient.get<CheckSessionResponse>("/user/check-session", {
//       cookieHeader,
//     });
//   },

//   getOtpCode: async (cookieHeader?: string): Promise<OtpResponse> => {
//     return apiClient.get<OtpResponse>("/user/session/get-otp", {
//       cookieHeader,
//     });
//   },

//   logout: async (): Promise<void> => {
//     return apiClient.post<void>("/user/logout");
//   },
// };
