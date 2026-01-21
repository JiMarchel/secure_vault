<script setup lang="ts">
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger } from './ui/dialog';
import { Button } from './ui/button';
import { Spinner } from './ui/spinner';
import { InputGroup, InputGroupAddon, InputGroupButton, InputGroupInput } from './ui/input-group';
import { HelpCircle, Shield } from 'lucide-vue-next';
import { Tooltip, TooltipProvider, TooltipTrigger } from './ui/tooltip';

withDefaults(defineProps<{
    title: string
    description?: string
    loading?: boolean
    submitText?: string
    // If true, the submit button is disabled
    disabled?: boolean
}>(), {
    submitText: 'Submit'
})

const emit = defineEmits(['submit'])
</script>

<template>
    <Dialog>
        <DialogTrigger as-child>
            <slot name="trigger" />
        </DialogTrigger>
        <DialogContent class="sm:max-w-md">
            <DialogHeader>
                <DialogTitle>{{ title }}</DialogTitle>
                <DialogDescription v-if="description">
                    {{ description }}
                </DialogDescription>
            </DialogHeader>

            <form class="space-y-4" @submit.prevent="emit('submit')">
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
                <slot />

                <slot name="footer">
                    <Button type="submit" class="w-full" :disabled="loading || disabled">
                        <Spinner v-if="loading" />
                        {{ submitText }}
                    </Button>
                </slot>
            </form>
        </DialogContent>
    </Dialog>
</template>
