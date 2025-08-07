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
import { ShieldAlert } from "lucide-react";
import Cookies from "js-cookie";
import { useRouter } from "next/navigation";

export default function Error({
  error,
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  const err = error.message.split("&&");
  const router = useRouter();

  function handleReset() {
    Cookies.remove("sc-verif-otp");
    router.push("/");
  }

  return (
    <div className="w-full bg-emerald-200 h-screen flex items-center justify-center">
      <Card className="w-xl border-red-500 border-2">
        <CardHeader className="text-red-500 flex flex-col items-center text-2xl justify-center gap-2">
          <CardTitle className="flex gap-2">
            <ShieldAlert /> Something went wrong!
          </CardTitle>
          <CardDescription className="text-red-500">{err[1]}</CardDescription>
        </CardHeader>
        <CardContent>
          <p>{err[0]}</p>
        </CardContent>
        <CardFooter>
          <Button onClick={() => handleReset()} className="cursor-pointer">
            Restart registration
          </Button>
        </CardFooter>
      </Card>
    </div>
  );
}
