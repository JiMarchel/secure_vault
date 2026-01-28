<script setup lang="ts">
import HomeNavbar from "@/components/HomeNavbar.vue";
import { Shield, Lock, Key, Database } from "lucide-vue-next";
import Badge from "~/components/ui/badge/Badge.vue";

definePageMeta({
  middleware: ["check-session", "guest"],
});

const techStack = [
  { name: "Rust + Axum", desc: "Backend API" },
  { name: "PostgreSQL + Redis", desc: "Database" },
  { name: "Nuxt 4", desc: "Frontend" },
  { name: "Rust WASM", desc: "Client-side Encryption" },
];
</script>

<template>
  <main class="flex-1 min-h-screen">
    <nav>
      <HomeNavbar />
    </nav>

    <section class="w-full py-16 min-h-screen md:py-28 lg:py-36 relative overflow-hidden flex flex-col justify-center">
      <div class="absolute inset-0 -z-10">
        <div class="absolute top-1/4 left-1/4 w-96 h-96 bg-primary/10 rounded-full blur-3xl"></div>
        <div class="absolute bottom-1/4 right-1/4 w-80 h-80 bg-primary/5 rounded-full blur-3xl"></div>
      </div>

      <div class="container mx-auto px-4 md:px-6">
        <!-- Hero Content -->
        <div class="flex flex-col items-center space-y-6 text-center mb-20">
          <Badge class="flex items-center gap-1.5 px-4 py-1.5" variant="secondary">
            <Shield class="h-4 w-4" />
            Project Skripsi
          </Badge>
          <div class="space-y-4 max-w-3xl">
            <h1 class="text-4xl font-bold tracking-tight sm:text-5xl md:text-6xl lg:text-7xl">
              Secure<span class="text-primary">Vault</span>
            </h1>
            <p class="text-xl md:text-2xl text-muted-foreground font-medium">
              Password Manager with Zero-Knowledge Architecture
            </p>
            <p class="mx-auto max-w-2xl text-muted-foreground md:text-lg leading-relaxed">
              Aplikasi manajemen password yang mengutamakan keamanan dengan enkripsi multi-layer.
              Master password tidak pernah dikirim ke server, memastikan hanya Anda yang dapat mengakses data anda.
            </p>
          </div>

          <div class="flex flex-wrap justify-center gap-3 mt-8">
            <Badge v-for="tech in techStack" :key="tech.name" variant="outline" class="px-3 py-1.5 text-sm">
              <span class="font-semibold">{{ tech.name }}</span>
              <span class="text-muted-foreground ml-1.5">{{ tech.desc }}</span>
            </Badge>
          </div>
        </div>

        <!-- Architecture Section (merged) -->
        <div class="max-w-4xl mx-auto">
          <div class="text-center mb-10">
            <h2 class="text-2xl font-bold tracking-tight sm:text-3xl mb-3">
              Arsitektur Keamanan
            </h2>
            <p class="text-muted-foreground text-sm">
              Bagaimana SecureVault melindungi data Anda
            </p>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-3 gap-5">
            <div class="flex flex-col items-center text-center p-5 rounded-xl bg-muted/50 border border-border/50">
              <div class="w-14 h-14 rounded-full bg-primary/10 flex items-center justify-center mb-3">
                <Key class="w-7 h-7 text-primary" />
              </div>
              <h3 class="font-semibold mb-2">1. Derivasi Kunci</h3>
              <p class="text-sm text-muted-foreground">
                Master password + salt di-hash menggunakan <span class="text-foreground font-medium">Argon2id</span>
                untuk menghasilkan DEK
              </p>
            </div>

            <div class="flex flex-col items-center text-center p-5 rounded-xl bg-muted/50 border border-border/50">
              <div class="w-14 h-14 rounded-full bg-primary/10 flex items-center justify-center mb-3">
                <Lock class="w-7 h-7 text-primary" />
              </div>
              <h3 class="font-semibold mb-2">2. Enkripsi WASM</h3>
              <p class="text-sm text-muted-foreground">
                Data dienkripsi dengan <span class="text-foreground font-medium">XChaCha20-Poly1305</span> via Rust WASM
                di browser
              </p>
            </div>

            <div class="flex flex-col items-center text-center p-5 rounded-xl bg-muted/50 border border-border/50">
              <div class="w-14 h-14 rounded-full bg-primary/10 flex items-center justify-center mb-3">
                <Database class="w-7 h-7 text-primary" />
              </div>
              <h3 class="font-semibold mb-2">3. Zero-Knowledge</h3>
              <p class="text-sm text-muted-foreground">
                Server hanya menyimpan <span class="text-foreground font-medium">ciphertext</span>, tidak dapat membaca
                isi vault
              </p>
            </div>
          </div>
        </div>
      </div>
    </section>

    <footer class="w-full py-8 border-t">
      <div class="container mx-auto px-4 md:px-6 text-center text-muted-foreground">
        <p class="text-sm">
          SecureVault — Project Password Manager
        </p>
      </div>
    </footer>
  </main>
</template>
