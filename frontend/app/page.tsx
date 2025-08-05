import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { CheckCircle, Shield } from "lucide-react";

export default function Home() {
  return (
   <main className="flex-1">
    <section className="w-full py-12 md:py-24 lg:py-44">
          <div className=" px-4 md:px-6">
            <div className="flex flex-col items-center space-y-4 text-center">
              <Badge  className="mb-4">
                <Shield className="w-3 h-3 mr-1" />
                Bank-level encryption
              </Badge>
              <div className="space-y-2">
                <h1 className="text-3xl font-bold tracking-tighter sm:text-4xl md:text-5xl lg:text-6xl/none">
                  Your passwords, <span className="text-emerald-600">perfectly secure</span>
                </h1>
                <p className="mx-auto max-w-[700px] text-gray-600 md:text-xl">
                  Generate, store, and autofill strong passwords across all your devices. SecureVault keeps your digital
                  life protected with military-grade encryption.
                </p>
              </div>
              <div className="space-x-4 mt-8">
                <Button variant="outline" size="lg">
                  Watch Demo
                </Button>
              </div>
              <div className="flex items-center space-x-4 text-sm text-gray-500 mt-4">
                <div className="flex items-center">
                  <CheckCircle className="w-4 h-4 text-emerald-600 mr-1" />
                  14-day free trial
                </div>
                <div className="flex items-center">
                  <CheckCircle className="w-4 h-4 text-emerald-600 mr-1" />
                  No credit card required
                </div>
              </div>
            </div>
          </div>
        </section>
   </main>
  );
}
