<script setup lang="ts">
import { errorHelper } from '~/lib/error-helper';
import {
  Card,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { FileKey2, KeyRound, CreditCard, Contact, ChevronRight } from 'lucide-vue-next';
import PasswordGroup from '~/components/input-group/PasswordGroup.vue';

const getItemStyle = (type: string) => {
  switch (type) {
    case 'Password': return { color: 'text-blue-500', bg: 'bg-blue-500/10', border: 'group-hover:border-blue-500/50', hoverColor: 'hover:text-blue-500' }
    case 'CreditCard': return { color: 'text-purple-500', bg: 'bg-purple-500/10', border: 'group-hover:border-purple-500/50', hoverColor: 'hover:text-purple-500' }
    case 'Contact': return { color: 'text-emerald-500', bg: 'bg-emerald-500/10', border: 'group-hover:border-emerald-500/50', hoverColor: 'hover:text-emerald-500' }
    case 'Note': return { color: 'text-amber-500', bg: 'bg-amber-500/10', border: 'group-hover:border-amber-500/50', hoverColor: 'hover:text-amber-500' }
    default: return { color: 'text-gray-500', bg: 'bg-gray-500/10', border: 'group-hover:border-gray-500/50', hoverColor: 'hover:text-gray-500' }
  }
}
import type { Vaults } from '~/utils/model/vault';
import { decryptVaultItem } from '~/lib/wasm/vault';
import NoteGroup from '~/components/input-group/NoteGroup.vue';
import CreditCardGroup from '~/components/input-group/CreditCardGroup.vue';
import ContactGroup from '~/components/input-group/ContactGroup.vue';

definePageMeta({
  layout: "dashboard",
  middleware: ["auth"],
});

const { vaults, errorVaults } = useVaults();
const { useDek } = useAuth()

if (errorVaults.value) {
  errorHelper(errorVaults.value);
}

const selectedItem = ref<any>(null)
const isEditOpen = ref(false)

async function openEditModal(item: Vaults) {
  try {
    const dek = useDek()
    const decryptedVault = await decryptVaultItem(dek, { encryptedData: item.encryptedData, nonce: item.nonce })
    const vaultData = JSON.parse(decryptedVault.plaintext)
    console.log(vaultData)
    selectedItem.value = {
      title: item.title,
      id: item.id,
      itemType: item.itemType,
      ...vaultData
    }
    isEditOpen.value = true
  } catch (error) {
  }
}

</script>
<template>
  <div class="p-6 w-full max-w-7xl mx-auto">
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 ">
      <Card v-for="item in vaults" :key="item.id"
        class="group relative overflow-hidden border border-border/50 bg-card/30 hover:bg-card/50 transition-all duration-300 hover:shadow-lg hover:-translate-y-1 cursor-pointer"
        :class="getItemStyle(item.itemType).border + ' ' + getItemStyle(item.itemType).hoverColor"
        @click="openEditModal(item)">
        <CardHeader class="flex flex-row items-center gap-4 p-5">
          <div class="p-3 rounded-xl transition-colors duration-300" :class="getItemStyle(item.itemType).bg">
            <KeyRound v-if="item.itemType === 'Password'" class="w-6 h-6" :class="getItemStyle(item.itemType).color" />
            <CreditCard v-else-if="item.itemType === 'CreditCard'" class="w-6 h-6"
              :class="getItemStyle(item.itemType).color" />
            <Contact v-else-if="item.itemType === 'Contact'" class="w-6 h-6"
              :class="getItemStyle(item.itemType).color" />
            <FileKey2 v-else class="w-6 h-6" :class="getItemStyle(item.itemType).color" />
          </div>

          <div class="flex-1 min-w-0">
            <CardTitle class="text-base font-semibold truncate">
              {{ item.title }}
            </CardTitle>
            <CardDescription class="text-xs capitalize mt-1 flex items-center gap-2">
              <span class="w-1.5 h-1.5 rounded-full" :class="getItemStyle(item.itemType).bg.replace('/10', '')"></span>
              {{ item.itemType }}
            </CardDescription>
          </div>

          <ChevronRight
            class="w-5 h-5 text-muted-foreground opacity-0 -translate-x-2 group-hover:opacity-100 group-hover:translate-x-0 transition-all duration-300" />
        </CardHeader>
      </Card>
    </div>

    <PasswordGroup v-if="selectedItem && selectedItem.itemType === 'Password'" :key="selectedItem.id" :update="true"
      v-model:open="isEditOpen" :id="selectedItem.id" :title="selectedItem.title"
      :websiteOrApp="selectedItem.websiteOrApp" :usernameOrEmail="selectedItem.usernameOrEmail"
      :password="selectedItem.password" />

    <NoteGroup v-if="selectedItem && selectedItem.itemType === 'Note'" :key="selectedItem.id" :update="true"
      v-model:open="isEditOpen" :id="selectedItem.id" :title="selectedItem.title" :note="selectedItem.note" />

    <CreditCardGroup v-if="selectedItem && selectedItem.itemType === 'CreditCard'" :key="selectedItem.id" :update="true"
      v-model:open="isEditOpen" :id="selectedItem.id" :title="selectedItem.title"
      :cardHolderName="selectedItem.cardHolderName" :cardNumber="selectedItem.cardNumber"
      :zipOrPostalCode="selectedItem.zipOrPostalCode" :cardExpirationDate="selectedItem.cardExpirationDate"
      :cardType="selectedItem.cardType" :cardCvv="selectedItem.cardCvv" :pin="selectedItem.pin" />

    <ContactGroup v-if="selectedItem && selectedItem.itemType === 'Contact'" :key="selectedItem.id" :update="true"
      v-model:open="isEditOpen" :id="selectedItem.id" :title="selectedItem.title" :fullName="selectedItem.fullName"
      :email="selectedItem.email" :phoneNumber="selectedItem.phoneNumber" :address="selectedItem.address"
      :city="selectedItem.city" :state="selectedItem.state" :country="selectedItem.country" />
  </div>
</template>