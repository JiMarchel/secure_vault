<script setup lang="ts">
import AppSidebar from '~/components/AppSidebar.vue';
import { SidebarProvider, SidebarTrigger } from '~/components/ui/sidebar';



</script>

<template>
    <SidebarProvider>
        <AppSidebar />
        <div>
            <SidebarTrigger />
            <slot />
        </div>
    </SidebarProvider>
</template>
