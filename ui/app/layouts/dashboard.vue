<script setup lang="ts">
import AppSidebar from '~/components/AppSidebar.vue';
import { SidebarProvider, SidebarTrigger } from '~/components/ui/sidebar';
import UnlockVaultModal from '~/components/UnlockVaultModal.vue';
import { errorHelper } from '~/lib/error-helper';

const { needsUnlock, user, unlockVault, logout } = useAuth();

async function handleUnlock(password: string) {
    try {
        await unlockVault(password);
    } catch (error: any) {
        await errorHelper(error)
    }
}
</script>

<template>
    <SidebarProvider>
        <AppSidebar />
        <div>
            <SidebarTrigger />
            <slot />
            <ClientOnly>
                <UnlockVaultModal v-if="needsUnlock" :userEmail="user?.email!" @unlock="handleUnlock"
                    @logout="logout" />
            </ClientOnly>
        </div>
    </SidebarProvider>
</template>
