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
import { Field, FieldError, FieldLabel } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { createUserIdentifier } from "@/lib/crypto/vault";
import { verifPasswordSchema } from "@/lib/schemas/verif-password";
import { zodResolver } from "@hookform/resolvers/zod";
import { Eye, EyeClosed } from "lucide-react";
import { useState } from "react";
import { Controller, useForm } from "react-hook-form";
import { z } from "zod";

interface VerifPasswordCardProps {
  username: string;
}

export const VerifPasswordCard = ({ username }: VerifPasswordCardProps) => {
  const form = useForm<z.infer<typeof verifPasswordSchema>>({
    resolver: zodResolver(verifPasswordSchema),
    defaultValues: {
      password: "",
      confirmPassword: "",
    },
  });

  const {
    formState: { isSubmitting },
  } = form;

  async function onSubmit(data: z.infer<typeof verifPasswordSchema>) {
    console.log(data);
    const userIdentifier = await createUserIdentifier("ASDasd123@");
    console.log(userIdentifier);
  }

  const [isHidden, setIsHidden] = useState(true);

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
        <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
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
            <Controller
              name="password"
              control={form.control}
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor="password">Master Password</FieldLabel>
                  <div className="flex gap-2">
                    <Input
                      {...field}
                      id="password"
                      aria-invalid={fieldState.invalid}
                      placeholder="Your master password here."
                      autoComplete="off"
                      type={isHidden ? "password" : "text"}
                    />
                    <Button
                      size="icon"
                      onClick={() => setIsHidden(!isHidden)}
                      type="button"
                    >
                      {isHidden ? <EyeClosed /> : <Eye />}
                    </Button>
                  </div>
                  {fieldState.invalid && (
                    <FieldError errors={[fieldState.error]} />
                  )}
                </Field>
              )}
            />

            <Controller
              name="confirmPassword"
              control={form.control}
              render={({ field, fieldState }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel htmlFor="confirmPassword">
                    Master Password
                  </FieldLabel>
                  <Input
                    {...field}
                    id="confirmPassword"
                    aria-invalid={fieldState.invalid}
                    placeholder="Confirm your master password here."
                    autoComplete="off"
                    type={isHidden ? "password" : "text"}
                  />
                  {fieldState.invalid && (
                    <FieldError errors={[fieldState.error]} />
                  )}
                </Field>
              )}
            />
          </CardContent>
          <CardFooter className="flex flex-col justify-start items-start">
            <Button type="submit" disabled={isSubmitting}>
              Submit
            </Button>
          </CardFooter>
        </form>
      </Card>
    </div>
  );
};
