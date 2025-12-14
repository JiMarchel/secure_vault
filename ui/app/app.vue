<script setup lang="ts">
import 'vue-sonner/style.css'
import { Toaster } from '~/components/ui/sonner';

</script>

<template>
    <Toaster rich-colors position="top-center" />
    <NuxtLayout>
        <NuxtPage />
    </NuxtLayout>
</template>
