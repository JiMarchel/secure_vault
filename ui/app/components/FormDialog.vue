<script setup lang="ts">
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger } from './ui/dialog';
import { Button } from './ui/button';
import { Spinner } from './ui/spinner';

const props = withDefaults(defineProps<{
    title: string
    description?: string
    loading?: boolean
    submitText?: string
    disabled?: boolean
    open?: boolean
}>(), {
    submitText: 'Submit',
    open: undefined
})

const emit = defineEmits<{
    submit: []
    'update:open': [value: boolean]
}>()

const isOpen = ref(props.open ?? false)

// Sync with external v-model
watch(() => props.open, (val) => {
    if (val !== undefined) isOpen.value = val
})

function updateOpen(val: boolean) {
    isOpen.value = val
    emit('update:open', val)
}
</script>

<template>
    <Dialog :open="isOpen" @update:open="updateOpen">
        <DialogTrigger v-if="$slots.trigger" as-child>
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
