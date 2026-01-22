<script setup lang="ts">
import { Eye, EyeOff, LockKeyhole } from "lucide-vue-next";
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
} from "~/components/ui/dialog";
import { Button } from "~/components/ui/button";
import { Spinner } from "~/components/ui/spinner";
import { useForm } from "@tanstack/vue-form";
import { z } from "zod";
import { FieldError, FieldGroup, FormInput } from "~/components/ui/field";
import { InputGroupButton } from "./ui/input-group";

const props = defineProps<{
    userEmail: string;
}>();

const emit = defineEmits<{
    unlock: [password: string];
    logout: [];
}>();

const unlockSchema = z.object({
    password: z.string().min(8, "Password must be at least 8 characters long").max(64, "Password must be at most 64 characters long"),
});

const form = useForm({
    defaultValues: {
        password: "",
    },
    validators: {
        onSubmit: unlockSchema,
    },
    onSubmit: async ({ value }) => {
        emit("unlock", value.password);
    },
});

const showPassword = ref(false);

function isInvalid(field: any) {
    return field.state.meta.isTouched && !field.state.meta.isValid
}
</script>

<template>
    <Dialog :open="true">
        <DialogContent :show-close-button="false" class="sm:max-w-md">
            <DialogHeader class="flex flex-row items-center gap-3">
                <div class="w-12 h-12 rounded-full bg-primary/10 flex items-center justify-center shrink-0">
                    <LockKeyhole class="w-6 h-6 text-primary" />
                </div>
                <div class="text-left">
                    <DialogTitle>Unlock Your Vault</DialogTitle>
                    <DialogDescription>{{ userEmail }}</DialogDescription>
                </div>
            </DialogHeader>

            <p class="text-muted-foreground text-sm">
                Your session is still active, but we need your password to
                unlock your encrypted vault.
            </p>

            <form @submit.prevent="form.handleSubmit" class="space-y-4">
                <FieldGroup>
                    <form.Field name="password" v-slot="{ field }">
                        <FormInput :field="field" placeholder="Enter your password"
                            :type="showPassword ? 'text' : 'password'">
                            <template #icon>
                                <LockKeyhole />
                            </template>

                            <template #addon>
                                <InputGroupButton @click="showPassword = !showPassword" type="button">
                                    <Eye v-if="showPassword" />
                                    <EyeOff v-else />
                                </InputGroupButton>
                            </template>
                        </FormInput>
                        <FieldError v-if="isInvalid(field)" :errors="field.state.meta.errors" />
                    </form.Field>
                </FieldGroup>

                <DialogFooter class="space-x-2 sm:space-x-1">
                    <Button type="button" variant="outline" :disabled="form.state.isSubmitting" @click="emit('logout')">
                        Logout
                    </Button>
                    <Button type="submit" :disabled="form.state.isSubmitting">
                        <Spinner v-if="form.state.isSubmitting" />
                        <span v-if="form.state.isSubmitting">Unlocking...</span>
                        <span v-else>Unlock</span>
                    </Button>
                </DialogFooter>
            </form>

            <p class="text-xs text-muted-foreground text-center">
                🔒 Your password never leaves your device
            </p>
        </DialogContent>
    </Dialog>
</template>
