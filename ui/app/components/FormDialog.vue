<script setup lang="ts">
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger } from './ui/dialog';
import { Button } from './ui/button';
import { Spinner } from './ui/spinner';

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
