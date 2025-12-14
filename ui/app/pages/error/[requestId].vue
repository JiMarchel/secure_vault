<script setup lang="ts">
import { AlertTriangle, Copy, Home as HomeIcon, RefreshCcw } from 'lucide-vue-next'
import { Button } from '../../components/ui/button'

const route = useRoute()
const router = useRouter()

const requestId = computed(() => String(route.params.requestId ?? ''))
const message = computed(() => String(route.query.message ?? 'Unexpected server error'))

const copied = ref(false)
async function copyRequestId() {
    try {
        await navigator.clipboard.writeText(requestId.value)
        copied.value = true
        setTimeout(() => (copied.value = false), 1500)
    } catch {
        // no-op: clipboard may be unavailable
    }
}

function goHome() {
    router.push('/')
}

function tryAgain() {
    router.back()
}
</script>

<template>
    <section class="min-h-[60vh] flex items-center justify-center px-4 md:px-8">
        <div class="w-full max-w-2xl rounded-xl border bg-card text-card-foreground shadow-sm">
            <div class="flex items-start gap-4 p-6">
                <div class="flex h-12 w-12 items-center justify-center rounded-lg bg-destructive/10 text-destructive">
                    <AlertTriangle class="h-7 w-7" />
                </div>
                <div class="flex-1">
                    <h1 class="text-xl md:text-2xl font-semibold">Something went wrong (5xx)</h1>
                    <p class="mt-2 text-sm text-muted-foreground">
                        {{ message }}
                    </p>

                    <div class="mt-4 grid gap-2 text-sm">
                        <div class="flex items-center gap-2">
                            <span class="font-medium">Request ID:</span>
                            <code class="rounded bg-muted px-2 py-1 text-xs">{{ requestId }}</code>
                            <Button variant="ghost" size="sm" class="gap-2" @click="copyRequestId">
                                <Copy class="h-4 w-4" />
                                <span v-if="!copied">Copy</span>
                                <span v-else>Copied</span>
                            </Button>
                        </div>
                        <p class="text-muted-foreground">Please share this ID with support to help diagnose the issue.</p>
                    </div>

                    <div class="mt-6 flex flex-wrap items-center gap-3">
                        <Button class="bg-primary hover:bg-primary/80" @click="tryAgain">
                            <RefreshCcw class="mr-2 h-4 w-4" /> Try Again
                        </Button>
                        <Button variant="outline" @click="goHome">
                            <HomeIcon class="mr-2 h-4 w-4" /> Go Home
                        </Button>
                    </div>
                </div>
            </div>
        </div>
    </section>
</template>