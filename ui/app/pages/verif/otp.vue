<script setup lang="ts">
import { useForm } from '@tanstack/vue-form';
import { Mail } from 'lucide-vue-next';
import { REGEXP_ONLY_DIGITS } from 'vue-input-otp';
import { Badge } from '~/components/ui/badge';
import { Button } from '~/components/ui/button';
import { Card, CardAction, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '~/components/ui/card';
import { Field, FieldError, FieldGroup } from '~/components/ui/field';
import type { User } from '~/utils/model/user';
import { verifOtp } from '~/utils/validation/auth';

definePageMeta({
    layout: "verif",
})

const userData: User | undefined = inject("userData");
// const refreshUserData = inject("refreshUserData")

const COOLDOWN_KEY = 'otp_resend_expires_at'
const resendCooldown = ref(0)
const isResending = ref(false)

// Restore countdown dari localStorage saat mount, atau set default 60s jika pertama kali
onMounted(() => {
    const expiresAt = localStorage.getItem(COOLDOWN_KEY)
    if (expiresAt) {
        const remaining = Math.max(0, Math.floor((parseInt(expiresAt) - Date.now()) / 1000))
        if (remaining > 0) {
            resendCooldown.value = remaining
            startCountdown()
        } else {
            localStorage.removeItem(COOLDOWN_KEY)
        }
    } else {
        // Pertama kali masuk halaman = user baru dapat email, set countdown 60s
        resendCooldown.value = 60
        const expiresAt = Date.now() + (60 * 1000)
        localStorage.setItem(COOLDOWN_KEY, expiresAt.toString())
        startCountdown()
    }
})

function startCountdown() {
    const interval = setInterval(() => {
        resendCooldown.value--
        if (resendCooldown.value <= 0) {
            clearInterval(interval)
            localStorage.removeItem(COOLDOWN_KEY)
        }
    }, 1000)
}

async function resendOtp() {
    if (resendCooldown.value > 0 || isResending.value) return

    isResending.value = true
    try {
        // TODO: panggil API resend OTP
        // await $fetch('/api/auth/resend-otp', { method: 'POST' })

        // Set countdown 60 detik & simpan timestamp expired
        resendCooldown.value = 60
        const expiresAt = Date.now() + (60 * 1000)
        localStorage.setItem(COOLDOWN_KEY, expiresAt.toString())

        startCountdown()
    } catch (error) {
        console.error('Failed to resend OTP:', error)
    } finally {
        isResending.value = false
    }
}

const form = useForm({
    defaultValues: {
        otp_code: "",
    },
    validators: {
        onSubmit: verifOtp
    },
    onSubmit: async ({ value }) => {
        console.log(value)

    },
})



// eslint-disable-next-line @typescript-eslint/no-explicit-any
function isInvalid(field: any) {
    return field.state.meta.isTouched && !field.state.meta.isValid
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
                    <Button variant="link" type="button" :disabled="resendCooldown > 0 || isResending"
                        @click="resendOtp">
                        <span v-if="isResending">Sending...</span>
                        <span v-else-if="resendCooldown > 0">Resend in {{ resendCooldown }}s</span>
                        <span v-else>Resend OTP</span>
                    </Button>
                </CardAction>
            </CardHeader>

            <form class="space-y-6" @submit.prevent="form.handleSubmit">
                <CardContent class="flex justify-center flex-col items-center gap-4">
                    <Badge class="px-2 py-1">
                        <Mail /> {{ userData?.email }}
                    </Badge>
                    <div class="w-full max-w-1/2">
                        <FieldGroup>
                            <form.Field name="otp_code">
                                <template #default="{ field }">

                                    <Field :data-invalid="isInvalid(field)">
                                        <InputOTP :model-value="field.state.value" :maxlength="6"
                                            :pattern="REGEXP_ONLY_DIGITS" @update:model-value="field.handleChange">
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

                <CardFooter class="flex flex-col justify-start items-start">
                    <!-- <ClientOnly
              fallback={
                <span class="text-gray-400">
                  Loading expiration status...
                </span>
              }
            >
              <ExpirationStatus expiresAt={data?.data?.otpExpiresAt} />
            </ClientOnly> -->

                    <span class="text-sm text-muted-foreground">
                        Please enter the OTP code sent to your email.
                    </span>

                    <Button type="submit" class="cursor-pointer">
                        Submit
                        <!-- {isPendingOtpVerif ? (
                <>
                  <span class="animate-spin mr-2">⏳</span>
                  Verifying...
                </>
              ) : (
                'Submit'
              )} -->
                    </Button>
                </CardFooter>
            </form>
        </Card>
    </div>

</template>