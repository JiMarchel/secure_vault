export interface OtpStatus {
  hasOtp: boolean
  expiresAt: string | null
  canResend: boolean
  resendAfter: number | null
}

export interface ResendOtpResponse {
  success: boolean
  cooldownSeconds: number
}