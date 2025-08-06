"use client";
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
import { verifOtpAction } from "@/lib/actions/verif-otp";
import { OtpVerifActionResponse } from "@/lib/types";
import { REGEXP_ONLY_DIGITS } from "input-otp";
import {  use, useActionState } from "react";


let initialvalue: OtpVerifActionResponse = {};

export const UserInfo = ({}) => {
  const [state, formAction, pending] = useActionState(
    verifOtpAction,
    initialvalue
  );

  return (
    <div className="fixed top-1/2 left-1/2 -translate-y-1/2 -translate-x-1/2 w-xl bg-gray-50 shadow-2xl">
      <Card className="w-full max-w-xl shadow-lg">
        <CardHeader>
          <CardTitle className="text-2xl">Verify OTP</CardTitle>
          <CardDescription>
            Please enter the OTP sent to your email, and please don't leave this
            page until you have verified your account.
          </CardDescription>
          {/* <CardAction>Resend OTP</CardAction> */}
        </CardHeader>
        <form action={formAction} className="space-y-6">
          <CardContent className="flex justify-center flex-col items-center gap-2">
            <InputOTP
              maxLength={6}
              pattern={REGEXP_ONLY_DIGITS}
              name="otp"
              defaultValue={state.inputs?.otp}
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
            <span className="text-red-500 text-sm">{state.errors?.otp}</span>
          </CardContent>
          <CardFooter className="flex flex-col justify-start items-start">
            <span className="text-red-500">{state.messageApi}</span>
            <span className="text-sm text-muted-foreground">
              Please enter the one-time password sent to your email.
            </span>
            <Button type="submit">Submit</Button>
          </CardFooter>
        </form>
      </Card>
    </div>
  );
};


