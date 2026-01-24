<script setup lang="ts">
import { HelpCircle, Shield, StickyNote } from 'lucide-vue-next';
import { InputGroup, InputGroupButton, InputGroupTextarea } from '../ui/input-group';
import FormDialog from '../FormDialog.vue';
import { SidebarMenuSubButton } from '../ui/sidebar';
import { Separator } from '../ui/separator';
import { useForm } from '@tanstack/vue-form';
import { addNote as addNoteValidation } from '~/utils/validation/vaults';
import { FieldGroup, FieldLabel, FormInput, Field, FieldError } from '../ui/field';
import { Tooltip, TooltipProvider, TooltipTrigger, TooltipContent } from '../ui/tooltip';
import { errorHelper } from '~/lib/error-helper';
import DeleteModal from './DeleteModal.vue';
import { useVaults } from '~/composables/use-vault';

interface Props {
    update?: boolean
    open?: boolean
    id?: string
    title?: string
    note?: string
}

const props = withDefaults(defineProps<Props>(), {
    update: false,
    open: undefined
})

const emit = defineEmits<{
    'update:open': [value: boolean]
}>()

const { note, isLoading, deleteVault } = useVaults()

const addNoteForm = useForm({
    defaultValues: {
        title: props.update ? (props.title ?? "") : "",
        note: props.update ? (props.note ?? "") : "",
    },
    validators: {
        onSubmit: addNoteValidation
    },
    onSubmit: async ({ value }) => {
        try {
            await note(value, props.update, props.id)
            if (!props.update) {
                addNoteForm.reset()
            }
        } catch (error) {
            await errorHelper(error)
        }
    },
})

const deleteNote = async (id: string) => {
    await deleteVault(id)
    emit('update:open', false)
}

function isInvalid(field: any) {
    return field.state.meta.isTouched && !field.state.meta.isValid
}
</script>

<template>
    <FormDialog :title="props.update ? 'Edit Note' : 'Add Note'"
        :description="props.update ? 'Edit your note' : 'Securely save your private notes'" :open="props.open"
        @update:open="emit('update:open', $event)" @submit="addNoteForm.handleSubmit" :loading="isLoading"
        :submitText="props.update ? 'Update Note' : 'Save Note'">

        <template v-if="!props.update" #trigger>
            <SidebarMenuSubButton>
                <StickyNote />
                <span>Secure Note</span>
            </SidebarMenuSubButton>
        </template>

        <Separator />

        <FieldGroup class="space-y-3">
            <div class="space-y-2">
                <addNoteForm.Field name="title" v-slot="{ field }">
                    <FormInput :field="field" placeholder="Title*" :defaultValue="props.update ? props.title : ''">
                        <template #icon>
                            <Shield />
                        </template>
                        <template #addon>
                            <TooltipProvider>
                                <Tooltip>
                                    <TooltipTrigger as-child>
                                        <InputGroupButton variant="ghost" aria-label="Help" size="icon-xs">
                                            <HelpCircle />
                                        </InputGroupButton>
                                    </TooltipTrigger>
                                    <TooltipContent>
                                        <p>Title for your note</p>
                                    </TooltipContent>
                                </Tooltip>
                            </TooltipProvider>
                        </template>
                    </FormInput>
                </addNoteForm.Field>

                <addNoteForm.Field name="note" v-slot="{ field }">
                    <Field :data-invalid="isInvalid(field)">
                        <FieldLabel>Note</FieldLabel>
                        <InputGroup>
                            <InputGroupTextarea :id="field.name" placeholder="Your private note..." :name="field.name"
                                :model-value="field.state.value" :aria-invalid="isInvalid(field)"
                                @blur="field.handleBlur"
                                @input="field.handleChange(($event.target as HTMLTextAreaElement).value)"
                                class="min-h-[150px]" />
                        </InputGroup>
                        <FieldError v-if="isInvalid(field)" :errors="field.state.meta.errors" />
                    </Field>
                </addNoteForm.Field>
            </div>
        </FieldGroup>

        <DeleteModal :id="props.id" :title="props.title" @delete="deleteNote" v-if="props.update" />

    </FormDialog>
</template>