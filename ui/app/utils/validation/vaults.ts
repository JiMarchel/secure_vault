import z from "zod";

const title = z
  .string()
  .min(1, "Title is required")
  .max(64, "Title must be at most 64 characters long");

export const addPassword = z.object({
  title,
  usernameOrEmail: z
    .string()
    .max(64, "Username or email must be at most 64 characters long"),
  password: z
    .string()
    .max(64, "Password must be at most 64 characters long"),
  websiteOrApp: z
    .string()
    .max(64, "Website or app must be at most 64 characters long"),
});
