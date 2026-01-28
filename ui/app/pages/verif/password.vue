<script setup lang="ts">
import { useForm } from '@tanstack/vue-form';
import { Eye, EyeClosed } from 'lucide-vue-next';
import { toast } from 'vue-sonner';
import { Button } from '~/components/ui/button';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '~/components/ui/card';
import { FieldGroup } from '~/components/ui/field';
import { errorHelper } from '~/lib/error-helper';
import { encryptUserIdentifier } from '~/lib/wasm/vault';
import type { User } from '~/utils/model/user';
import { verifPassword } from '~/utils/validation/auth';

definePageMeta({
    layout: "verif",
    middleware: ["check-session", "signup"]
})

const config = useRuntimeConfig()
const userData: User | undefined = inject("userData");
const isPasswordViewed = ref(false);

const form = useForm({
    defaultValues: {
        password: "",
        confirmPassword: "",
    },
    validators: {
        onSubmit: verifPassword,
    },
    onSubmit: async ({ value }) => {
        try {
            const user = await encryptUserIdentifier(value.password)
            await $fetch(`${config.public.apiBaseUrl}/auth/verif/identifier`, {
                method: "PATCH",
                headers: {
                    "Content-Type": "application/json"
                },
                body: user,
                credentials: "include"
            })
        } catch (error) {
            await errorHelper(error)
        } finally {
            toast.success("Password created successfully")
            await navigateTo("/dashboard")
        }
    }
})

// eslint-disable-next-line @typescript-eslint/no-explicit-any
function isInvalid(field: any) {
    return field.state.meta.isTouched && !field.state.meta.isValid
}
</script>
<template>
    <div class="fixed top-1/2 left-1/2 -translate-y-1/2 -translate-x-1/2 w-xl shadow-2xl">
        <Card class="w-full max-w-xl shadow-lg">
            <CardHeader>
                <CardTitle class="text-2xl">Creating Master Password</CardTitle>
                <CardDescription>
                    Hello <b class="text-primary">{{ userData?.username }}</b> this is the
                    last step, please create you master password, and please don't leave
                    this page until you have created your master password.
                </CardDescription>
            </CardHeader>
            <form class="space-y-6" @submit.prevent="form.handleSubmit">
                <CardContent class="flex flex-col gap-2">
                    <div class="bg-emerald-300 text-black p-2 rounded-md">
                        <p class="text-[0.850rem]">
                            Use a passphrase of 4+ random words (e.g., 'CorrectHorseBatteryStaple'). Mix uppercase,
                            lowercase, numbers, and symbols for better security.
                        </p>
                    </div>
                    <FieldGroup>
                        <form.Field name="password">
                            <template #default="{ field }">
                                <Field :data-invalid="isInvalid(field)">
                                    <FieldLabel :for="field.name">
                                        Password
                                    </FieldLabel>
                                    <div class="flex gap-2">
                                        <Input :id="field.name" :name="field.name" :model-value="field.state.value"
                                            :aria-invalid="isInvalid(field)" placeholder="Your password here..."
                                            autocomplete="off" :type="isPasswordViewed ? 'text' : 'password'"
                                            @blur="field.handleBlur" @input="field.handleChange($event.target.value)" />
                                        <Button size="icon" type="button" @click="isPasswordViewed = !isPasswordViewed"
                                            variant="outline">
                                            <Eye v-if="isPasswordViewed" />
                                            <EyeClosed v-else />
                                        </Button>
                                    </div>
                                    <FieldError v-if="isInvalid(field)" :errors="field.state.meta.errors" />
                                </Field>
                            </template>
                        </form.Field>
                        <form.Field name="confirmPassword">
                            <template #default="{ field }">
                                <Field :data-invalid="isInvalid(field)">
                                    <FieldLabel :for="field.name">
                                        Confirm Password
                                    </FieldLabel>
                                    <Input :id="field.name" :name="field.name" :model-value="field.state.value"
                                        :aria-invalid="isInvalid(field)" placeholder="Confirm your password here..."
                                        :type="isPasswordViewed ? 'text' : 'password'" autocomplete="off"
                                        @blur="field.handleBlur" @input="field.handleChange($event.target.value)" />

                                    <FieldError v-if="isInvalid(field)" :errors="field.state.meta.errors" />
                                </Field>
                            </template>
                        </form.Field>

                    </FieldGroup>
                </CardContent>
                <CardFooter class="flex flex-col justify-start items-start">
                    <Button type="submit">
                        Submit
                    </Button>
                </CardFooter>
            </form>
        </Card>
    </div>

</template>
