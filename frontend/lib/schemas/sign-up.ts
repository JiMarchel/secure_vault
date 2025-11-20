import z from "zod";

export const signUpSchema = z.object({
  username: z.string().min(3, "Usernames minimal 3 characters").max(100),
  email: z
    .email("Invalid email address")
    .refine(
      (e) => e.toLowerCase().endsWith("@gmail.com"),
      "Sorry for now just support gmail mail"
    ),
});

export type signUpType = z.infer<typeof signUpSchema>;
