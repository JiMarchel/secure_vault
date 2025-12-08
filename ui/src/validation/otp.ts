import z from "zod";

export const otpVerification = z.object({
  otpCode: z.string().length(6, "OTP must be exactly 6 digits long."),
});