import { createFileRoute } from '@tanstack/react-router'
import { useState } from 'react'
import { Eye, EyeClosed } from 'lucide-react'
import { signUpMiddleware } from '@/middleware/signup'
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { useAppForm } from '@/components/ui/custom-form'
import { verifPassword } from '@/validation/auth'
import { createUserIdentifier } from '@/lib/enc-dex'
import { FieldGroup } from '@/components/ui/field'

export const Route = createFileRoute('/verification/password/')({
  server: {
    middleware: [signUpMiddleware],
  },
  component: RouteComponent,
  loader: ({ serverContext }) => {
    return { user: serverContext?.user.data }
  },
})

function RouteComponent() {
  const { user } = Route.useLoaderData()
  const [isPasswordVisible, setIsPasswordVisible] = useState(false)
  const isSubmitting = false

  const form = useAppForm({
    validators: {
      onSubmit: verifPassword,
    },
    defaultValues: {
      password: '',
      confirmPassword: '',
    },
    onSubmit: async ({ value }) => {
      console.log('Password Data Submitted: ', value)
      await createUserIdentifier(value.password)
    },
  })

  return (
    <div className="fixed top-1/2 left-1/2 -translate-y-1/2 -translate-x-1/2 w-xl bg-gray-50 shadow-2xl">
      <Card className="w-full max-w-xl shadow-lg">
        <CardHeader>
          <CardTitle className="text-2xl">Creating Master Password</CardTitle>
          <CardDescription>
            Hello <b className="text-primary">{user?.username}</b> this is the
            last step, please create you master password, and please don't leave
            this page until you have created your master password.
          </CardDescription>
        </CardHeader>
        <form
          onSubmit={(e) => {
            e.preventDefault()
            form.handleSubmit()
          }}
          className="space-y-6"
        >
          <CardContent className="flex flex-col gap-2">
            <div className="bg-emerald-200 p-2 rounded-md">
              <p className="text-[0.850rem]">
                When creating a strong password, think "passphrase" instead of
                "password." A long sequence of four or more random, unrelated
                words (like CorrectHorseBatteryStaple) is much harder for
                computers to guess yet easier for you to remember. For enhanced
                security, mix in uppercase and lowercase letters, numbers, and
                symbols ($ ! % &). Never use easily guessable personal
                information like your name, birthday, or pet's name.
              </p>
            </div>

            <FieldGroup>
              <div className='flex gap-2 '>
                <form.AppField name="password">
                  {(field) => (
                    <field.Input
                      label="Password"
                      type={isPasswordVisible ? 'text' : 'password'}
                    />
                  )}
                </form.AppField>
                <Button
                  type="button"
                  size="icon"
                  onClick={() => setIsPasswordVisible(!isPasswordVisible)}
                  className='mt-8'
                >
                  {isPasswordVisible ? <Eye /> : <EyeClosed />}
                </Button>
              </div>
              <form.AppField name="confirmPassword">
                {(field) => (
                  <field.Input
                    label="Confirm Password"
                    type={isPasswordVisible ? 'text' : 'password'}
                  />
                )}
              </form.AppField>
            </FieldGroup>
          </CardContent>
          <CardFooter className="flex flex-col justify-start items-start">
            <Button type="submit" disabled={isSubmitting}>
              Submit
            </Button>
          </CardFooter>
        </form>
      </Card>
    </div>
  )
}
