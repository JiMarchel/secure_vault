import { Eye, Shield } from 'lucide-react'
import { Link, useLocation } from '@tanstack/react-router'
import { useState } from 'react'
import { useMutation } from '@tanstack/react-query'
import { Button } from './ui/button'
import { Label } from './ui/label'
import { Input } from './ui/input'
import { FieldGroup } from './ui/field'
import { useAppForm } from './ui/custom-form'
import type { z } from 'zod'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import { cn } from '@/lib/utils'
import { signUp } from '@/validation/auth'
import { fetchAPI } from '@/lib/custom-fetch'

export const Navbar = () => {
  const location = useLocation()
  const [isPasswordVisible, setIsPasswordVisible] = useState(false)
  const [isOpen, setIsOpen] = useState(false)

  const { mutate, isPending } = useMutation({
    mutationFn: async (value: z.infer<typeof signUp>) => {
      const res = await fetchAPI(`${import.meta.env.VITE_API_BASE_URL}/auth`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        credentials: 'include',
        body: JSON.stringify(value),
      })

      return res
    },
    onSuccess: (data) => {
      setIsOpen(false)
      // Using window.location.href because cookie auth_session not set yet and middleware redirect us back
      if (data.message === 'verif_password') {
        window.location.href = '/verification/password'
      } else {
        window.location.href = '/verification/otp'
      }
    },
  })

  const signUpForm = useAppForm({
    defaultValues: {
      username: '',
      email: '',
    },
    validators: {
      onSubmit: signUp,
    },
    onSubmit: ({ value }) => {
      mutate(value)
    },
  })

  return (
    <header className="sticky top-0 z-50 w-full border-b bg-white/95 backdrop-blur supports-backdrop-filter:bg-white/60">
      <div className="flex h-16 items-center justify-between px-4 md:px-6">
        <Link className="flex items-center space-x-2" to="/">
          <Shield className="h-8 w-8 text-primary" />
          <span className="text-2xl font-bold text-gray-900">SecureVault</span>
        </Link>

        <div
          className={cn('flex items-center space-x-3', {
            hidden: location.pathname !== '/',
          })}
        >
          <Dialog>
            <DialogTrigger asChild>
              <Button variant="ghost" size="sm">
                Sign In
              </Button>
            </DialogTrigger>
            <DialogContent className="sm:max-w-md">
              <DialogHeader>
                <DialogTitle>Welcome back</DialogTitle>
                <DialogDescription>
                  Sign in to your SecureVault account
                </DialogDescription>
              </DialogHeader>
              <form className="space-y-4">
                <div className="space-y-2">
                  <Label htmlFor="signin-email">Email</Label>
                  <Input
                    id="signin-email"
                    type="email"
                    placeholder="Enter your email"
                    required
                  />
                </div>
                <div className="space-y-2">
                  <Label htmlFor="signin-password">Password</Label>
                  <div className="flex gap-2 items-center">
                    <Input
                      id="signin-password"
                      type={isPasswordVisible ? 'text' : 'password'}
                      placeholder="Enter your password"
                      required
                    />
                    <Eye
                      onClick={() => setIsPasswordVisible(!isPasswordVisible)}
                      className="cursor-pointer"
                    />
                  </div>
                </div>

                <div className="flex items-center justify-between">
                  <Link
                    to="."
                    className="text-sm text-primary hover:text-primary/90"
                  >
                    Forgot password?
                  </Link>
                </div>
                <Button
                  type="submit"
                  className="w-full bg-primary hover:bg-primary/90"
                >
                  Sign In
                </Button>
              </form>
            </DialogContent>
          </Dialog>

          <Dialog open={isOpen} onOpenChange={setIsOpen}>
            <DialogTrigger asChild>
              <Button
                size="sm"
                className="bg-primary hover:bg-primary/90 cursor-pointer"
              >
                Get Started
              </Button>
            </DialogTrigger>
            <DialogContent className="sm:max-w-md">
              <DialogHeader>
                <DialogTitle>Create your account</DialogTitle>
                <DialogDescription>
                  Start securing your passwords today
                </DialogDescription>
              </DialogHeader>
              <form
                className="space-y-4"
                onSubmit={(e) => {
                  e.preventDefault()
                  signUpForm.handleSubmit()
                }}
              >
                <FieldGroup>
                  <signUpForm.AppField name="username">
                    {(field) => <field.Input label="Username" />}
                  </signUpForm.AppField>
                  <signUpForm.AppField name="email">
                    {(field) => <field.Input label="Email" />}
                  </signUpForm.AppField>
                </FieldGroup>

                <Button
                  type="submit"
                  disabled={isPending}
                  className="w-full bg-primary hover:bg-primary/90"
                >
                  {isPending ? 'Submitting...' : 'Get Started'}
                </Button>
              </form>
            </DialogContent>
          </Dialog>
        </div>
      </div>
    </header>
  )
}
