<script setup lang="ts">
import { Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupContent, SidebarGroupLabel, SidebarMenu, SidebarMenuButton, SidebarMenuItem, SidebarMenuSub, SidebarMenuSubItem } from '@/components/ui/sidebar'
import { ChevronUp, CirclePlusIcon, User2 } from 'lucide-vue-next';
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from './ui/dropdown-menu';
import { Spinner } from './ui/spinner';
import { Button } from './ui/button';
import PasswordGroup from './input-group/PasswordGroup.vue';
import NoteGroup from './input-group/NoteGroup.vue';
import CreditCardGroup from './input-group/CreditCardGroup.vue';
import ContactGroup from './input-group/ContactGroup.vue';

const { user, logout, isLoading } = useAuth()

</script>

<template>
    <Sidebar>
        <SidebarContent>
            <SidebarGroup>
                <SidebarGroupLabel>Add new</SidebarGroupLabel>
                <SidebarGroupContent>
                    <SidebarMenu>
                        <SidebarMenuItem>
                            <SidebarMenuButton>
                                <CirclePlusIcon />
                                <span>Add Vault</span>
                            </SidebarMenuButton>
                            <SidebarMenuSub>
                                <SidebarMenuSubItem class="space-y-3 cursor-default">
                                    <PasswordGroup />
                                    <NoteGroup />
                                    <CreditCardGroup />
                                    <ContactGroup />
                                </SidebarMenuSubItem>
                            </SidebarMenuSub>
                        </SidebarMenuItem>
                    </SidebarMenu>
                </SidebarGroupContent>
            </SidebarGroup>
        </SidebarContent>
        <!-- rest of file unchanged -->
        <SidebarFooter>
            <SidebarMenu>
                <SidebarMenuItem>
                    <DropdownMenu>
                        <DropdownMenuTrigger as-child>
                            <SidebarMenuButton>
                                <User2 />
                                <ClientOnly>
                                    {{ user?.username }}
                                    <template #fallback>
                                        <Spinner />
                                    </template>
                                </ClientOnly>
                                <ChevronUp class="ml-auto" />
                            </SidebarMenuButton>
                        </DropdownMenuTrigger>
                        <DropdownMenuContent side="top" class="w-(--reka-popper-anchor-width)">
                            <DropdownMenuItem>
                                <span>Account</span>
                            </DropdownMenuItem>
                            <DropdownMenuItem>
                                <ClientOnly>
                                    <Spinner v-if="isLoading" />
                                    <Button v-else @click="logout" class="cursor-pointer w-full" variant="destructive"
                                        size="sm">Sign
                                        out</Button>
                                </ClientOnly>
                            </DropdownMenuItem>
                        </DropdownMenuContent>
                    </DropdownMenu>
                </SidebarMenuItem>
            </SidebarMenu>
        </SidebarFooter>
    </Sidebar>
</template>
