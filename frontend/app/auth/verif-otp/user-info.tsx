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
import { Input } from "@/components/ui/input";
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
import { useActionState } from "react";

let otpInitialvalue: OtpVerifActionResponse = {};

interface UserInfoProps {
  id: string | undefined;
  username: string;
  email: string;
  otpExpiresAt: string;
}

export const UserInfo = ({
  username,
  email,
  otpExpiresAt,
  id,
}: UserInfoProps) => {
  const [otpState, otpFormAction, otpPending] = useActionState(
    verifOtpAction,
    otpInitialvalue
  );

  const { countdown, start } = useCountDown();
  const now = new Date();
  const expiresAt = new Date(otpExpiresAt);
  const difference = expiresAt.getTime() - now.getTime();

  const handleResendAction = async (formData: FormData) => {
    start(60);
    await updateVerifOtp(null, formData);
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
            <form action={handleResendAction}>
              <Input
                type="text"
                defaultValue={id}
                name="id"
                className="hidden"
                readOnly
              />
              <Button variant="link" type="submit" disabled={countdown > 0}>
                Resend OTP {countdown > 0 ? `in ${countdown}` : null}
              </Button>
            </form>
          </CardAction>
        </CardHeader>
        <form action={otpFormAction} className="space-y-6">
          <CardContent className="flex justify-center flex-col items-center gap-4">
            <Badge className="px-2 py-1">
              <Mail /> {email}
            </Badge>
            <input name="id" value={id} className="hidden" readOnly/>
            <InputOTP
              maxLength={6}
              pattern={REGEXP_ONLY_DIGITS}
              name="otp"
              defaultValue={otpState.inputs?.otp_code}
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
            <span className="text-red-500 text-sm">{otpState.errors?.otp_code}</span>
          </CardContent>
          <CardFooter className="flex flex-col justify-start items-start">
            {difference < 0 ? (
              <span className="text-red-500">
                OTP expired, please request a new one
              </span>
            ) : (
              <span className="text-emerald-500">
                Your OTP is expired in 5 minutes
              </span>
            )}

            <span className="text-red-500">{otpState.messageApi}</span>
            <span className="text-sm text-muted-foreground">
              Please enter the one-time password sent to your email.
            </span>
            <Button type="submit" disabled={otpPending}>
              Submit
            </Button>
          </CardFooter>
        </form>
      </Card>
    </div>
  );
};
