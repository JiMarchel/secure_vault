<script setup lang="ts">
import { errorHelper } from '~/lib/error-helper';
import {
  Card,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { KeyRound } from 'lucide-vue-next';
import PasswordGroup from '~/components/input-group/PasswordGroup.vue';
import type { PasswordDecrypted, Vaults } from '~/utils/model/vault';
import { decryptVaultItem } from '~/lib/wasm/vault';
import type { updatePasswordType } from '~/utils/validation/vaults';

definePageMeta({
  layout: "dashboard",
  middleware: ["auth"],
});

const { vaults, errorVaults } = useVaults();
const { useDek } = useAuth()

if (errorVaults.value) {
  errorHelper(errorVaults.value);
}

// For edit modal
const selectedItem = ref<updatePasswordType | null>(null)
const isEditOpen = ref(false)

async function openEditModal(item: Vaults) {
  try {
    const dek = useDek() // Get DEK when needed, will trigger unlock modal if not available
    const decryptedVault = await decryptVaultItem(dek, { encryptedData: item.encryptedData, nonce: item.nonce })
    const vaultData: PasswordDecrypted = JSON.parse(decryptedVault.plaintext)
    selectedItem.value = {
      title: item.title,
      id: item.id,
      ...vaultData
    }
    isEditOpen.value = true
  } catch (error) {
    // useDek() sets needsUnlock=true, so UnlockVaultModal will show
    // No need to throw or show error toast
  }
}

</script>
<template>
  <div class="p-6 space-y-3 w-full max-w-4xl mx-auto ">
    <Card v-for="item in vaults" :key="item.id"
      class="hover:shadow-lg transition-shadow duration-200 w-full cursor-pointer" @click="openEditModal(item)">
      <CardHeader class="flex items-center gap-4">
        <KeyRound />
        <div>

          <CardTitle>{{ item.title }}</CardTitle>
          <CardDescription>{{ item.itemType }}</CardDescription>
        </div>
      </CardHeader>
    </Card>

    <!-- Edit Modal -->
    <PasswordGroup v-if="selectedItem" :key="selectedItem.id" :update="true" v-model:open="isEditOpen"
      :id="selectedItem.id" :title="selectedItem.title" :websiteOrApp="selectedItem.websiteOrApp"
      :usernameOrEmail="selectedItem.usernameOrEmail" :password="selectedItem.password" />
  </div>
</template>