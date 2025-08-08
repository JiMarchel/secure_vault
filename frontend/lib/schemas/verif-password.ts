import { z, RefinementCtx } from "zod";

export const verifPasswordSchema = z
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
    confirm_password: z
      .string()
      .min(8, "Confirm password must be at least 8 characters long.")
      .max(100, "Confirm password must be at most 100 characters long."),
    id: z.uuid("Invalid ID format."),
  })
  .superRefine(
    (
      data: { password: string; confirm_password: string; id: string },
      ctx: RefinementCtx
    ) => {
      if (data.confirm_password !== data.password) {
        ctx.addIssue({
          code: "custom",
          path: ["confirm_password"],
          message: "Passwords do not match.",
        });
      }
    }
  )
  .strict();
