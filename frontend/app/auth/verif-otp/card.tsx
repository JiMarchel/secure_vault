"use client";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  InputOTP,
  InputOTPGroup,
  InputOTPSeparator,
  InputOTPSlot,
} from "@/components/ui/input-otp";
import useCountDown from "@/hooks/useCountDown";
import { updateVerifOtp } from "@/lib/actions/update-verif-otp";
import { verifOtpAction } from "@/lib/actions/verif-otp";
import { OtpVerifActionResponse } from "@/lib/types";
import { REGEXP_ONLY_DIGITS } from "input-otp";
import { Mail } from "lucide-react";
import { useActionState, useState } from "react";

let otpInitialvalue: OtpVerifActionResponse = {};

type VerifOtpProp = {
  otp_code: string;
  otp_expires_at: string;
  username: string;
  email: string;
  id: string;
  cookieString: string;
};

type VerifyOtpFormProps = {
  id: string;
  email: string;
  difference: number;
};

const VerifyOtpForm = ({ id, email, difference }: VerifyOtpFormProps) => {
  const [otpState, otpAction, isOtpPending] = useActionState(
    verifOtpAction,
    otpInitialvalue
  );

  return (
    <form action={otpAction} className="space-y-6">
      <CardContent className="flex justify-center flex-col items-center gap-4">
        <Badge className="px-2 py-1">
          <Mail /> {email}
        </Badge>
        {/* <input name="id" defaultValue={id} className="hidden" readOnly /> */}
        <InputOTP
          maxLength={6}
          pattern={REGEXP_ONLY_DIGITS}
          name="otp_code"
          disabled={isOtpPending}
        >
          <InputOTPGroup>
            <InputOTPSlot index={0} />
            <InputOTPSlot index={1} />
            <InputOTPSlot index={2} />
          </InputOTPGroup>
          <InputOTPSeparator />
          <InputOTPGroup>
            <InputOTPSlot index={3} />
            <InputOTPSlot index={4} />
            <InputOTPSlot index={5} />
          </InputOTPGroup>
        </InputOTP>
        {otpState.errors?.otp_code && (
          <span className="text-red-500 text-sm">
            {otpState.errors.otp_code[0]}
          </span>
        )}
        {otpState.messageApi && (
          <span className="text-red-500 text-sm">{otpState.messageApi}</span>
        )}
      </CardContent>
      <CardFooter className="flex flex-col justify-start items-start">
        {difference < 0 ? (
          <span className="text-red-500">
            OTP expired, please request a new one
          </span>
        ) : (
          <span className="text-emerald-500">
            Your OTP expires in 10 minutes
          </span>
        )}
        <span className="text-sm text-muted-foreground">
          Please enter the OTP code sent to your email.
        </span>
        <Button type="submit" disabled={isOtpPending || difference < 0}>
          {isOtpPending ? (
            <>
              <span className="animate-spin mr-2">⏳</span>
              Verifying...
            </>
          ) : (
            "Submit"
          )}
        </Button>
      </CardFooter>
    </form>
  );
};

export const VerifOtpCard = ({
  otp_code,
  otp_expires_at,
  username,
  email,
  id,
  cookieString,
}: VerifOtpProp) => {
  const { countdown, start } = useCountDown();
  const [isResending, setIsResending] = useState(false);
  const now = new Date();
  const expiresAt = new Date(otp_expires_at);
  const difference = expiresAt.getTime() - now.getTime();

  const handleResendAction = async () => {
    try {
      setIsResending(true);
      start(120);
      await updateVerifOtp(null, cookieString);
    } catch (error) {
      console.error("Failed to resend OTP:", error);
    } finally {
      setIsResending(false);
    }
  };

  return (
    <div className="fixed top-1/2 left-1/2 -translate-y-1/2 -translate-x-1/2 w-xl bg-gray-50 shadow-2xl">
      <Card className="w-full max-w-xl shadow-lg">
        <CardHeader>
          <CardTitle className="text-2xl">Verify OTP</CardTitle>
          <CardDescription>
            Hello <b className="text-primary">{username}</b>, please enter the
            OTP sent to your email, and please don't leave this page until you
            have verified your account.
          </CardDescription>
          <CardAction>
            <Button
              variant="link"
              type="button"
              disabled={countdown > 0 || isResending}
              onClick={handleResendAction}
            >
              {isResending ? (
                <>
                  <span className="animate-spin mr-2">⏳</span>
                  Sending...
                </>
              ) : countdown > 0 ? (
                `Resend OTP in ${countdown}`
              ) : (
                "Resend OTP"
              )}
            </Button>
          </CardAction>
        </CardHeader>
        <VerifyOtpForm id={id} email={email} difference={difference} />
      </Card>
    </div>
  );
};
