<script lang="ts" setup>
import { HelpCircle, Shield } from 'lucide-vue-next';
import { InputGroup, InputGroupAddon, InputGroupInput } from '../ui/input-group';
import { Label } from '../ui/label';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '../ui/tooltip';
import PasswordGroup from './PasswordGroup.vue';

defineProps({
    title: String
})

</script>

<template>
    <InputGroup>
        <InputGroupAddon align="inline-start">
            <Shield />
        </InputGroupAddon>
        <InputGroupInput name="title" placeholder="Title*" />
        <InputGroupAddon align="inline-end">
            <TooltipProvider>
                <Tooltip>
                    <TooltipTrigger as-child>
                        <InputGroupButton variant="ghost" aria-label="Help" size="icon-xs">
                            <HelpCircle />
                        </InputGroupButton>
                    </TooltipTrigger>
                    <TooltipContent>
                        <p>Name for your vault</p>
                    </TooltipContent>
                </Tooltip>
            </TooltipProvider>
        </InputGroupAddon>
    </InputGroup>

    <PasswordGroup v-if="title === 'Password'" />

    <template v-else-if="title === 'Secure Note'">
        <div class="grid gap-2">
            <Label>Note</Label>
            <textarea
                class="flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
                placeholder="Type your note here..."></textarea>
        </div>
    </template>

    <template v-else-if="title === 'Credit Card'">
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

    <template v-else-if="title === 'Contact Info'">
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
</template>
