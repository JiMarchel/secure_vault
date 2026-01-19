<script lang="ts" setup>
import type { Button } from './ui/button';
import { Spinner } from './ui/spinner';

const { logout, isLoading, user } = useAuth()

</script>

<template>
    <header class="sticky top-0 z-50 w-full border-b py-2">
        <div class="flex justify-between mx-8 ">
            <Button>
                <ClientOnly>
                    {{ user?.username }}
                    <template #fallback>
                        <Spinner />
                    </template>
                </ClientOnly>
            </Button>
            <Button :disabled="isLoading" class="cursor-pointer" @click="logout">
                <Spinner v-if="isLoading" />
                Logout
            </Button>
        </div>
    </header>
</template>