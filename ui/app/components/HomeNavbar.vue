<script setup lang="ts">
import { Shield } from 'lucide-vue-next';
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger } from './ui/dialog';
import { Button } from './ui/button';
import { useForm } from '@tanstack/vue-form'
import { login as loginValidation, signUp } from '~/utils/validation/auth';
import { Field } from './ui/field';
import { Input } from './ui/input';
import type { signUpType } from '~/utils/model/auth';
import { errorHelper } from '~/lib/error-helper';
import type { SuccessResponse } from '~/utils/model/response';
import { toast } from 'vue-sonner';
import { useAuth } from '~/composable/auth';
import { Spinner } from './ui/spinner';

const config = useRuntimeConfig();
const {login, isLoading} = useAuth()
const signUpForm = useForm({
    defaultValues: {
        username: "",
        email: ""
    },
    validators: {
        onSubmit: signUp
    },
    onSubmit: async ({ value }) => {
        try {
            const res = await $fetch<SuccessResponse<signUpType>>(`${config.public.apiBaseUrl}/auth`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: value,
                credentials: "include",
            })

            if (res.message === 'verif_password') {
                toast.success('Account already created! Please create your master password.')
                await navigateTo('/verif/password')
            } else {
                toast.success('Verification OTP sent to your email!')
                await navigateTo('/verif/otp')
            }
        } catch (error) {
            await errorHelper(error)
        }
    },
})

const loginForm = useForm({
    defaultValues: {
        email: "",
        password: ""
    },
    validators: {
        onSubmit: loginValidation
    },
    onSubmit: async ({ value }) => {
        try {
            await login(value)
        } catch (error) {
            await errorHelper(error)
        }
    },
})

// eslint-disable-next-line @typescript-eslint/no-explicit-any
function isInvalid(field: any) {
    return field.state.meta.isTouched && !field.state.meta.isValid
}
</script>

<template>
    <header class="sticky top-0 z-50 w-full border-b ">
        <div class="flex h-16 items-center justify-between px-4 md:px-6">
            <NuxtLink class="flex items-center space-x-2" href="/">
                <Shield class="h-8 w-8 text-primary" />
                <span class="text-2xl font-bold">SecureVault</span>
            </NuxtLink>

            <div class="flex items-center space-x-3">
                <Dialog>
                    <DialogTrigger as-child>
                        <Button variant="ghost" size="sm">
                            Sign In
                        </Button>
                    </DialogTrigger>
                    <DialogContent class="sm:max-w-md">
                        <DialogHeader>
                            <DialogTitle>Welcome back</DialogTitle>
                            <DialogDescription>
                                Sign in to your SecureVault account
                            </DialogDescription>
                        </DialogHeader>
                        <form class="space-y-4" @submit.prevent="loginForm.handleSubmit">
                            <FieldGroup>
                                <loginForm.Field name="email">
                                    <template #default="{ field }">
                                        <Field :data-invalid="isInvalid(field)">
                                            <FieldLabel :for="field.name">
                                                Email
                                            </FieldLabel>
                                            <Input
:id="field.name" :name="field.name" :model-value="field.state.value"
                                                :aria-invalid="isInvalid(field)" placeholder="Your email here..."
                                                autocomplete="off" @blur="field.handleBlur"
                                                @input="field.handleChange($event.target.value)" />
                                            <FieldError v-if="isInvalid(field)" :errors="field.state.meta.errors" />
                                        </Field>
                                    </template>
                                </loginForm.Field>
                                <loginForm.Field name="password">
                                    <template #default="{ field }">
                                        <Field :data-invalid="isInvalid(field)">
                                            <FieldLabel :for="field.name">
                                                Password
                                            </FieldLabel>
                                            <Input
:id="field.name" :name="field.name" :model-value="field.state.value"
                                                :aria-invalid="isInvalid(field)" placeholder="Your password here..."
                                                autocomplete="off" type="password"
                                                @blur="field.handleBlur" @input="field.handleChange($event.target.value)"/>
                                            <FieldError v-if="isInvalid(field)" :errors="field.state.meta.errors" />
                                        </Field>
                                    </template>
                                </loginForm.Field>
                            </FieldGroup>

                            <div class="flex items-center justify-between">
                                <NuxtLink href="#" class="text-sm text-primary hover:text-primary/70">
                                    Forgot password?
                                </NuxtLink>
                            </div>
                            <Button type="submit" class="w-full bg-primary hover:bg-primary/70" :disabled="isLoading">
                                <Spinner v-if="isLoading"/>
                                Sign In
                            </Button>
                        </form>
                    </DialogContent>
                </Dialog>

                <Dialog>
                    <DialogTrigger as-child>
                        <Button size="sm" class="bg-primary hover:bg-primary/70 cursor-pointer">
                            Get Started
                        </Button>
                    </DialogTrigger>
                    <DialogContent class="sm:max-w-md">
                        <DialogHeader>
                            <DialogTitle>Create your account</DialogTitle>
                            <DialogDescription>
                                Start securing your passwords today
                            </DialogDescription>
                        </DialogHeader>
                        <form class="space-y-4" @submit.prevent="signUpForm.handleSubmit">
                            <FieldGroup>
                                <signUpForm.Field name="username">
                                    <template #default="{ field }">
                                        <Field :data-invalid="isInvalid(field)">
                                            <FieldLabel :for="field.name">
                                                Username
                                            </FieldLabel>
                                            <Input
:id="field.name" :name="field.name" :model-value="field.state.value"
                                                :aria-invalid="isInvalid(field)" placeholder="Your username here..."
                                                autocomplete="off" @blur="field.handleBlur"
                                                @input="field.handleChange($event.target.value)" />
                                            <FieldError v-if="isInvalid(field)" :errors="field.state.meta.errors" />
                                        </Field>
                                    </template>
                                </signUpForm.Field>
                                <signUpForm.Field name="email">
                                    <template #default="{ field }">
                                        <Field :data-invalid="isInvalid(field)">
                                            <FieldLabel :for="field.name">
                                                Email
                                            </FieldLabel>
                                            <Input
:id="field.name" :name="field.name" :model-value="field.state.value"
                                                :aria-invalid="isInvalid(field)" placeholder="Your email here..."
                                                autocomplete="off" @blur="field.handleBlur"
                                                @input="field.handleChange($event.target.value)" />
                                            <FieldError v-if="isInvalid(field)" :errors="field.state.meta.errors" />
                                        </Field>
                                    </template>
                                </signUpForm.Field>
                            </FieldGroup>
                            <Button type="submit" :disabled="isLoading">
                                <Spinner v-if="isLoading"/>
                                Submit
                            </Button>
                        </form>
                    </DialogContent>
                </Dialog>
            </div>
        </div>
    </header>

</template>