import z from "zod";

export const otpVerifSchema = z.object({
  otp_code: z.string().length(6, "OTP must be exactly 6 digits long."),
  id: z.uuid(),
});
