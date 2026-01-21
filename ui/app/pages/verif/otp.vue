<script setup lang="ts">
import { useForm } from '@tanstack/vue-form'
import { Mail, Clock } from 'lucide-vue-next'
import { REGEXP_ONLY_DIGITS } from 'vue-input-otp'
import { toast } from 'vue-sonner'
import { Badge } from '~/components/ui/badge'
import { Button } from '~/components/ui/button'
import {
    Card,
    CardAction,
    CardContent,
    CardDescription,
    CardFooter,
    CardHeader,
    CardTitle,
} from '~/components/ui/card'
import { Field, FieldError, FieldGroup } from '~/components/ui/field'
import { Alert, AlertDescription } from '~/components/ui/alert'
import { verifOtp } from '~/utils/validation/auth'
import type { User } from '~/utils/model/user'
import { Spinner } from '~/components/ui/spinner'

definePageMeta({
    layout: 'verif',
    middleware: ['check-session', 'signup'],
})

const userData = inject<User>('userData')

const {
    otpStatus,
    isLoading,
    isResending,
    resendCooldown,
    canResend,
    otpExpired,
    timeUntilExpiry,
    resendOtp,
    verifyOtp,
} = useOtpVerification()

const form = useForm({
    defaultValues: {
        otp_code: '',
    },
    validators: {
        onSubmit: verifOtp,
    },
    onSubmit: async ({ value }) => {
        const success = await verifyOtp(value.otp_code)

        if (success) {
            toast.success('Please create your master password.')
            await navigateTo('/verif/password')
        }
    },
})

function isInvalid(field: any) {
    return field.state.meta.isTouched && !field.state.meta.isValid
}

function formatTime(seconds: number): string {
    const mins = Math.floor(seconds / 60)
    const secs = seconds % 60
    return `${mins}:${secs.toString().padStart(2, '0')}`
}
</script>

<template>
    <div class="fixed top-1/2 left-1/2 -translate-y-1/2 -translate-x-1/2 w-xl shadow-2xl">
        <Card class="w-full max-w-xl shadow-lg">
            <CardHeader>
                <CardTitle class="text-2xl">Verify OTP</CardTitle>
                <CardDescription>
                    Hello <b class="text-primary">{{ userData?.username }}</b>, please enter
                    the OTP sent to your email, and please don't leave this page until
                    you have verified your account.
                </CardDescription>
                <CardAction>
                    <Button variant="link" type="button" :disabled="!canResend || isResending" @click="resendOtp">
                        <span v-if="isResending">Sending...</span>
                        <span v-else-if="resendCooldown > 0">
                            Resend in {{ resendCooldown }}s
                        </span>
                        <span v-else>Resend OTP</span>
                    </Button>
                </CardAction>
            </CardHeader>

            <form class="space-y-6" @submit.prevent="form.handleSubmit">
                <CardContent class="flex justify-center flex-col items-center gap-4">
                    <Badge class="px-2 py-1">
                        <Mail class="w-4 h-4 mr-1" />
                        {{ userData?.email }}
                    </Badge>

                    <div v-if="!isLoading && otpStatus" class="w-full">
                        <Alert v-if="otpExpired" variant="destructive">
                            <AlertDescription>
                                Your OTP has expired. Please request a new one.
                            </AlertDescription>
                        </Alert>

                        <Alert v-else-if="otpStatus.hasOtp && timeUntilExpiry > 0">
                            <Clock class="w-4 h-4" />
                            <AlertDescription>
                                OTP expires in {{ formatTime(timeUntilExpiry) }}
                            </AlertDescription>
                        </Alert>
                    </div>

                    <div class="w-full max-w-1/2">
                        <FieldGroup>
                            <form.Field name="otp_code">
                                <template #default="{ field }">
                                    <Field :data-invalid="isInvalid(field)">
                                        <InputOTP :model-value="field.state.value" :maxlength="6"
                                            :pattern="REGEXP_ONLY_DIGITS" :disabled="otpExpired"
                                            @update:model-value="field.handleChange">
                                            <InputOTPGroup>
                                                <InputOTPSlot :index="0" />
                                                <InputOTPSlot :index="1" />
                                                <InputOTPSlot :index="2" />
                                            </InputOTPGroup>
                                            <InputOTPSeparator />
                                            <InputOTPGroup>
                                                <InputOTPSlot :index="3" />
                                                <InputOTPSlot :index="4" />
                                                <InputOTPSlot :index="5" />
                                            </InputOTPGroup>
                                        </InputOTP>
                                        <FieldError v-if="isInvalid(field)" :errors="field.state.meta.errors" />
                                    </Field>
                                </template>
                            </form.Field>
                        </FieldGroup>
                    </div>
                </CardContent>

                <CardFooter class="flex flex-col justify-start items-start gap-2">
                    <span class="text-sm text-muted-foreground">
                        Please enter the 6-digit OTP code sent to your email.
                    </span>

                    <Button type="submit" class="cursor-pointer" :disabled="otpExpired || form.state.isSubmitting">
                        <span v-if="form.state.isSubmitting" class="flex items-center gap-2">
                            <Spinner />
                            Verifying...
                        </span>
                        <span v-else>Submit</span>
                    </Button>
                </CardFooter>
            </form>
        </Card>
    </div>
</template>