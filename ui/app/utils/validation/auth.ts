import { z } from "zod";
import type { RefinementCtx } from "zod";

export const email = z
  .email("Invalid email address")
  .refine(
    (e) => e.toLowerCase().endsWith("@gmail.com"),
    "Sorry for now just support gmail address"
  );

export const login = z.object({
  email: z
    .email("Invalid email address")
    .refine(
      (e) => e.toLowerCase().endsWith("@gmail.com"),
      "Sorry for now just support gmail address"
    ),
    password: z.string().min(8, "Password must be at least 8 characters long."),
  });


export const signUp = z.object({
  username: z.string().min(3).max(256, "Username is too long"),
  email: z
    .email("Invalid email address")
    .refine(
      (e) => e.toLowerCase().endsWith("@gmail.com"),
      "Sorry for now just support gmail address"
    ),
});

export const verifOtp = z.object({
  otp_code: z.string().length(6, "OTP must be exactly 6 digits long."),
});

export const verifPassword = z
  .object({
    password: z
      .string()
      .min(8, "Password must be at least 8 characters long.")
      .max(100, "Password must be at most 100 characters long.")
      .refine((val: string) => /[A-Z]/.test(val), {
        message: "Password must contain at least one uppercase letter.",
      })
      .refine((val: string) => /[a-z]/.test(val), {
        message: "Password must contain at least one lowercase letter.",
      })
      .refine((val: string) => /\d/.test(val), {
        message: "Password must contain at least one number.",
      })
      .refine((val: string) => /[!@#$%^&*(),.?":{}|<>]/.test(val), {
        message: "Password must contain at least one special character.",
      }),
    confirmPassword: z
      .string()
      .min(8, "Confirm password must be at least 8 characters long.")
      .max(100, "Confirm password must be at most 100 characters long."),
  })
  .superRefine(
    (
      data: { password: string; confirmPassword: string },
      ctx: RefinementCtx
    ) => {
      if (data.confirmPassword !== data.password) {
        ctx.addIssue({
          code: "custom",
          path: ["confirmPassword"],
          message: "Passwords do not match.",
        });
      }
    }
  )
  .strict();
