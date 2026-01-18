<script setup lang="ts">
import { LockOpen, CheckCircle, XCircle, Loader2, Home as HomeIcon, RefreshCcw } from 'lucide-vue-next';
import { toast } from 'vue-sonner';
import { Button } from '~/components/ui/button';
import type { SuccessResponse } from '~/utils/model/response';

definePageMeta({
  layout: 'default',
});

const route = useRoute();
const router = useRouter();
const config = useRuntimeConfig();

const token = computed(() => route.params.token as string);

const status = ref<'loading' | 'success' | 'error'>('loading');
const errorMessage = ref('');
const successMessage = ref('');

async function unlockAccount() {
  status.value = 'loading';

  try {
    const response = await $fetch<SuccessResponse<null>>(
      `${config.public.apiBaseUrl}/auth/unlock-account`,
      {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: { token: token.value },
      }
    );

    status.value = 'success';
    successMessage.value = response.message || 'Account unlocked successfully!';
    toast.success(successMessage.value);
  } catch (error: unknown) {
    status.value = 'error';

    if (error && typeof error === 'object' && 'data' in error) {
      const errData = error.data as { message?: string };
      errorMessage.value = errData.message || 'Failed to unlock account';
    } else {
      errorMessage.value = 'Failed to unlock account. The token may be invalid or expired.';
    }

    toast.error(errorMessage.value);
  }
}

function goHome() {
  router.push('/');
}

onMounted(() => {
  if (token.value) {
    unlockAccount();
  } else {
    status.value = 'error';
    errorMessage.value = 'Invalid unlock link. No token provided.';
  }
});
</script>

<template>
  <section class="min-h-[60vh] flex items-center justify-center px-4 md:px-8">
    <div class="w-full max-w-2xl rounded-xl border bg-card text-card-foreground shadow-sm">
      <div class="flex items-start gap-4 p-6">
        <!-- Icon -->
        <div
          v-if="status === 'loading'"
          class="flex h-12 w-12 items-center justify-center rounded-lg bg-muted"
        >
          <Loader2 class="h-7 w-7 animate-spin text-muted-foreground" />
        </div>
        <div
          v-else-if="status === 'success'"
          class="flex h-12 w-12 items-center justify-center rounded-lg bg-green-500/10 text-green-500"
        >
          <CheckCircle class="h-7 w-7" />
        </div>
        <div
          v-else
          class="flex h-12 w-12 items-center justify-center rounded-lg bg-destructive/10 text-destructive"
        >
          <XCircle class="h-7 w-7" />
        </div>

        <!-- Content -->
        <div class="flex-1">
          <h1 class="text-xl md:text-2xl font-semibold">
            <span v-if="status === 'loading'">Unlocking Account...</span>
            <span v-else-if="status === 'success'">Account Unlocked</span>
            <span v-else>Unlock Failed</span>
          </h1>
          <p class="mt-2 text-sm text-muted-foreground">
            <span v-if="status === 'loading'">Please wait while we unlock your account.</span>
            <span v-else-if="status === 'success'">{{ successMessage }}</span>
            <span v-else>{{ errorMessage }}</span>
          </p>

          <div v-if="status === 'success'" class="mt-4 flex items-center gap-2 text-sm text-muted-foreground">
            <LockOpen class="h-4 w-4" />
            <span>You can now log in again</span>
          </div>

          <div class="mt-6 flex flex-wrap items-center gap-3">
            <template v-if="status === 'success'">
              <Button class="bg-primary hover:bg-primary/80" @click="goHome">
                <HomeIcon class="mr-2 h-4 w-4" /> Go to Login
              </Button>
            </template>
            <template v-else-if="status === 'error'">
              <Button class="bg-primary hover:bg-primary/80" @click="unlockAccount">
                <RefreshCcw class="mr-2 h-4 w-4" /> Try Again
              </Button>
              <Button variant="outline" @click="goHome">
                <HomeIcon class="mr-2 h-4 w-4" /> Back to Login
              </Button>
            </template>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>
