<script setup lang="ts">
import { Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupContent, SidebarGroupLabel, SidebarMenu, SidebarMenuButton, SidebarMenuItem, SidebarMenuSub, SidebarMenuSubButton, SidebarMenuSubItem } from '@/components/ui/sidebar'
import { ChevronUp, CirclePlus, CirclePlusIcon, Contact, CreditCard, FileKey, KeyRound, User2 } from 'lucide-vue-next';
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from './ui/dropdown-menu';
import { Spinner } from './ui/spinner';
import { Button } from './ui/button';
import PasswordGroup from './input-group/PasswordGroup.vue';

const sidebarItems = [
    {
        title: "Add Vault",
        icon: CirclePlus,
        items: [
            {
                title: 'Password',
                icon: KeyRound,
                description: 'Add a new password entry to your vault.'
            },
            {
                title: 'Secure Note',
                url: '#',
                icon: FileKey,
                description: 'Save a secure note that only you can read.'
            },
            {
                title: 'Credit Card',
                icon: CreditCard,
                description: 'Store your credit card information securely.'
            },
            {
                title: 'Contact Info',
                icon: Contact,
                description: 'Keep important contact details safe.'
            },
        ]
    },
]

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
                                <CirclePlusIcon/>
                                <span>Add Vault</span>
                            </SidebarMenuButton>
                            <SidebarMenuSub>
                                <SidebarMenuSubItem>
                                    <PasswordGroup />
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
                                        size="sm">Sign out</Button>
                                </ClientOnly>
                            </DropdownMenuItem>
                        </DropdownMenuContent>
                    </DropdownMenu>
                </SidebarMenuItem>
            </SidebarMenu>
        </SidebarFooter>
    </Sidebar>
</template>
