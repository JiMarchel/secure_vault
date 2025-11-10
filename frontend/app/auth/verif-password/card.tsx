"use client";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { verifOtpAction } from "@/lib/actions/verif-otp";
import { verifPasswordAction } from "@/lib/actions/verif-password";
import { createUserIdentifier } from "@/lib/crypto/vault";
import {
  OtpVerifActionResponse,
  VerifPasswordActionResponse,
} from "@/lib/types";
import { Eye, EyeClosed, ShieldCheck } from "lucide-react";
import { useActionState, useState } from "react";

let passwordInitialValue: VerifPasswordActionResponse = {};

interface VerifPasswordCardProps {
  id: string | undefined;
  username: string;
}

export const VerifPasswordCard = ({ id, username }: VerifPasswordCardProps) => {
  const userIdentifier = createUserIdentifier("ASDasd123@").then((v) => v);
  console.log({ userIdentifier });
  const [isHidden, setIsHidden] = useState(true);
  const [state, action, pending] = useActionState(
    verifPasswordAction,
    passwordInitialValue
  );

  const passwordError = state.errors?.password || "";
  const confirmPasswordError = state.errors?.confirm_password || "";
  const splitErrors = [...passwordError, ...confirmPasswordError];


  return (
    <div className="fixed top-1/2 left-1/2 -translate-y-1/2 -translate-x-1/2 w-xl bg-gray-50 shadow-2xl">
      <Card className="w-full max-w-xl shadow-lg">
        <CardHeader>
          <CardTitle className="text-2xl">Creating Master Password</CardTitle>
          <CardDescription>
            Hello <b className="text-primary">{username}</b> this is the last
            step, please create you master password, and please don't leave this
            page until you have created your master password.
          </CardDescription>
        </CardHeader>
        <form action={action} className="space-y-6">
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
            <input name="id" value={id} className="hidden" readOnly />
            <Label>Master Password</Label>
            <div className="flex gap-2">
              <Input
                type={isHidden ? "password" : "text"}
                name="password"
                placeholder="Enter your master password"
                required
                className="w-full"
                disabled={pending}
                defaultValue={state.inputs?.password}
              />
              <Button
                size="icon"
                onClick={() => setIsHidden(!isHidden)}
                type="button"
              >
                {isHidden ? <EyeClosed /> : <Eye />}
              </Button>
            </div>
            <Label>Confirm Master Password</Label>
            <Input
              type={isHidden ? "password" : "text"}
              name="confirm_password"
              placeholder="Confirm your master password"
              required
              className="w-full"
              disabled={pending}
              defaultValue={state.inputs?.confirm_password}
            />
            {splitErrors.map((v: string) => (
              <span key={v} className="text-red-500 text-sm">
                {v}
              </span>
            ))}
          </CardContent>
          <CardFooter className="flex flex-col justify-start items-start">
            <span className="text-red-500">{state.messageApi}</span>
            <Button type="submit" disabled={pending}>
              Submit
            </Button>
          </CardFooter>
        </form>
      </Card>
    </div>
  );
};
