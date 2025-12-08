import { ClientOnly, createFileRoute } from '@tanstack/react-router'
import { Mail } from 'lucide-react'
import { toast } from 'sonner'
import { useEffect, useState } from 'react'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import type { otpExpiresAt } from '@/model/user'
import type z from 'zod'
import { otpVerification } from '@/validation/otp'
import { Button } from '@/components/ui/button'
import {
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { signUpMiddleware } from '@/middleware/signup'
import { fetchAPI } from '@/lib/custom-fetch'
import useCountDown from '@/hooks/use-countdown'
import { useAppForm } from '@/components/ui/custom-form'
import { FieldGroup } from '@/components/ui/field'

export const Route = createFileRoute('/verification/otp/')({
  server: {
    middleware: [signUpMiddleware],
  },
  component: RouteComponent,
  loader: ({ serverContext }) => {
    return { user: serverContext?.user.data }
  },
})

function ExpirationStatus({ expiresAt }: { expiresAt: string | undefined }) {
  const [isExpired, setIsExpired] = useState(false)

  useEffect(() => {
    if (!expiresAt) return

    const checkExpiration = () => {
      const now = new Date()
      const expiry = new Date(expiresAt)
      setIsExpired(expiry.getTime() - now.getTime() < 0)
    }

    checkExpiration()
    const interval = setInterval(checkExpiration, 1000)
    return () => clearInterval(interval)
  }, [expiresAt])

  if (isExpired) {
    return (
      <span className="text-red-500">
        OTP expired, please request a new one
      </span>
    )
  }

  return (
    <span className="text-emerald-500">Your OTP expires in 10 minutes</span>
  )
}

function RouteComponent() {
  const { user } = Route.useLoaderData()
  const queryClient = useQueryClient()

  const { data } = useQuery({
    queryKey: ['otp-record'],
    queryFn: async () => {
      const res = await fetchAPI<otpExpiresAt>(
        `${import.meta.env.VITE_API_BASE_URL}/session/otp/expire`,
        {
          method: 'GET',
          credentials: 'include',
        },
      )
      return res
    },
    staleTime: 1000 * 60 * 5,
  })

  const { countdown, start } = useCountDown()

  const { mutate: resendOtp, isPending: isResending } = useMutation({
    mutationFn: async () => {
      const res = await fetchAPI(
        `${import.meta.env.VITE_API_BASE_URL}/session/otp/resend`,
        {
          credentials: 'include',
          method: 'PATCH',
        },
      )
      return res
    },
    onSuccess: (dataResending) => {
      queryClient.invalidateQueries({ queryKey: ['otp-record'] })
      toast.success(dataResending.message, { duration: 5000 })
      start(60)
    },
  })

  const { mutate: mutateOtpVerif, isPending: isPendingOtpVerif } = useMutation({
    mutationFn: async (value: z.infer<typeof otpVerification>) => {
      return await fetchAPI(
        `${import.meta.env.VITE_API_BASE_URL}/auth/verif/otp`,
        {
          credentials: 'include',
          method: 'PATCH',
          body: JSON.stringify(value),
        },
      )
    },
    onSuccess: () => {
      toast.success('OTP verified successfully!')
    },
  })

  const isExpired = data?.data?.otpExpiresAt
    ? new Date(data.data.otpExpiresAt).getTime() < Date.now()
    : false

  const form = useAppForm({
    defaultValues: {
      otpCode: '',
    },
    validators: {
      onSubmit: otpVerification,
    },
    onSubmit: ({ value }) => {
      console.log(value)
    },
  })

  return (
    <div className="fixed top-1/2 left-1/2 -translate-y-1/2 -translate-x-1/2 w-xl bg-gray-50 shadow-2xl">
      <Card className="w-full max-w-xl shadow-lg">
        <CardHeader>
          <CardTitle className="text-2xl">Verify OTP</CardTitle>
          <CardDescription>
            Hello <b className="text-primary">{user?.username}</b>, please enter
            the OTP sent to your email, and please don't leave this page until
            you have verified your account.
          </CardDescription>
          <CardAction>
            <Button
              variant="link"
              type="button"
              disabled={countdown > 0 || isResending || isPendingOtpVerif}
              onClick={() => resendOtp()}
            >
              {isResending ? (
                <>
                  <span className="animate-spin mr-2">⏳</span>
                  Sending...
                </>
              ) : countdown > 0 ? (
                `Resend OTP in ${countdown}`
              ) : (
                'Resend OTP'
              )}
            </Button>
          </CardAction>
        </CardHeader>

        <form
          className="space-y-6"
          onSubmit={(e) => {
            e.preventDefault()
            form.handleSubmit()
          }}
        >
          <CardContent className="flex justify-center flex-col items-center gap-4">
            <Badge className="px-2 py-1">
              <Mail /> {user?.email}
            </Badge>
            <div className='max-w-xs w-full'>
            <FieldGroup>
              <form.AppField name="otpCode">
                {(field) => <field.Otp />}
              </form.AppField>
            </FieldGroup>
            </div>
          </CardContent>

          <CardFooter className="flex flex-col justify-start items-start">
            <ClientOnly
              fallback={
                <span className="text-gray-400">
                  Loading expiration status...
                </span>
              }
            >
              <ExpirationStatus expiresAt={data?.data?.otpExpiresAt} />
            </ClientOnly>

            <span className="text-sm text-muted-foreground">
              Please enter the OTP code sent to your email.
            </span>

            <Button type="submit" disabled={isPendingOtpVerif || isExpired} className='cursor-pointer'>
              {isPendingOtpVerif ? (
                <>
                  <span className="animate-spin mr-2">⏳</span>
                  Verifying...
                </>
              ) : (
                'Submit'
              )}
            </Button>
          </CardFooter>
        </form>
      </Card>
    </div>
  )
}
