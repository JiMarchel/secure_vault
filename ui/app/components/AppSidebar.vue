<script setup lang="ts">
import { Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupContent, SidebarGroupLabel, SidebarMenu, SidebarMenuButton, SidebarMenuItem } from '@/components/ui/sidebar'
import { ChevronUp, Contact, CreditCard, FileKey, KeyRound, User2 } from 'lucide-vue-next';
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from './ui/dropdown-menu';
import { Spinner } from './ui/spinner';
import { Button } from './ui/button';
import { Input } from './ui/input';
import { Label } from './ui/label';
import FormDialog from './FormDialog.vue';

const addNewItems = [
    {
        title: 'Password',
        url: '#',
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
        url: '#',
        icon: CreditCard,
        description: 'Store your credit card information securely.'
    },
    {
        title: 'Contact Info',
        url: '#',
        icon: Contact,
        description: 'Keep important contact details safe.'
    },
]

const { user, logout, isLoading } = useAuth()

function handleSubmit(title: string) {
    console.log('Submitting', title)
    // TODO: Implement submission logic
}
</script>

<template>
    <Sidebar>
        <SidebarContent>
            <SidebarGroup>
                <SidebarGroupLabel>Add new</SidebarGroupLabel>
                <SidebarGroupContent>
                    <SidebarMenu>
                        <SidebarMenuItem v-for="item in addNewItems" :key="item.title">
                            <FormDialog :title="`Add ${item.title}`" :description="item.description"
                                @submit="handleSubmit(item.title)">
                                <template #trigger>
                                    <SidebarMenuButton>
                                        <component :is="item.icon" />
                                        <span>{{ item.title }}</span>
                                    </SidebarMenuButton>
                                </template>

                                <div class="grid gap-4 py-4">
                                    <div class="grid gap-2">
                                        <Label>Name</Label>
                                        <Input placeholder="e.g. My Personal Account" />
                                    </div>

                                    <!-- Dynamic Fields based on Type -->
                                    <template v-if="item.title === 'Password'">
                                        <div class="grid gap-2">
                                            <Label>Username</Label>
                                            <Input placeholder="username" />
                                        </div>
                                        <div class="grid gap-2">
                                            <Label>Password</Label>
                                            <Input type="password" placeholder="********" />
                                        </div>
                                        <div class="grid gap-2">
                                            <Label>Website</Label>
                                            <Input placeholder="https://example.com" />
                                        </div>
                                    </template>

                                    <template v-else-if="item.title === 'Secure Note'">
                                        <div class="grid gap-2">
                                            <Label>Note</Label>
                                            <textarea
                                                class="flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
                                                placeholder="Type your note here..."></textarea>
                                        </div>
                                    </template>

                                    <template v-else-if="item.title === 'Credit Card'">
                                        <div class="grid gap-2">
                                            <Label>Card Number</Label>
                                            <Input placeholder="0000 0000 0000 0000" />
                                        </div>
                                        <div class="grid grid-cols-2 gap-4">
                                            <div class="grid gap-2">
                                                <Label>Expiry Date</Label>
                                                <Input placeholder="MM/YY" />
                                            </div>
                                            <div class="grid gap-2">
                                                <Label>CVV</Label>
                                                <Input placeholder="123" />
                                            </div>
                                        </div>
                                        <div class="grid gap-2">
                                            <Label>Cardholder Name</Label>
                                            <Input placeholder="John Doe" />
                                        </div>
                                    </template>

                                    <template v-else-if="item.title === 'Contact Info'">
                                        <div class="grid gap-2">
                                            <Label>Full Name</Label>
                                            <Input placeholder="John Doe" />
                                        </div>
                                        <div class="grid gap-2">
                                            <Label>Address</Label>
                                            <Input placeholder="123 Main St" />
                                        </div>
                                        <div class="grid gap-2">
                                            <Label>Phone</Label>
                                            <Input placeholder="+1 234 567 890" />
                                        </div>
                                    </template>
                                </div>
                            </FormDialog>
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
