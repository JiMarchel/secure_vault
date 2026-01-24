<script setup lang="ts">
import { CreditCard, Calendar, Lock, User, Hash, MapPin, HelpCircle, Shield, Eye, EyeOff } from 'lucide-vue-next';
import { InputGroupButton } from '../ui/input-group';
import FormDialog from '../FormDialog.vue';
import { SidebarMenuSubButton } from '../ui/sidebar';
import { Separator } from '../ui/separator';
import { useForm } from '@tanstack/vue-form';
import { addCreditCard as addCreditCardValidation } from '~/utils/validation/vaults';
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
    cardHolderName?: string
    cardNumber?: string
    cardExpirationDate?: string
    cardCvv?: string
    pin?: string
    zipOrPostalCode?: string
}

const props = withDefaults(defineProps<Props>(), {
    update: false,
    open: undefined
})

const emit = defineEmits<{
    'update:open': [value: boolean]
}>()

const { creditCard, isLoading, deleteVault } = useVaults()

const addCreditCardForm = useForm({
    defaultValues: {
        title: props.update ? (props.title ?? "") : "",
        cardHolderName: props.update ? (props.cardHolderName ?? "") : "",
        cardNumber: props.update ? (props.cardNumber ?? "") : "",
        cardExpirationDate: props.update ? (props.cardExpirationDate ?? "") : "",
        cardCvv: props.update ? (props.cardCvv ?? "") : "",
        pin: props.update ? (props.pin ?? "") : "",
        zipOrPostalCode: props.update ? (props.zipOrPostalCode ?? "") : "",
    },
    validators: {
        onSubmit: addCreditCardValidation
    },
    onSubmit: async ({ value }) => {
        try {
            await creditCard(value, props.update, props.id)
            if (!props.update) {
                addCreditCardForm.reset()
            }
        } catch (error) {
            await errorHelper(error)
        }
    },
})

const deleteCard = async (id: string) => {
    await deleteVault(id)
    emit('update:open', false)
}

const showCVV = ref(false)
const showPin = ref(false)
</script>

<template>
    <FormDialog :title="props.update ? 'Edit Credit Card' : 'Add Credit Card'"
        :description="props.update ? 'Edit your credit card details' : 'Securely store your credit card information'"
        :open="props.open" @update:open="emit('update:open', $event)" @submit="addCreditCardForm.handleSubmit"
        :loading="isLoading" :submitText="props.update ? 'Update Card' : 'Save Card'">

        <template v-if="!props.update" #trigger>
            <SidebarMenuSubButton>
                <CreditCard />
                <span>Credit Card</span>
            </SidebarMenuSubButton>
        </template>

        <Separator />

        <FieldGroup class="space-y-3">
            <div class="space-y-2">
                <addCreditCardForm.Field name="title" v-slot="{ field }">
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
                                        <p>A friendly name for this card</p>
                                    </TooltipContent>
                                </Tooltip>
                            </TooltipProvider>
                        </template>
                    </FormInput>
                </addCreditCardForm.Field>

                <FieldLabel>Card Details</FieldLabel>

                <addCreditCardForm.Field name="cardHolderName" v-slot="{ field }">
                    <FormInput :field="field" placeholder="Cardholder Name"
                        :defaultValue="props.update ? props.cardHolderName : ''">
                        <template #icon>
                            <User />
                        </template>
                    </FormInput>
                </addCreditCardForm.Field>

                <addCreditCardForm.Field name="cardNumber" v-slot="{ field }">
                    <FormInput :field="field" placeholder="Card Number"
                        :defaultValue="props.update ? props.cardNumber : ''">
                        <template #icon>
                            <CreditCard />
                        </template>
                    </FormInput>
                </addCreditCardForm.Field>

                <div class="grid grid-cols-2 gap-2">
                    <addCreditCardForm.Field name="cardExpirationDate" v-slot="{ field }">
                        <FormInput :field="field" placeholder="Expiration (MM/YY)"
                            :defaultValue="props.update ? props.cardExpirationDate : ''">
                            <template #icon>
                                <Calendar />
                            </template>
                        </FormInput>
                    </addCreditCardForm.Field>

                    <addCreditCardForm.Field name="cardCvv" v-slot="{ field }">
                        <FormInput :field="field" placeholder="CVV" :defaultValue="props.update ? props.cardCvv : ''"
                            :type="showCVV ? 'text' : 'password'">
                            <template #icon>
                                <Lock />
                            </template>
                            <template #addon>
                                <InputGroupButton variant="ghost" aria-label="Show CVV" size="icon-xs"
                                    @click="showCVV = !showCVV" type="button">
                                    <Eye v-if="showCVV" />
                                    <EyeOff v-else />
                                </InputGroupButton>
                            </template>
                        </FormInput>
                    </addCreditCardForm.Field>
                </div>

                <div class="grid grid-cols-2 gap-2">
                    <addCreditCardForm.Field name="pin" v-slot="{ field }">
                        <FormInput :field="field" placeholder="PIN" :defaultValue="props.update ? props.pin : ''"
                            :type="showPin ? 'text' : 'password'">
                            <template #icon>
                                <Hash />
                            </template>
                            <template #addon>
                                <InputGroupButton variant="ghost" aria-label="Show PIN" size="icon-xs"
                                    @click="showPin = !showPin" type="button">
                                    <Eye v-if="showPin" />
                                    <EyeOff v-else />
                                </InputGroupButton>
                            </template>
                        </FormInput>
                    </addCreditCardForm.Field>

                    <addCreditCardForm.Field name="zipOrPostalCode" v-slot="{ field }">
                        <FormInput :field="field" placeholder="Zip / Postal Code"
                            :defaultValue="props.update ? props.zipOrPostalCode : ''">
                            <template #icon>
                                <MapPin />
                            </template>
                        </FormInput>
                    </addCreditCardForm.Field>
                </div>

            </div>
        </FieldGroup>

        <DeleteModal :id="props.id" :title="props.title" @delete="deleteCard" v-if="props.update" />

    </FormDialog>
</template>
