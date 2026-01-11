<script setup lang="ts">
import type { SuccessResponse } from '~/utils/model/response';
import type { User } from '~/utils/model/user';

const config = useRuntimeConfig();

const { data: userData, refresh } = useFetch<SuccessResponse<User>>(`${config.public.apiBaseUrl}/session/me`, {
    server: false,
    credentials: 'include',
})

provide("userData", computed(() => userData.value?.data ?? null));
provide("refreshUserData", refresh);
</script>

<template>
    <div>
        <slot />
    </div>
</template>
