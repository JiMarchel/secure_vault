<script setup lang="ts">
import Button from "@/components/ui/button/Button.vue";
import HomeNavbar from "@/components/HomeNavbar.vue";
import { CheckCircle, Shield } from "lucide-vue-next";
import Badge from "~/components/ui/badge/Badge.vue";

definePageMeta({
  middleware: ["check-session", "guest"],
});
</script>

<template>
  <main class="flex-1">
    <nav>
      <HomeNavbar />
    </nav>
    <section class="w-full py-12 md:py-24 lg:py-44">
      <div class=" px-4 md:px-6">
        <div class="flex flex-col items-center space-y-4 text-center">
          <Badge class="flex items-center" variant="secondary">
            <Shield class="h-4 mr-1 w-5" variant="" />
            Kom***nfo level encryption
          </Badge>
          <div class="space-y-2">
            <h1 class="text-3xl font-bold tracking-tighter sm:text-4xl md:text-5xl lg:text-6xl/none">
              Your passwords, <span class="text-primary">perfectly secure</span>
            </h1>
            <p class="mx-auto max-w-175 md:text-xl">
              Generate, store, and autofill strong passwords across all your devices. SecureVault keeps your digital
              life protected with military-grade encryption.
            </p>
          </div>
          <div class="space-x-4 mt-8">
            <Button variant="outline" size="lg">
              Watch Demo
            </Button>
          </div>
          <div class="flex items-center space-x-4 text-sm mt-4">
            <div class="flex items-center">
              <CheckCircle class="w-4 h-4 text-primary mr-1" />
              14-day free trial
            </div>
            <div class="flex items-center">
              <CheckCircle class="w-4 h-4 text-primary mr-1" />
              No credit card required
            </div>
          </div>
        </div>
      </div>
    </section>
  </main>
</template>
