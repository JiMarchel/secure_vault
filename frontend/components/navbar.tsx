"use client";
import { Shield } from "lucide-react";
import Link from "next/link";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "./ui/dialog";
import { Button } from "./ui/button";
import { Label } from "./ui/label";
import { Input } from "./ui/input";
import { useActionState } from "react";
import { signUpAction } from "@/lib/actions/sign-up";
import { signUpActionResponse } from "@/lib/types";
import { usePathname } from "next/navigation";
import { cn } from "@/lib/utils";

let initialState: signUpActionResponse = {};

export const Navbar = () => {
  const [state, formAction, pending] = useActionState(
    signUpAction,
    initialState
  );

  const pathname = usePathname();
  return (
    <header className="sticky top-0 z-50 w-full border-b bg-white/95 backdrop-blur supports-backdrop-filter:bg-white/60">
      <div className="flex h-16 items-center justify-between px-4 md:px-6">
        <Link className="flex items-center space-x-2" href="/">
          <Shield className="h-8 w-8 text-emerald-600" />
          <span className="text-2xl font-bold text-gray-900">SecureVault</span>
        </Link>

        <div
          className={cn("flex items-center space-x-3", {
            hidden: pathname !== "/",
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

                <div className="flex items-center justify-between">
                  <Link
                    href="#"
                    className="text-sm text-emerald-600 hover:text-emerald-700"
                  >
                    Forgot password?
                  </Link>
                </div>
                <Button
                  type="submit"
                  className="w-full bg-emerald-600 hover:bg-emerald-700"
                >
                  Sign In
                </Button>
              </form>
            </DialogContent>
          </Dialog>

          <Dialog>
            <DialogTrigger asChild>
              <Button
                size="sm"
                className="bg-emerald-600 hover:bg-emerald-700 cursor-pointer"
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
               <form className="space-y-4" action={formAction}>
                <div className="space-y-2">
                  <Label htmlFor="username">Username</Label>
                  <Input
                    id="username"
                    type="text"
                    placeholder="Enter your username"
                    required
                    name="username"
                    defaultValue={state.inputs?.username}
                    disabled={pending}
                  />
                  <span className="text-red-500 text-sm py-0">
                    {state?.errors?.username}
                  </span>
                </div>
                <div className="space-y-2">
                  <Label htmlFor="email">Email</Label>
                  <Input
                    id="email"
                    type="email"
                    placeholder="Enter your email"
                    required
                    name="email"
                    defaultValue={state.inputs?.email}
                    disabled={pending}
                  />
                  <span className="text-red-500 text-sm py-0">
                    {state?.errors?.email}
                  </span>
                </div>
                <p className="text-red-500">
                  {state.messageApi}
                </p>
                <Button type="submit" disabled={pending}>
                  {pending ? "Signing Up..." : "Sign Up"}
                </Button>
              </form>
            </DialogContent>
          </Dialog>
        </div>
      </div>
    </header>
  );
};
