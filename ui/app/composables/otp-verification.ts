import { ref, onMounted, onUnmounted, computed } from 'vue'
import { toast } from 'vue-sonner'
import { errorHelper } from '~/lib/error-helper'
import type { OtpStatus, ResendOtpResponse } from '~/utils/model/otp'
import type { SuccessResponse } from '~/utils/model/response'

export function useOtpVerification() {
  const { $api } = useNuxtApp()

  const otpStatus = ref<OtpStatus | null>(null)
  const isLoading = ref(false)
  const isResending = ref(false)
  const resendCooldown = ref(0)
  
  let countdownInterval: NodeJS.Timeout | null = null

  const canResend = computed(() => {
    return otpStatus.value?.canResend && resendCooldown.value === 0
  })

  const otpExpired = computed(() => {
    if (!otpStatus.value?.expiresAt) return false
    return new Date(otpStatus.value.expiresAt) < new Date()
  })

  const timeUntilExpiry = computed(() => {
    if (!otpStatus.value?.expiresAt) return 0
    const expiryTime = new Date(otpStatus.value.expiresAt).getTime()
    const now = new Date().getTime()
    return Math.max(0, Math.floor((expiryTime - now) / 1000))
  })

  function startCountdown(seconds: number) {
    resendCooldown.value = seconds

    if (countdownInterval) {
      clearInterval(countdownInterval)
    }

    countdownInterval = setInterval(() => {
      resendCooldown.value--
      if (resendCooldown.value <= 0) {
        clearInterval(countdownInterval!)
        countdownInterval = null
        fetchOtpStatus()
      }
    }, 1000)
  }

  async function fetchOtpStatus() {
    isLoading.value = true
    try {
      const response = await $api<SuccessResponse<OtpStatus>>(
        '/session/otp/status'
      )
      
      otpStatus.value = response.data!

      if (response.data?.resendAfter && response.data.resendAfter > 0) {
        startCountdown(response.data.resendAfter)
      }
    } catch (error) {
      await errorHelper(error)
      console.error('Failed to fetch OTP status:', error)
    } finally {
      isLoading.value = false
    }
  }

  async function resendOtp() {
    if (!canResend.value || isResending.value) return

    isResending.value = true
    try {
      const response = await $api<SuccessResponse<ResendOtpResponse>>(
        '/session/otp/resend',
        {
          method: 'PATCH',
        }
      )

      toast.success('OTP has been resent to your email!')

      if (response.data?.cooldownSeconds) {
        startCountdown(response.data.cooldownSeconds)
      }
      await fetchOtpStatus()
    } catch (error: any) {
      await errorHelper(error)
      if (error?.data?.details?.retry_after) {
        startCountdown(error.data.details.retry_after)
      }
    } finally {
      isResending.value = false
    }
  }

  async function verifyOtp(code: string) {
    try {
      await $api<SuccessResponse<void>>('/session/otp/verify', {
        method: 'POST',
        body: { otpCode: code },
      })

      toast.success('OTP verified successfully!')
      return true
    } catch (error) {
      await errorHelper(error)
      return false
    }
  }

  onMounted(() => {
    fetchOtpStatus()
  })

  onUnmounted(() => {
    if (countdownInterval) {
      clearInterval(countdownInterval)
    }
  })

  return {
    otpStatus,
    isLoading,
    isResending,
    resendCooldown,
    canResend,
    otpExpired,
    timeUntilExpiry,
    fetchOtpStatus,
    resendOtp,
    verifyOtp,
  }
}