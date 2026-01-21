<script setup lang="ts">
import { computed } from 'vue'
import { InputGroup, InputGroupAddon, InputGroupInput } from '../input-group'
import { Field, FieldError } from '.'

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const props = defineProps<{
    field: any
    placeholder?: string
    label?: string
    autocomplete?: string
}>()

const isInvalid = computed(() => {
    return props.field.state.meta.isTouched && !props.field.state.meta.isValid
})
</script>

<template>
    <Field :data-invalid="isInvalid">
        <slot name="label" />
        <InputGroup>
            <InputGroupAddon v-if="$slots.icon" align="inline-start">
                <slot name="icon" />
            </InputGroupAddon>
            <InputGroupInput :id="field.name" :placeholder="placeholder" :name="field.name" :model-value="field.state.value"
                :aria-invalid="isInvalid" :autocomplete="autocomplete ?? 'off'" @blur="field.handleBlur"
                @input="field.handleChange(($event.target as HTMLInputElement).value)" />
            <InputGroupAddon v-if="$slots.addon" align="inline-end">
                <slot name="addon" />
            </InputGroupAddon>
        </InputGroup>
        <FieldError v-if="isInvalid" :errors="field.state.meta.errors" />
    </Field>
</template>
