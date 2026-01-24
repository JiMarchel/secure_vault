<script setup lang="ts">
import { errorHelper } from '~/lib/error-helper';
import { useDebounceFn } from '@vueuse/core'
import {
  Card,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { FileKey2, KeyRound, CreditCard, Contact, ChevronRight, Search, X } from 'lucide-vue-next';
import PasswordGroup from '~/components/input-group/PasswordGroup.vue';
import type { Vaults } from '~/utils/model/vault';
import { decryptVaultItem } from '~/lib/wasm/vault';
import NoteGroup from '~/components/input-group/NoteGroup.vue';
import CreditCardGroup from '~/components/input-group/CreditCardGroup.vue';
import ContactGroup from '~/components/input-group/ContactGroup.vue';

const getItemStyle = (type: string) => {
  switch (type) {
    case 'Password': return { color: 'text-blue-500', bg: 'bg-blue-500/10', border: 'group-hover:border-blue-500/50', hoverColor: 'hover:text-blue-500' }
    case 'CreditCard': return { color: 'text-purple-500', bg: 'bg-purple-500/10', border: 'group-hover:border-purple-500/50', hoverColor: 'hover:text-purple-500' }
    case 'Contact': return { color: 'text-emerald-500', bg: 'bg-emerald-500/10', border: 'group-hover:border-emerald-500/50', hoverColor: 'hover:text-emerald-500' }
    case 'Note': return { color: 'text-amber-500', bg: 'bg-amber-500/10', border: 'group-hover:border-amber-500/50', hoverColor: 'hover:text-amber-500' }
    default: return { color: 'text-gray-500', bg: 'bg-gray-500/10', border: 'group-hover:border-gray-500/50', hoverColor: 'hover:text-gray-500' }
  }
}

definePageMeta({
  layout: "dashboard",
  middleware: ["auth"],
});

const { vaults, errorVaults, searchVaults, isLoading } = useVaults();
const { useDek } = useAuth()

const searchQuery = ref('')
const searchResults = ref<Vaults[]>([])
const isSearching = ref(false)

if (errorVaults.value) {
  errorHelper(errorVaults.value);
}

const selectedItem = ref<any>(null)
const isEditOpen = ref(false)

const displayedVaults = computed(() => {
  return searchQuery.value.trim() ? searchResults.value : vaults.value || []
})

const performSearch = useDebounceFn(async () => {
  if (!searchQuery.value.trim()) {
    searchResults.value = []
    isSearching.value = false
    return
  }

  isSearching.value = true
  try {
    const results = await searchVaults(searchQuery.value)
    searchResults.value = results!
  } catch (error) {
    console.error('Search error:', error)
  } finally {
    isSearching.value = false
  }
}, 300)

watch(searchQuery, () => {
  performSearch()
})
const clearSearch = () => {
  searchQuery.value = ''
  searchResults.value = []
}

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
    console.error('Decrypt error:', error)
  }
}

</script>

<template>
  <div class="p-6 w-full max-w-7xl mx-auto">
    <!-- Search Bar -->
    <div class="mb-6">
      <div class="relative">
        <Search class="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-muted-foreground" />
        <input
          v-model="searchQuery"
          type="text"
          placeholder="Search vaults by title..."
          class="w-full pl-10 pr-10 py-3 border border-border rounded-lg bg-card/50 focus:outline-none focus:ring-2 focus:ring-primary transition-all"
        />
        <button
          v-if="searchQuery"
          @click="clearSearch"
          class="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground transition-colors"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Search Info -->
      <div v-if="searchQuery" class="mt-2 text-sm text-muted-foreground flex items-center gap-2">
        <span v-if="isSearching">Searching...</span>
        <span v-else-if="searchResults.length > 0">
          Found {{ searchResults.length }} result{{ searchResults.length !== 1 ? 's' : '' }}
        </span>
        <span v-else-if="!isSearching && searchResults.length === 0">
          No results found for "{{ searchQuery }}"
        </span>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="isSearching || isLoading" class="flex justify-center items-center h-64">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
    </div>

    <!-- Empty State -->
    <div v-else-if="!displayedVaults || displayedVaults.length === 0" class="text-center py-16">
      <div class="inline-flex items-center justify-center w-16 h-16 rounded-full bg-muted mb-4">
        <Search class="w-8 h-8 text-muted-foreground" />
      </div>
      <h3 class="text-lg font-semibold mb-2">
        {{ searchQuery ? 'No results found' : 'No vaults yet' }}
      </h3>
      <p class="text-muted-foreground">
        {{ searchQuery ? `Try searching with different keywords` : 'Create your first vault to get started' }}
      </p>
    </div>

    <!-- Vaults Grid -->
    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      <Card 
        v-for="item in displayedVaults" 
        :key="item.id"
        class="group relative overflow-hidden border border-border/50 bg-card/30 hover:bg-card/50 transition-all duration-300 hover:shadow-lg hover:-translate-y-1 cursor-pointer"
        :class="getItemStyle(item.itemType).border + ' ' + getItemStyle(item.itemType).hoverColor"
        @click="openEditModal(item)"
      >
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

    <!-- Edit Modals -->
    <PasswordGroup 
      v-if="selectedItem && selectedItem.itemType === 'Password'" 
      :key="selectedItem.id" 
      :update="true"
      v-model:open="isEditOpen" 
      :id="selectedItem.id" 
      :title="selectedItem.title"
      :websiteOrApp="selectedItem.websiteOrApp" 
      :usernameOrEmail="selectedItem.usernameOrEmail"
      :password="selectedItem.password" 
    />

    <NoteGroup 
      v-if="selectedItem && selectedItem.itemType === 'Note'" 
      :key="selectedItem.id" 
      :update="true"
      v-model:open="isEditOpen" 
      :id="selectedItem.id" 
      :title="selectedItem.title" 
      :note="selectedItem.note" 
    />

    <CreditCardGroup 
      v-if="selectedItem && selectedItem.itemType === 'CreditCard'" 
      :key="selectedItem.id" 
      :update="true"
      v-model:open="isEditOpen" 
      :id="selectedItem.id" 
      :title="selectedItem.title"
      :cardHolderName="selectedItem.cardHolderName" 
      :cardNumber="selectedItem.cardNumber"
      :zipOrPostalCode="selectedItem.zipOrPostalCode" 
      :cardExpirationDate="selectedItem.cardExpirationDate"
      :cardType="selectedItem.cardType" 
      :cardCvv="selectedItem.cardCvv" 
      :pin="selectedItem.pin" 
    />

    <ContactGroup 
      v-if="selectedItem && selectedItem.itemType === 'Contact'" 
      :key="selectedItem.id" 
      :update="true"
      v-model:open="isEditOpen" 
      :id="selectedItem.id" 
      :title="selectedItem.title" 
      :fullName="selectedItem.fullName"
      :email="selectedItem.email" 
      :phoneNumber="selectedItem.phoneNumber" 
      :address="selectedItem.address"
      :city="selectedItem.city" 
      :state="selectedItem.state" 
      :country="selectedItem.country" 
    />
  </div>
</template>