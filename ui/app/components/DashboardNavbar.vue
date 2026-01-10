<script lang="ts" setup>
import type { Button } from './ui/button';
import { useAuth } from '~/composable/auth';
import { Spinner } from './ui/spinner';

const {logout, isLoading} = useAuth()

</script>

<template>
      <header class="sticky top-0 z-50 w-full border-b py-2">
        <div class="flex justify-between mx-8 ">
            <Button>
                wk
            </Button>
            <Button :disabled="isLoading" class="cursor-pointer" @click="logout">
                <Spinner v-if="isLoading"/>
                Logout
            </Button>
        </div>
    </header>
</template>