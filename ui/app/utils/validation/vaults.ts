import z from "zod";

const title = z
  .string()
  .min(1, "Title is required")
  .max(64, "Title must be at most 64 characters long");

const usernameOrEmail = z
  .string()
  .max(64, "Username or email must be at most 64 characters long");

const password = z
  .string()
  .max(64, "Password must be at most 64 characters long");

const websiteOrApp = z
  .string()
  .max(64, "Website or app must be at most 64 characters long");

export const addPassword = z.object({
  title,
  usernameOrEmail,
  password,
  websiteOrApp,
});

export type addPasswordType = z.infer<typeof addPassword>

export const updatePassword = z.object({
  id: z.uuid({ version: "v4" }),
  title,
  usernameOrEmail,
  password,
  websiteOrApp,
});

export type updatePasswordType = z.infer<typeof updatePassword>