import z from 'zod'

export const signUp = z.object({
  username: z.string().min(3).max(256, 'Username is too long'),
  email: z
    .email('Invalid email address')
    .refine(
      (e) => e.toLowerCase().endsWith('@gmail.com'),
      'Sorry for now just support gmail address',
    ),
})

export type signUpType = z.infer<typeof signUp>
