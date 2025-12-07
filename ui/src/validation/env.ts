import z from 'zod'

// const envSchema = z.object({
//   API_BASE_URL: z.url(),
// })

// export const serverEnv = envSchema.parse(process.env)

const envSchemaClient = z.object({
  VITE_API_BASE_URL: z.string(),
})

export const clientEnv = envSchemaClient.parse(import.meta.env)
