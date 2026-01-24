<script setup lang="ts">
import { Contact, Mail, MapPin, Phone, User, Globe, HelpCircle, Shield } from 'lucide-vue-next';
import { InputGroup, InputGroupButton } from '../ui/input-group';
import FormDialog from '../FormDialog.vue';
import { SidebarMenuSubButton } from '../ui/sidebar';
import { Separator } from '../ui/separator';
import { useForm } from '@tanstack/vue-form';
import { addContact as addContactValidation } from '~/utils/validation/vaults';
import { FieldGroup, FieldLabel, FormInput } from '../ui/field';
import { Tooltip, TooltipProvider, TooltipTrigger, TooltipContent } from '../ui/tooltip';
import { errorHelper } from '~/lib/error-helper';
import DeleteModal from './DeleteModal.vue';
import { useVaults } from '~/composables/use-vault';

interface Props {
    update?: boolean
    open?: boolean
    id?: string
    title?: string
    fullName?: string
    email?: string
    phoneNumber?: string
    address?: string
    city?: string
    state?: string
    country?: string
}

const props = withDefaults(defineProps<Props>(), {
    update: false,
    open: undefined
})

const emit = defineEmits<{
    'update:open': [value: boolean]
}>()

const { contact, isLoading, deleteVault } = useVaults()

const addContactForm = useForm({
    defaultValues: {
        title: props.update ? (props.title ?? "") : "",
        fullName: props.update ? (props.fullName ?? "") : "",
        email: props.update ? (props.email ?? "") : "",
        phoneNumber: props.update ? (props.phoneNumber ?? "") : "",
        address: props.update ? (props.address ?? "") : "",
        city: props.update ? (props.city ?? "") : "",
        state: props.update ? (props.state ?? "") : "",
        country: props.update ? (props.country ?? "") : "",
    },
    validators: {
        onSubmit: addContactValidation
    },
    onSubmit: async ({ value }) => {
        try {
            await contact(value, props.update, props.id)
            if (!props.update) {
                addContactForm.reset()
            }
        } catch (error) {
            await errorHelper(error)
        }
    },
})

const deleteContact = async (id: string) => {
    await deleteVault(id)
    emit('update:open', false)
}
</script>

<template>
    <FormDialog :title="props.update ? 'Edit Contact' : 'Add Contact'"
        :description="props.update ? 'Edit contact details' : 'Store contact information securely'" :open="props.open"
        @update:open="emit('update:open', $event)" @submit="addContactForm.handleSubmit" :loading="isLoading"
        :submitText="props.update ? 'Update Contact' : 'Save Contact'">

        <template v-if="!props.update" #trigger>
            <SidebarMenuSubButton>
                <Contact />
                <span>Contact Info</span>
            </SidebarMenuSubButton>
        </template>

        <Separator />

        <FieldGroup class="space-y-3">
            <div class="space-y-2">
                <addContactForm.Field name="title" v-slot="{ field }">
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
                                        <p>A friendly name for this contact</p>
                                    </TooltipContent>
                                </Tooltip>
                            </TooltipProvider>
                        </template>
                    </FormInput>
                </addContactForm.Field>

                <FieldLabel>Identity</FieldLabel>

                <addContactForm.Field name="fullName" v-slot="{ field }">
                    <FormInput :field="field" placeholder="Full Name"
                        :defaultValue="props.update ? props.fullName : ''">
                        <template #icon>
                            <User />
                        </template>
                    </FormInput>
                </addContactForm.Field>

                <div class="grid grid-cols-2 gap-2">
                    <addContactForm.Field name="email" v-slot="{ field }">
                        <FormInput :field="field" placeholder="Email" :defaultValue="props.update ? props.email : ''">
                            <template #icon>
                                <Mail />
                            </template>
                        </FormInput>
                    </addContactForm.Field>

                    <addContactForm.Field name="phoneNumber" v-slot="{ field }">
                        <FormInput :field="field" placeholder="Phone Number"
                            :defaultValue="props.update ? props.phoneNumber : ''">
                            <template #icon>
                                <Phone />
                            </template>
                        </FormInput>
                    </addContactForm.Field>
                </div>

                <FieldLabel>Address</FieldLabel>

                <addContactForm.Field name="address" v-slot="{ field }">
                    <FormInput :field="field" placeholder="Address" :defaultValue="props.update ? props.address : ''">
                        <template #icon>
                            <MapPin />
                        </template>
                    </FormInput>
                </addContactForm.Field>

                <div class="grid grid-cols-2 gap-2">
                    <addContactForm.Field name="city" v-slot="{ field }">
                        <FormInput :field="field" placeholder="City" :defaultValue="props.update ? props.city : ''">
                        </FormInput>
                    </addContactForm.Field>

                    <addContactForm.Field name="state" v-slot="{ field }">
                        <FormInput :field="field" placeholder="State/Province"
                            :defaultValue="props.update ? props.state : ''">
                        </FormInput>
                    </addContactForm.Field>
                </div>

                <addContactForm.Field name="country" v-slot="{ field }">
                    <FormInput :field="field" placeholder="Country" :defaultValue="props.update ? props.country : ''">
                        <template #icon>
                            <Globe />
                        </template>
                    </FormInput>
                </addContactForm.Field>

            </div>
        </FieldGroup>

        <DeleteModal :id="props.id" :title="props.title" @delete="deleteContact" v-if="props.update" />

    </FormDialog>
</template>
